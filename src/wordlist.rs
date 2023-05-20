use lazy_static::lazy_static;

use crate::trie::Trie;

const WORDLIST_TEXT: &str = include_str!("../wordlist.txt");

lazy_static! {
    pub static ref DEFAULT_WORDLIST: Trie = {
        let words: Vec<&str> = WORDLIST_TEXT.lines().collect();
        load(&words)
    };
}

#[must_use]
pub fn load(wordlist: &[&str]) -> Trie {
    let mut trie = Trie::new();
    for word in wordlist {
        trie.insert(&word.to_uppercase());
    }
    trie
}
