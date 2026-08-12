//! Service phát hiện lệnh phát triển từ các file đặc trưng trong thư mục project.

use std::path::Path;

use crate::models::dev_runner::{CommandCategory, CommandSource, DevCommand};

/// Quét thư mục `repo_path` và trả về danh sách lệnh phát triển phát hiện được.
pub fn detect_commands(repo_path: &str) -> Vec<DevCommand> {
    let root = Path::new(repo_path);
    let mut commands = Vec::new();

    detect_npm(root, &mut commands);
    detect_flutter(root, &mut commands);
    detect_maven(root, &mut commands);
    detect_gradle(root, &mut commands);
    detect_cargo(root, &mut commands);
    detect_go(root, &mut commands);
    detect_python(root, &mut commands);
    detect_dotnet(root, &mut commands);
    detect_make(root, &mut commands);
    detect_docker(root, &mut commands);

    commands
}

fn make_id(category: &CommandCategory, label: &str) -> String {
    format!("{:?}:{}", category, label).to_lowercase()
}

fn auto_cmd(label: &str, command: &str, category: CommandCategory, source_file: &str) -> DevCommand {
    DevCommand {
        id: make_id(&category, label),
        label: label.to_string(),
        command: command.to_string(),
        category,
        source: CommandSource::Auto,
        source_file: source_file.to_string(),
    }
}

// ─── Node.js (package.json) ───

fn detect_npm(root: &Path, out: &mut Vec<DevCommand>) {
    let pkg = root.join("package.json");
    if !pkg.exists() {
        return;
    }
    let Ok(content) = std::fs::read_to_string(&pkg) else { return };
    let Ok(v) = serde_json::from_str::<serde_json::Value>(&content) else { return };

    let Some(scripts) = v.get("scripts").and_then(|s| s.as_object()) else { return };

    let manager = if root.join("pnpm-lock.yaml").exists() {
        "pnpm"
    } else if root.join("yarn.lock").exists() {
        "yarn"
    } else if root.join("bun.lockb").exists() || root.join("bun.lock").exists() {
        "bun"
    } else {
        "npm"
    };

    let priority = ["dev", "start", "serve", "build", "test", "lint", "preview", "watch", "typecheck", "storybook"];

    let mut sorted: Vec<(&String, &serde_json::Value)> = scripts.iter().collect();
    sorted.sort_by(|(a, _), (b, _)| {
        let ia = priority.iter().position(|p| a.contains(p)).unwrap_or(usize::MAX);
        let ib = priority.iter().position(|p| b.contains(p)).unwrap_or(usize::MAX);
        ia.cmp(&ib).then(a.cmp(b))
    });

    for (name, _) in sorted {
        let cmd = format!("{manager} run {name}");
        out.push(auto_cmd(name, &cmd, CommandCategory::Npm, "package.json"));
    }
}

// ─── Flutter (pubspec.yaml) ───

fn detect_flutter(root: &Path, out: &mut Vec<DevCommand>) {
    if !root.join("pubspec.yaml").exists() {
        return;
    }
    out.push(auto_cmd("flutter run", "flutter run", CommandCategory::Flutter, "pubspec.yaml"));
    out.push(auto_cmd("flutter run (release)", "flutter run --release", CommandCategory::Flutter, "pubspec.yaml"));
    out.push(auto_cmd("flutter build apk", "flutter build apk", CommandCategory::Flutter, "pubspec.yaml"));
    out.push(auto_cmd("flutter build ios", "flutter build ios", CommandCategory::Flutter, "pubspec.yaml"));
    out.push(auto_cmd("flutter test", "flutter test", CommandCategory::Flutter, "pubspec.yaml"));
    out.push(auto_cmd("flutter pub get", "flutter pub get", CommandCategory::Flutter, "pubspec.yaml"));
    out.push(auto_cmd("dart analyze", "dart analyze", CommandCategory::Flutter, "pubspec.yaml"));
}

// ─── Maven (pom.xml) ───

fn detect_maven(root: &Path, out: &mut Vec<DevCommand>) {
    if !root.join("pom.xml").exists() {
        return;
    }
    let mvn = if root.join("mvnw").exists() || root.join("mvnw.cmd").exists() {
        "./mvnw"
    } else {
        "mvn"
    };
    out.push(auto_cmd("spring-boot:run", &format!("{mvn} spring-boot:run"), CommandCategory::Maven, "pom.xml"));
    out.push(auto_cmd("clean install", &format!("{mvn} clean install"), CommandCategory::Maven, "pom.xml"));
    out.push(auto_cmd("clean package", &format!("{mvn} clean package"), CommandCategory::Maven, "pom.xml"));
    out.push(auto_cmd("test", &format!("{mvn} test"), CommandCategory::Maven, "pom.xml"));
    out.push(auto_cmd("compile", &format!("{mvn} compile"), CommandCategory::Maven, "pom.xml"));
}

// ─── Gradle (build.gradle / build.gradle.kts) ───

fn detect_gradle(root: &Path, out: &mut Vec<DevCommand>) {
    if !root.join("build.gradle").exists() && !root.join("build.gradle.kts").exists() {
        return;
    }
    let gradle = if root.join("gradlew").exists() || root.join("gradlew.bat").exists() {
        "./gradlew"
    } else {
        "gradle"
    };
    let source = if root.join("build.gradle.kts").exists() { "build.gradle.kts" } else { "build.gradle" };
    out.push(auto_cmd("bootRun", &format!("{gradle} bootRun"), CommandCategory::Gradle, source));
    out.push(auto_cmd("build", &format!("{gradle} build"), CommandCategory::Gradle, source));
    out.push(auto_cmd("clean build", &format!("{gradle} clean build"), CommandCategory::Gradle, source));
    out.push(auto_cmd("test", &format!("{gradle} test"), CommandCategory::Gradle, source));
}

