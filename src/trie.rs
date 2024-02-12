use std::collections::HashMap;

pub struct Trie {
    children: HashMap<char, Trie>,
    value: Option<String>,
}

impl Trie {
    pub fn new() -> Self {
        Trie {
            children: HashMap::new(),
            value: None
        }
    }

    pub fn insert(&mut self, value: &str) {
        let mut child = self;
        for char in value.chars() {
            child = child.children.entry(char).or_insert_with(Trie::new)
        }
        child.value = Some(value.to_string())
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut child = self;
        for char in prefix.chars() {
            match child.children.get(&char) {
                Some(n) => child = n,
                None => return results
            }
        }
        self.collect(child, &mut results);
        results
    }

    fn collect<'a>(&self, root: &'a Trie, results: &mut Vec<String>) {
        root.value.clone().map(|v| results.push(v));
        for (_, child) in &root.children {
            self.collect(&child, results)
        }
    }
}
