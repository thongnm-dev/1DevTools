//! Service cho module Docker Desktop.
//!
//! Toàn bộ thao tác được thực hiện bằng cách gọi `docker` CLI của hệ điều hành
//! (cùng cách tiếp cận với `git_service`) — tận dụng Docker context/credential sẵn
//! có của người dùng, không cần thêm dependency HTTP client nặng để nói chuyện với
//! Docker Engine API.

use std::io::Read;
use std::process::{Command, Stdio};
use std::sync::mpsc;

use crate::app::error::AppError;
use crate::app::result::AppResult;
use crate::models::docker::{DockerContainer, DockerImage};

/// Tạo `Command` docker đã cấu hình sẵn (ẩn cửa sổ console trên Windows).
fn docker_cmd() -> Command {
    let mut cmd = Command::new("docker");
    configure(&mut cmd);
    cmd
}

/// Cấu hình chung cho mọi lệnh docker chạy ngầm (không cần hiện cửa sổ).
fn configure(cmd: &mut Command) {
    #[cfg(target_os = "windows")]
    {
        use std::os::windows::process::CommandExt;
        const CREATE_NO_WINDOW: u32 = 0x0800_0000;
        cmd.creation_flags(CREATE_NO_WINDOW);
    }
}

/// Chạy một lệnh docker và trả về stdout (đã trim). Lỗi (exit code != 0) trả về
/// `AppError` chứa stderr (hoặc stdout nếu stderr rỗng).
fn run(args: &[&str]) -> AppResult<String> {
    let output = docker_cmd().args(args).output().map_err(|e| {
        AppError::new(format!(
            "Không chạy được docker: {e}. Hãy chắc chắn đã cài Docker và Docker đang chạy."
        ))
    })?;

    if output.status.success() {
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr).trim().to_string();
        let stdout = String::from_utf8_lossy(&output.stdout).trim().to_string();
        let msg = if !stderr.is_empty() { stderr } else { stdout };
        Err(AppError::new(if msg.is_empty() {
            "Lệnh docker thất bại.".to_string()
        } else {
            msg
        }))
    }
}

/// Kiểm tra Docker CLI có gọi được và daemon có đang chạy không (dùng cho banner cảnh báo).
pub fn is_available() -> bool {
    run(&["version", "--format", "{{.Server.Version}}"]).is_ok()
}

/// Tự khởi động Docker (khi banner báo "Docker is not running").
///
/// Chỉ "bắn và quên" (spawn không chờ) — khởi động Docker Desktop/daemon có thể
/// mất vài chục giây, nên FE tự poll lại `is_available()` sau đó để biết khi nào
/// sẵn sàng, không chờ tiến trình này thoát.
pub fn start_desktop() -> AppResult<()> {
    #[cfg(target_os = "windows")]
    {
        let program_files = std::env::var("ProgramFiles").unwrap_or_else(|_| r"C:\Program Files".to_string());
        let candidate = format!("{program_files}\\Docker\\Docker\\Docker Desktop.exe");
        if !std::path::Path::new(&candidate).exists() {
            return Err(AppError::new(format!(
                "Không tìm thấy Docker Desktop tại \"{candidate}\". Hãy mở Docker Desktop thủ công."
            )));
        }
        Command::new(&candidate)
            .spawn()
            .map_err(|e| AppError::new(format!("Không mở được Docker Desktop: {e}")))?;
        Ok(())
    }
    #[cfg(target_os = "macos")]
    {
        Command::new("open")
            .args(["-a", "Docker"])
            .spawn()
            .map_err(|e| AppError::new(format!("Không mở được Docker: {e}")))?;
        Ok(())
    }
    #[cfg(all(unix, not(target_os = "macos")))]
    {
        // Ưu tiên `pkexec` để hiện hộp thoại xin quyền admin bằng GUI (PolicyKit).
        // Fallback `systemctl` thẳng — chỉ thành công nếu tiến trình app đã có quyền
        // sẵn (vd. đang chạy dưới sudo), nếu không sẽ lỗi và FE báo người dùng tự chạy.
        if Command::new("pkexec")
            .args(["systemctl", "start", "docker"])
            .spawn()
            .is_err()
        {
            Command::new("systemctl")
                .args(["start", "docker"])
                .spawn()
                .map_err(|e| {
                    AppError::new(format!(
                        "Không khởi động được Docker: {e}. Hãy chạy `sudo systemctl start docker` thủ công."
                    ))
                })?;
        }
        Ok(())
    }
}

/// Lấy một field dạng chuỗi từ JSON object, mặc định rỗng nếu thiếu/không phải chuỗi.
fn field(v: &serde_json::Value, key: &str) -> String {
    v.get(key).and_then(|x| x.as_str()).unwrap_or_default().to_string()
}

