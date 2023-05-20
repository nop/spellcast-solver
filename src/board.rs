use std::cell::Cell;
use std::fmt;

const SIZE: usize = 5;

/// The board is a 5x5 grid of letters.
///
/// # Example
/// ```pre
/// D R U P R
/// O S O T F
/// O F R W G
/// A S I A H
/// N I Z J A
/// ```
#[derive(Debug, Eq, PartialEq)]
pub struct Board {
    pub cells: [[Cell<char>; SIZE]; SIZE],
}

impl Board {
    /// Create a new board from the given chars.
    ///
    /// # Panics
    /// Panics if the length of `chars` is not 25.
    #[must_use]
    pub fn from_chars(chars: &[char]) -> Board {
        assert_eq!(chars.len(), SIZE * SIZE, "Board must be 5x5");
        let board = Board {
            // FIXME: I can't use the [Cell::new(' '); S] syntax here because it's not Copy.
            cells: [
                [Cell::new(' '),Cell::new(' '), Cell::new(' '), Cell::new(' '), Cell::new(' '),],
                [Cell::new(' '),Cell::new(' '), Cell::new(' '), Cell::new(' '), Cell::new(' '),],
                [Cell::new(' '),Cell::new(' '), Cell::new(' '), Cell::new(' '), Cell::new(' '),],
                [Cell::new(' '),Cell::new(' '), Cell::new(' '), Cell::new(' '), Cell::new(' '),],
                [Cell::new(' '),Cell::new(' '), Cell::new(' '), Cell::new(' '), Cell::new(' '),],
                ],
        };
        for (i, &c) in chars.iter().enumerate() {
            let x = i % SIZE;
            let y = i / SIZE;
            board.cells[y][x].set(c);
        }
        board
    }

    pub fn set(&mut self, position: Point, value: char) {
        self.cells[position.1][position.0].set(value);
    }

    #[must_use]
    pub fn get(&self, position: Point) -> char {
        self.cells[position.1][position.0].get()
    }
}

impl fmt::Display for Board {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        for y in 0..SIZE {
            for x in 0..SIZE {
                write!(f, "{} ", self.get(Point::new(x, y)))?;
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Point(pub usize, pub usize);

impl Point {
    #[must_use]
    pub fn new(x: usize, y: usize) -> Self {
        Self(x, y)
    }

    #[must_use]
    pub fn from_index(index: usize, size: usize) -> Self {
        Self(index % size, index / size)
    }

    #[must_use]
    pub fn to_index(&self, size: usize) -> usize {
        self.1 * size + self.0
    }
}