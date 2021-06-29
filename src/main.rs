mod cmd;
mod numberguesser;
mod tictactoe;

use std::io;

use ax::ai;
use ax::combinator::{
    enumerate_action, if_then_else, map_action, map_err, map_result, render, repeat_until_terminal,
    take_turn, Either,
};
use ax::{Action, AsBytes, Player, State};

use rand;

fn play_tic_tac_toe<SE, S>(p1: impl Player<S>, p2: impl Player<S>) -> impl FnMut(S) -> Result<S, ()>
where
    S: State<SE> + AsBytes,
{
    let mut p1 = take_turn(p1);
    let mut p2 = take_turn(p2);

    repeat_until_terminal(map_action(
        map_result(
            enumerate_action(if_then_else(
                |(i, _)| i % 2 == 0,
                move |(_, s)| p1(s),
                move |(_, s)| p2(s),
            )),
            |r| match r {
                Result::Ok(e) => Ok(match e {
                    Either::Left(s) => s,
                    Either::Right(s) => s,
                }),
                Result::Err(e) => Err(match e {
                    Either::Left(e) => e,
                    Either::Right(e) => e,
                }),
            },
        ),
        map_err(render(io::stdout()), |_| ()),
    ))
}

fn main() {
    let play = cmd::build_play(vec![cmd::build_tic_tac_toe(), cmd::build_number_guesser()]);

    let train = cmd::build_train(vec![cmd::build_nn(), cmd::build_rnn()]);

    let ax = cmd::build_cli(vec![play, train]);

    let args = ax.get_matches();

    match args.subcommand() {
        None => {}
        Some(("play", args)) => match args.subcommand() {
            None => {}
            Some(("number-guesser", args)) => {
                let high: i64 = args.value_of("high").unwrap().parse().unwrap();
                let low: i64 = args.value_of("low").unwrap().parse().unwrap();

                let rng = rand::thread_rng();
                let state = numberguesser::State::new(low, high, rng);

                let mut run = repeat_until_terminal(map_action(
                    take_turn(numberguesser::Human),
                    map_err(render(io::stdout()), |_| ()),
                ));

                run.apply(state).expect("should have succeeded");
            }
            Some(("tic-tac-toe", args)) => {
                let mode: &str = args.value_of("mode").unwrap();
                let ais: Vec<&str> = args.values_of("ai").unwrap().collect();

                let state: tictactoe::Board<&str> = tictactoe::Board::new();

                let state = match mode {
                    "human-vs-ai" => {
                        let human = tictactoe::Human("X");

                        match ais[0] {
                            "random" => {
                                let mut run = play_tic_tac_toe(
                                    human,
                                    tictactoe::Random::new("O", rand::thread_rng()),
                                );

                                run.apply(state)
                            }
                            "negamax" => {
                                let mut run = play_tic_tac_toe(
                                    human,
                                    ai::Negamax::with_hueristic(
                                        "O",
                                        usize::MAX,
                                        |state: &tictactoe::Board<&str>| match state.is_winner("O")
                                        {
                                            Some(true) => 1,
                                            Some(false) => -1,
                                            None => {
                                                panic!("negamax: expected state to be terminal")
                                            }
                                        },
                                    ),
                                );

                                run.apply(state)
                            }
                            s => panic!("human-vs-ai: unsupported ai: {}", s),
                        }
                    }
                    "human-vs-human" => {
                        let mut run =
                            play_tic_tac_toe(tictactoe::Human("X"), tictactoe::Human("O"));

                        run.apply(state)
                    }
                    "ai-vs-ai" => {
                        let mut run = play_tic_tac_toe(
                            tictactoe::Random::new("X", rand::thread_rng()),
                            tictactoe::Random::new("O", rand::thread_rng()),
                        );

                        run.apply(state)
                    }
                    _ => Err(()),
                };

                state.expect("failed");
            }
            Some((s, _)) => panic!("play: unknown command: {}", s),
        },
        Some(("train", _args)) => {
            todo!();
        }
        Some((s, _)) => panic!("ax: unknown command: {}", s),
    }
}
