use rusqlite::{self, Connection, params};
use std::error::Error;
use std::usize;
use uuid::{self, Uuid};

use crate::internal::Folder;
use crate::internal::Tab;
use crate::internal::Workspace;
use crate::util;
use crate::zen::translator;

fn create_tab(
    conn: &Connection,
    tab: &Tab,
    workspace_id: &String,
    folder_parent_uuid: Option<&String>,
) -> Result<usize, Box<dyn Error>> {
    let now = util::get_unix_epoch();
    let tab_id = Uuid::new_v4().to_string();
    let res = conn.execute(
        "INSERT INTO zen_pins (
            uuid, title, url, workspace_uuid, created_at, updated_at, folder_parent_uuid
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7
        )",
        params![
            tab_id,
            tab.title,
            tab.link,
            workspace_id,
            now,
            now,
            folder_parent_uuid
        ],
    )?;
    Ok(res)
}

fn create_folder(
    conn: &Connection,
    folder: &Folder,
    workspace_id: &String,
    folder_parent_uuid: Option<&String>,
) -> Result<usize, Box<dyn Error>> {
    let now = util::get_unix_epoch();
    let folder_id = Uuid::new_v4().to_string();
    let is_folder_collapsed = true;
    let is_group = true;
    let res = conn.execute(
        "INSERT INTO zen_pins (
            uuid, title, workspace_uuid, is_group, created_at, updated_at, folder_parent_uuid, is_folder_collapsed
        ) VALUES (
            ?1, ?2, ?3, ?4, ?5, ?6, ?7, ?8
        )",
        params![
            folder_id,
            folder.name,
            workspace_id,
            is_group,
            now,
            now,
            folder_parent_uuid,
            is_folder_collapsed
        ],
    )?;
    for tab in &folder.tabs {
        create_tab(&conn, &tab, workspace_id, Some(&folder_id))?;
    }
    for folder in &folder.folders {
        create_folder(&conn, &folder, &workspace_id, Some(&folder_id))?;
    }
    Ok(res)
}

pub fn sidebar_data(conn: &Connection, workspaces: &Vec<Workspace>) -> Result<(), Box<dyn Error>> {
    for workspace in workspaces {
        let now = util::get_unix_epoch();
        let workspace_id = Uuid::new_v4().to_string();
        conn.execute(
            "INSERT INTO zen_workspaces (uuid, name, icon, created_at, updated_at) VALUES (?1, ?2, ?3, ?4, ?5)",
            params![workspace_id, workspace.title, translator::create_icon_name(&workspace.icon),  now, now],
        )?;
        for tab in &workspace.tabs {
            create_tab(conn, &tab, &workspace_id, None)?;
        }
        for folder in &workspace.folders {
            create_folder(conn, &folder, &workspace_id, None)?;
        }
    }

    Ok(())
}
