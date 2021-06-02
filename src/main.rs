mod cmd;
mod numberguesser;
mod tictactoe;

fn main() {
    let play = cmd::build_play(vec![cmd::build_tic_tac_toe(), cmd::build_number_guesser()]);

    let train = cmd::build_train(vec![cmd::build_nn(), cmd::build_rnn()]);

    let ax = cmd::build_cli(vec![play, train]);

    let args = ax.get_matches();

    match args.subcommand() {
        None => {}
        Some(("play", args)) => {
            todo!();
        }
        Some(("train", args)) => {
            todo!();
        }
        Some(_) => panic!("unknown command"),
    }
}
