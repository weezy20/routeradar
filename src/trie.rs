use std::collections::HashMap;

/// TrieNode struct
#[derive(Debug)]
pub struct TrieNode {
    /// map of Node's children
    pub children: HashMap<String, TrieNode>,
}

impl TrieNode {
    /// Constructor for TrieNode
    pub fn new() -> Self {
        TrieNode {
            children: HashMap::new(),
        }
    }

    /// Insert a route into the Tride
    pub fn insert(&mut self, route_parts: &[&str]) {
        if let Some(part) = route_parts.first() {
            let child = self
                .children
                .entry(part.to_string())
                .or_insert(TrieNode::new());

            child.insert(&route_parts[1..]);
        }
    }

    /// Display the trie structure
    pub fn display(&self, indent: usize) {
        for (part, child) in &self.children {
            println!("{}- {}", " ".repeat(indent), part);
            child.display(indent + 2);
        }
    }
}
