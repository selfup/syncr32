use std;
use std::fs;
use state::State;
use std::fs::File;

#[derive(Debug)]
pub struct Actions {}

impl Actions {
    pub fn new() -> Actions {
        Actions {}
    }

    pub fn frontload_root_dirs(&self, state: &mut State) {
        grab_root_dirs(state).unwrap();
    }

    pub fn grab_path_meta_data(&self, state: &mut State) {
        read_date_info(state).unwrap();
    }
}

// helper Result<T> functions

fn grab_root_dirs(state: &mut State) -> std::io::Result<&mut State> {
    for entry in fs::read_dir(state.root_path)? {
        let path = entry?;
        state.directories.push(path.path());
    }

    Ok(state)
}

fn read_date_info(state: &mut State) -> std::io::Result<&mut State> {
    let dirs = state.directories.clone();

    for file_name in dirs {
        let f = File::open(file_name)?;
        let metadata = f.metadata()?;
        state.path_meta_data.push(metadata);
    }

    Ok(state)
}
