use sqlx::SqlitePool;
use tauri::State;

use crate::application::error::AppError;
use crate::domain::memory::{CreateMemoryInput, Memory, UpdateMemoryInput};
use crate::infrastructure::repositories::memory_repository::{self, MemoryRepositoryError};

#[tauri::command]
pub async fn create_memory(
    database_pool: State<'_, SqlitePool>,
    input: CreateMemoryInput,
) -> Result<Memory, AppError> {
    validate_memory_input(&input.title, &input.content, input.source.as_deref())?;

    memory_repository::create_memory(database_pool.inner(), input)
        .await
        .map_err(map_repository_error)
}

#[tauri::command]
pub async fn list_memories(database_pool: State<'_, SqlitePool>) -> Result<Vec<Memory>, AppError> {
    memory_repository::list_memories(database_pool.inner())
        .await
        .map_err(map_repository_error)
}

#[tauri::command]
pub async fn update_memory(
    database_pool: State<'_, SqlitePool>,
    memory_id: String,
    input: UpdateMemoryInput,
) -> Result<Memory, AppError> {
    if memory_id.trim().is_empty() {
        return Err(AppError::validation("A memory ID is required."));
    }

    validate_memory_input(&input.title, &input.content, input.source.as_deref())?;

    memory_repository::update_memory(database_pool.inner(), &memory_id, input)
        .await
        .map_err(map_repository_error)
}

fn validate_memory_input(title: &str, content: &str, source: Option<&str>) -> Result<(), AppError> {
    let trimmed_title = title.trim();
    let trimmed_content = content.trim();

    if trimmed_title.is_empty() {
        return Err(AppError::validation("A memory title is required."));
    }

    if trimmed_title.chars().count() > 160 {
        return Err(AppError::validation(
            "A memory title cannot exceed 160 characters.",
        ));
    }

    if trimmed_content.is_empty() {
        return Err(AppError::validation("Memory content is required."));
    }

    if trimmed_content.chars().count() > 10_000 {
        return Err(AppError::validation(
            "Memory content cannot exceed 10,000 characters.",
        ));
    }

    if let Some(source) = source {
        if source.trim().chars().count() > 500 {
            return Err(AppError::validation(
                "A memory source cannot exceed 500 characters.",
            ));
        }
    }

    Ok(())
}

fn map_repository_error(error: MemoryRepositoryError) -> AppError {
    match error {
        MemoryRepositoryError::Database(sqlx::Error::RowNotFound) => AppError::not_found("Memory"),

        MemoryRepositoryError::Database(database_error) => {
            AppError::database(database_error.to_string())
        }

        MemoryRepositoryError::InvalidStoredValue { field, value } => {
            AppError::internal(format!("Invalid stored memory value for {field}: {value}"))
        }
    }
}
