use fenlib::fen::*;
use std::time::Instant;

// Values from https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_VAL: u32 = 100;
const KNIGHT_VAL: u32 = 320;
const BISHOP_VAL: u32 = 330;
const ROOK_VAL: u32 = 500;
const QUEEN_VAL: u32 = 900;
const KING_VAL: u32 = 20000;

const EQUAL: u32 = 0x80000000;

const INFINITY: u32 = u32::max_value();
const MAX_DEPTH: u32 = 4;

pub fn eval(boards: &[u64; 12]) -> u32 {
    let mut score: u32 = EQUAL;

    score += PAWN_VAL * boards[0].count_ones();
    score -= PAWN_VAL * boards[6].count_ones();

    score += KNIGHT_VAL * boards[1].count_ones();
    score -= KNIGHT_VAL * boards[7].count_ones();

    score += BISHOP_VAL * boards[2].count_ones();
    score -= BISHOP_VAL * boards[8].count_ones();

    score += ROOK_VAL * boards[3].count_ones();
    score -= ROOK_VAL * boards[9].count_ones();

    score += QUEEN_VAL * boards[4].count_ones();
    score -= QUEEN_VAL * boards[10].count_ones();

    score += KING_VAL * boards[5].count_ones();
    score -= KING_VAL * boards[11].count_ones();

    score
}

#[derive(Debug, Clone)]
pub struct BotV1 {
    fen: Fen,
}

impl BotV1 {
    pub fn new() -> Self {
        let fen: Fen = Fen::new();

        Self { fen, }
    }

    pub fn from_fen(fen_str: &str) -> Self {
        let fen: Fen = Fen::from_str(fen_str);

        Self { fen, }
    }

    pub fn get_move(&mut self) -> String {
        let t1: Instant = Instant::now();

        let possible_moves: Vec<[u64; 3]> = self.fen.get_all_possible_moves();
        let mut best_move: [u64; 3] = possible_moves[0];
        let mut best_score: u32 = match self.fen.white_to_move {
            true => 0x0,
            false => INFINITY
        };

        for move1 in possible_moves {
            let mut new_fen: Fen = self.fen.clone();
            new_fen.move_to_fen(&move1);
            let new_score: u32 = minimax(new_fen, MAX_DEPTH - 1);

            if self.fen.white_to_move && new_score > best_score {
                best_score = new_score;
                best_move = move1;
            } else if new_score < best_score {
                best_score = new_score;
                best_move = move1;
            }
        }

        self.fen.move_to_fen(&best_move);

        println!("Calculating move took {:?}", t1.elapsed());

        fenlib::parsing::move_to_lan(&best_move)
    }

    pub fn receive_move(&mut self, lan: &str) {
        self.fen.lan_to_fen(lan);
    }
}

pub fn minimax(fen: Fen, depth: u32) -> u32 {
    if depth == 0 {
        let eval: u32 = eval(&fen.boards);
        return eval
    }

    if fen.white_to_move {
        let mut value: u32 = 0x0;
        let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();
        for move1 in possible_moves {
            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(&move1);
            let new_value: u32 = minimax(new_fen, depth - 1);
            if new_value > value {
                value = new_value
            }
        }
        return value;
    } else {
        let mut value: u32 = INFINITY;
        let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();
        for move1 in possible_moves {
            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(&move1);
            let new_value: u32 = minimax(new_fen, depth - 1);
            if new_value < value {
                value = new_value
            }
        }
        return value;
    }
}