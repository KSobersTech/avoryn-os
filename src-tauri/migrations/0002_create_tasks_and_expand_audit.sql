-- AVORYN Foundation
-- Tasks schema and audit expansion

CREATE TABLE tasks (
    id TEXT PRIMARY KEY NOT NULL,

    project_id TEXT,

    title TEXT NOT NULL
        CHECK (length(trim(title)) BETWEEN 1 AND 160),

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

    status TEXT NOT NULL DEFAULT 'todo'
        CHECK (
            status IN (
                'todo',
                'in_progress',
                'blocked',
                'completed',
                'cancelled'
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
    ),

    FOREIGN KEY (project_id)
        REFERENCES projects(id)
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);

CREATE INDEX idx_tasks_active_updated
    ON tasks(deleted_at, updated_at DESC);

CREATE INDEX idx_tasks_status
    ON tasks(status)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_tasks_workspace
    ON tasks(workspace)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_tasks_project
    ON tasks(project_id)
    WHERE deleted_at IS NULL
      AND project_id IS NOT NULL;

CREATE INDEX idx_tasks_due_date
    ON tasks(due_date)
    WHERE deleted_at IS NULL
      AND due_date IS NOT NULL;


-- Rebuild audit_events so it can support
-- both projects and tasks.

ALTER TABLE audit_events
    RENAME TO audit_events_legacy;

CREATE TABLE audit_events (
    id TEXT PRIMARY KEY NOT NULL,

    request_id TEXT NOT NULL,

    event_type TEXT NOT NULL,

    resource_type TEXT NOT NULL
        CHECK (
            resource_type IN (
                'project',
                'task'
            )
        ),

    resource_id TEXT NOT NULL,

    outcome TEXT NOT NULL
        CHECK (
            outcome IN (
                'success',
                'failure'
            )
        ),

    created_at TEXT NOT NULL,

    CHECK (
        (
            resource_type = 'project'
            AND event_type IN (
                'project.created',
                'project.updated'
            )
        )
        OR
        (
            resource_type = 'task'
            AND event_type IN (
                'task.created',
                'task.updated'
            )
        )
    )
);


-- Preserve all existing project audit history.

INSERT INTO audit_events (
    id,
    request_id,
    event_type,
    resource_type,
    resource_id,
    outcome,
    created_at
)
SELECT
    id,
    request_id,
    event_type,
    resource_type,
    resource_id,
    outcome,
    created_at
FROM audit_events_legacy;

DROP TABLE audit_events_legacy;


-- Restore audit indexes.

CREATE INDEX idx_audit_events_resource
    ON audit_events(
        resource_type,
        resource_id,
        created_at DESC
    );

CREATE INDEX idx_audit_events_request
    ON audit_events(request_id);


-- Validate polymorphic audit references.
-- Project audit events must reference a real project.
-- Task audit events must reference a real task.

CREATE TRIGGER validate_audit_event_resource_insert
BEFORE INSERT ON audit_events
FOR EACH ROW
BEGIN
    SELECT CASE
        WHEN NEW.resource_type = 'project'
             AND NOT EXISTS (
                 SELECT 1
                 FROM projects
                 WHERE id = NEW.resource_id
             )
        THEN RAISE(
            ABORT,
            'audit project resource does not exist'
        )

        WHEN NEW.resource_type = 'task'
             AND NOT EXISTS (
                 SELECT 1
                 FROM tasks
                 WHERE id = NEW.resource_id
             )
        THEN RAISE(
            ABORT,
            'audit task resource does not exist'
        )
    END;
END;


CREATE TRIGGER validate_audit_event_resource_update
BEFORE UPDATE OF resource_type, resource_id
ON audit_events
FOR EACH ROW
BEGIN
    SELECT CASE
        WHEN NEW.resource_type = 'project'
             AND NOT EXISTS (
                 SELECT 1
                 FROM projects
                 WHERE id = NEW.resource_id
             )
        THEN RAISE(
            ABORT,
            'audit project resource does not exist'
        )

        WHEN NEW.resource_type = 'task'
             AND NOT EXISTS (
                 SELECT 1
                 FROM tasks
                 WHERE id = NEW.resource_id
             )
        THEN RAISE(
            ABORT,
            'audit task resource does not exist'
        )
    END;
END;


-- Preserve the old ON DELETE RESTRICT behavior
-- for audited resources.

CREATE TRIGGER prevent_audited_project_delete
BEFORE DELETE ON projects
FOR EACH ROW
WHEN EXISTS (
    SELECT 1
    FROM audit_events
    WHERE resource_type = 'project'
      AND resource_id = OLD.id
)
BEGIN
    SELECT RAISE(
        ABORT,
        'cannot delete project with audit history'
    );
END;


CREATE TRIGGER prevent_audited_task_delete
BEFORE DELETE ON tasks
FOR EACH ROW
WHEN EXISTS (
    SELECT 1
    FROM audit_events
    WHERE resource_type = 'task'
      AND resource_id = OLD.id
)
BEGIN
    SELECT RAISE(
        ABORT,
        'cannot delete task with audit history'
    );
END;