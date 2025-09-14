use arc4zen::arc;
use std::fs;

#[test]
fn arc_export_test() {
    let arc_json_structure = fs::read_to_string("tests/data/minimal.json").unwrap();

    let sidebar_data = arc::export::sidebar_data(&arc_json_structure).unwrap();
    let workspace = sidebar_data.first().unwrap();
    let tab = workspace.tabs.first().unwrap();
    let folder = workspace.folders.first().unwrap();
    let nested_tab = folder.tabs.first().unwrap();

    assert!(workspace.title == "workspace");
    assert!(workspace.icon == "grid");
    assert!(workspace.tabs.iter().count() == 1);
    assert!(tab.title == "A tab");
    assert!(tab.link == "https://topleveltab.com");
    assert!(workspace.folders.iter().count() == 1);
    assert!(folder.name == "A folder");
    assert!(nested_tab.title == "A nested tab");
    assert!(nested_tab.link == "https://nestedtab.com");
}
