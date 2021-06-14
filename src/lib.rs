//! A "functional" game engine.

pub mod ai;
pub mod combinator;

/// Status
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum Status {
    Valid,
    Terminal,
}

/// State
pub trait State<E> {
    fn status(&self) -> Result<Status, E>;
}

/// Action
pub trait Action<AE, SE, S1, S2>
where
    S1: State<SE>,
    S2: State<SE>,
{
    fn apply(&mut self, state: S1) -> Result<S2, AE>;

    /// map over the result of an action.
    fn map<S3: State<SE>, F: Fn(S1) -> S3>(self, f: F) -> Map<Self, F, S2>
    where
        Self: core::marker::Sized,
    {
        Map {
            prev: self,
            f,
            phantom: core::marker::PhantomData,
        }
    }
}

pub struct Map<A, F, S> {
    prev: A,
    f: F,
    phantom: core::marker::PhantomData<S>,
}

impl<AE, SE, S1, S2, S3, A, F> Action<AE, SE, S1, S3> for Map<A, F, S2>
where
    S1: State<SE>,
    S2: State<SE>,
    S3: State<SE>,
    A: Action<AE, SE, S1, S2>,
    F: Fn(S2) -> S3,
{
    fn apply(&mut self, state: S1) -> Result<S3, AE> {
        match self.prev.apply(state) {
            Ok(s) => Ok((self.f)(s)),
            Err(err) => Err(err),
        }
    }
}

impl<AE, SE, S1, S2, F> Action<AE, SE, S1, S2> for F
where
    S1: State<SE>,
    S2: State<SE>,
    F: FnMut(S1) -> Result<S2, AE>,
{
    fn apply(&mut self, state: S1) -> Result<S2, AE> {
        self(state)
    }
}

/// FiniteState
pub trait FiniteState<E>: State<E> {
    fn next_possibilities(&self) -> Option<Vec<Self>>
    where
        Self: Sized;
}

/// Player
pub trait Player<State> {
    fn take_turn(&mut self, state: State) -> State;
}

/// Helper trait for types that can be viewed as a byte slice
pub trait AsBytes {
    /// Casts the input type to a byte slice
    fn as_bytes(&self) -> Vec<u8>;
}

impl<'a> AsBytes for &'a str {
    #[inline(always)]
    fn as_bytes(&self) -> Vec<u8> {
        (*self).as_bytes().into()
    }
}

impl AsBytes for str {
    #[inline(always)]
    fn as_bytes(&self) -> Vec<u8> {
        let b: &[u8] = self.as_ref();
        b.into()
    }
}

impl<'a> AsBytes for &'a [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> Vec<u8> {
        (*self).into()
    }
}

impl AsBytes for [u8] {
    #[inline(always)]
    fn as_bytes(&self) -> Vec<u8> {
        self.into()
    }
}
