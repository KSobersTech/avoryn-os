use sqlx::SqlitePool;
use tauri::State;

use crate::domain::project::{CreateProjectInput, Project, UpdateProjectInput};
use crate::infrastructure::repositories::project_repository;

#[tauri::command]
pub async fn create_project(
    database_pool: State<'_, SqlitePool>,
    input: CreateProjectInput,
) -> Result<Project, String> {
    validate_project_name(&input.name)?;

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

#[tauri::command]
pub async fn update_project(
    database_pool: State<'_, SqlitePool>,
    project_id: String,
    input: UpdateProjectInput,
) -> Result<Project, String> {
    validate_project_name(&input.name)?;

    project_repository::update_project(database_pool.inner(), &project_id, input)
        .await
        .map_err(|error| error.to_string())
}

fn validate_project_name(name: &str) -> Result<(), String> {
    if name.trim().is_empty() {
        return Err("A project name is required.".to_string());
    }

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::validate_project_name;

    #[test]
    fn rejects_empty_name() {
        let result = validate_project_name("");

        assert_eq!(result, Err("A project name is required.".to_string()));
    }

    #[test]
    fn rejects_whitespace_only_names() {
        for name in ["   ", "\t\r\n", "\u{2003}"] {
            let result = validate_project_name(name);

            assert_eq!(
                result,
                Err("A project name is required.".to_string()),
                "expected rejection for {name:?}"
            );
        }
    }

    #[test]
    fn accepts_valid_name_with_surrounding_whitespace() {
        let name = String::from(" \tProject name\r\n ");

        let result = validate_project_name(&name);

        assert_eq!(result, Ok(()));
        assert_eq!(name, " \tProject name\r\n ");
    }
}
