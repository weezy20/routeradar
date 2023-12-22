use std::collections::HashMap;

use crate::config::Route;

/// TrieNode struct
#[derive(Debug)]
pub struct TrieNode {
    /// map of Node's children
    pub children: HashMap<String, TrieNode>,
    /// marks the route as a parent
    pub is_parent: bool,
}

impl TrieNode {
    /// Constructor for TrieNode
    pub fn new(is_parent: bool) -> Self {
        TrieNode {
            children: HashMap::new(),
            is_parent,
        }
    }

    /// Insert a route into the Tride
    pub fn insert(&mut self, path_segments: &[&str]) {
        if path_segments.is_empty() {
            return;
        }

        // check if the current node is root "/"
        let is_root = path_segments.len() == 1 && path_segments[0] == "/";

        if is_root {
            self.is_parent = true;
            self.insert(&path_segments[1..]);
        } else {
            if let Some((first, rest)) = path_segments.split_first() {
                let child = self
                    .children
                    .entry(first.to_string())
                    .or_insert_with(|| TrieNode::new(false));

                // insert the rest of the path
                child.insert(rest);

                // set parent flag for the current node only if it has actual children
                if !child.children.is_empty() {
                    self.is_parent = true
                };
            }
        }
    }

    /// Display the trie structure
    pub fn display(&self, indent: usize) {
        for (part, child) in &self.children {
            println!(
                "{}- {} (Parent: {})",
                " ".repeat(indent),
                part,
                self.is_parent
            );
            child.display(indent + 2);
        }
    }

    /// does something cool
    pub fn does(&self) -> Vec<Route> {
        let mut domo: Vec<Route> = Vec::new();
        for (part, child) in &self.children {
            let child_routes = child.does();

            domo.push(Route {
                path: part.to_owned(),
                children: if self.is_parent {
                    Some(child_routes)
                } else {
                    None
                },
            })
        }
        return domo;
    }
}
