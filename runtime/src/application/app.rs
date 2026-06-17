use std::path::PathBuf;

pub struct App {
    pub id: String,
    pub path: PathBuf,
    pub entry: PathBuf,
    pub permissions: Vec<String>,
    pub is_sys: bool,
    pub icon: Option<String>,
}
