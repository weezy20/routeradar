use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use crate::config::{Error, ErrorKind, Mode};

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

/// Gets the mode to operate on by traversing the files of the given path.
pub fn get_mode(path: &PathBuf) -> anyhow::Result<Mode, Error> {
    for entry in WalkDir::new(path)
        .into_iter()
        .filter_entry(|e| !is_ignored(e))
        .filter_map(|e| e.ok())
        .filter(|e| e.path().is_file())
    {
        let file = entry.file_name();

        if file == "next.config.js" {
            return Ok(Mode::Next);
        } else if file == "svelte.config.js" {
            return Ok(Mode::Svelte);
        }
    }
    Err(Error::new(
        ErrorKind::InvalidPath,
        Some(path.canonicalize().expect("canonicalize path")),
        "Invalid path provided.".to_string(),
    ))
}

/// Get the default root route path for a mode
pub fn get_root_path(mode: &Mode) -> PathBuf {
    match mode {
        Mode::Next => return "./app/".into(),
        Mode::Svelte => return "./src/routes/".into(),
    }
}
