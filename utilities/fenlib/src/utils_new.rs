/// This represents the left board of the two boards
pub const BOARD1: u128 = 0xFF00FF00FF00FF00FF00FF00FF00FF00;

/// This represents the right board of the two boards
pub const BOARD2: u128 = 0xFF00FF00FF00FF00FF00FF00FF00FF;

/// Bitboard representing an empty set (no pieces).
pub const EMPTY: u128 = 0x0;

/// Bitboard with only the top-left square (A8) set.
pub const FIRST: u128 = 0x80000000000000000000000000000000;

/// Bitboard with the top (8th) rank set.
pub const RANK: u128 = 0xFFFF0000000000000000000000000000;

/// Bitboard with the leftmost (A-file) set.
pub const FILE: u128 = 0x80008000800080008000800080008000;

pub const RANK_0: u128 = RANK >> 0 * 16; // Rank 1
pub const RANK_1: u128 = RANK >> 1 * 16; // Rank 2
pub const RANK_2: u128 = RANK >> 2 * 16; // Rank 3
pub const RANK_3: u128 = RANK >> 3 * 16; // Rank 4
pub const RANK_4: u128 = RANK >> 4 * 16; // Rank 5
pub const RANK_5: u128 = RANK >> 5 * 16; // Rank 6
pub const RANK_6: u128 = RANK >> 6 * 16; // Rank 7
pub const RANK_7: u128 = RANK >> 7 * 16; // Rank 8

pub const FILE_0: u128 = FILE >> 0; // File a
pub const FILE_1: u128 = FILE >> 1; // File b
pub const FILE_2: u128 = FILE >> 2; // File c
pub const FILE_3: u128 = FILE >> 3; // File d
pub const FILE_4: u128 = FILE >> 4; // File e
pub const FILE_5: u128 = FILE >> 5; // File f
pub const FILE_6: u128 = FILE >> 6; // File g
pub const FILE_7: u128 = FILE >> 7; // File h

// I am unsure whether these are necessary

pub const FILE_8: u128 = FILE >> 8; // File i
pub const FILE_9: u128 = FILE >> 9; // File j
pub const FILE_10: u128 = FILE >> 10; // File k
pub const FILE_11: u128 = FILE >> 11; // File l
pub const FILE_12: u128 = FILE >> 12; // File m
pub const FILE_13: u128 = FILE >> 13; // File n
pub const FILE_14: u128 = FILE >> 14; // File o
pub const FILE_15: u128 = FILE >> 15; // File p

pub const RANKS: [u128; 8] = [RANK_0, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7];
pub const FILES: [u128; 8] = [FILE_0, FILE_1, FILE_2, FILE_3, FILE_4, FILE_5, FILE_6, FILE_7];

// -------------------- Piece Names --------------------

// NOTE: it should not matter if the pawns are in the wrong place in the array,
// as long as all pawns are in separate places.

pub const PAWN_A: usize = 0;
pub const PAWN_B: usize = 1;
pub const PAWN_C: usize = 2;
pub const PAWN_D: usize = 3;
pub const PAWN_E: usize = 4;
pub const PAWN_F: usize = 5;
pub const PAWN_G: usize = 6;
pub const PAWN_H: usize = 7;

pub const KING: usize = 8;
pub const QUEEN: usize = 9;
pub const BISHOP_K: usize = 10;
pub const BISHOP_Q: usize = 11;
pub const KNIGHT_K: usize = 12;
pub const KNIGHT_Q: usize = 13;
pub const ROOK_K: usize = 14;
pub const ROOK_Q: usize = 15;

// -------------------- Info index --------------------

// Locations in game_info where the respective game info is stored
pub const TURN: u128 = 0x8000000;
pub const HALFMOVE: u128 = 0xFFFF000000000000;
pub const FULLMOVE: u128 = 0xFFFF00000000;
pub const CASTLING: u128 = 0xF0000000;

// How far the 4 bits of each respective piece must move for the right location in game_info
pub const PAWN_A_W_PROM: u128 = 0;
pub const PAWN_B_W_PROM: u128 = 1;
pub const PAWN_C_W_PROM: u128 = 2;
pub const PAWN_D_W_PROM: u128 = 3;
pub const PAWN_E_W_PROM: u128 = 4;
pub const PAWN_F_W_PROM: u128 = 5;
pub const PAWN_G_W_PROM: u128 = 6;
pub const PAWN_H_W_PROM: u128 = 7;
pub const PAWN_A_B_PROM: u128 = 8;
pub const PAWN_B_B_PROM: u128 = 9;
pub const PAWN_C_B_PROM: u128 = 10;
pub const PAWN_D_B_PROM: u128 = 11;
pub const PAWN_E_B_PROM: u128 = 12;
pub const PAWN_F_B_PROM: u128 = 13;
pub const PAWN_G_B_PROM: u128 = 14;
pub const PAWN_H_B_PROM: u128 = 15;

// -------------------- White Castling --------------------



// -------------------- Black Castling --------------------



// -------------------- Castling Info Flags --------------------



// -------------------- Promotion Flags --------------------

pub const QUEEN_PROM: u128 = FIRST;
pub const BISHOP_PROM: u128 = FIRST >> 1;
pub const KNIGHT_PROM: u128 = FIRST >> 2;
pub const ROOK_PROM: u128 = FIRST >> 3;

// -------------------- Default Starting Position --------------------

/// Standard FEN string for the initial chess position.
pub const DEFAULT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// ==================== Helper Functions ====================

/// Combines all pieces of a color into a single bitboard.
pub fn get_all_pieces(pieces: &[u128; 16]) -> u128 {
    pieces[PAWN_A] |
    pieces[PAWN_B] |
    pieces[PAWN_C] |
    pieces[PAWN_D] |
    pieces[PAWN_E] |
    pieces[PAWN_F] |
    pieces[PAWN_G] |
    pieces[PAWN_H] |
    pieces[KING] | 
    pieces[QUEEN] |
    pieces[BISHOP_K] |
    pieces[BISHOP_Q] |
    pieces[KNIGHT_K] |
    pieces[KNIGHT_Q] |
    pieces[ROOK_K] |
    pieces[ROOK_Q]
}

pub fn promote_pawn(index: usize, is_white: bool, promote_info: u128) -> u128 {
    if is_white {
        match index {
            PAWN_A => return promote_info >> PAWN_A_W_PROM,
            PAWN_B => return promote_info >> PAWN_B_W_PROM,
            PAWN_C => return promote_info >> PAWN_C_W_PROM,
            PAWN_D => return promote_info >> PAWN_D_W_PROM,
            PAWN_E => return promote_info >> PAWN_E_W_PROM,
            PAWN_F => return promote_info >> PAWN_F_W_PROM,
            PAWN_G => return promote_info >> PAWN_G_W_PROM,
            PAWN_H => return promote_info >> PAWN_H_W_PROM,
            _ => panic!("promote_pawn: This pawn index is not allowed")
        }
    } else {
        match index {
            PAWN_A => return promote_info >> PAWN_A_B_PROM,
            PAWN_B => return promote_info >> PAWN_B_B_PROM,
            PAWN_C => return promote_info >> PAWN_C_B_PROM,
            PAWN_D => return promote_info >> PAWN_D_B_PROM,
            PAWN_E => return promote_info >> PAWN_E_B_PROM,
            PAWN_F => return promote_info >> PAWN_F_B_PROM,
            PAWN_G => return promote_info >> PAWN_G_B_PROM,
            PAWN_H => return promote_info >> PAWN_H_B_PROM,
            _ => panic!("promote_pawn: This pawn index is not allowed")
        }
    }
}