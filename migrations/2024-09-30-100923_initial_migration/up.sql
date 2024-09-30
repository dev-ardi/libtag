-- Your SQL goes here
CREATE TABLE files (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    path TEXT NOT NULL,
    name TEXT,
    author TEXT,
    artist TEXT,
    description TEXT,
    notes TEXT,
    created INT8,

    is_exif BOOLEAN,
    gps_altitude INTEGER,
    gps_latitude INTEGER,
    x_res INTEGER,
    y_res INTEGER,

    deleted BOOLEAN NOT NULL,

    updated_at INT8 DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(path)
	
);
CREATE INDEX idx_files_path ON files(path);

CREATE TABLE tags (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    parent INTEGER NOT NULL,
    name TEXT NOT NULL,
    description TEXT,
    created_at INT8 DEFAULT CURRENT_TIMESTAMP,
    updated_at INT8 DEFAULT CURRENT_TIMESTAMP,
    UNIQUE(name)
);
CREATE INDEX idx_tags_name ON tags(name);
CREATE UNIQUE INDEX idx_unique_tag_name ON tags(name);

CREATE TABLE tag_file (
    id INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    label_id INTEGER NOT NULL,
    tag_id INTEGER NOT NULL,
    created_at INT8 DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (label_id) REFERENCES files(id) ON DELETE CASCADE,
    FOREIGN KEY (tag_id) REFERENCES tags(id) ON DELETE CASCADE
);

