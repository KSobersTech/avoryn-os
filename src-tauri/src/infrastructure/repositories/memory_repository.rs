use std::{
    error::Error,
    fmt::{Display, Formatter},
};

use sqlx::{FromRow, SqlitePool};
use uuid::Uuid;

use crate::domain::{
    memory::{CreateMemoryInput, Memory, MemoryImportance, MemoryKind, UpdateMemoryInput},
    project::Workspace,
};

#[derive(Debug)]
pub enum MemoryRepositoryError {
    Database(sqlx::Error),
    InvalidStoredValue { field: &'static str, value: String },
}

impl Display for MemoryRepositoryError {
    fn fmt(&self, formatter: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Database(_) => {
                write!(formatter, "AVORYN could not access the memory database.")
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

impl Error for MemoryRepositoryError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        match self {
            Self::Database(error) => Some(error),
            Self::InvalidStoredValue { .. } => None,
        }
    }
}

impl From<sqlx::Error> for MemoryRepositoryError {
    fn from(error: sqlx::Error) -> Self {
        Self::Database(error)
    }
}

#[derive(Debug, FromRow)]
struct MemoryRow {
    id: String,
    project_id: Option<String>,
    title: String,
    content: String,
    workspace: String,
    memory_kind: String,
    importance: String,
    source: Option<String>,
    occurred_at: Option<String>,
    created_at: String,
    updated_at: String,
    deleted_at: Option<String>,
}

impl TryFrom<MemoryRow> for Memory {
    type Error = MemoryRepositoryError;

    fn try_from(row: MemoryRow) -> Result<Self, Self::Error> {
        let MemoryRow {
            id,
            project_id,
            title,
            content,
            workspace,
            memory_kind,
            importance,
            source,
            occurred_at,
            created_at,
            updated_at,
            deleted_at,
        } = row;

        Ok(Self {
            id,
            project_id,
            title,
            content,
            workspace: parse_workspace(&workspace)?,
            memory_kind: parse_memory_kind(&memory_kind)?,
            importance: parse_memory_importance(&importance)?,
            source,
            occurred_at,
            created_at,
            updated_at,
            deleted_at,
        })
    }
}

pub async fn create_memory(
    pool: &SqlitePool,
    input: CreateMemoryInput,
) -> Result<Memory, MemoryRepositoryError> {
    let CreateMemoryInput {
        project_id,
        title,
        content,
        workspace,
        memory_kind,
        importance,
        source,
        occurred_at,
    } = input;

    let memory_id = Uuid::new_v4().to_string();
    let audit_event_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();

    let normalized_title = title.trim().to_owned();

    let normalized_content = content.trim().to_owned();

    let normalized_source = source
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let mut transaction = pool.begin().await?;

    let memory_row = sqlx::query_as::<_, MemoryRow>(
        r#"
            INSERT INTO memories (
                id,
                project_id,
                title,
                content,
                workspace,
                memory_kind,
                importance,
                source,
                occurred_at,
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
                ?,
                ?,
                ?,
                ?,
                strftime(
                    '%Y-%m-%dT%H:%M:%fZ',
                    'now'
                ),
                strftime(
                    '%Y-%m-%dT%H:%M:%fZ',
                    'now'
                ),
                NULL
            )
            RETURNING
                id,
                project_id,
                title,
                content,
                workspace,
                memory_kind,
                importance,
                source,
                occurred_at,
                created_at,
                updated_at,
                deleted_at
            "#,
    )
    .bind(&memory_id)
    .bind(project_id)
    .bind(normalized_title)
    .bind(normalized_content)
    .bind(workspace_as_str(workspace))
    .bind(memory_kind_as_str(memory_kind))
    .bind(memory_importance_as_str(importance))
    .bind(normalized_source)
    .bind(occurred_at)
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
            'memory.created',
            'memory',
            ?,
            'success',
            strftime(
                '%Y-%m-%dT%H:%M:%fZ',
                'now'
            )
        )
        "#,
    )
    .bind(audit_event_id)
    .bind(request_id)
    .bind(&memory_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Memory::try_from(memory_row)
}

pub async fn list_memories(pool: &SqlitePool) -> Result<Vec<Memory>, MemoryRepositoryError> {
    let rows = sqlx::query_as::<_, MemoryRow>(
        r#"
            SELECT
                id,
                project_id,
                title,
                content,
                workspace,
                memory_kind,
                importance,
                source,
                occurred_at,
                created_at,
                updated_at,
                deleted_at
            FROM memories
            WHERE deleted_at IS NULL
            ORDER BY
                updated_at DESC,
                title COLLATE NOCASE ASC
            "#,
    )
    .fetch_all(pool)
    .await?;

    rows.into_iter().map(Memory::try_from).collect()
}

pub async fn update_memory(
    pool: &SqlitePool,
    memory_id: &str,
    input: UpdateMemoryInput,
) -> Result<Memory, MemoryRepositoryError> {
    let UpdateMemoryInput {
        project_id,
        title,
        content,
        workspace,
        memory_kind,
        importance,
        source,
        occurred_at,
    } = input;

    let normalized_title = title.trim().to_owned();

    let normalized_content = content.trim().to_owned();

    let normalized_source = source
        .map(|value| value.trim().to_owned())
        .filter(|value| !value.is_empty());

    let audit_event_id = Uuid::new_v4().to_string();
    let request_id = Uuid::new_v4().to_string();

    let mut transaction = pool.begin().await?;

    let memory_row = sqlx::query_as::<_, MemoryRow>(
        r#"
            UPDATE memories
            SET
                project_id = ?,
                title = ?,
                content = ?,
                workspace = ?,
                memory_kind = ?,
                importance = ?,
                source = ?,
                occurred_at = ?,
                updated_at = strftime(
                    '%Y-%m-%dT%H:%M:%fZ',
                    'now'
                )
            WHERE id = ?
              AND deleted_at IS NULL
            RETURNING
                id,
                project_id,
                title,
                content,
                workspace,
                memory_kind,
                importance,
                source,
                occurred_at,
                created_at,
                updated_at,
                deleted_at
            "#,
    )
    .bind(project_id)
    .bind(normalized_title)
    .bind(normalized_content)
    .bind(workspace_as_str(workspace))
    .bind(memory_kind_as_str(memory_kind))
    .bind(memory_importance_as_str(importance))
    .bind(normalized_source)
    .bind(occurred_at)
    .bind(memory_id)
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
            'memory.updated',
            'memory',
            ?,
            'success',
            strftime(
                '%Y-%m-%dT%H:%M:%fZ',
                'now'
            )
        )
        "#,
    )
    .bind(audit_event_id)
    .bind(request_id)
    .bind(memory_id)
    .execute(&mut *transaction)
    .await?;

    transaction.commit().await?;

    Memory::try_from(memory_row)
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