// ─── Cargo (Cargo.toml) ───

fn detect_cargo(root: &Path, out: &mut Vec<DevCommand>) {
    if !root.join("Cargo.toml").exists() {
        return;
    }
    out.push(auto_cmd("cargo run", "cargo run", CommandCategory::Cargo, "Cargo.toml"));
    out.push(auto_cmd("cargo build", "cargo build", CommandCategory::Cargo, "Cargo.toml"));
    out.push(auto_cmd("cargo test", "cargo test", CommandCategory::Cargo, "Cargo.toml"));
    out.push(auto_cmd("cargo check", "cargo check", CommandCategory::Cargo, "Cargo.toml"));
    out.push(auto_cmd("cargo clippy", "cargo clippy", CommandCategory::Cargo, "Cargo.toml"));

    // Tauri project
    if root.join("src-tauri").exists() || root.join("tauri.conf.json").exists() {
        out.push(auto_cmd("tauri dev", "cargo tauri dev", CommandCategory::Cargo, "Cargo.toml"));
        out.push(auto_cmd("tauri build", "cargo tauri build", CommandCategory::Cargo, "Cargo.toml"));
    }
}

// ─── Go (go.mod) ───

fn detect_go(root: &Path, out: &mut Vec<DevCommand>) {
    if !root.join("go.mod").exists() {
        return;
    }
    out.push(auto_cmd("go run .", "go run .", CommandCategory::Go, "go.mod"));
    out.push(auto_cmd("go build", "go build ./...", CommandCategory::Go, "go.mod"));
    out.push(auto_cmd("go test", "go test ./...", CommandCategory::Go, "go.mod"));
}

// ─── Python (pyproject.toml / requirements.txt / manage.py) ───

fn detect_python(root: &Path, out: &mut Vec<DevCommand>) {
    let has_pyproject = root.join("pyproject.toml").exists();
    let has_requirements = root.join("requirements.txt").exists();
    let has_manage = root.join("manage.py").exists();

    if !has_pyproject && !has_requirements && !has_manage {
        return;
    }

    let source = if has_pyproject { "pyproject.toml" } else { "requirements.txt" };

    if has_manage {
        out.push(auto_cmd("django runserver", "python manage.py runserver", CommandCategory::Python, "manage.py"));
        out.push(auto_cmd("django migrate", "python manage.py migrate", CommandCategory::Python, "manage.py"));
        out.push(auto_cmd("django test", "python manage.py test", CommandCategory::Python, "manage.py"));
    }

    if has_pyproject {
        out.push(auto_cmd("pip install -e .", "pip install -e .", CommandCategory::Python, source));
    }

    out.push(auto_cmd("pytest", "pytest", CommandCategory::Python, source));
}

// ─── .NET (*.csproj / *.sln) ───

fn detect_dotnet(root: &Path, out: &mut Vec<DevCommand>) {
    let has_csproj = std::fs::read_dir(root)
        .ok()
        .map(|entries| entries.filter_map(|e| e.ok()).any(|e| {
            let name = e.file_name().to_string_lossy().to_string();
            name.ends_with(".csproj") || name.ends_with(".sln")
        }))
        .unwrap_or(false);

    if !has_csproj {
        return;
    }
    out.push(auto_cmd("dotnet run", "dotnet run", CommandCategory::Dotnet, "*.csproj"));
    out.push(auto_cmd("dotnet build", "dotnet build", CommandCategory::Dotnet, "*.csproj"));
    out.push(auto_cmd("dotnet test", "dotnet test", CommandCategory::Dotnet, "*.csproj"));
    out.push(auto_cmd("dotnet watch run", "dotnet watch run", CommandCategory::Dotnet, "*.csproj"));
}

// ─── Makefile ───

fn detect_make(root: &Path, out: &mut Vec<DevCommand>) {
    let makefile = root.join("Makefile");
    if !makefile.exists() {
        return;
    }
    let Ok(content) = std::fs::read_to_string(&makefile) else { return };

    for line in content.lines() {
        if let Some(target) = line.strip_suffix(':') {
            let target = target.trim();
            if !target.is_empty() && !target.starts_with('.') && !target.contains(' ') && !target.contains('%') {
                out.push(auto_cmd(target, &format!("make {target}"), CommandCategory::Make, "Makefile"));
            }
        }
    }
}

// ─── Docker Compose ───

fn detect_docker(root: &Path, out: &mut Vec<DevCommand>) {
    let compose_files = ["docker-compose.yml", "docker-compose.yaml", "compose.yml", "compose.yaml"];
    let found = compose_files.iter().find(|f| root.join(f).exists());
    let Some(file) = found else { return };

    out.push(auto_cmd("compose up", &format!("docker compose -f {file} up"), CommandCategory::Docker, file));
    out.push(auto_cmd("compose up -d", &format!("docker compose -f {file} up -d"), CommandCategory::Docker, file));
    out.push(auto_cmd("compose down", &format!("docker compose -f {file} down"), CommandCategory::Docker, file));
    out.push(auto_cmd("compose build", &format!("docker compose -f {file} build"), CommandCategory::Docker, file));
}
