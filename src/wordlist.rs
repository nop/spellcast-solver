use std::path::Path;

use lazy_static::lazy_static;

use crate::trie::Trie;

const DEFAULT_WORDLIST_PATH: &str = "wordlist.txt";

lazy_static! {
    pub static ref DEFAULT_WORDLIST: Trie = {
        let path = std::path::Path::new(DEFAULT_WORDLIST_PATH);
        let default_wordlist = trim_default_wordlist(path).expect("Failed to read wordlist");
        let words: Vec<&str> = default_wordlist.lines().collect();
        load(&words)
    };
}

#[must_use]
pub fn load(wordlist: &[&str]) -> Trie {
    let mut trie = Trie::new();
    for word in extract_valid_words(wordlist) {
        trie.insert(&word);
    }
    trie
}

/// Extract the valid words from the wordlist.
/// - word must be at least 3 chars long
/// - word must be all lowercase
/// - remove words with 's suffix
fn extract_valid_words(contents: &[&str]) -> Vec<String> {
    let re = regex::Regex::new(r"^[a-z]{3,}$").unwrap();
    let words: Vec<String> =
        contents.iter()
            .filter(|word| re.is_match(word) || word.is_empty())
            .map(|word| (*word).to_string())
            .collect();
    words
}

/// Read the wordlist from the given path up to the end of the header.
fn trim_default_wordlist(path: &Path) -> std::io::Result<String> {
    use std::io::{Seek, SeekFrom};
    use std::fs::File;
    use std::io::Read;

    const DEFAULT_WORDLIST_HEADER_LEN: u64 = 1686;

    let mut file = File::open(path)?;
    let mut contents = String::new();
    file.seek(SeekFrom::Start(DEFAULT_WORDLIST_HEADER_LEN))?;
    file.read_to_string(&mut contents)?;
    Ok(contents)
}
