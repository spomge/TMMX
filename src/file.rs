use std::env;
use std::path::PathBuf;

// gets the executable directory
pub fn get_file_exe_dir() -> PathBuf {
    env::current_exe().expect("Failed to get current executable path")
}

// Find a json file
pub fn find_json_object(name: &str) -> PathBuf {

    let exe_dir = get_file_exe_dir();
    let information_dir = exe_dir.parent()
        .expect("Failed to get parent directory of executable");

    information_dir.join(name)
}