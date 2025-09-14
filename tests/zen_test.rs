use arc4zen::arc;
use arc4zen::zen;
use rusqlite;
use std::fs;

mod testutil;

#[test]
fn zen_test() {
    let arc_json_data = fs::read_to_string("tests/data/minimal.json").unwrap();

    let arc_data = arc::export::sidebar_data(&arc_json_data).unwrap();

    let zen_db_conn = rusqlite::Connection::open_in_memory().unwrap();
    testutil::setup_zen_db(&zen_db_conn);
    zen::import::sidebar_data(&zen_db_conn, &arc_data).unwrap();
}