/// Đọc giá trị của một label từ chuỗi `Labels` của `docker ps` (dạng
/// `key1=value1,key2=value2,...`). Trả về rỗng nếu không có label đó.
fn parse_label(labels: &str, key: &str) -> String {
    for pair in labels.split(',') {
        if let Some((k, v)) = pair.split_once('=') {
            if k == key {
                return v.to_string();
            }
        }
    }
    String::new()
}

/// Lấy % CPU + bộ nhớ đang dùng cho các container đang chạy (best-effort — lỗi ở
/// đây không nên làm hỏng cả danh sách container, chỉ thiếu 2 cột này).
/// Trả về map id đầy đủ → (cpu, memory).
fn container_stats() -> Vec<(String, String, String)> {
    let Ok(out) = run(&["stats", "--no-stream", "--no-trunc", "--format", "{{json .}}"]) else {
        return Vec::new();
    };
    let mut result = Vec::new();
    for line in out.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let Ok(v) = serde_json::from_str::<serde_json::Value>(line) else {
            continue;
        };
        let id = field(&v, "ID");
        if id.is_empty() {
            continue;
        }
        result.push((id, field(&v, "CPUPerc"), field(&v, "MemUsage")));
    }
    result
}

/// Danh sách container. `all = false` chỉ lấy container đang chạy (như `docker ps`),
/// `all = true` lấy cả container đã dừng (như `docker ps -a`).
pub fn list_containers(all: bool) -> AppResult<Vec<DockerContainer>> {
    let mut args: Vec<&str> = vec!["ps", "--no-trunc", "--format", "{{json .}}"];
    if all {
        args.insert(1, "-a");
    }
    let out = run(&args)?;
    let mut result = Vec::new();
    for line in out.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| AppError::new(format!("Không đọc được kết quả `docker ps`: {e}")))?;
        let labels = field(&v, "Labels");
        result.push(DockerContainer {
            id: field(&v, "ID"),
            name: field(&v, "Names"),
            image: field(&v, "Image"),
            command: field(&v, "Command"),
            status: field(&v, "Status"),
            state: field(&v, "State"),
            ports: field(&v, "Ports"),
            created: field(&v, "CreatedAt"),
            cpu: "—".to_string(),
            memory: "—".to_string(),
            compose_project: parse_label(&labels, "com.docker.compose.project"),
            compose_working_dir: parse_label(&labels, "com.docker.compose.project.working_dir"),
        });
    }

    // Chỉ gọi `docker stats` khi có ít nhất 1 container đang chạy — tránh gọi
    // không cần thiết (và các phiên bản docker cũ có thể treo chờ khi rỗng).
    if result.iter().any(|c| c.state == "running") {
        let stats = container_stats();
        for c in result.iter_mut() {
            if let Some((_, cpu, mem)) = stats.iter().find(|(id, _, _)| c.id.starts_with(id.as_str())) {
                c.cpu = cpu.clone();
                c.memory = mem.clone();
            }
        }
    }

    Ok(result)
}

/// Danh sách image (bỏ qua các layer trung gian ẩn danh — mặc định của `docker images`).
pub fn list_images() -> AppResult<Vec<DockerImage>> {
    let out = run(&["images", "--no-trunc", "--format", "{{json .}}"])?;
    let mut result = Vec::new();
    for line in out.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let v: serde_json::Value = serde_json::from_str(line)
            .map_err(|e| AppError::new(format!("Không đọc được kết quả `docker images`: {e}")))?;
        result.push(DockerImage {
            id: field(&v, "ID").trim_start_matches("sha256:").to_string(),
            repository: field(&v, "Repository"),
            tag: field(&v, "Tag"),
            created: field(&v, "CreatedSince"),
            size: field(&v, "Size"),
        });
    }
    Ok(result)
}

pub fn start_container(id: &str) -> AppResult<String> {
    run(&["start", id])
}

pub fn stop_container(id: &str) -> AppResult<String> {
    run(&["stop", id])
}

pub fn restart_container(id: &str) -> AppResult<String> {
    run(&["restart", id])
}

pub fn remove_container(id: &str, force: bool) -> AppResult<String> {
    if force {
        run(&["rm", "-f", id])
    } else {
        run(&["rm", id])
    }
}

pub fn remove_image(id: &str, force: bool) -> AppResult<String> {
    if force {
        run(&["rmi", "-f", id])
    } else {
        run(&["rmi", id])
    }
}

pub fn prune_containers() -> AppResult<String> {
    run(&["container", "prune", "-f"])
}

pub fn prune_images(dangling_only: bool) -> AppResult<String> {
    if dangling_only {
        run(&["image", "prune", "-f"])
    } else {
        run(&["image", "prune", "-a", "-f"])
    }
}

pub fn prune_system() -> AppResult<String> {
    run(&["system", "prune", "-f"])
}

