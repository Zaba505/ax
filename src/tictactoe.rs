use std::cell::RefCell;
use std::fmt;
use std::io::{self, Write};

use ax;

use rand::Rng;

#[derive(Debug, PartialEq)]
pub struct Board<P> {
    pieces: Vec<(usize, P)>,
}

impl<P> Board<P> {
    pub fn new() -> Self {
        Self {
            pieces: Vec::with_capacity(9),
        }
    }
}

impl<P, U> From<U> for Board<P>
where
    U: Into<Vec<(usize, P)>>,
{
    fn from(pieces: U) -> Self {
        let mut pieces: Vec<(usize, P)> = pieces.into();
        pieces.sort_unstable_by(|(i, _), (j, _)| j.cmp(i));
        Self { pieces }
    }
}

impl<P: PartialEq> Board<P> {
    pub fn is_winner(&self, piece: P) -> Option<bool> {
        if self.pieces.len() < 3 {
            return None;
        }
        for i in 0..3 {
            // rows
            let a = self
                .pieces
                .iter()
                .find(|(j, _)| *j == 3 * i)
                .map(|(_, p)| p);
            let b = self
                .pieces
                .iter()
                .find(|(j, _)| *j == 3 * i + 1)
                .map(|(_, p)| p);
            let c = self
                .pieces
                .iter()
                .find(|(j, _)| *j == 3 * i + 2)
                .map(|(_, p)| p);

            if a == b && b == c && c != None {
                return c.map(|p| *p == piece);
            }

            // columns
            let a = self.pieces.iter().find(|(j, _)| *j == i).map(|(_, p)| p);
            let b = self
                .pieces
                .iter()
                .find(|(j, _)| *j == i + 3)
                .map(|(_, p)| p);
            let c = self
                .pieces
                .iter()
                .find(|(j, _)| *j == i + 6)
                .map(|(_, p)| p);

            if a == b && b == c && c != None {
                return c.map(|p| *p == piece);
            }
        }
        for i in 0..2 {
            let a = self
                .pieces
                .iter()
                .find(|(j, _)| *j == i * 2)
                .map(|(_, p)| p);
            let b = self.pieces.iter().find(|(j, _)| *j == 4).map(|(_, p)| p);
            let c = self
                .pieces
                .iter()
                .find(|(j, _)| *j == 8 - 2 * i)
                .map(|(_, p)| p);

            if a == b && b == c && c != None {
                return c.map(|p| *p == piece);
            }
        }
        None
    }

    pub fn has_empty(&self) -> bool {
        return self.pieces.len() < 9;
    }

    pub fn place_piece(&mut self, index: usize, piece: P) -> Result<(), Error> {
        let mut idx = 0;
        for (i, _) in &self.pieces {
            if *i == index {
                return Err(Error::SpotOccupied);
            }
            if index > *i {
                break;
            }
            idx += 1;
        }
        self.pieces.insert(idx, (index, piece));
        Ok(())
    }
}

#[derive(Debug, PartialEq)]
pub enum Error {
    SpotOccupied,
}

impl<P> fmt::Display for Board<P>
where
    P: fmt::Display + Default,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let a = &P::default();
        let one = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 0)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let two = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 1)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let thr = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 2)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let four = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 3)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let five = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 4)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let six = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 5)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let seven = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 6)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let eight = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 7)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        let nine = self
            .pieces
            .iter()
            .find(|(j, _)| *j == 8)
            .map(|(_, p)| p)
            .or_else(|| Some(a))
            .unwrap();
        writeln!(f, "{} | {} | {}", one, two, thr)?;
        writeln!(f, "---------")?;
        writeln!(f, "{} | {} | {}", four, five, six)?;
        writeln!(f, "---------")?;
        writeln!(f, "{} | {} | {}", seven, eight, nine)
    }
}

impl<P> ax::AsBytes for Board<P>
where
    P: fmt::Display + Default,
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

impl<P> IntoIterator for Board<P> {
    // Option<P> avoids needing P: Default constraint.
    type Item = Option<P>;
    type IntoIter = Pieces<P>;

    fn into_iter(self) -> Self::IntoIter {
        Pieces {
            pieces: self.pieces,
            idx: 0,
        }
    }
}

#[derive(Debug)]
pub struct Pieces<P> {
    pieces: Vec<(usize, P)>,
    idx: usize,
}

impl<P> Iterator for Pieces<P> {
    // Option<P> avoids needing P: Default constraint.
    type Item = Option<P>;

    fn next(&mut self) -> Option<Self::Item> {
        if self.idx == 9 {
            return None;
        }
        if self.pieces.len() == 0 && self.idx < 9 {
            self.idx += 1;
            return Some(None);
        }
        if self.idx == self.pieces[self.pieces.len() - 1].0 {
            self.idx += 1;
            return self.pieces.pop().map(|(_, p)| Some(p));
        }
        self.idx += 1;
        return Some(None);
    }
}

