-- sqlite3 database initialization script

CREATE TABLE fashion (
    id TEXT PRIMARY KEY, -- UUIDv7
    name TEXT NOT NULL COLLATE NOCASE,
    description TEXT NOT NULL DEFAULT '',
    character TEXT NOT NULL COLLATE NOCASE DEFAULT '',
    wardrobe_template TEXT NOT NULL, -- GW2 base64 chat link
    travel_template TEXT NOT NULL, -- GW2 base64 chat link
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    UNIQUE (name, character)
) STRICT;

CREATE TABLE tag (
    id TEXT PRIMARY KEY, -- UUIDv7
    name TEXT NOT NULL UNIQUE COLLATE NOCASE,
    created_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now')),
    updated_at TEXT NOT NULL DEFAULT (strftime('%Y-%m-%dT%H:%M:%fZ', 'now'))
) STRICT;

CREATE TABLE fashion_tag (
    fashion_id TEXT NOT NULL REFERENCES fashion(id) ON DELETE CASCADE,
    tag_id TEXT NOT NULL REFERENCES tag(id) ON DELETE CASCADE,
    PRIMARY KEY (fashion_id, tag_id)
) STRICT, WITHOUT ROWID;

CREATE INDEX idx_fashion_tag_tag_id ON fashion_tag(tag_id);
