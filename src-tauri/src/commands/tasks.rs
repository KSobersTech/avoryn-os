use sqlx::SqlitePool;
use tauri::State;

use crate::domain::task::{CreateTaskInput, Task, UpdateTaskInput};
use crate::infrastructure::repositories::task_repository;

#[tauri::command]
pub async fn create_task(
    database_pool: State<'_, SqlitePool>,
    input: CreateTaskInput,
) -> Result<Task, String> {
    if input.title.trim().is_empty() {
        return Err("A task title is required.".to_string());
    }

    task_repository::create_task(database_pool.inner(), input)
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn list_tasks(database_pool: State<'_, SqlitePool>) -> Result<Vec<Task>, String> {
    task_repository::list_tasks(database_pool.inner())
        .await
        .map_err(|error| error.to_string())
}

#[tauri::command]
pub async fn update_task(
    database_pool: State<'_, SqlitePool>,
    task_id: String,
    input: UpdateTaskInput,
) -> Result<Task, String> {
    if input.title.trim().is_empty() {
        return Err("A task title is required.".to_string());
    }

    task_repository::update_task(database_pool.inner(), &task_id, input)
        .await
        .map_err(|error| error.to_string())
}
