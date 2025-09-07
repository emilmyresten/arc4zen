use serde_json;

use crate::internal::Folder;
use crate::internal::Tab;
use crate::internal::Workspace;

fn get_containers(json_data: &serde_json::Value) -> Result<&serde_json::Value, String> {
    let sidebar_containers: &Vec<serde_json::Value> = match &json_data["sidebar"]["containers"] {
        serde_json::Value::Array(v) => v,
        _ => return Err("No containers found".to_string()),
    };
    let containers = sidebar_containers
        .iter()
        .find(|e| match e {
            serde_json::Value::Object(o) => o.contains_key("spaces") && o.contains_key("items"),
            _ => false,
        })
        .expect("Couldn't find object containing spaces");
    Ok(containers)
}

fn get_tabs(items: &Vec<&serde_json::Value>, container_id: &String) -> Vec<Tab> {
    let tabs: Vec<Tab> = items
        .iter()
        .filter(|i| {
            let children_ids = match &i["childrenIds"] {
                serde_json::Value::Array(v) => v,
                _ => panic!("childrenIds tag not present"),
            };

            i["parentID"] == *container_id && children_ids.is_empty()
        })
        .map(|i| {
            let title;
            let custom_title = i["title"].to_string().trim_matches('"').to_string();
            if custom_title != "null" {
                title = custom_title
            } else {
                title = i["data"]["tab"]["savedURL"]
                    .to_string()
                    .trim_matches('"')
                    .to_string();
            }

            return Tab {
                title: title,
                link: i["data"]["tab"]["savedURL"]
                    .to_string()
                    .trim_matches('"')
                    .to_string(),
            };
        })
        .collect();
    tabs
}
fn get_folders(items: &Vec<&serde_json::Value>, pinned_container_id: &String) -> Vec<Folder> {
    // Folders are items whose parent is the workspace id, and that contains children.
    // this functions is recursive to traverse the flat list of items, resolving every folder
    // The base-case is not apparent, but it will stop once a folder does not contain any nested folders,
    // As thd recursive call is not executed on a map on an empty list.
    let folders: Vec<Folder> = items
        .iter()
        .filter(|i| {
            let children_ids = match &i["childrenIds"] {
                serde_json::Value::Array(v) => v,
                _ => panic!("childrenIds tag not present"),
            };

            i["parentID"] == *pinned_container_id && !children_ids.is_empty()
        })
        .map(|i| Folder {
            id: i["id"].to_string().trim_matches('"').to_string(),
            name: i["title"].to_string().trim_matches('"').to_string(),
            folders: get_folders(&items, &i["id"].to_string().trim_matches('"').to_string()),
            tabs: get_tabs(&items, &i["id"].to_string().trim_matches('"').to_string()),
        })
        .collect();
    folders
}

fn get_pinned_container_id(container_ids: &serde_json::Value) -> String {
    match container_ids {
        serde_json::Value::Array(v) => {
            let x = v
                .iter()
                .enumerate()
                .find(|(i, _)| {
                    if *i > 0 && v[i - 1] == "pinned" {
                        true
                    } else {
                        false
                    }
                })
                .map(|o| o.1.to_string());
            x.unwrap()
        }
        _ => String::new(),
    }
}

fn create_internal_workspace(
    space: &serde_json::Value,
    items: &Vec<&serde_json::Value>,
) -> Workspace {
    let workspace_id = space["id"].to_string().trim_matches('"').to_string();
    let workspace_title = space["title"].to_string().trim_matches('"').to_string();
    let workspace_icon = space["customInfo"]["iconType"]["icon"]
        .to_string()
        .trim_matches('"')
        .to_string();
    let pinned_container_id = get_pinned_container_id(&space["containerIDs"])
        .trim_matches('"')
        .to_string();

    let folders = get_folders(items, &pinned_container_id);
    let toplevel_tabs = get_tabs(&items, &pinned_container_id);

    Workspace {
        id: workspace_id,
        pinned_container_id: pinned_container_id,
        title: workspace_title,
        icon: workspace_icon,
        folders: folders,
        tabs: toplevel_tabs,
    }
}

fn get_spaces(containers: &serde_json::Value) -> Result<Vec<Workspace>, String> {
    let items: Vec<&serde_json::Value> = match &containers["items"] {
        serde_json::Value::Array(v) => v,
        _ => return Err("Couldn't find item array".to_string()),
    }
    .iter()
    .filter(|s| match s {
        serde_json::Value::Object(_) => true,
        _ => false,
    })
    .collect();

    let spaces = match &containers["spaces"] {
        serde_json::Value::Array(spaces) => spaces
            .iter()
            .filter(|s| match s {
                serde_json::Value::Object(_) => true,
                _ => false,
            })
            .collect(),
        _ => Vec::new(),
    }
    .iter()
    .map(|s| create_internal_workspace(s, &items))
    .collect();

    Ok(spaces)
}

pub fn sidebar_data(raw_data: &String) -> Result<Vec<Workspace>, String> {
    let json_data: serde_json::Value =
        serde_json::from_str(raw_data).expect("Failed to parse json");

    let containers = get_containers(&json_data).unwrap();
    let spaces = get_spaces(&containers);

    spaces
}
