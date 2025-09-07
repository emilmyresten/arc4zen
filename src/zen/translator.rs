fn normalize_icon(icon: &String) -> String {
    if icon == "grid" {
        icon.to_owned() + "-2x2"
    } else {
        icon.to_string()
    }
}
pub fn create_icon_name(icon: &String) -> String {
    let mut prefix = String::from("chrome://browser/skin/zen-icons/selectable/");
    let normalized_icon = normalize_icon(icon);
    prefix.push_str(&normalized_icon);
    prefix.push_str(".svg");
    prefix
}