fn memory_kind_as_str(memory_kind: MemoryKind) -> &'static str {
    match memory_kind {
        MemoryKind::Note => "note",
        MemoryKind::Fact => "fact",
        MemoryKind::Preference => "preference",
        MemoryKind::Lesson => "lesson",
        MemoryKind::Milestone => "milestone",
    }
}

fn memory_importance_as_str(importance: MemoryImportance) -> &'static str {
    match importance {
        MemoryImportance::Low => "low",
        MemoryImportance::Medium => "medium",
        MemoryImportance::High => "high",
        MemoryImportance::Critical => "critical",
    }
}

fn parse_workspace(value: &str) -> Result<Workspace, MemoryRepositoryError> {
    match value {
        "engineering" => Ok(Workspace::Engineering),
        "school" => Ok(Workspace::School),
        "career" => Ok(Workspace::Career),
        "life" => Ok(Workspace::Life),
        "system" => Ok(Workspace::System),

        _ => Err(invalid_value("workspace", value)),
    }
}

fn parse_memory_kind(value: &str) -> Result<MemoryKind, MemoryRepositoryError> {
    match value {
        "note" => Ok(MemoryKind::Note),
        "fact" => Ok(MemoryKind::Fact),
        "preference" => Ok(MemoryKind::Preference),
        "lesson" => Ok(MemoryKind::Lesson),
        "milestone" => Ok(MemoryKind::Milestone),

        _ => Err(invalid_value("memory_kind", value)),
    }
}

fn parse_memory_importance(value: &str) -> Result<MemoryImportance, MemoryRepositoryError> {
    match value {
        "low" => Ok(MemoryImportance::Low),
        "medium" => Ok(MemoryImportance::Medium),
        "high" => Ok(MemoryImportance::High),
        "critical" => Ok(MemoryImportance::Critical),

        _ => Err(invalid_value("importance", value)),
    }
}

fn invalid_value(field: &'static str, value: &str) -> MemoryRepositoryError {
    MemoryRepositoryError::InvalidStoredValue {
        field,
        value: value.to_owned(),
    }
}
