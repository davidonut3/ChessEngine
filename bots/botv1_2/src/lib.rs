use fenlib::fen::*;
use fenlib::utils::*;
use fenlib::parsing;
use std::time::Instant;
use std::time::Duration;

/*
BotV1_2 implements:

- minimax algorithm to find the best move
- eval function based on material score
- time constraint for searching tree
- iterative deepening

*/

// Values from https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_VAL: u32 = 100;
const KNIGHT_VAL: u32 = 320;
const BISHOP_VAL: u32 = 330;
const ROOK_VAL: u32 = 500;
const QUEEN_VAL: u32 = 900;
const KING_VAL: u32 = 20000;

const EQUAL: u32 = 0x80000000;

const INFINITY: u32 = u32::max_value();
const MAX_TIME_MILI: u64 = 0x3E8; //1000 miliseconds per move

pub fn eval(fen: Fen) -> u32 {
    let mut score: u32 = EQUAL;

    score += PAWN_VAL * fen.array[PAWN_W].count_ones();
    score -= PAWN_VAL * fen.array[PAWN_B].count_ones();

    score += KNIGHT_VAL * fen.array[KNIGHT_W].count_ones();
    score -= KNIGHT_VAL * fen.array[KNIGHT_B].count_ones();

    score += BISHOP_VAL * fen.array[BISHOP_W].count_ones();
    score -= BISHOP_VAL * fen.array[BISHOP_B].count_ones();

    score += ROOK_VAL * fen.array[ROOK_W].count_ones();
    score -= ROOK_VAL * fen.array[ROOK_B].count_ones();

    score += QUEEN_VAL * fen.array[QUEEN_W].count_ones();
    score -= QUEEN_VAL * fen.array[QUEEN_B].count_ones();

    score += KING_VAL * fen.array[KING_W].count_ones();
    score -= KING_VAL * fen.array[KING_B].count_ones();

    score
}

pub fn get_worst_eval(white_to_move: bool) -> u32 {
    match white_to_move {
        true => 0x0,
        false => INFINITY
    }
}

pub fn get_better_move(white_to_move: bool, old: u32, new: u32) -> u32 {
    let new_is_greater: bool = new > old;

    if (white_to_move && new_is_greater) || (!white_to_move && !new_is_greater) {
        return new;
    } else {
        return old;
    }
}

#[derive(Debug, Clone)]
pub struct Bot {
    fen: Fen,
}

impl Bot {
    pub fn new() -> Self {
        let fen: Fen = Fen::new();

        Self { fen, }
    }

    pub fn from_fen(fen_str: &str) -> Self {
        let fen: Fen = Fen::from_str(fen_str);

        Self { fen, }
    }

    pub fn minimax(&self, fen: Fen, depth: u32, start_time: Instant, max_time: Duration) -> Option<u32> {

        if start_time.elapsed() >= max_time {
            return None
        }
        
        if depth == 0 {
            return Some(eval(fen))
        }

        let white_to_move: bool = fen.white_to_move();
        let mut value: u32 = get_worst_eval(white_to_move);
        let (moves, move_count) = fen.get_legal_moves_array();

        for i in 0..move_count {

            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(moves[i]);

            if let Some(new_value) = self.minimax(new_fen, depth - 1, start_time, max_time) {
                value = get_better_move(white_to_move, value, new_value);
            } else {
                return None;
            }
        }

        return Some(value)
    }

    pub fn get_move(&mut self) -> String {
        let start_time: Instant = Instant::now();
        let max_time: Duration = Duration::from_millis(MAX_TIME_MILI);
        let white_to_move: bool = self.fen.white_to_move();

        let (moves, move_count) = self.fen.get_legal_moves_array();
        let mut best_move: Move = moves[0];
        let mut best_score: u32 = get_worst_eval(white_to_move);
        let mut depth: u32 = 1;

        loop {
            let best_prev_move: Move = best_move;
            let best_prev_score: u32 = best_score;

            for i in 0..move_count {
                
                let mut new_fen: Fen = self.fen.clone();
                let move1 = moves[i];
                new_fen.move_to_fen(move1);
        
                if let Some(new_score) = self.minimax(new_fen, depth, start_time, max_time) {

                    if (white_to_move && new_score > best_score) || (!white_to_move && new_score < best_score) {
                        best_score = new_score;
                        best_move = move1;
                    }

                } else {

                    if best_score > best_prev_score {
                        self.fen.move_to_fen(best_move);
                        return parsing::move_to_lan(&best_move)
                    } else {
                        self.fen.move_to_fen(best_prev_move);
                        return parsing::move_to_lan(&best_prev_move)
                    }

                }
            }

            depth += 1;
        }
    }

    pub fn receive_move(&mut self, lan: &str) {
        self.fen.lan_to_fen(lan);
    }
}