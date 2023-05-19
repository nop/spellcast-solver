use spellcast_solver::trie::Trie;

fn main() {
    let example_wordlist = vec![
        "cat",
        "car",
        "cab",
        "cow",
        "cob",
        "",
        "cop",
        "can",
        "bat",
        "bar",
        "ban",
        "back",
        "bark",
        "barn",
        "barnyard",
    ];

    println!("Example wordlist: {:?}", example_wordlist);

    let mut words = Trie::new();
    for example in example_wordlist {
        let mut word = example.to_string();
        words.insert(&word);
        if !word.is_empty() {
            // add "s" to the end of every word, except the empty string
            word.push_str("s");
            words.insert(&word);
        }
    }

    println!("{:#}", words);
}