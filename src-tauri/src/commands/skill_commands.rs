//! Tauri IPC commands cho thư viện Skill (CRUD).

use crate::app::error::{log_err, AppError, AppErrorPayload};
use crate::database::skill_store;
use crate::models::skill::{Skill, SkillRequest};

/// Danh sách toàn bộ skill, mới cập nhật gần nhất lên đầu.
#[tauri::command]
pub fn skill_list() -> Result<Vec<Skill>, AppErrorPayload> {
    let data = skill_store::load().map_err(log_err)?;
    let mut skills = data.skills;
    skills.sort_by(|a, b| b.updated_at.cmp(&a.updated_at));
    Ok(skills)
}

/// Tạo skill mới.
#[tauri::command]
pub fn skill_create(request: SkillRequest) -> Result<Skill, AppErrorPayload> {
    let mut data = skill_store::load().map_err(log_err)?;
    data.next_id += 1;
    let now = chrono::Local::now().to_rfc3339();
    let skill = Skill {
        id: data.next_id,
        name: request.name,
        description: request.description,
        icon: request.icon,
        category: request.category,
        instructions: request.instructions,
        tags: request.tags,
        created_at: now.clone(),
        updated_at: now,
    };
    data.skills.push(skill.clone());
    skill_store::save(&data).map_err(log_err)?;
    Ok(skill)
}

/// Cập nhật skill.
#[tauri::command]
pub fn skill_update(id: i64, request: SkillRequest) -> Result<Skill, AppErrorPayload> {
    let mut data = skill_store::load().map_err(log_err)?;
    let skill = data
        .skills
        .iter_mut()
        .find(|s| s.id == id)
        .ok_or_else(|| log_err(AppError::new(format!("Skill #{id} không tồn tại"))))?;
    skill.name = request.name;
    skill.description = request.description;
    skill.icon = request.icon;
    skill.category = request.category;
    skill.instructions = request.instructions;
    skill.tags = request.tags;
    skill.updated_at = chrono::Local::now().to_rfc3339();
    let result = skill.clone();
    skill_store::save(&data).map_err(log_err)?;
    Ok(result)
}

/// Xoá skill.
#[tauri::command]
pub fn skill_delete(id: i64) -> Result<(), AppErrorPayload> {
    let mut data = skill_store::load().map_err(log_err)?;
    data.skills.retain(|s| s.id != id);
    skill_store::save(&data).map_err(log_err)?;
    Ok(())
}
