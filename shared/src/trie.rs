use std::collections::HashMap;

pub struct Trie {
    children: HashMap<char, Trie>,
    value: Option<String>,
}

impl Trie {
    pub fn new(words: Vec<&str>) -> Self {
        let mut trie = Trie::empty();
        for word in words {
            trie.insert(word)
        }
        trie
    }

    pub fn empty() -> Self {
        Trie {
            children: HashMap::new(),
            value: None,
        }
    }

    pub fn insert(&mut self, value: &str) {
        let mut child = self;
        for char in value.chars() {
            child = child.children.entry(char).or_insert_with(Trie::empty)
        }
        child.value = Some(value.to_string())
    }

    pub fn starts_with(&self, prefix: &str) -> Vec<String> {
        let mut results = Vec::new();
        let mut child = self;
        for char in prefix.chars() {
            match child.children.get(&char) {
                Some(n) => child = n,
                None => return results,
            }
        }
        child.collect(&mut results);
        results
    }

    fn collect(&self, results: &mut Vec<String>) {
        if let Some(v) = self.value.clone() {
            results.push(v)
        }
        for child in self.children.values() {
            child.collect(results)
        }
    }
}

#[cfg(test)]
mod tests {
    extern crate rstest;

    use rstest::*;

    use super::*;

    #[rstest]
    #[case("te", "test")]
    #[case("und", "undo")]
    #[case("u", "undo, unknown")]
    #[case("ok", "ok")]
    #[case("okay", "")]
    fn should_search(#[case] input: &str, #[case] output: &str) {
        let trie = Trie::new(vec!["test", "undo", "unknown", "ok"]);
        let mut results = trie.starts_with(input);
        results.sort();
        assert_eq!(results.join(", "), output);
    }
}
