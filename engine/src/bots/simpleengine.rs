use std::time::Duration;
use std::time::Instant;

use crate::bots::matchup::Engine;
use crate::fenlib::fen::Fen;
use crate::utils::*;

const PAWN: usize = 0;
const KNIGHT: usize = 1;
const BISHOP: usize = 2;
const ROOK: usize = 3;
const QUEEN: usize = 4;
const KING: usize = 5;

const VALUES: [i32; 6] = [ 100, 310, 320, 500, 900, 20000 ];

#[derive(Debug, Clone)]
pub struct SimpleEngine {
    fen: Fen,
}

impl Engine for SimpleEngine {
    fn new_game(fen_str: &str) -> Self {
        SimpleEngine { fen: Fen::from_str(fen_str) }
    }

    fn select_move(&mut self, max_time: Duration) -> Move {
        let start_time: Instant = Instant::now();

        let (moves, move_count) = self.fen.get_legal_moves_array();

        let mut best_move_overall: Move = moves[0];

        let mut depth: i32 = 1;
        while start_time.elapsed() < max_time {

            let mut best_move: Move = moves[0];
            let mut best_score: i32 = -INFINITY;

            for i in 0..move_count {
                let move1: Move = moves[i];
                let mut new_fen = self.fen.clone();
                new_fen.move_to_fen(move1);

                let score: i32 = -self.negamax(&new_fen, depth, start_time, max_time);

                if start_time.elapsed() >= max_time {
                    return best_move_overall;
                }

                if score > best_score {
                    best_score = score;
                    best_move = move1;
                }
            }

            best_move_overall = best_move;
            depth += 1;
        }

        best_move_overall
    }

    fn apply_move(&mut self, move1: Move) {
        self.fen.move_to_fen(move1);
    }

    fn name(&self) -> String {
        "SimpleEngine".to_string()
    }
}

impl SimpleEngine {
    fn negamax(&self, fen: &Fen, depth: i32, start_time: Instant, max_time: Duration) -> i32 {
        
        // We assume that the time of the eval function is negligible
        if start_time.elapsed() >= max_time || depth == 0 {
            return self.eval(fen)
        }

        let mut max: i32 = -INFINITY;
        let (moves, move_count) = fen.get_legal_moves_array();
        
        for i in 0..move_count {
            let move1: Move = moves[i];
            let mut new_fen = fen.clone();
            new_fen.move_to_fen(move1);

            let score: i32 = -self.negamax(&new_fen, depth - 1, start_time, max_time);

            if score > max { max = score }
        }

        return max
    }

    fn eval(&self, fen: &Fen) -> i32 {
        match fen.game_outcome(None) {
            GameOutcome::WhiteWins => return i32::MAX,
            GameOutcome::BlackWins => return i32::MIN,
            GameOutcome::Draw => return 0,
            GameOutcome::MaxPliesReached => return 0,
            GameOutcome::Error => panic!("eval: fen is not valid"),
            GameOutcome::Ongoing => (),
        };

        let score: i32 =    
            (   fen.array[PAWN_W].count_ones() as i32          - fen.array[PAWN_B].count_ones() as i32          ) * VALUES[PAWN]        + 
            (   fen.array[KNIGHT_W].count_ones() as i32        - fen.array[KNIGHT_B].count_ones() as i32        ) * VALUES[KNIGHT]      + 
            (   fen.array[BISHOP_W].count_ones() as i32        - fen.array[BISHOP_B].count_ones() as i32        ) * VALUES[BISHOP]      + 
            (   fen.array[ROOK_W].count_ones() as i32          - fen.array[ROOK_B].count_ones() as i32          ) * VALUES[ROOK]        + 
            (   fen.array[QUEEN_W].count_ones() as i32         - fen.array[QUEEN_B].count_ones() as i32         ) * VALUES[QUEEN]       + 
            (   fen.array[KING_W].count_ones() as i32          - fen.array[KING_B].count_ones() as i32          ) * VALUES[KING];

        if fen.white_to_move() { score } else { -score }
    }
}
