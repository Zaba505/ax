# Ax

**Ax** is a functional game engine meant to simplify experimentation
with AI for \"simple\" games.

By borrowing ideas from parser combinator libraries, like [nom](https://github.com/Geal/nom), a very
expressive API can be provided for developing games with some added Rustic
benefits like zero copy. For example, the classic game "I'm thinking of a number from..."
can be implemented in just a handful of lines:

```rust
let state = numberguesser::State::new(low, high, rng);

let mut run = repeat_until_terminal(map_action(
    take_turn(numberguesser::Human),
    map_err(render(io::stdout()), |_| ()),
));

run.apply(state).expect("should have succeeded");
```
