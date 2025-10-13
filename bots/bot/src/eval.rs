use fenlib::fen::*;
use fenlib::utils::*;

// Inspired by https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

const PAWN: usize = 0;
const KNIGHT: usize = 1;
const BISHOP: usize = 2;
const ROOK: usize = 3;
const QUEEN: usize = 4;
const KING: usize = 5;

const MIDGAME_VALUES: [u32; 6] = [ 82, 337, 365, 477, 1025, 20000 ];
const ENDGAME_VALUES: [u32; 6] = [ 94, 281, 297, 512, 936, 20000 ];

pub fn midgame_material_score(fen: &Fen) -> i32 {
    let mut white_material_score: u32 = 0;

    white_material_score += MIDGAME_VALUES[PAWN] * fen.array[PAWN_W].count_ones();
    white_material_score += MIDGAME_VALUES[KNIGHT] * fen.array[KNIGHT_W].count_ones();
    white_material_score += MIDGAME_VALUES[BISHOP] * fen.array[BISHOP_W].count_ones();
    white_material_score += MIDGAME_VALUES[ROOK] * fen.array[ROOK_W].count_ones();
    white_material_score += MIDGAME_VALUES[QUEEN] * fen.array[QUEEN_W].count_ones();
    white_material_score += MIDGAME_VALUES[KING] * fen.array[KING_W].count_ones();

    let mut black_material_score: u32 = 0;

    black_material_score += MIDGAME_VALUES[PAWN] * fen.array[PAWN_B].count_ones();
    black_material_score += MIDGAME_VALUES[KNIGHT] * fen.array[KNIGHT_B].count_ones();
    black_material_score += MIDGAME_VALUES[BISHOP] * fen.array[BISHOP_B].count_ones();
    black_material_score += MIDGAME_VALUES[ROOK] * fen.array[ROOK_B].count_ones();
    black_material_score += MIDGAME_VALUES[QUEEN] * fen.array[QUEEN_B].count_ones();
    black_material_score += MIDGAME_VALUES[KING] * fen.array[KING_B].count_ones();

    let score: i32 = (white_material_score as i32) - (black_material_score as i32);

    score
}

pub fn endgame_material_score(fen: &Fen) -> i32 {
    let mut white_material_score: u32 = 0;

    white_material_score += ENDGAME_VALUES[PAWN] * fen.array[PAWN_W].count_ones();
    white_material_score += ENDGAME_VALUES[KNIGHT] * fen.array[KNIGHT_W].count_ones();
    white_material_score += ENDGAME_VALUES[BISHOP] * fen.array[BISHOP_W].count_ones();
    white_material_score += ENDGAME_VALUES[ROOK] * fen.array[ROOK_W].count_ones();
    white_material_score += ENDGAME_VALUES[QUEEN] * fen.array[QUEEN_W].count_ones();
    white_material_score += ENDGAME_VALUES[KING] * fen.array[KING_W].count_ones();

    let mut black_material_score: u32 = 0;

    black_material_score += ENDGAME_VALUES[PAWN] * fen.array[PAWN_B].count_ones();
    black_material_score += ENDGAME_VALUES[KNIGHT] * fen.array[KNIGHT_B].count_ones();
    black_material_score += ENDGAME_VALUES[BISHOP] * fen.array[BISHOP_B].count_ones();
    black_material_score += ENDGAME_VALUES[ROOK] * fen.array[ROOK_B].count_ones();
    black_material_score += ENDGAME_VALUES[QUEEN] * fen.array[QUEEN_B].count_ones();
    black_material_score += ENDGAME_VALUES[KING] * fen.array[KING_B].count_ones();

    let score: i32 = (white_material_score as i32) - (black_material_score as i32);

    score
}

pub fn eval(fen: &Fen) -> i32 {
    
}