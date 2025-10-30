use std::time::Duration;

use crate::bots::matchup::Engine;
use crate::fenlib::fen::Fen;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct DumbEngine {
    fen: Fen,
}

impl Engine for DumbEngine {
    fn new_game(fen_str: &str) -> Self {
        DumbEngine { fen: Fen::from_str(fen_str) }
    }

    fn select_move(&mut self, _t: Duration) -> Move {
        let (moves, _move_count) = self.fen.get_legal_moves_array();

        moves[0]
    }

    fn apply_move(&mut self, move1: Move) {
        self.fen.move_to_fen(move1);
    }
}
