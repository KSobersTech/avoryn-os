use sqlx::SqlitePool;
use tauri::State;

use crate::domain::project::{CreateProjectInput, Project};
use crate::infrastructure::repositories::project_repository;

#[tauri::command]
pub async fn create_project(
    database_pool: State<'_, SqlitePool>,
    input: CreateProjectInput,
) -> Result<Project, String> {
    if input.name.trim().is_empty() {
        return Err("A project name is required.".to_string());
    }

    project_repository::create_project(database_pool.inner(), input)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_projects(database_pool: State<'_, SqlitePool>) -> Result<Vec<Project>, String> {
    project_repository::list_projects(database_pool.inner())
        .await
        .map_err(|error| error.to_string())
}
