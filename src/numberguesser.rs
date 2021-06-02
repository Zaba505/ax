use std::fmt;
use std::io::{self, Write};

use ax;

use rand::Rng;

#[derive(Debug, PartialEq, Copy, Clone)]
pub struct State {
    guess: Option<i64>,
    number: i64, // the number to guess
    low: i64,
    high: i64,
}

impl State {
    /// Initializes the game state by picking a random number
    /// between low and high for the player to guess.
    ///
    pub fn new(low: i64, high: i64, mut r: impl Rng) -> Self {
        let number = r.gen_range(low..high);
        Self::with_number(low, high, number)
    }

    /// Initializes a game state where the player must guess the provided number.
    pub fn with_number(low: i64, high: i64, number: i64) -> Self {
        Self {
            guess: None,
            number,
            low,
            high,
        }
    }

    /// Allows the player to make a guess.
    pub fn guess(mut self, n: i64) -> Self {
        self.guess = Some(n);
        self
    }
}

impl fmt::Display for State {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let guess = self.guess.unwrap();
        if guess > self.number {
            writeln!(f, "Too high.")
        } else if guess < self.number {
            writeln!(f, "Too low.")
        } else {
            writeln!(f, "Correct!")
        }
    }
}

impl ax::State for State {
    fn status(&self) -> ax::Status {
        let guess = self.guess.unwrap(); // TODO: self.guess == None => invalid state
        if guess > self.number {
            ax::Status::Valid
        } else if guess < self.number {
            ax::Status::Valid
        } else {
            ax::Status::Terminal
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct Human;

impl ax::Player<State> for Human {
    fn take_turn(&mut self, state: State) -> State {
        let mut input = String::new();

        io::stdout()
            .write_all(b"Guess a number: ")
            .expect("failed to write input message");
        io::stdout().flush().expect("failed to flush input message");
        io::stdin()
            .read_line(&mut input)
            .expect("failed to read user input");

        let guess = input.trim();

        state.guess(guess.parse().unwrap())
    }
}
