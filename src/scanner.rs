use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use crate::config::Mode;

fn is_ignored(entry: &DirEntry) -> bool {
    let path = entry.path();
    let file_name = path.file_name();
    match file_name {
        Some(file_name) => {
            let file_name = file_name.to_str().unwrap();
            ["node_modules", ".git", "target", ".svelte-kit"].contains(&file_name)
        }
        None => false,
    }
}

/// Print all the files and folders of a given path
pub fn get_mode(path: &PathBuf) -> Result<Mode, ()> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e))
        .filter_map(|e| e.ok())
        .filter(|e| !e.metadata().unwrap().is_dir())
    {
        let file = entry.file_name().to_str().unwrap();

        if file.contains("next.config") {
            return Ok(Mode::Next);
        } else if file.contains("svelte.config") {
            return Ok(Mode::Svelte);
        }
    }
    // TODO:return () or state that no valid config found.
    todo!()
}
