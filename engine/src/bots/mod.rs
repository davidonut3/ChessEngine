use std::time::Duration;

pub mod matchup;
pub mod dumbengine;
pub mod randomengine;
pub mod simpleengine;

use crate::bots::matchup::{Engine, run_games};
use crate::games::get_random_games;

use crate::bots::dumbengine::DumbEngine;
use crate::bots::randomengine::RandomEngine;
use crate::bots::simpleengine::SimpleEngine;

pub fn run_matchup(print_matches: bool, time_per_move: Duration, number_of_games: usize) {
    let engine1: fn(&str) -> SimpleEngine = SimpleEngine::new_game;
    let engine2: fn(&str) -> SimpleEngine = SimpleEngine::new_game;

    let fen_strs: &[String] = &get_random_games()[0..number_of_games];

    let result: matchup::MatchResult = run_games(fen_strs, time_per_move, engine1, engine2);
    result.print_match(print_matches);
}