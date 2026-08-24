use serde::{Deserialize, Serialize};

use crate::domain::project::{ProjectPriority, Workspace};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TaskStatus {
    Todo,
    InProgress,
    Blocked,
    Completed,
    Cancelled,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Task {
    pub id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub workspace: Workspace,
    pub status: TaskStatus,
    pub priority: ProjectPriority,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
    pub completed_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateTaskInput {
    pub project_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub workspace: Workspace,
    pub priority: ProjectPriority,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateTaskInput {
    pub project_id: Option<String>,
    pub title: String,
    pub description: Option<String>,
    pub workspace: Workspace,
    pub status: TaskStatus,
    pub priority: ProjectPriority,
    pub start_date: Option<String>,
    pub due_date: Option<String>,
}
