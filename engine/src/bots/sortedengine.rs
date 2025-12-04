use std::time::Duration;
use std::time::Instant;

use crate::bots::matchup::Engine;
use crate::bots::opening_book::get_opening_move;
use crate::fenlib::fen::Fen;
use crate::fenlib::attacks;
use crate::parsing;
use crate::utils::*;

const PAWN: usize = 0;
const KNIGHT: usize = 1;
const BISHOP: usize = 2;
const ROOK: usize = 3;
const QUEEN: usize = 4;
const KING: usize = 5;

// Mate/king value cannot be infinity, since that may result in integer overflows
const MATE_VALUE: i32 = 20000;

const MIDGAME_VALUES: [i32; 6] = [ 82, 337, 365, 477, 1025, MATE_VALUE ];
const ENDGAME_VALUES: [i32; 6] = [ 94, 281, 297, 512, 936, MATE_VALUE ];
const GAME_PHASE_VALUES: [i32; 5] = [0, 1, 1, 2, 4];

#[derive(Debug, Clone)]
pub struct SortedEngine {
    fen: Fen,
    use_opening_book: bool,
}

impl Engine for SortedEngine {
    fn new_game(fen_str: &str) -> Self {
        SortedEngine { fen: Fen::from_str(fen_str), use_opening_book: true }
    }

