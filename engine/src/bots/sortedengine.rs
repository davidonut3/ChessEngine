use std::time::Duration;
use std::time::Instant;

use crate::bots::matchup::Engine;
use crate::bots::opening_book::get_opening_move;
use crate::fenlib::fen::Fen;
use crate::parsing;
use crate::utils::*;

// Mate/king value cannot be infinity, since that may result in integer overflows
const MATE_VALUE: i32 = 20000;

const MIDGAME_VALUES: [i32; 6] = [ 82, 337, 365, 477, 1025, MATE_VALUE ];
const ENDGAME_VALUES: [i32; 6] = [ 94, 281, 297, 512, 936, MATE_VALUE ];
const GAME_PHASE_VALUES: [i32; 5] = [0, 1, 1, 2, 4];

const SCORING_NUMBER: i32 = 100_000;

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
        if move_count == 0 { panic!("select_move: No moves available") };

        if let Some(move1) = self.opening_move(moves, true) { return move1 }

        let mut best_move: Move;

        let mut depth: i32 = 1;
        loop {
            best_move = self.negamax_root(moves, move_count, start_time, max_time, depth);

            if start_time.elapsed() > max_time { break; }

            depth += 1;
        }

        println!("{:?}", start_time.elapsed());

        best_move
    }

    fn apply_move(&mut self, move1: Move) {
        self.fen.move_to_fen(move1);
    }

    fn name(&self) -> String {
        "SortedEngine".to_string()
    }
}

impl SortedEngine {
    fn opening_move(&mut self, moves: MoveArray, pick_random: bool) -> Option<Move> {
        if self.use_opening_book {
            let partial_zobrist: u64 = self.fen.get_partial_zobrist();

            if let Some(opening_move) = get_opening_move(partial_zobrist, pick_random) {

                let move1: Move = parsing::compact_to_move(&opening_move);

                // In the very rare case we get a hash collision with an opening position, we have to make sure the move we find is legal
                if moves.contains(&move1) { return Some(move1) }

            } else {
                self.use_opening_book = false;
            }
        }

        return None
    }

    fn negamax(&self, fen: &Fen, depth: i32, mut alpha: i32, beta: i32) -> i32 {

        let (moves, move_count) = fen.get_legal_moves_array();

        let mut scored_moves: Vec<(Move, i32, Fen)> = Vec::with_capacity(move_count);

        for i in 0..move_count {

            let move1: Move = moves[i];
            let mut score: i32 = 0;

            let attacker: i32;
            if move1[2] != 0 {
                // In case of promotion, we add 100_000 to score and set attacker to 0 since it must be a pawn
                score += SCORING_NUMBER;
                attacker = 0;
            } else {
                // This only panics if there is no piece that moves, in which case we want a panic
                attacker = fen.piece_on_square_no_color(move1[0]).unwrap() as i32;
            }

            // We subtract the score of the attacker to prevent king shuffling among other things
            score -= attacker + 1;

            if let Some(victim) = fen.piece_on_square_no_color(move1[1]) {
                // In case of a capture, we add 100_000 + 10 times the value of the victim to the score
                score += SCORING_NUMBER + (victim + 1) as i32 * 10;
            }

            let mut new_fen = fen.clone();
            new_fen.move_to_fen(move1);

            // In case the move we will do results in a check, we add 100_000 to the score
            if new_fen.player_in_check(new_fen.white_to_move()) {
                score += SCORING_NUMBER;
            }

            scored_moves.push((move1, score, new_fen));

        }

        scored_moves.sort_unstable_by(|a, b| b.1.cmp(&a.1));

        // WE CAN REMOVE MOVE FROM SCORED_MOVES
        println!();
        for i in 0..move_count {
            println!("{:?} {:?}", parsing::move_to_lan(&scored_moves[i].0), scored_moves[i].1)
        }

        // We break early if there is a stalemate or a checkmate
        if move_count == 0 {
            if fen.player_in_check(fen.white_to_move()) { return -MATE_VALUE - depth; } else { return 0; }
        };

        if depth == 0 { return self.eval(fen) }

        let mut value: i32 = -INFINITY;
        
        for i in 0..move_count {
            let score: i32 = -self.negamax(&scored_moves[i].2, depth - 1, -beta, -alpha);

            if score > value { value = score }
            if value > alpha { alpha = value }

            if alpha >= beta { break }
        }

        value
    }

    fn negamax_root(&self, moves: MoveArray, move_count: usize, start_time: Instant, max_time: Duration, depth: i32) -> Move {

        // println!("\n{:?}\n", depth);

        let mut alpha: i32 = -INFINITY;
        let beta: i32 = INFINITY;

        let mut best_move: Move = moves[0];
        let mut best_score: i32 = -INFINITY;

        for i in 0..move_count {
            if start_time.elapsed() > max_time { break; }

            let move1: Move = moves[i];
            let mut new_fen = self.fen.clone();
            new_fen.move_to_fen(move1);

            let score: i32 = -self.negamax(&new_fen, depth, -beta, -alpha);

            // println!("Move {}, score {:?}", parsing::move_to_lan(&move1), score);

            if score > best_score {
                best_score = score;
                best_move = move1;
            }

            if score > alpha { alpha = score }
        }

        // println!("Best move {}", parsing::move_to_lan(&best_move));

        best_move
    }

    fn eval(&self, fen: &Fen) -> i32 {
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

        let score: i32 = (midgame_score * midgame_phase + endgame_score * endgame_phase) / 24;

        if fen.white_to_move() { score } else { -score }
    }

    fn _sort_moves(&self, fen: Fen, moves: MoveArray, move_count: usize) -> MoveArray {
        /* How we score moves:

        We add +100_000 for each of the following:

        Check
        Capture
        Promotion

        Whenever there is a capture, we add a score for MVV-LVA
        Whenever there is not a capture, we subtract attacker value so that pawns go first, king goes last

        Sort high to low 

        queen = 5, pawn = 1

        Victim queen, attacking pawn, should appear first, 10v-a = 49

        Victim pawn, attacking queen, should appear last, 10v-a = 5

        a v p   n   b   r   q   k   
        p   9   19  29  39  49  59
        n   8   18  28  38  48  58
        b   7   17  27  37  47  57
        r   6   16  26  36  46  56
        q   5   15  25  35  45  55
        k   4   14  24  34  44  54 
        */

        // We assume the moves are legal

        let sorted: MoveArray = [[0; 3]; MAX_MOVES];
        let mut scored_moves: Vec<(Move, i32)> = Vec::with_capacity(move_count);

        for i in 0..move_count {

            let move1: Move = moves[i];
            let mut score: i32 = 0;

            let attacker: i32;
            if move1[2] != 0 {
                // In case of promotion, we add 100_000 to score and set attacker to 0 since it must be a pawn
                score += SCORING_NUMBER;
                attacker = 0;
            } else {
                // This only panics if there is no piece that moves, in which case we want a panic
                attacker = fen.piece_on_square_no_color(move1[0]).unwrap() as i32;
            }

            // We subtract the score of the attacker to prevent king shuffling among other things
            score -= attacker + 1;

            if let Some(victim) = fen.piece_on_square_no_color(move1[1]) {
                // In case of a capture, we add 100_000 + 10 times the value of the victim to the score
                score += SCORING_NUMBER + (victim + 1) as i32 * 10;
            }

            // Something to check whether the move puts opponent in check

            scored_moves.push((move1, score));

        }

        sorted
    }
}
