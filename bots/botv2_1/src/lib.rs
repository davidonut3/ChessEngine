use fenlib::fen::*;
use fenlib::utils::*;
use fenlib::parsing;
use std::time::Instant;
use std::time::Duration;

/*
BotV2_1 implements:

- minimax algorithm to find the best move
- alpha-beta pruning to disregard bad moves
- eval function based on material score
- time constraint for searching tree
- iterative deepening
- repetition prevention

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
const NEG_INFINITY: u32 = 0;
const MAX_TIME_MILI: u64 = 1000;
const EPSILON: u32 = 110;

pub fn eval(fen: &Fen) -> u32 {
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
        true => NEG_INFINITY,
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
    prev_moves: Vec<Move>,
}

impl Bot {
    pub fn new() -> Self {
        let fen: Fen = Fen::new();

        Self { fen, prev_moves: Vec::new() }
    }

    pub fn from_fen(fen_str: &str) -> Self {
        let fen: Fen = Fen::from_str(fen_str);

        Self { fen, prev_moves: Vec::new() }
    }

    pub fn minimax(&self, fen: Fen, depth: u32, alpha: u32, beta: u32, start_time: Instant, max_time: Duration) -> Option<u32> {

        if start_time.elapsed() >= max_time {
            return None
        }
        
        if depth == 0 {
            return Some(eval(&fen))
        }

        let white_to_move: bool = fen.white_to_move();
        let mut value: u32 = get_worst_eval(white_to_move);

        let mut new_alpha: u32 = alpha;
        let mut new_beta: u32 = beta;

        let (moves, move_count) = fen.get_legal_moves_array();

        for i in 0..move_count {

            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(moves[i]);

            if let Some(new_value) = self.minimax(new_fen, depth - 1, new_alpha, new_beta, start_time, max_time) {
                value = get_better_move(white_to_move, value, new_value);

                if white_to_move {
                    if value > new_alpha {
                        new_alpha = value;
                    }
                } else {
                    if value < new_beta {
                        new_beta = value;
                    }
                }

                if new_beta <= new_alpha {
                    break;
                }

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

        if move_count == 0 {
            panic!("get_move: No moves available")
        }

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
        
                if let Some(new_score) = self.minimax(new_fen, depth, NEG_INFINITY, INFINITY, start_time, max_time) {

                    if (white_to_move && new_score > best_score) || (!white_to_move && new_score < best_score) {
                        best_score = new_score;
                        best_move = move1;
                    }

                } else {

                    if best_score < best_prev_score {
                        best_move = best_prev_move
                    }

                    let mut is_prev_move: bool = false;

                    for prev_move in &self.prev_moves {
                        if &best_move == prev_move {
                            is_prev_move = true;
                        }
                    }

                    if is_prev_move {
                        for j in 0..move_count {

                            let move2 = moves[j];
                            
                            let mut new_fen = self.fen.clone();
                            new_fen.move_to_fen(move2);
                            let new_score = eval(&new_fen);

                            let mut is_prev_move: bool = false;

                            for prev_move in &self.prev_moves {
                                if &move2 == prev_move {
                                    is_prev_move = true;
                                }
                            }

                            if !is_prev_move && best_score - new_score < EPSILON {
                                best_move = move2;
                                break;
                            }
                        }
                    }

                    self.prev_moves.push(best_move);

                    self.fen.move_to_fen(best_move);
                    return parsing::move_to_lan(&best_move)
                }
            }

            depth += 1;
        }
    }

    pub fn receive_move(&mut self, lan: &str) {
        self.fen.lan_to_fen(lan);
    }
}