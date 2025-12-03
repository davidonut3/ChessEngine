use std::time::Duration;

pub mod matchup;
pub mod dumbengine;
pub mod randomengine;
pub mod simpleengine;
pub mod alphaengine;
pub mod sortedengine;

pub mod opening_book;

use crate::bots::matchup::{Engine, run_games};
use crate::games::get_random_games;

// use crate::bots::dumbengine::DumbEngine;
// use crate::bots::randomengine::RandomEngine;
// use crate::bots::simpleengine::SimpleEngine;
// use crate::bots::alphaengine::AlphaEngine;
use crate::bots::sortedengine::SortedEngine;

pub fn run_matchup(print_matches: bool, time_per_move: Duration, number_of_games: usize) {
    let engine1: fn(&str) -> SortedEngine = SortedEngine::new_game;
    let engine2: fn(&str) -> SortedEngine = SortedEngine::new_game;

    let fen_strs: &[String] = &get_random_games()[0..number_of_games];

    let result: matchup::MatchResult = run_games(fen_strs, time_per_move, engine1, engine2);
    result.print_match(print_matches);
}