pub fn get_db_path(base_dir: &String) -> String {
    let location = String::new() + &base_dir + "/Library/Application Support/Zen";
    let db_path = location + "/Profiles/tpwvj61x.Default (release)/places.sqlite";
    db_path
}
