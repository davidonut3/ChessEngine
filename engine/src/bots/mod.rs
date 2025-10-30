use std::time::Duration;

mod matchup;
mod bot0;

use crate::bots::matchup::{Engine, run_games};
use crate::bots::bot0::DumbEngine;
use crate::games::get_random_games;

const TIME_PER_MOVE: Duration = Duration::from_millis(100);   

pub fn run_matchup() {
    let engine1: fn(&str) -> DumbEngine = DumbEngine::new_game;
    let engine2: fn(&str) -> DumbEngine = DumbEngine::new_game;

    let fen_strs: &[String] = &get_random_games()[0..500];

    let result: matchup::MatchResult = run_games(fen_strs, TIME_PER_MOVE, engine1, engine2);
    result.print_match(false);
}