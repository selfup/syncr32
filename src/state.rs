use std;

#[derive(Debug)]
pub struct State {
    pub root_path: &'static str,
    pub directories: Vec<std::path::PathBuf>,
    pub path_meta_data: Vec<std::fs::Metadata>,
}

impl State {
    pub fn new() -> State {
        State {
            root_path: env!("BACKUP_ROOT"),
            directories: vec![],
            path_meta_data: vec![],
        }
    }
}
