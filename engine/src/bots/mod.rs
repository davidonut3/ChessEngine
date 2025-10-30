use std::time::Duration;

pub mod matchup;
pub mod dumbengine;
pub mod randomengine;

use crate::bots::matchup::{Engine, run_games};
use crate::games::get_random_games;
use crate::utils::*;

use crate::bots::dumbengine::DumbEngine;
use crate::bots::randomengine::RandomEngine;

pub fn run_matchup() {
    let engine1: fn(&str) -> DumbEngine = DumbEngine::new_game;
    let engine2: fn(&str) -> RandomEngine = RandomEngine::new_game;

    let time_per_move = Duration::from_millis(TIME_PER_MOVE_MILI);
    let fen_strs: &[String] = &get_random_games()[0..500];

    let result: matchup::MatchResult = run_games(fen_strs, time_per_move, engine1, engine2);
    result.print_match(false);
}