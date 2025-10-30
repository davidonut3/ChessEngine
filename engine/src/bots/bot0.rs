use crate::bots::matchup::Engine;
use crate::fenlib::fen::Fen;
use crate::utils::*;

pub struct DumbEngine {
    pos: Fen,
}

impl Engine for DumbEngine {
    fn new_game(fen_str: &str) -> Self {
        DumbEngine { pos: Fen::from_str(fen_str) }
    }

    fn select_move(&mut self, _t: std::time::Duration) -> Move {
        let (moves, _move_count) = self.pos.get_legal_moves_array();

        moves[0]
    }

    fn apply_move(&mut self, move1: Move) {
        self.pos.move_to_fen(move1);
    }
}
