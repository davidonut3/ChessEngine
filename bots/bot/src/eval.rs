use fenlib::fen::*;
use fenlib::utils::*;

// Inspired by https://www.chessprogramming.org/PeSTO%27s_Evaluation_Function

/*

NOTES:

To increase efficiency, we can check whether we are in complete midgame or endgame, so we skip a bunch of computations

The current 'fast' structure (potentially?) breaks when we incorporate piece square tables

Remove king checking here and add it in the algorithm

*/

const PAWN: usize = 0;
const KNIGHT: usize = 1;
const BISHOP: usize = 2;
const ROOK: usize = 3;
const QUEEN: usize = 4;
const KING: usize = 5;

const MIDGAME_VALUES: [i32; 6] = [ 82, 337, 365, 477, 1025, 20000 ];
const ENDGAME_VALUES: [i32; 6] = [ 94, 281, 297, 512, 936, 20000 ];
const GAME_PHASE_VALUES: [i32; 5] = [0, 1, 1, 2, 4];

pub fn eval(fen: &Fen) -> i32 {
    let white_pawn_val      = fen.array[PAWN_W].count_ones();
    let black_pawn_val      = fen.array[PAWN_B].count_ones();
    let white_knight_val    = fen.array[KNIGHT_W].count_ones();
    let black_knight_val    = fen.array[KNIGHT_B].count_ones();
    let white_bishop_val    = fen.array[BISHOP_W].count_ones();
    let black_bishop_val    = fen.array[BISHOP_B].count_ones();
    let white_rook_val      = fen.array[ROOK_W].count_ones();
    let black_rook_val      = fen.array[ROOK_B].count_ones();
    let white_queen_val     = fen.array[QUEEN_W].count_ones();
    let black_queen_val     = fen.array[QUEEN_B].count_ones();
    let white_king_val      = fen.array[KING_W].count_ones();
    let black_king_val      = fen.array[KING_B].count_ones();

    let game_phase = 
        GAME_PHASE_VALUES[KNIGHT]   * (white_knight_val + black_knight_val) as i32 +
        GAME_PHASE_VALUES[BISHOP]   * (white_bishop_val + black_bishop_val) as i32 +
        GAME_PHASE_VALUES[ROOK]     * (white_rook_val + black_rook_val) as i32 +
        GAME_PHASE_VALUES[QUEEN]    * (white_queen_val + black_queen_val) as i32;

    let pawn_diff       = white_pawn_val as i32         - black_pawn_val as i32;
    let knight_diff     = white_knight_val as i32       - black_knight_val as i32;
    let bishop_diff     = white_bishop_val as i32       - black_bishop_val as i32;
    let rook_diff       = white_rook_val as i32         - black_rook_val as i32;
    let queen_diff      = white_queen_val as i32        - black_queen_val as i32;
    let king_diff       = white_king_val as i32         - black_king_val as i32;

    let midgame_score = 
        MIDGAME_VALUES[PAWN]        * pawn_diff +
        MIDGAME_VALUES[KNIGHT]      * knight_diff +
        MIDGAME_VALUES[BISHOP]      * bishop_diff +
        MIDGAME_VALUES[ROOK]        * rook_diff +
        MIDGAME_VALUES[QUEEN]       * queen_diff +
        MIDGAME_VALUES[KING]        * king_diff;

    let endgame_score = 
        ENDGAME_VALUES[PAWN]        * pawn_diff +
        ENDGAME_VALUES[KNIGHT]      * knight_diff +
        ENDGAME_VALUES[BISHOP]      * bishop_diff +
        ENDGAME_VALUES[ROOK]        * rook_diff +
        ENDGAME_VALUES[QUEEN]       * queen_diff +
        ENDGAME_VALUES[KING]        * king_diff;

    let midgame_phase = game_phase.min(24);
    let endgame_phase = 24 - midgame_phase;

    let score = (midgame_score * midgame_phase + endgame_score * endgame_phase) / 24;

    if fen.white_to_move() { score } else { -score }
}