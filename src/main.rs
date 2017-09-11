mod state;
mod actions;

use state::State;
use actions::Actions;

fn main() {
    let mut state = State::new();
    let actions = Actions::new();
    actions.frontload_root_dirs(&mut state);
    actions.grab_path_meta_data(&mut state);

    println!("{:?}", state);
}

#[test]
fn integration_test() {
    let mut state = State::new();
    let actions = Actions::new();
    actions.frontload_root_dirs(&mut state);
    actions.grab_path_meta_data(&mut state);

    assert_eq!(std::path::Path::new("./src/fixtures/.dotfile"),
               state.directories[0]);
    assert_eq!(std::path::Path::new("./src/fixtures/test.md"),
               state.directories[1]);
    assert_eq!(std::path::Path::new("./src/fixtures/test.txt"),
               state.directories[2]);
}
