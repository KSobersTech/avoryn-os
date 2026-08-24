use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::domain::project::{
    CreateProjectInput, Project, ProjectKind, ProjectPriority, ProjectStatus, UpdateProjectInput,
    Workspace,
};

#[derive(Debug)]
pub enum ProjectRepositoryError {
    Database(sqlx::Error),
    InvalidStoredValue { field: &'static str, value: String },
}

impl Display for ProjectRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(_) => {
                write!(formatter, "AVORYN could not access the project database.")
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

impl Error for ProjectRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::InvalidStoredValue { .. } => None,
        }
    }
}

impl From<sqlx::Error> for ProjectRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Debug, FromRow)]
struct ProjectRow {
    id: String,
    name: String,
    description: Option<String>,
    workspace: String,
    project_kind: String,
    status: String,
    priority: String,
    start_date: Option<String>,
    due_date: Option<String>,
    completed_at: Option<String>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl TryFrom<ProjectRow> for Project {
    type Error = ProjectRepositoryError;

    fn try_from(row: ProjectRow) -> Result<Self, Self::Error> {
        let ProjectRow {
            id,
            name,
            description,
            workspace,
            project_kind,
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
            name,
            description,
            workspace: parse_workspace(&workspace)?,
            project_kind: parse_project_kind(&project_kind)?,
            status: parse_project_status(&status)?,
            priority: parse_project_priority(&priority)?,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}

pub async fn create_project(
    pool: &SqlitePool,
    input: CreateProjectInput,
) -> Result<Project, ProjectRepositoryError> {
    let CreateProjectInput {
        name,
        description,
        workspace,
        project_kind,
        priority,
        start_date,
        due_date,
    } = input;

    let project_id = Uuid::new_v4().to_string();
    let audit_event_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();

    let normalized_name = name.trim().to_owned();
    let normalized_description = description
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let mut transaction = pool.begin().await?;

    let project_row = sqlx::query_as::<_, ProjectRow>(
        r#"
        INSERT INTO projects (
            id,
            name,
            description,
            workspace,
            project_kind,
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
            'planned',
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
            name,
            description,
            workspace,
            project_kind,
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
    .bind(&project_id)
    .bind(normalized_name)
    .bind(normalized_description)
    .bind(workspace_as_str(workspace))
    .bind(project_kind_as_str(project_kind))
    .bind(project_priority_as_str(priority))
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
            'project.created',
            'project',
            ?,
            'success',
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        )
        "#,
    )
    .bind(audit_event_id)
    .bind(request_id)
    .bind(&project_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Project::try_from(project_row)
}

pub async fn list_projects(pool: &SqlitePool) -> Result<Vec<Project>, ProjectRepositoryError> {
    let rows = sqlx::query_as::<_, ProjectRow>(
        r#"
        SELECT
            id,
            name,
            description,
            workspace,
            project_kind,
            status,
            priority,
            start_date,
            due_date,
            completed_at,
            created_at,
            updated_at,
            deleted_at
        FROM projects
        WHERE deleted_at IS NULL
        ORDER BY updated_at DESC, name COLLATE NOCASE ASC
        "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Project::try_from).collect()
}

pub async fn update_project(
    pool: &SqlitePool,
    project_id: &str,
    input: UpdateProjectInput,
) -> Result<Project, ProjectRepositoryError> {
    let UpdateProjectInput {
        name,
        description,
        workspace,
        project_kind,
        status,
        priority,
        start_date,
        due_date,
    } = input;

    let normalized_name = name.trim().to_owned();

    let normalized_description = description
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let audit_event_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();

    let mut transaction = pool.begin().await?;

    let project_row = sqlx::query_as::<_, ProjectRow>(
        r#"
        UPDATE projects
        SET
            name = ?,
            description = ?,
            workspace = ?,
            project_kind = ?,
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
            name,
            description,
            workspace,
            project_kind,
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
    .bind(normalized_name)
    .bind(normalized_description)
    .bind(workspace_as_str(workspace))
    .bind(project_kind_as_str(project_kind))
    .bind(project_status_as_str(status))
    .bind(project_priority_as_str(priority))
    .bind(start_date)
    .bind(due_date)
    .bind(project_status_as_str(status))
    .bind(project_id)
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
            'project.updated',
            'project',
            ?,
            'success',
            strftime('%Y-%m-%dT%H:%M:%fZ', 'now')
        )
        "#,
    )
    .bind(audit_event_id)
    .bind(request_id)
    .bind(project_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Project::try_from(project_row)
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

fn project_kind_as_str(project_kind: ProjectKind) -> &'static str {
    match project_kind {
        ProjectKind::Personal => "personal",
        ProjectKind::Client => "client",
        ProjectKind::Portfolio => "portfolio",
        ProjectKind::Academic => "academic",
    }
}

fn project_priority_as_str(priority: ProjectPriority) -> &'static str {
    match priority {
        ProjectPriority::Low => "low",
        ProjectPriority::Medium => "medium",
        ProjectPriority::High => "high",
        ProjectPriority::Critical => "critical",
    }
}
fn project_status_as_str(status: ProjectStatus) -> &'static str {
    match status {
        ProjectStatus::Planned => "planned",
        ProjectStatus::Active => "active",
        ProjectStatus::Paused => "paused",
        ProjectStatus::Completed => "completed",
        ProjectStatus::Archived => "archived",
    }
}
fn parse_workspace(value: &str) -> Result<Workspace, ProjectRepositoryError> {
    match value {
        "engineering" => Ok(Workspace::Engineering),
        "school" => Ok(Workspace::School),
        "career" => Ok(Workspace::Career),
        "life" => Ok(Workspace::Life),
        "system" => Ok(Workspace::System),
        _ => Err(invalid_value("workspace", value)),
    }
}

fn parse_project_kind(value: &str) -> Result<ProjectKind, ProjectRepositoryError> {
    match value {
        "personal" => Ok(ProjectKind::Personal),
        "client" => Ok(ProjectKind::Client),
        "portfolio" => Ok(ProjectKind::Portfolio),
        "academic" => Ok(ProjectKind::Academic),
        _ => Err(invalid_value("project_kind", value)),
    }
}

fn parse_project_status(value: &str) -> Result<ProjectStatus, ProjectRepositoryError> {
    match value {
        "planned" => Ok(ProjectStatus::Planned),
        "active" => Ok(ProjectStatus::Active),
        "paused" => Ok(ProjectStatus::Paused),
        "completed" => Ok(ProjectStatus::Completed),
        "archived" => Ok(ProjectStatus::Archived),
        _ => Err(invalid_value("status", value)),
    }
}

fn parse_project_priority(value: &str) -> Result<ProjectPriority, ProjectRepositoryError> {
    match value {
        "low" => Ok(ProjectPriority::Low),
        "medium" => Ok(ProjectPriority::Medium),
        "high" => Ok(ProjectPriority::High),
        "critical" => Ok(ProjectPriority::Critical),
        _ => Err(invalid_value("priority", value)),
    }
}

fn invalid_value(field: &'static str, value: &str) -> ProjectRepositoryError {
    ProjectRepositoryError::InvalidStoredValue {
        field,
        value: value.to_owned(),
    }
}
