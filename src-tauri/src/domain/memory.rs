use serde::{Deserialize, Serialize};

use crate::domain::project::Workspace;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryKind {
    Note,
    Fact,
    Preference,
    Lesson,
    Milestone,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum MemoryImportance {
    Low,
    Medium,
    High,
    Critical,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Memory {
    pub id: String,
    pub project_id: Option<String>,
    pub title: String,
    pub content: String,
    pub workspace: Workspace,
    pub memory_kind: MemoryKind,
    pub importance: MemoryImportance,
    pub source: Option<String>,
    pub occurred_at: Option<String>,
    pub created_at: String,
    pub updated_at: String,
    pub deleted_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMemoryInput {
    pub project_id: Option<String>,
    pub title: String,
    pub content: String,
    pub workspace: Workspace,
    pub memory_kind: MemoryKind,
    pub importance: MemoryImportance,
    pub source: Option<String>,
    pub occurred_at: Option<String>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateMemoryInput {
    pub project_id: Option<String>,
    pub title: String,
    pub content: String,
    pub workspace: Workspace,
    pub memory_kind: MemoryKind,
    pub importance: MemoryImportance,
    pub source: Option<String>,
    pub occurred_at: Option<String>,
}
