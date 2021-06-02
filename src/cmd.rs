extern crate clap;

use clap::{App, AppSettings, Arg};

pub fn build_cli(subs: impl IntoIterator<Item = App<'static>>) -> App<'static> {
    App::new("ax")
        .version("1.0")
        .author("Carson Derr <cakub6@gmx.com>")
        .about("ax is a functional game engine meant to simplify experimentation with AI for \"simple\" games.")
        .global_setting(AppSettings::VersionlessSubcommands)
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .arg(Arg::new("config")
            .short('c')
            .long("config")
            .default_value("config.yaml")
            .value_name("FILE")
            .about("Override config file name used by ax.")
        )
        .arg(Arg::new("verbosity")
            .short('v')
            .long("verbose")
            .multiple_occurrences(true)
            .about("Set log verbosity.")
            .global(true)
        )
        .subcommands(subs)
}

pub fn build_play(games: impl IntoIterator<Item = App<'static>>) -> App<'static> {
    App::new("play")
        .about("Play various games which are all built with the ax game engine.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(games)
}

pub fn build_tic_tac_toe() -> App<'static> {
    App::new("tic-tac-toe")
        .about("The classic hashtag game.")
        .arg(
            Arg::new("mode")
                .about("Choose game play mode.")
                .default_value("human-vs-ai")
                .possible_values(&["human-vs-ai", "human-vs-human", "ai-vs-ai"]),
        )
        .arg(
            Arg::new("ai")
                .about("Select AI models.")
                .short('a')
                .long("ai")
                .multiple_occurrences(true)
                .min_values(1)
                .max_values(2)
                .default_values(&["random", "random"])
                .possible_values(&["random", "negamax", "neatnn"]),
        )
}

pub fn build_number_guesser() -> App<'static> {
    App::new("number-guesser").about("The classic \"I'm thinking of a number...\" game.")
        .long_about("The classic \"I'm thinking of a number...\" game has finally been brought to the terminal.
        This variation stills allows players to guess the answer themselves,
        but for those lazier folks an AI can be utilized to play the game for you.")
        .setting(AppSettings::AllowNegativeNumbers)
        .arg(Arg::new("low")
            .about("Set minimum value for range of numbers.")
            .short('l')
            .long("low")
            .default_value("0")
        )
        .arg(Arg::new("high")
            .about("Set maximum value for range of numbers.")
            .short('h')
            .long("high")
            .default_value("100")
        )
        .arg(Arg::new("ai")
            .about("Use AI to play for you.")
            .short('a')
            .long("with-ai")
        )
}

pub fn build_train(
    ais: impl IntoIterator<Item = (App<'static>, &'static [&'static str])>,
) -> App<'static> {
    App::new("train")
        .about("Train various AI for each game.")
        .setting(AppSettings::SubcommandRequiredElseHelp)
        .subcommands(ais.into_iter().map(|(ai, values)| {
            ai.setting(AppSettings::SubcommandRequiredElseHelp)
                .subcommand(
                    App::new("for")
                        .about("Select which game to train for.")
                        .arg(
                            Arg::new("game")
                                .possible_values(values)
                                .required(true)
                                .index(1),
                        ),
                )
        }))
}

pub fn build_nn() -> (App<'static>, &'static [&'static str]) {
    (
        App::new("nueral-network")
            .alias("nn")
            .about("A (Deep) Nueral Network")
            .arg(
                Arg::new("alg")
                    .short('a')
                    .long("algorithm")
                    .about("Algorithm used for training.")
                    .required(true)
                    .possible_values(&["backprop", "neat"]),
            )
            .arg(
                Arg::new("layer")
                    .short('l')
                    .long("layer")
                    .about("Provide (starting) count of nodes in \"hidden\" layers.")
                    .multiple_occurrences(true),
            ),
        &["tic-tac-toe", "number-guesser"],
    )
}

pub fn build_rnn() -> (App<'static>, &'static [&'static str]) {
    (
        App::new("recurrent-nueral-network")
            .alias("rnn")
            .about("Recurrent nueral network")
            .arg(
                Arg::new("alg")
                    .short('a')
                    .long("algorithm")
                    .about("Algorithm used for training.")
                    .required(true)
                    .possible_values(&["backprop", "neat"]),
            )
            .arg(
                Arg::new("layer")
                    .short('l')
                    .long("layer")
                    .about("Provide (starting) count of nodes in \"hidden\" layers.")
                    .multiple_occurrences(true),
            ),
        &["tic-tac-toe", "number-guesser"],
    )
}
