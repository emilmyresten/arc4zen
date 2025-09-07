pub fn get_db_path(base_dir: &String) -> String {
    let location = String::new() + base_dir + "/Library/Application Support/Arc";
    let db_path = location + "/StorableSidebar.json";
    db_path
}
