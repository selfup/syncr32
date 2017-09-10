use std;

// fallback for testing
static BACKUP_PATH: &'static str = "./src/fixtures";

#[derive(Debug)]
pub struct State {
    pub root_path: &'static str,
    pub directories: Vec<std::path::PathBuf>,
    pub path_meta_data: Vec<std::fs::Metadata>,
}

impl State {
    pub fn new() -> State {
        let root_path;
        let env_var = option_env!("BACKUP_ROOT");

        match env_var {
            Some(var) => {
                root_path = var;
            }
            None => {
                root_path = BACKUP_PATH;
            }
        }

        State {
            root_path: root_path,
            directories: vec![],
            path_meta_data: vec![],
        }
    }
}
