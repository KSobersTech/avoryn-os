-- AVORYN Phase 5
-- Initial Projects and Audit schema

CREATE TABLE projects (
    id TEXT PRIMARY KEY NOT NULL,

    name TEXT NOT NULL
        CHECK (length(trim(name)) BETWEEN 1 AND 120),

    description TEXT
        CHECK (
            description IS NULL
            OR length(description) <= 2000
        ),

    workspace TEXT NOT NULL
        CHECK (
            workspace IN (
                'engineering',
                'school',
                'career',
                'life',
                'system'
            )
        ),

    project_kind TEXT NOT NULL
        CHECK (
            project_kind IN (
                'personal',
                'client',
                'portfolio',
                'academic'
            )
        ),

    status TEXT NOT NULL DEFAULT 'planned'
        CHECK (
            status IN (
                'planned',
                'active',
                'paused',
                'completed',
                'archived'
            )
        ),

    priority TEXT NOT NULL DEFAULT 'medium'
        CHECK (
            priority IN (
                'low',
                'medium',
                'high',
                'critical'
            )
        ),

    start_date TEXT
        CHECK (
            start_date IS NULL
            OR (
                length(start_date) = 10
                AND start_date GLOB '????-??-??'
            )
        ),

    due_date TEXT
        CHECK (
            due_date IS NULL
            OR (
                length(due_date) = 10
                AND due_date GLOB '????-??-??'
            )
        ),

    completed_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,

    CHECK (
        start_date IS NULL
        OR due_date IS NULL
        OR due_date >= start_date
    )
);

CREATE TABLE audit_events (
    id TEXT PRIMARY KEY NOT NULL,
    request_id TEXT NOT NULL,

    event_type TEXT NOT NULL
        CHECK (
            event_type IN (
                'project.created',
                'project.updated'
            )
        ),

    resource_type TEXT NOT NULL
        CHECK (resource_type = 'project'),

    resource_id TEXT NOT NULL,

    outcome TEXT NOT NULL
        CHECK (
            outcome IN (
                'success',
                'failure'
            )
        ),

    created_at TEXT NOT NULL,

    FOREIGN KEY (resource_id)
        REFERENCES projects(id)
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

CREATE INDEX idx_projects_active_updated
    ON projects(deleted_at, updated_at DESC);

CREATE INDEX idx_projects_status
    ON projects(status)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_projects_workspace
    ON projects(workspace)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_audit_events_resource
    ON audit_events(resource_type, resource_id, created_at DESC);

CREATE INDEX idx_audit_events_request
    ON audit_events(request_id);