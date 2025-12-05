use std::panic;
use std::time::Duration;

use crate::bots::matchup::*;

pub fn debug_run_all_games<E1: Engine + std::panic::UnwindSafe,
                           E2: Engine + std::panic::UnwindSafe>(
    fens: &[String],
    time_per_move: Duration,
    engine1_constr: fn(&str) -> E1,
    engine2_constr: fn(&str) -> E2,
    white_is_engine1: bool,
) {
    let mut ok = 0;
    let mut failed = 0;

    for (i, fen) in fens.iter().enumerate() {
        println!("Running {} / {}", i + 1, fens.len());

        let result = panic::catch_unwind(|| {
            run_game(
                fen,
                time_per_move,
                engine1_constr,
                engine2_constr,
                white_is_engine1,
            )
        });

        match result {
            Ok(_game_result) => {
                ok += 1;
            }
            Err(err) => {
                failed += 1;
                println!("\n💥 CRASH DETECTED 💥");
                println!("Game #: {}", i + 1);
                println!("FEN: {}", fen);

                // Optional: print panic message
                if let Some(msg) = err.downcast_ref::<&str>() {
                    println!("Panic message: {}", msg);
                } else if let Some(msg) = err.downcast_ref::<String>() {
                    println!("Panic message: {}", msg);
                }

                println!("Stopping after first crash.");
                break;
            }
        }
    }

    println!("\nFinished.");
    println!("OK games: {}", ok);
    println!("Failed games: {}", failed);
}
