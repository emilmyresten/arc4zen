use rusqlite;
use std::env;
use std::fs;

mod arc;
mod internal;
mod util;
mod zen;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let home_directory = env::var("HOME")?;

    let arc_db_path = arc::data::get_db_path(&home_directory);
    let arc_raw_data = fs::read_to_string(&arc_db_path)?;

    let zen_db_path = zen::data::get_db_path(&home_directory);
    let conn = rusqlite::Connection::open(zen_db_path)?;

    let workspaces = arc::export::sidebar_data(&arc_raw_data)?;
    zen::import::sidebar_data(&conn, &workspaces)?;
    Ok(())
}
