use std::fs::File;
use std::io::{Read, Write};

use crate::tasks::TaskManager;

/// Loads a `TaskManager` from a JSON file at the given path.
///
/// If the file does not exist, cannot be read, or cannot be parsed,
/// this function returns an empty `TaskManager` instead of failing.
pub fn load_tasks(path: &str) -> TaskManager {
    let file = File::open(path);
    let mut file = match file {
        Ok(f) => f,
        Err(_) => {
            // No file yet → start with empty tasks
            return TaskManager::new();
        }
    };

    let mut contents = String::new();
    if let Err(err) = file.read_to_string(&mut contents) {
        eprintln!("Failed to read tasks file: {err}");
        return TaskManager::new();
    }

    match serde_json::from_str::<TaskManager>(&contents) {
        Ok(manager) => manager,
        Err(err) => {
            eprintln!("Failed to parse tasks JSON: {err}");
            TaskManager::new()
        }
    }
}

/// Saves the current `TaskManager` to a JSON file at the given path.
///
/// The data is written in a pretty-printed JSON format. Any I/O error
/// is returned to the caller so it can be logged or handled as needed.
pub fn save_tasks(path: &str, manager: &TaskManager) -> std::io::Result<()> {
    let json =
        serde_json::to_string_pretty(manager).expect("Failed to serialize TaskManager to JSON");

    let mut file = File::create(path)?;
    file.write_all(json.as_bytes())
}
