-- AVORYN Foundation
-- Memories schema and audit expansion

CREATE TABLE memories (
    id TEXT PRIMARY KEY NOT NULL,

    project_id TEXT,

    title TEXT NOT NULL
        CHECK (length(trim(title)) BETWEEN 1 AND 160),

    content TEXT NOT NULL
        CHECK (length(trim(content)) BETWEEN 1 AND 10000),

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

    memory_kind TEXT NOT NULL
        CHECK (
            memory_kind IN (
                'note',
                'fact',
                'preference',
                'lesson',
                'milestone'
            )
        ),

    importance TEXT NOT NULL DEFAULT 'medium'
        CHECK (
            importance IN (
                'low',
                'medium',
                'high',
                'critical'
            )
        ),

    source TEXT
        CHECK (
            source IS NULL
            OR length(source) <= 500
        ),

    occurred_at TEXT,
    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,
    deleted_at TEXT,

    FOREIGN KEY (project_id)
        REFERENCES projects(id)
        ON UPDATE CASCADE
        ON DELETE RESTRICT
);


CREATE INDEX idx_memories_active_updated
    ON memories(deleted_at, updated_at DESC);

CREATE INDEX idx_memories_workspace
    ON memories(workspace)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_memories_kind
    ON memories(memory_kind)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_memories_importance
    ON memories(importance)
    WHERE deleted_at IS NULL;

CREATE INDEX idx_memories_project
    ON memories(project_id)
    WHERE deleted_at IS NULL
      AND project_id IS NOT NULL;


-- Rebuild audit_events so it can support
-- projects, tasks, and memories.

DROP TRIGGER IF EXISTS validate_audit_event_resource_insert;
DROP TRIGGER IF EXISTS validate_audit_event_resource_update;
DROP TRIGGER IF EXISTS prevent_audited_project_delete;
DROP TRIGGER IF EXISTS prevent_audited_task_delete;

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
                'task',
                'memory'
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
        OR
        (
            resource_type = 'memory'
            AND event_type IN (
                'memory.created',
                'memory.updated'
            )
        )
    )
);


-- Preserve all existing audit history.

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

        WHEN NEW.resource_type = 'memory'
             AND NOT EXISTS (
                 SELECT 1
                 FROM memories
                 WHERE id = NEW.resource_id
             )
        THEN RAISE(
            ABORT,
            'audit memory resource does not exist'
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

        WHEN NEW.resource_type = 'memory'
             AND NOT EXISTS (
                 SELECT 1
                 FROM memories
                 WHERE id = NEW.resource_id
             )
        THEN RAISE(
            ABORT,
            'audit memory resource does not exist'
        )
    END;
END;


-- Preserve ON DELETE RESTRICT behavior
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


CREATE TRIGGER prevent_audited_memory_delete
BEFORE DELETE ON memories
FOR EACH ROW
WHEN EXISTS (
    SELECT 1
    FROM audit_events
    WHERE resource_type = 'memory'
      AND resource_id = OLD.id
)
BEGIN
    SELECT RAISE(
        ABORT,
        'cannot delete memory with audit history'
    );
END;