/// Đọc từng dòng (tách theo `\n`, bỏ `\r`) từ một reader, gọi `on` cho mỗi dòng hoàn
/// chỉnh; phần còn lại chưa kết thúc bằng `\n` khi EOF cũng được coi là một dòng.
fn read_lines<R: Read, F: FnMut(String)>(reader: &mut R, mut on: F) {
    let mut buf = [0u8; 4096];
    let mut line = String::new();
    loop {
        match reader.read(&mut buf) {
            Ok(0) => break,
            Ok(n) => {
                let chunk = String::from_utf8_lossy(&buf[..n]);
                for ch in chunk.chars() {
                    if ch == '\n' {
                        on(std::mem::take(&mut line));
                    } else if ch != '\r' {
                        line.push(ch);
                    }
                }
            }
            Err(_) => break,
        }
    }
    if !line.is_empty() {
        on(line);
    }
}

/// Chạy một lệnh dài (build/compose), stream từng dòng stdout+stderr qua `on_line`
/// theo đúng thứ tự phát sinh (đọc song song 2 luồng, gộp qua `mpsc::channel`).
/// Trả về toàn bộ output khi thành công.
fn run_streamed<F: FnMut(String)>(mut cmd: Command, mut on_line: F) -> AppResult<String> {
    cmd.stdout(Stdio::piped()).stderr(Stdio::piped());
    let mut child = cmd.spawn().map_err(|e| {
        AppError::new(format!(
            "Không chạy được docker: {e}. Hãy chắc chắn đã cài Docker và Docker đang chạy."
        ))
    })?;

    let (tx, rx) = mpsc::channel::<String>();
    let mut handles = Vec::new();

    if let Some(mut stdout) = child.stdout.take() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            read_lines(&mut stdout, |l| {
                let _ = tx.send(l);
            });
        }));
    }
    if let Some(mut stderr) = child.stderr.take() {
        let tx = tx.clone();
        handles.push(std::thread::spawn(move || {
            read_lines(&mut stderr, |l| {
                let _ = tx.send(l);
            });
        }));
    }
    drop(tx);

    let mut all = String::new();
    for line in rx {
        all.push_str(&line);
        all.push('\n');
        on_line(line);
    }
    for h in handles {
        let _ = h.join();
    }

    let status = child
        .wait()
        .map_err(|e| AppError::new(format!("Lỗi chờ tiến trình docker: {e}")))?;

    if status.success() {
        Ok(all)
    } else if all.trim().is_empty() {
        Err(AppError::new("Lệnh docker thất bại."))
    } else {
        Err(AppError::new(all.trim().to_string()))
    }
}

/// Build một image từ Dockerfile, stream output ra `on_line`.
///
/// - `dockerfile` rỗng → dùng "Dockerfile" mặc định trong `context_path`.
/// - `tag` rỗng → build không gắn tag (`docker build` không kèm `-t`).
/// - `no_cache` → "Clean and build": bỏ qua toàn bộ build cache (`--no-cache`) và
///   luôn kéo lại base image mới nhất (`--pull`), thay vì rebuild tăng dần như
///   `docker build`/`up --build` mặc định (vẫn tái dùng layer cache cũ).
pub fn build_with_progress<F: FnMut(String)>(
    context_path: &str,
    dockerfile: &str,
    tag: &str,
    no_cache: bool,
    on_line: F,
) -> AppResult<String> {
    let mut cmd = docker_cmd();
    cmd.arg("build").arg("--progress=plain");
    if no_cache {
        cmd.arg("--no-cache").arg("--pull");
    }
    if !dockerfile.trim().is_empty() {
        cmd.arg("-f").arg(dockerfile);
    }
    if !tag.trim().is_empty() {
        cmd.arg("-t").arg(tag);
    }
    cmd.arg(context_path);
    run_streamed(cmd, on_line)
}

/// `docker compose -f <file> up -d [--build]`, stream output ra `on_line`.
///
/// `clean` → "Clean and build": chạy `docker compose build --no-cache --pull`
/// trước (bỏ qua cache, kéo lại base image mới nhất) rồi mới `up -d`, thay vì
/// dùng `up --build` (rebuild tăng dần, vẫn tái dùng cache cũ).
pub fn compose_up_with_progress<F: FnMut(String)>(
    compose_file: &str,
    clean: bool,
    mut on_line: F,
) -> AppResult<String> {
    let mut combined = String::new();

    if clean {
        let mut build_cmd = docker_cmd();
        build_cmd.args(["compose", "-f", compose_file, "build", "--no-cache", "--pull"]);
        combined.push_str(&run_streamed(build_cmd, |line| on_line(line))?);
        combined.push('\n');
    }

    let mut up_cmd = docker_cmd();
    up_cmd.args(["compose", "-f", compose_file, "up", "-d"]);
    if !clean {
        up_cmd.arg("--build");
    }
    combined.push_str(&run_streamed(up_cmd, |line| on_line(line))?);
    Ok(combined)
}

/// `docker compose -f <file> down`.
pub fn compose_down(compose_file: &str) -> AppResult<String> {
    run(&["compose", "-f", compose_file, "down"])
}
