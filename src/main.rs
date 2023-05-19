use std::path::PathBuf;

use clap::Parser;

use spellcast_solver::wordlist;
use wordlist::DEFAULT_WORDLIST;

#[derive(Parser, Debug)]
struct Args {
    /// The list of valid words, separated by newlines.
    #[clap(short, long)]
    wordlist: Option<PathBuf>,
    /// The edge size of the board.
    #[clap(short, long)]
    size: Option<usize>,
}

const DEFAULT_BOARD_SIZE: usize = 5;

fn main() {
    let args = Args::parse();

    let words = if let Some(path) = args.wordlist {
        wordlist::load(
            &std::fs::read_to_string(path).expect("Failed to read wordlist")
                .lines()
                .collect::<Vec<&str>>())
    } else {
        DEFAULT_WORDLIST.clone()
    };

    // println!("Trie: {:?}", trie);
    println!("Word count: {}", words.word_count());
}
