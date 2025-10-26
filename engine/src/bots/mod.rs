use std::time::Duration;

use crate::utils::*;
use crate::fenlib::fen::Fen;

// If two engines reach this number of plies, we stop the game,
// so that each game takes only MAX_NUMBER_OF_PLIES * time_per_move time.
const MAX_NUMBER_OF_PLIES: i32 = 500;

pub enum Side {
    White,
    Black,
}

pub struct GameResult {
    pub start_fen: String,
    pub moves: Vec<Move>,
    pub outcome: GameOutcome,
    pub time_per_move: Duration,
}

pub trait Engine {
    fn new_game(fen: &str) -> Self where Self: Sized;

    fn select_move(&mut self, time_per_move: Duration) -> Move;

    fn apply_move(&mut self, move1: Move);
}

pub fn run_game<E: Engine>(fen_str: &str, time_per_move: Duration, white_engine: fn(&str) -> E, black_engine: fn(&str) -> E) -> GameResult {
    let mut white = white_engine(fen_str);
    let mut black = black_engine(fen_str);

    let mut moves: Vec<Move> = Vec::new();
    let mut outcome: GameOutcome = GameOutcome::MaxPliesReached;
    let mut side: Side = Side::White;

    for _ply in 0..MAX_NUMBER_OF_PLIES {
        let move1 = match side {
            Side::White => {
                let move1: Move = white.select_move(time_per_move);
                black.apply_move(move1);
                move1
            }
            Side::Black => {
                let move1: Move = black.select_move(time_per_move);
                white.apply_move(move1);
                move1
            }
        };
        moves.push(move1);

        // Check game state

        side = match side { Side::White => Side::Black, Side::Black => Side::White };
    }

    GameResult { start_fen: fen_str.to_string(), moves: moves, outcome: outcome, time_per_move: time_per_move }
}