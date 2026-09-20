export type Workspace =
    | "engineering"
    | "school"
    | "career"
    | "life"
    | "system";

export type MemoryKind =
    | "note"
    | "fact"
    | "preference"
    | "lesson"
    | "milestone";

export type MemoryImportance =
    | "low"
    | "medium"
    | "high"
    | "critical";

export interface Memory {
    id: string;
    projectId: string | null;
    title: string;
    content: string;
    workspace: Workspace;
    memoryKind: MemoryKind;
    importance: MemoryImportance;
    source: string | null;
    occurredAt: string | null;
    createdAt: string;
    updatedAt: string;
    deletedAt: string | null;
}

export interface CreateMemoryInput {
    projectId: string | null;
    title: string;
    content: string;
    workspace: Workspace;
    memoryKind: MemoryKind;
    importance: MemoryImportance;
    source: string | null;
    occurredAt: string | null;
}

export type UpdateMemoryInput = CreateMemoryInput;
