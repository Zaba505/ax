use std::io;

use crate::{Action, AsBytes, Player, State, Status};

/// Map over the result of an action.
pub fn map<AE, SE, S1, S2, S3, A>(
    mut action: A,
    f: impl Fn(S2) -> S3,
) -> impl FnMut(S1) -> Result<S3, AE>
where
    S1: State<SE>,
    S2: State<SE>,
    S3: State<SE>,
    A: Action<AE, SE, S1, S2>,
{
    move |state: S1| {
        let s = action.apply(state)?;
        Ok(f(s))
    }
}

/// Map over the error of an action.
pub fn map_err<AE1, AE2, SE, S1, S2, A, F>(mut action: A, f: F) -> impl FnMut(S1) -> Result<S2, AE2>
where
    S1: State<SE>,
    S2: State<SE>,
    A: Action<AE1, SE, S1, S2>,
    F: Fn(AE1) -> AE2,
{
    move |state: S1| action.apply(state).map_err(|e| f(e))
}

/// Map one action over the result of another.
pub fn map_action<AE, SE, S1, S2, S3, A1, A2>(
    mut first: A1,
    mut second: A2,
) -> impl FnMut(S1) -> Result<S3, AE>
where
    S1: State<SE>,
    S2: State<SE>,
    S3: State<SE>,
    A1: Action<AE, SE, S1, S2>,
    A2: Action<AE, SE, S2, S3>,
{
    move |state: S1| {
        let state = first.apply(state)?;
        second.apply(state)
    }
}

pub fn map_result<AE1, AE2, SE, S1, S2, S3, A, F>(
    mut action: A,
    f: F,
) -> impl FnMut(S1) -> Result<S3, AE2>
where
    S1: State<SE>,
    S2: State<SE>,
    S3: State<SE>,
    A: Action<AE1, SE, S1, S2>,
    F: Fn(Result<S2, AE1>) -> Result<S3, AE2>,
{
    move |state: S1| f(action.apply(state))
}

/// Repeat an action a specific number of times.
pub fn repeat<AE, SE, S, A>(mut action: A, n: usize) -> impl FnMut(S) -> Result<S, AE>
where
    S: State<SE>,
    A: Action<AE, SE, S, S>,
{
    move |state: S| {
        let mut state = state;
        for _ in 0..n {
            state = action.apply(state)?;
        }
        Ok(state)
    }
}

/// Repeat until game reaches a terminal state.
pub fn repeat_until_terminal<AE, SE, S, A>(mut action: A) -> impl FnMut(S) -> Result<S, AE>
where
    S: State<SE>,
    A: Action<AE, SE, S, S>,
{
    move |mut state: S| loop {
        state = action.apply(state)?;
        match state.status() {
            Ok(s) => {
                if s == Status::Terminal {
                    break Ok(state);
                }
            }
            Err(_) => {}
        }
    }
}

/// Apply an index to the state to track each call of an action.
pub fn enumerate<AE, SE, S>() -> impl FnMut(S) -> Result<(usize, S), AE>
where
    S: State<SE>,
{
    let mut index = -1;
    move |state: S| {
        index += 1;
        Ok((index as usize, state))
    }
}

/// Enumerate action.
pub fn enumerate_action<AE, SE, S1, S2, A>(mut action: A) -> impl FnMut(S1) -> Result<S2, AE>
where
    S1: State<SE>,
    S2: State<SE>,
    A: Action<AE, SE, (usize, S1), S2>,
{
    let mut index = -1;
    move |state: S1| {
        index += 1;
        action.apply((index as usize, state))
    }
}

impl<E, S: State<E>> State<E> for (usize, S) {
    fn status(&self) -> Result<Status, E> {
        self.1.status()
    }
}

impl<S> AsBytes for (usize, S)
where
    S: AsBytes,
{
    fn as_bytes(&self) -> Vec<u8> {
        self.1.as_bytes()
    }
}

pub enum Either<L, R> {
    Left(L),
    Right(R),
}

impl<E, L, R> State<E> for Either<L, R>
where
    L: State<E>,
    R: State<E>,
{
    fn status(&self) -> Result<crate::Status, E> {
        match self {
            Either::Left(s) => s.status(),
            Either::Right(s) => s.status(),
        }
    }
}

impl<L, R> AsBytes for Either<L, R>
where
    L: AsBytes,
    R: AsBytes,
{
    fn as_bytes(&self) -> Vec<u8> {
        match self {
            Either::Left(s) => s.as_bytes(),
            Either::Right(s) => s.as_bytes(),
        }
    }
}

/// Conditionally apply actions
pub fn if_then_else<AE1, AE2, SE, S1, S2, S3, A1, A2>(
    mut f: impl FnMut(&S1) -> bool,
    mut a: A1,
    mut b: A2,
) -> impl FnMut(S1) -> Result<Either<S2, S3>, Either<AE1, AE2>>
where
    S1: State<SE>,
    S2: State<SE>,
    S3: State<SE>,
    A1: Action<AE1, SE, S1, S2>,
    A2: Action<AE2, SE, S1, S3>,
{
    move |state: S1| {
        if f(&state) {
            a.apply(state)
                .map(|s| Either::Left(s))
                .map_err(|e| Either::Left(e))
        } else {
            b.apply(state)
                .map(|s| Either::Right(s))
                .map_err(|e| Either::Right(e))
        }
    }
}

/// Render the game state.
pub fn render<E, S>(mut out: impl io::Write) -> impl FnMut(S) -> Result<S, io::Error>
where
    S: State<E> + AsBytes,
{
    move |state: S| {
        out.write(&state.as_bytes())?;
        Ok(state)
    }
}

/// Take a players turn.
pub fn take_turn<SE, S>(mut player: impl Player<S>) -> impl FnMut(S) -> Result<S, ()>
where
    S: State<SE>,
{
    move |state: S| Ok(player.take_turn(state))
}
