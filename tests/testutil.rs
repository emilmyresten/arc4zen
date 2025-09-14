use rusqlite::{self, Connection, params};

pub fn setup_zen_db(conn: &Connection) {
    conn.execute(
        "CREATE TABLE zen_workspaces (
          id INTEGER PRIMARY KEY,
          uuid TEXT UNIQUE NOT NULL,
          name TEXT NOT NULL,
          icon TEXT,
          container_id INTEGER,
          position INTEGER NOT NULL DEFAULT 0,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          theme_type TEXT,
          theme_colors TEXT,
          theme_opacity REAL,
          theme_rotation INTEGER,
          theme_texture REAL
        );",
        params![],
    )
    .unwrap();
    conn.execute(
        "CREATE TABLE zen_pins (
          id INTEGER PRIMARY KEY,
          uuid TEXT UNIQUE NOT NULL,
          title TEXT NOT NULL,
          url TEXT,
          container_id INTEGER,
          workspace_uuid TEXT,
          position INTEGER NOT NULL DEFAULT 0,
          is_essential BOOLEAN NOT NULL DEFAULT 0,
          is_group BOOLEAN NOT NULL DEFAULT 0,
          created_at INTEGER NOT NULL,
          updated_at INTEGER NOT NULL,
          edited_title BOOLEAN NOT NULL DEFAULT 0,
          is_folder_collapsed BOOLEAN NOT NULL DEFAULT 0,
          folder_icon TEXT DEFAULT NULL,
          folder_parent_uuid TEXT DEFAULT NULL
        );",
        params![],
    )
    .unwrap();
}
