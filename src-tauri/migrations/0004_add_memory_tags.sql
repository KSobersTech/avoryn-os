-- AVORYN Foundation
-- Shared tags and Memory tag relationships

CREATE TABLE tags (
    id TEXT PRIMARY KEY NOT NULL,

    name TEXT NOT NULL COLLATE NOCASE
        CHECK (
            length(trim(name)) BETWEEN 1 AND 64
            AND name = trim(name)
        ),

    created_at TEXT NOT NULL,
    updated_at TEXT NOT NULL,

    UNIQUE (name)
);


CREATE TABLE memory_tags (
    memory_id TEXT NOT NULL,
    tag_id TEXT NOT NULL,
    created_at TEXT NOT NULL,

    PRIMARY KEY (memory_id, tag_id),

    FOREIGN KEY (memory_id)
        REFERENCES memories(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE,

    FOREIGN KEY (tag_id)
        REFERENCES tags(id)
        ON UPDATE CASCADE
        ON DELETE CASCADE
);


-- Fast reverse lookup:
-- "show every Memory connected to this tag."

CREATE INDEX idx_memory_tags_tag
    ON memory_tags(tag_id, memory_id);


-- Do not allow a new tag relationship to be added
-- to a Memory that has already been archived.

CREATE TRIGGER prevent_tagging_deleted_memory
BEFORE INSERT ON memory_tags
FOR EACH ROW
WHEN EXISTS (
    SELECT 1
    FROM memories
    WHERE id = NEW.memory_id
      AND deleted_at IS NOT NULL
)
BEGIN
    SELECT RAISE(
        ABORT,
        'cannot tag archived memory'
    );
END;