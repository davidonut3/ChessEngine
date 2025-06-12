use fenlib::fen::*;
use fenlib::utils::*;
use fenlib::parsing;
use std::time::Instant;
use std::time::Duration;

// Values from https://www.chessprogramming.org/Simplified_Evaluation_Function
const PAWN_VAL: u32 = 100;
const KNIGHT_VAL: u32 = 320;
const BISHOP_VAL: u32 = 330;
const ROOK_VAL: u32 = 500;
const QUEEN_VAL: u32 = 900;
const KING_VAL: u32 = 20000;

pub fn eval(fen: &Fen) -> i32 {
    let mut white_material_score: u32 = 0;

    white_material_score += PAWN_VAL * fen.array[PAWN_W].count_ones();
    white_material_score += KNIGHT_VAL * fen.array[KNIGHT_W].count_ones();
    white_material_score += BISHOP_VAL * fen.array[BISHOP_W].count_ones();
    white_material_score += ROOK_VAL * fen.array[ROOK_W].count_ones();
    white_material_score += QUEEN_VAL * fen.array[QUEEN_W].count_ones();
    white_material_score += KING_VAL * fen.array[KING_W].count_ones();

    let mut black_material_score: u32 = 0;

    black_material_score += PAWN_VAL * fen.array[PAWN_B].count_ones();
    black_material_score += KNIGHT_VAL * fen.array[KNIGHT_B].count_ones();
    black_material_score += BISHOP_VAL * fen.array[BISHOP_B].count_ones();
    black_material_score += ROOK_VAL * fen.array[ROOK_B].count_ones();
    black_material_score += QUEEN_VAL * fen.array[QUEEN_B].count_ones();
    black_material_score += KING_VAL * fen.array[KING_B].count_ones();

    let score: i32 = (white_material_score as i32) - (black_material_score as i32);

    score
}