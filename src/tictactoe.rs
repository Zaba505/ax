use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};
use std::ops::Index;

use ax;

use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Board<P> {
    def: P,
    pieces: Vec<(usize, P)>,
}

impl<P: Default> Board<P> {
    pub fn new() -> Self {
        Self {
            def: P::default(),
            pieces: Vec::with_capacity(9),
        }
    }
}

impl<P, U> From<U> for Board<P>
where
    P: Default,
    U: Into<Vec<(usize, P)>>,
{
    fn from(pieces: U) -> Self {
        Self {
            def: P::default(),
            pieces: pieces.into(),
        }
    }
}

impl<P: PartialEq> Board<P> {
    pub fn is_winner(&self, piece: P) -> Option<bool> {
        if self[0] == self[1] && self[1] == self[2] {
            return Some(self[2] == piece);
        }
        if self[3] == self[4] && self[4] == self[5] {
            return Some(self[5] == piece);
        }
        if self[6] == self[7] && self[7] == self[8] {
            return Some(self[8] == piece);
        }
        if self[0] == self[3] && self[3] == self[6] {
            return Some(self[6] == piece);
        }
        if self[1] == self[4] && self[4] == self[7] {
            return Some(self[7] == piece);
        }
        if self[2] == self[5] && self[5] == self[8] {
            return Some(self[8] == piece);
        }
        if self[0] == self[4] && self[4] == self[8] {
            return Some(self[8] == piece);
        }
        if self[2] == self[4] && self[4] == self[6] {
            return Some(self[6] == piece);
        }
        None
    }

    pub fn has_empty(&self) -> bool {
        self[0] == self.def
            || self[1] == self.def
            || self[2] == self.def
            || self[3] == self.def
            || self[4] == self.def
            || self[5] == self.def
            || self[6] == self.def
            || self[7] == self.def
            || self[8] == self.def
    }

    pub fn place_piece(&mut self, index: usize, piece: P) -> Result<(), Error> {
        for (i, _) in &self.pieces {
            if *i == index {
                return Err(Error::SpotOccupied);
            }
        }
        self.pieces.push((index, piece));
        Ok(())
    }
}

pub enum Error {
    SpotOccupied,
}

impl<P> fmt::Display for Board<P>
where
    P: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        writeln!(f, "{} | {} | {}", self[0], self[1], self[2])?;
        writeln!(f, "---------")?;
        writeln!(f, "{} | {} | {}", self[3], self[4], self[5])?;
        writeln!(f, "---------")?;
        writeln!(f, "{} | {} | {}", self[6], self[7], self[8])
    }
}

impl<P> ax::AsBytes for Board<P>
where
    P: fmt::Display,
{
    fn as_bytes(&self) -> Vec<u8> {
        let mut s = String::new();
        fmt::write(&mut s, format_args!("{}", self)).expect("unexpected error");
        s.into_bytes()
    }
}

impl<P> ax::State<()> for Board<P>
where
    P: PartialEq + Default,
{
    fn status(&self) -> Result<ax::Status, ()> {
        let winner = self.is_winner(P::default());
        if let Some(false) = winner {
            return Ok(ax::Status::Terminal);
        }
        if !self.has_empty() {
            return Ok(ax::Status::Terminal);
        }
        Ok(ax::Status::Valid)
    }
}

impl<P> ax::FiniteState<()> for Board<P>
where
    P: PartialEq + Default + Copy,
{
    fn next_possibilities(&self) -> Option<Vec<Self>>
    where
        Self: Sized,
    {
        let next_piece = self.pieces[self.pieces.len() - 2].1;
        Some(
            self.pieces
                .iter()
                .map(|p| p.0)
                .fold(vec![0, 1, 2, 3, 4, 5, 6, 7, 8], |acc, i| {
                    acc.into_iter().filter(|n| i != *n).collect::<Vec<usize>>()
                })
                .iter()
                .map(|i| {
                    let mut v = Vec::with_capacity(9);
                    v.push((*i, next_piece));
                    v.extend(self.pieces.clone());
                    Board::from(v)
                })
                .collect(),
        )
    }
}

impl<P> Index<usize> for Board<P> {
    type Output = P;

    fn index(&self, index: usize) -> &Self::Output {
        if let Some(piece) = self.pieces.iter().find(|p| p.0 == index) {
            return &piece.1;
        }
        &self.def
    }
}

/// Human
pub struct Human<P>(pub P);

impl<P> ax::Player<Board<P>> for Human<P>
where
    P: fmt::Display + PartialEq + Default + Copy,
{
    fn take_turn(&mut self, board: Board<P>) -> Board<P> {
        let mut input = String::new();

        io::stdout()
            .write_all(b"Enter a position: ")
            .expect("failed to write input message");
        io::stdout().flush().expect("failed to flush input message");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");

        let pos = input.trim();

        let mut board = board;
        let res = board.place_piece(pos.parse().unwrap(), self.0);
        if let Err(_) = res {
            return self.take_turn(board);
        }
        board
    }
}

/// This AI employs the simplest strategy of just randomly picking
/// spots on the board to place its piece.
///
#[derive(Debug)]
pub struct Random<P: fmt::Display + PartialEq + Default + Copy, R: Rng> {
    piece: P,
    rng: RefCell<R>,
}

impl<P, R> Random<P, R>
where
    P: fmt::Display + PartialEq + Default + Copy,
    R: Rng,
{
    pub fn new(piece: P, rng: R) -> Self {
        Random {
            piece,
            rng: RefCell::new(rng),
        }
    }
}

impl<P, R> ax::Player<Board<P>> for Random<P, R>
where
    P: fmt::Display + PartialEq + Default + Copy,
    R: Rng,
{
    fn take_turn(&mut self, board: Board<P>) -> Board<P> {
        let pos = self.rng.borrow_mut().gen_range(0..9);

        let mut board = board;
        let res = board.place_piece(pos, self.piece);
        if let Err(_) = res {
            return self.take_turn(board);
        }
        board
    }
}
