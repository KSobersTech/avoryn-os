use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::domain::{
    project::{ProjectPriority, Workspace},
    task::{CreateTaskInput, Task, TaskStatus, UpdateTaskInput},
};

#[derive(Debug)]
pub enum TaskRepositoryError {
    Database(sqlx::Error),
    InvalidStoredValue { field: &'static str, value: String },
}

impl Display for TaskRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(_) => {
                write!(formatter, "AVORYN could not access the task database.")
            }
            Self::InvalidStoredValue { field, value } => {
                write!(
                    formatter,
                    "AVORYN found an unsupported value in {field}: {value}"
                )
            }
        }
    }
}

impl Error for TaskRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::InvalidStoredValue { .. } => None,
        }
    }
}

impl From<sqlx::Error> for TaskRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Debug, FromRow)]
struct TaskRow {
    id: String,
    project_id: Option<String>,
    title: String,
    description: Option<String>,
    workspace: String,
    status: String,
    priority: String,
    start_date: Option<String>,
    due_date: Option<String>,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl TryFrom<TaskRow> for Task {
    type Error = TaskRepositoryError;

    fn try_from(row: TaskRow) -> Result<Self, Self::Error> {
        let TaskRow {
            id,
            project_id,
            title,
            description,
            workspace,
            status,
            priority,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at,
        } = row;

        Ok(Self {
            id,
            project_id,
            title,
            description,
            workspace: parse_workspace(&workspace)?,
            status: parse_task_status(&status)?,
            priority: parse_priority(&priority)?,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}

pub async fn create_task(
    pool: &SqlitePool,
    input: CreateTaskInput,
) -> Result<Task, TaskRepositoryError> {
    let CreateTaskInput {
        project_id,
        title,
        description,
        workspace,
        priority,
        start_date,
        due_date,
    } = input;

    let task_id = Uuid::new_v4().to_string();
    let audit_event_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();

    let normalized_title = title.trim().to_owned();

    let normalized_description = description
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let normalized_project_id = project_id
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let mut transaction = pool.begin().await?;

    let task_row = sqlx::query_as::<_, TaskRow>(
        r#"
        INSERT INTO tasks (
            id,
            project_id,
            title,
            description,
            workspace,
            status,
            priority,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at
        )
        VALUES (
            ?,
            ?,
            ?,
            ?,
            ?,
            'todo',
            ?,
            ?,
            ?,
            NULL,
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now'),
            NULL
        )
        RETURNING
            id,
            project_id,
            title,
            description,
            workspace,
            status,
            priority,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at
        "#,
    )
    .bind(&task_id)
    .bind(normalized_project_id)
    .bind(normalized_title)
    .bind(normalized_description)
    .bind(workspace_as_str(workspace))
    .bind(priority_as_str(priority))
    .bind(start_date)
    .bind(due_date)
    .fetch_one(&mut *transaction)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO audit_events (
            id,
            request_id,
            event_type,
            resource_type,
            resource_id,
            outcome,
            created_at
        )
        VALUES (
            ?,
            ?,
            'task.created',
            'task',
            ?,
            'success',
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        )
        "#,
    )
    .bind(audit_event_id)
    .bind(request_id)
    .bind(&task_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Task::try_from(task_row)
}

pub async fn list_tasks(pool: &SqlitePool) -> Result<Vec<Task>, TaskRepositoryError> {
    let rows = sqlx::query_as::<_, TaskRow>(
        r#"
        SELECT
            id,
            project_id,
            title,
            description,
            workspace,
            status,
            priority,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at
        FROM tasks
        WHERE deleted_at IS NULL
        ORDER BY updated_at DESC, title COLLATE NOCASE ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Task::try_from).collect()
}

pub async fn update_task(
    pool: &SqlitePool,
    task_id: &str,
    input: UpdateTaskInput,
) -> Result<Task, TaskRepositoryError> {
    let UpdateTaskInput {
        project_id,
        title,
        description,
        workspace,
        status,
        priority,
        start_date,
        due_date,
    } = input;

    let normalized_title = title.trim().to_owned();

    let normalized_description = description
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let normalized_project_id = project_id
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let audit_event_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();

    let mut transaction = pool.begin().await?;

    let task_row = sqlx::query_as::<_, TaskRow>(
        r#"
        UPDATE tasks
        SET
            project_id = ?,
            title = ?,
            description = ?,
            workspace = ?,
            status = ?,
            priority = ?,
            start_date = ?,
            due_date = ?,
            completed_at = CASE
                WHEN ? = 'completed'
                    THEN COALESCE(
                        completed_at,
                        strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
                    )
                ELSE NULL
            END,
            updated_at = strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        WHERE id = ?
          AND deleted_at IS NULL
        RETURNING
            id,
            project_id,
            title,
            description,
            workspace,
            status,
            priority,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at
        "#,
    )
    .bind(normalized_project_id)
    .bind(normalized_title)
    .bind(normalized_description)
    .bind(workspace_as_str(workspace))
    .bind(task_status_as_str(status))
    .bind(priority_as_str(priority))
    .bind(start_date)
    .bind(due_date)
    .bind(task_status_as_str(status))
    .bind(task_id)
    .fetch_one(&mut *transaction)
    .await?;

    sqlx::query(
        r#"
        INSERT INTO audit_events (
            id,
            request_id,
            event_type,
            resource_type,
            resource_id,
            outcome,
            created_at
        )
        VALUES (
            ?,
            ?,
            'task.updated',
            'task',
            ?,
            'success',
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        )
        "#,
    )
    .bind(audit_event_id)
    .bind(request_id)
    .bind(task_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Task::try_from(task_row)
}

fn workspace_as_str(workspace: Workspace) -> &'static str {
    match workspace {
        Workspace::Engineering => "engineering",
        Workspace::School => "school",
        Workspace::Career => "career",
        Workspace::Life => "life",
        Workspace::System => "system",
    }
}

fn priority_as_str(priority: ProjectPriority) -> &'static str {
    match priority {
        ProjectPriority::Low => "low",
        ProjectPriority::Medium => "medium",
        ProjectPriority::High => "high",
        ProjectPriority::Critical => "critical",
    }
}

fn task_status_as_str(status: TaskStatus) -> &'static str {
    match status {
        TaskStatus::Todo => "todo",
        TaskStatus::InProgress => "in_progress",
        TaskStatus::Blocked => "blocked",
        TaskStatus::Completed => "completed",
        TaskStatus::Cancelled => "cancelled",
    }
}

fn parse_workspace(value: &str) -> Result<Workspace, TaskRepositoryError> {
    match value {
        "engineering" => Ok(Workspace::Engineering),
        "school" => Ok(Workspace::School),
        "career" => Ok(Workspace::Career),
        "life" => Ok(Workspace::Life),
        "system" => Ok(Workspace::System),
        _ => Err(invalid_value("workspace", value)),
    }
}

fn parse_task_status(value: &str) -> Result<TaskStatus, TaskRepositoryError> {
    match value {
        "todo" => Ok(TaskStatus::Todo),
        "in_progress" => Ok(TaskStatus::InProgress),
        "blocked" => Ok(TaskStatus::Blocked),
        "completed" => Ok(TaskStatus::Completed),
        "cancelled" => Ok(TaskStatus::Cancelled),
        _ => Err(invalid_value("status", value)),
    }
}

fn parse_priority(value: &str) -> Result<ProjectPriority, TaskRepositoryError> {
    match value {
        "low" => Ok(ProjectPriority::Low),
        "medium" => Ok(ProjectPriority::Medium),
        "high" => Ok(ProjectPriority::High),
        "critical" => Ok(ProjectPriority::Critical),
        _ => Err(invalid_value("priority", value)),
    }
}

fn invalid_value(field: &'static str, value: &str) -> TaskRepositoryError {
    TaskRepositoryError::InvalidStoredValue {
        field,
        value: value.to_owned(),
    }
}
