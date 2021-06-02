//! A "functional" game engine.

pub mod ai;

pub use ax_derive::{run_game, run_game_out};

/// Status
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    Valid,
    Terminal,
}

/// State
pub trait State {
    fn status(&self) -> Status;
}

/// FiniteState
pub trait FiniteState: State {
    fn next_possibilities(&self) -> Option<Vec<Self>>
    where
        Self: Sized;
}

/// Player
pub trait Player<State> {
    fn take_turn(&mut self, state: State) -> State;
}
