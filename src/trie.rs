use std::collections::HashMap;
use std::fmt;

/// A trie data structure for storing words.
#[derive(Debug, Default, Clone)]
pub struct Trie {
    children: HashMap<char, Trie>,
    is_end: bool,
}

impl Trie {
    #[must_use]
    pub fn new() -> Self {
        Trie {
            children: HashMap::new(),
            is_end: false,
        }
    }

    /// Insert a word into the trie.
    pub fn insert(&mut self, word: &str) {
        let mut current = self;
        for char in word.chars() {
            current = current.children.entry(char).or_insert(Trie::new());
        }
        current.is_end = true;
    }

    /// Return true if the word is in the trie.
    #[must_use]
    pub fn search(&self, word: &str) -> bool {
        let mut current = self;
        for char in word.chars() {
            match current.children.get(&char) {
                Some(node) => current = node,
                None => return false,
            }
        }
        current.is_end
    }

    /// Return a vector of words in the trie that start with the prefix.
    #[must_use]
    pub fn predictive_search(&self, word: &str) -> Vec<String> {
        let mut current = self;
        let mut words = Vec::new();
        for char in word.chars() {
            match current.children.get(&char) {
                Some(node) => current = node,
                None => return words,
            }
        }
        words = current.words();
        words.sort();
        words
    }

    /// Return true if the trie contains a word that starts with the prefix.
    #[must_use]
    pub fn starts_with(&self, prefix: &str) -> bool {
        let mut current = self;
        for char in prefix.chars() {
            match current.children.get(&char) {
                Some(node) => current = node,
                None => return false,
            }
        }
        true
    }

    /// Return the number of words in the trie.
    #[must_use]
    pub fn word_count(&self) -> usize {
        let mut count = 0;
        if self.is_end {
            count += 1;
        }
        for child in self.children.values() {
            count += child.word_count();
        }
        count
    }

    /// Return a vector of all words in the trie.
    #[must_use]
    pub fn words(&self) -> Vec<String> {
        let mut words = Vec::new();
        if self.is_end {
            words.push(String::new());
        }
        for (char, child) in &self.children {
            let mut child_words = child.words();
            for word in &mut child_words {
                word.insert(0, *char);
            }
            words.append(&mut child_words);
        }
        words
    }

    // for use in the fmt::Display implementation
    fn indent_format(&self, f: &mut fmt::Formatter, indent_level: usize) -> fmt::Result {
        const INDENT_WIDTH: usize = 2;

        for (char, child) in &self.children {
            if child.is_end {
                writeln!(f, "{:>#indent$} (end)", char, indent = indent_level * INDENT_WIDTH)?;
            } else {
                writeln!(f, "{:>#indent$}", char, indent = indent_level * INDENT_WIDTH)?;
            }
            child.indent_format(f, indent_level + 1)?;
        }
        Ok(())
    }
}

impl fmt::Display for Trie {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.is_end {
            writeln!(f, "\"\"")?;
        }

        if f.alternate() {
            return self.indent_format(f, 1);
        }

        // just print the words
        let mut words = self.words();
        words.sort();
        for word in words {
            writeln!(f, "{word}")?;
        }

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn empty() {
        let trie = Trie::new();
        assert!(!trie.search("hello"));
        assert!(!trie.search(""));
        assert_eq!(trie.word_count(), 0);
    }

    #[test]
    fn empty_string() {
        let mut trie = Trie::new();
        trie.insert("hello");
        assert!(!trie.search(""));
        trie.insert("");
        assert!(trie.search(""));
        assert_eq!(trie.word_count(), 2);
    }

    #[test]
    fn hello_world() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        assert!(trie.search("hello"));
        assert!(trie.search("world"));
        assert!(!trie.search("hell"));
        assert!(!trie.search("wor"));
        assert!(!trie.search("worlds"));
    }

    #[test]
    fn word_count() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        assert_eq!(trie.word_count(), 2);
    }

    #[test]
    fn words() {
        let mut trie = Trie::new();
        trie.insert("hello");
        trie.insert("world");
        let words = trie.words();
        assert_eq!(words.len(), 2);
        assert!(words.contains(&"hello".to_string()));
        assert!(words.contains(&"world".to_string()));
    }
}