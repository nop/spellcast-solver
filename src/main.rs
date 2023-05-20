use std::io;
use std::io::BufRead;

use clap::Parser;
use board::Board;

use spellcast_solver::{board, wordlist};
use wordlist::DEFAULT_WORDLIST;

#[derive(Parser, Debug)]
struct Args {
}

fn main() {
    let letters: Vec<char> = {
        let mut line = String::new();
        io::stdin().lock().read_line(&mut line).expect("Failed to read line");
        line.trim()
            .chars()
            .map(|c| c.to_ascii_uppercase())
            .collect::<Vec<char>>()
    };

    let board = Board::from_chars(&letters);

    // println!("Trie: {:?}", trie);
    println!("Word count: {}", &DEFAULT_WORDLIST.word_count());
    println!("Board: {board}");
}

#[cfg(test)]
mod tests {
    // D R U P R
    // O S O T F
    // O F R W G
    // A S I A H
    // N I Z J A

    use spellcast_solver::board::Point;
    use super::*;

    #[test]
    fn load_board() {
        let board_string = Board::from_chars(&"DRUPROSOTFOFRWGASIAHNIZJA".chars().collect::<Vec<char>>());
        let board_matrix = Board::from_chars(&[
            'D', 'R', 'U', 'P', 'R',
            'O', 'S', 'O', 'T', 'F',
            'O', 'F', 'R', 'W', 'G',
            'A', 'S', 'I', 'A', 'H',
            'N', 'I', 'Z', 'J', 'A',
        ]);

        assert_eq!(board_string, board_matrix);
        assert_eq!(board_matrix.get(Point::new(0, 0)), 'D');
    }
}