    fn select_move(&mut self, max_time: Duration) -> Move {
        let start_time: Instant = Instant::now();

        let (moves, move_count) = self.fen.get_legal_moves_array();

        if self.use_opening_book {
            let partial_zobrist = self.fen.get_partial_zobrist();

            if let Some(opening_move) = get_opening_move(partial_zobrist) {
                let move1 = parsing::compact_to_move(&opening_move);

                // In the very rare case we get a hash collision with an opening position, we have to make sure the move we find is legal
                if moves.contains(&move1) { return move1 }
            } else {
                self.use_opening_book = false;
            }
        }

        let mut best_move_overall: Move = moves[0];

        let mut depth: i32 = 1;
        while start_time.elapsed() < max_time {

            let mut alpha: i32 = -INFINITY;
            let beta: i32 = INFINITY;

            let mut best_move: Move = moves[0];
            let mut best_score: i32 = -INFINITY;

            for i in 0..move_count {
                let move1: Move = moves[i];
                let mut new_fen = self.fen.clone();
                new_fen.move_to_fen(move1);

                let score: i32 = -self.negamax(&new_fen, depth, start_time, max_time, -beta, -alpha);

                if score > best_score {
                    best_score = score;
                    best_move = move1;
                }

                if score > alpha { alpha = score }
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
        "SortedEngine".to_string()
    }
}

impl SortedEngine {
    fn negamax(&self, fen: &Fen, depth: i32, start_time: Instant, max_time: Duration, mut alpha: i32, beta: i32) -> i32 {
        
        // We assume that the time of the eval function is negligible
        if start_time.elapsed() >= max_time || depth == 0 {
            return self.eval(fen)
        }

        let (moves, move_count) = fen.get_legal_moves_array();

        if move_count == 0 {
            let in_check: bool = fen.player_in_check(fen.white_to_move());

            // We break early if there is a stalemate or a checkmate
            if in_check { return -MATE_VALUE; } else { return 0; }
        };

        let mut value: i32 = -INFINITY;
        
        for i in 0..move_count {
            let move1: Move = moves[i];
            let mut new_fen = fen.clone();
            new_fen.move_to_fen(move1);

            if new_fen.game_outcome(None) == GameOutcome::Error { panic!("negamax: fen is not valid {:?} move {:?}", new_fen.to_string(), parsing::move_to_lan(&move1)) }

            let score: i32 = -self.negamax(&new_fen, depth - 1, start_time, max_time, -beta, -alpha);

            if score > value { value = score }
            if value > alpha { alpha = value }

            if alpha >= beta { break }
            if start_time.elapsed() >= max_time { break }
        }

        value
    }

    fn eval(&self, fen: &Fen) -> i32 {
        match fen.game_outcome(None) {
            GameOutcome::WhiteWins => return i32::MAX,
            GameOutcome::BlackWins => return i32::MIN,
            GameOutcome::Draw => return 0,
            GameOutcome::MaxPliesReached => return 0,
            GameOutcome::Error => panic!("eval: fen is not valid {:?}", fen.to_string()),
            GameOutcome::Ongoing => (),
        };

        let white_to_move = fen.white_to_move();

        let white_pawn_val: u32      = fen.array[PAWN_W].count_ones();
        let black_pawn_val: u32      = fen.array[PAWN_B].count_ones();
        let white_knight_val: u32    = fen.array[KNIGHT_W].count_ones();
        let black_knight_val: u32    = fen.array[KNIGHT_B].count_ones();
        let white_bishop_val: u32    = fen.array[BISHOP_W].count_ones();
        let black_bishop_val: u32    = fen.array[BISHOP_B].count_ones();
        let white_rook_val: u32      = fen.array[ROOK_W].count_ones();
        let black_rook_val: u32      = fen.array[ROOK_B].count_ones();
        let white_queen_val: u32     = fen.array[QUEEN_W].count_ones();
        let black_queen_val: u32     = fen.array[QUEEN_B].count_ones();
        let white_king_val: u32      = fen.array[KING_W].count_ones();
        let black_king_val: u32      = fen.array[KING_B].count_ones();

        let game_phase: i32 = 
            GAME_PHASE_VALUES[KNIGHT]   * (white_knight_val + black_knight_val) as i32 +
            GAME_PHASE_VALUES[BISHOP]   * (white_bishop_val + black_bishop_val) as i32 +
            GAME_PHASE_VALUES[ROOK]     * (white_rook_val + black_rook_val) as i32 +
            GAME_PHASE_VALUES[QUEEN]    * (white_queen_val + black_queen_val) as i32;

        let pawn_diff: i32       = white_pawn_val as i32         - black_pawn_val as i32;
        let knight_diff: i32     = white_knight_val as i32       - black_knight_val as i32;
        let bishop_diff: i32     = white_bishop_val as i32       - black_bishop_val as i32;
        let rook_diff: i32       = white_rook_val as i32         - black_rook_val as i32;
        let queen_diff: i32      = white_queen_val as i32        - black_queen_val as i32;
        let king_diff: i32       = white_king_val as i32         - black_king_val as i32;

        let midgame_score: i32 = 
            MIDGAME_VALUES[PAWN]        * pawn_diff +
            MIDGAME_VALUES[KNIGHT]      * knight_diff +
            MIDGAME_VALUES[BISHOP]      * bishop_diff +
            MIDGAME_VALUES[ROOK]        * rook_diff +
            MIDGAME_VALUES[QUEEN]       * queen_diff +
            MIDGAME_VALUES[KING]        * king_diff;

        let endgame_score: i32 = 
            ENDGAME_VALUES[PAWN]        * pawn_diff +
            ENDGAME_VALUES[KNIGHT]      * knight_diff +
            ENDGAME_VALUES[BISHOP]      * bishop_diff +
            ENDGAME_VALUES[ROOK]        * rook_diff +
            ENDGAME_VALUES[QUEEN]       * queen_diff +
            ENDGAME_VALUES[KING]        * king_diff;

        let midgame_phase: i32 = game_phase.min(24);
        let endgame_phase: i32 = 24 - midgame_phase;

        // We reward minimizing the number of moves of the opponent king to encourage attacking in endgame
        let opp_king_attacks = if white_to_move { attacks::king_attack(fen.array[KING_B]) } else { attacks::king_attack(fen.array[KING_W]) };
        let current_attacks = if white_to_move { attacks::get_white_attacks(&fen.array) } else { attacks::get_black_attacks(&fen.array) };
        let opp_king_movement = (opp_king_attacks & !current_attacks).count_ones() as i32;

        let material_score: i32 = (midgame_score * midgame_phase + endgame_score * endgame_phase) / 24;
        let score = material_score - 5 * opp_king_movement;

        if white_to_move { score } else { -score }
    }
}
