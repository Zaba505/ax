extern crate proc_macro;
extern crate quote;

use proc_macro::{TokenStream, TokenTree};
use proc_macro_error::proc_macro_error;
use quote::quote;

/// Generates a turn based game loop.
#[proc_macro]
#[proc_macro_error]
pub fn run_game(input: TokenStream) -> TokenStream {
    println!("{}", input);
    input
}

/// Generates a turn-based game loop with
/// state output.
///
#[proc_macro]
#[proc_macro_error]
pub fn run_game_out(input: TokenStream) -> TokenStream {
    let toks = input.into_iter().collect::<Vec<TokenTree>>();
    let mut args = toks.split(|tok| match tok {
        TokenTree::Punct(p) => p.as_char() == ',',
        _ => false,
    });

    let out = args.nth(0).unwrap();
    let state = args.nth(0).unwrap();
    let players = args;

    let expanded = quote! {
        {
            use ax::{State, Player, Status};
            use std::io::Write;

            let mut turn = 0;
            let mut p1 = p1;
            let mut p2 = p2;
            let mut state = state;

            let res = write!(out, "{}", state);
            match res {
                Ok(_) => {
                    loop {
                        if state.status() == Status::Terminal {
                            break Ok(state)
                        }

                        state = match turn % 2 {
                            0 => p1.take_turn(state),
                            1 => p2.take_turn(state),
                            _ => state,
                        };

                        if state.status() == Status::Valid {
                            let res = write!(out, "{}", state);
                            if res.is_err() {
                                break Err(res.unwrap_err());
                            }
                            turn += 1;
                        }
                    }
                }
                Err(err) => Err(err)
            }
        }
    };

    TokenStream::from(expanded)
}
