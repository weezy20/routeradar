use std::path::PathBuf;

use walkdir::{DirEntry, WalkDir};

use crate::{
    config::{Error, ErrorKind, Mode},
    trie::TrieNode,
};

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
        Mode::Next => return PathBuf::from("app/"),
        Mode::Svelte => return PathBuf::from("src/routes/"),
    }
}

/// Generate all routes
/// Recursively scan the given path and generate routes based on the folder structure.
pub fn generate_routes(path: &PathBuf) -> Result<TrieNode, Error> {
    let mut route_trie = TrieNode::new(true);

    for entry in WalkDir::new(path).into_iter().filter_map(|e| e.ok()) {
        if entry.path().is_file() && entry.path().file_name().unwrap() == "+page.svelte" {
            // Calculate relative path from base path
            let relative_path = entry
                .path()
                .strip_prefix(path)
                .unwrap()
                .to_string_lossy()
                .into_owned();

            // Construct cleaned path without "+page.svelte" for children routes
            let cleaned_path =
                if relative_path.ends_with("/+page.svelte") && relative_path != "/+page.svelte" {
                    relative_path.trim_end_matches("/+page.svelte").to_owned()
                } else {
                    "/".to_owned()
                };

            let route_parts: Vec<&str> =
                cleaned_path.split('/').filter(|&s| !s.is_empty()).collect();
            route_trie.insert(&route_parts)
        }
    }

    Ok(route_trie)
}
