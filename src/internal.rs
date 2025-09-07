#[derive(Debug)]
pub struct Workspace {
    pub id: String,
    pub pinned_container_id: String,
    pub title: String,
    pub icon: String,
    pub folders: Vec<Folder>,
    pub tabs: Vec<Tab>,
}

#[derive(Debug)]
pub struct Folder {
    pub id: String,
    pub name: String,
    pub folders: Vec<Folder>,
    pub tabs: Vec<Tab>,
}

#[derive(Debug)]
pub struct Tab {
    pub title: String,
    pub link: String,
}
