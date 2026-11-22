export type Workspace =
    | "engineering"
    | "school"
    | "career"
    | "life"
    | "system";

export type ProjectKind =
    | "personal"
    | "client"
    | "portfolio"
    | "academic";

export type ProjectPriority =
    | "low"
    | "medium"
    | "high"
    | "critical";

export type Project = {
    id: string;
    name: string;
    description: string | null;
    workspace: Workspace;
    projectKind: ProjectKind;
    status: string;
    priority: ProjectPriority;
    startDate: string | null;
    dueDate: string | null;
    completedAt: string | null;
    createdAt: string;
    updatedAt: string;
    deletedAt: string | null;
};
