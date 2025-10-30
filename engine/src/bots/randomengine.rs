use std::time::Duration;
use rand::prelude::*;

use crate::bots::matchup::Engine;
use crate::fenlib::fen::Fen;
use crate::utils::*;

#[derive(Debug, Clone)]
pub struct RandomEngine {
    fen: Fen,
}

impl Engine for RandomEngine {
    fn new_game(fen_str: &str) -> Self {
        RandomEngine { fen: Fen::from_str(fen_str) }
    }

    fn select_move(&mut self, _t: Duration) -> Move {
        let (moves, move_count) = self.fen.get_legal_moves_array();

        let mut rng: ThreadRng = rand::rng();
        let index: usize = rng.random_range(0..move_count);
        
        moves[index]
    }

    fn apply_move(&mut self, move1: Move) {
        self.fen.move_to_fen(move1);
    }
}