impl<'a, P> ax::FiniteState<()> for Board<P>
where
    P: PartialEq + Default + Copy,
{
    type Item = Board<P>;
    type States = States<P>;

    fn possible_states(mut self) -> Self::States
    where
        Self: Sized,
    {
        self.pieces.sort_unstable_by(|(i, _), (j, _)| i.cmp(j));
        States { state: self }
    }
}

#[derive(Debug)]
pub struct States<P> {
    state: Board<P>,
}

impl<P> Iterator for States<P> {
    type Item = Board<P>;

    fn next(&mut self) -> Option<Self::Item> {
        todo!()
    }
}

/// Human
#[derive(Debug)]
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

#[cfg(test)]
mod tests {
    use super::*;
    use rand;

    #[test]
    fn empty_pieces() {
        let b: Board<&str> = Board::from(Vec::new());
        let mut pieces = b.into_iter();

        for i in 0..10 {
            if i < 9 {
                assert_eq!(Some(None), pieces.next());
            } else {
                assert_eq!(
                    None,
                    pieces.next(),
                    "a tic-tac-toe board should only have 9 pieces!"
                );
            }
        }
    }

    #[test]
    fn sparse_pieces() {
        let b = Board::from(vec![(0, "X"), (4, "X"), (8, "X")]);
        let mut pieces = b.into_iter();

        assert_eq!(Some(Some("X")), pieces.next(), "expected piece('X') at:  0");
        for i in 0..3 {
            assert_eq!(Some(None), pieces.next(), "expected no piece at: {}", i + 1);
        }
        assert_eq!(Some(Some("X")), pieces.next(), "expected piece('X') at: 4");
        for i in 0..3 {
            assert_eq!(Some(None), pieces.next(), "expected no piece at: {}", i + 4);
        }
        assert_eq!(Some(Some("X")), pieces.next(), "expected piece('X') at: 8");
    }

    #[test]
    fn sparse_pieces_2() {
        let b = Board::from(vec![(2, "X"), (4, "X"), (8, "X")]);
        let mut pieces = b.into_iter();

        for i in 0..2 {
            assert_eq!(Some(None), pieces.next(), "expected no piece at: {}", i);
        }
        assert_eq!(Some(Some("X")), pieces.next(), "expected piece('X') at: 2");
        assert_eq!(Some(None), pieces.next(), "expected no piece at: 3");
        assert_eq!(Some(Some("X")), pieces.next(), "expected piece('X') at: 4");
        for i in 0..3 {
            assert_eq!(Some(None), pieces.next(), "expected no piece at: {}", i + 5);
        }
        assert_eq!(Some(Some("X")), pieces.next(), "expected piece('X') at: 8");
    }

    #[test]
    fn place_piece() {
        let mut b: Board<&str> = Board::new();

        assert_eq!(Ok(()), b.place_piece(1, "X"));
        assert_eq!(Ok(()), b.place_piece(0, "X"));
    }

    #[test]
    fn place_piece_random_order() {
        let mut r = rand::thread_rng();
        let i = r.gen_range(0..9);
        let j = loop {
            let n = r.gen_range(0..9);
            if n != i {
                break n;
            }
        };
        let k = loop {
            let n = r.gen_range(0..9);
            if n != i && n != j {
                break n;
            }
        };
        let mut b: Board<&str> = Board::new();

        assert_eq!(Ok(()), b.place_piece(i, "X"));
        assert_eq!(Ok(()), b.place_piece(j, "X"));
        assert_eq!(Ok(()), b.place_piece(k, "X"));
    }

    #[test]
    fn place_piece_on_occupied_spot() {
        let mut b: Board<&str> = Board::new();

        assert_eq!(Ok(()), b.place_piece(0, "X"));
        assert_eq!(Err(Error::SpotOccupied), b.place_piece(0, "X"));
    }

    #[test]
    fn is_winner() {
        let possibilities = vec![
            (0, 1, 2),
            (3, 4, 5),
            (6, 7, 8),
            (0, 3, 6),
            (1, 4, 7),
            (2, 5, 8),
            (6, 7, 8),
            (0, 4, 8),
            (2, 4, 6),
        ];
        for pos in possibilities {
            let mut b = Board::new();

            b.place_piece(pos.0, "X").expect("");
            b.place_piece(pos.1, "X").expect("");
            b.place_piece(pos.2, "X").expect("");

            assert_eq!(
                Some(true),
                b.is_winner("X"),
                "expected winner for combo: {:?}",
                pos
            );
        }
    }

    #[test]
    fn is_not_winner() {
        let b = Board::from(vec![(0, "X"), (3, "X"), (6, "X")]);

        assert_eq!(Some(false), b.is_winner("O"));
    }
}
