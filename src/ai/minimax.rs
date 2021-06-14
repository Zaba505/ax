use std::fmt;
use std::marker::PhantomData;

use crate as ax;

/// This AI uses the Negamax algorithm to determine its moves.
#[derive(Debug)]
pub struct Negamax<P, E, S, F>
where
    P: fmt::Display + PartialEq + Default + Copy,
    E: fmt::Debug,
    S: ax::State<E>,
    F: Fn(&S) -> i8,
{
    _e: PhantomData<E>,
    _d: PhantomData<S>,
    piece: P,
    max_depth: usize,
    hue: F,
}

impl<E, P, S, F> Negamax<P, E, S, F>
where
    P: fmt::Display + PartialEq + Default + Copy,
    E: fmt::Debug,
    S: ax::FiniteState<E>,
    F: Fn(&S) -> i8,
{
    /// Provide a max depth and hueristic for Negamax to use when scoring possible moves.
    pub fn with_hueristic(piece: P, max_depth: usize, f: F) -> Self {
        Self {
            _e: PhantomData,
            _d: PhantomData,
            piece,
            max_depth,
            hue: f,
        }
    }

    fn negamax(&self, node: &S, depth: usize, color: i8) -> i8 {
        if depth == 0 || node.status().unwrap() == ax::Status::Terminal {
            return color * (self.hue)(node);
        }

        -1 * node
            .next_possibilities()
            .expect("node is terminal")
            .into_iter()
            .map(|n| self.negamax(&n, depth - 1, -1 * color))
            .max()
            .unwrap()
    }
}

impl<E, S, P, F> ax::Player<S> for Negamax<P, E, S, F>
where
    P: fmt::Display + PartialEq + Default + Copy,
    E: fmt::Debug,
    S: ax::FiniteState<E>,
    F: Fn(&S) -> i8,
{
    fn take_turn(&mut self, state: S) -> S {
        let posses = state
            .next_possibilities()
            .expect("node is terminal already");

        let best = posses
            .into_iter()
            .map(|n| {
                let score = self.negamax(&n, self.max_depth, -1);
                (n, score)
            })
            .max_by(|(_, s1), (_, s2)| s1.cmp(s2))
            .map(|(s, _)| s)
            .expect("no maximum found");

        best
    }
}
