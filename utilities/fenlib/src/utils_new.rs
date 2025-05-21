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

pub const RANKS: [u128; 8] = [RANK_0, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7];
pub const FILES: [u128; 8] = [FILE_0, FILE_1, FILE_2, FILE_3, FILE_4, FILE_5, FILE_6, FILE_7];

// -------------------- Info Index --------------------

pub const PAWNS: usize = 0;
pub const KINGS: usize = 1;
pub const QUEENS: usize = 2;
pub const BISHOPS: usize = 3;
pub const KNIGHTS: usize = 4;
pub const ROOKS: usize = 5;
pub const PIECES: usize = 6;
pub const INFO: usize = 7;

pub const ARRAY_SIZE: usize = 8;

// Default array for Fen object
pub const DEFAULT_FEN: [u128; ARRAY_SIZE] = [
    0b00000000000000000000000011111111000000000000000000000000000000000000000000000000000000000000000011111111000000000000000000000000,
    0b00000000000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100000000000,
    0b00000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000000000,
    0b00000000001001000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000010010000000000,
    0b00000000010000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000100001000000000,
    0b00000000100000010000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000001000000100000000,
    0b00000000111111110000000011111111000000000000000000000000000000000000000000000000000000000000000011111111000000001111111100000000,
    0b00000000000000000000000000000001000000001111100000000000000000000000000000000000000000000000000000000000000000000000000000000000,
];

// Locations in game_info where the respective game info is stored
pub const TURN: u128 = 0x800000000000000000000;
pub const HALFMOVE: u128 = 0xFF0000000000000000000000000000;
pub const FULLMOVE: u128 = 0xFF000000000000000000000000;
pub const CASTLING: u128 = 0xF000000000000000000000;

// -------------------- White Castling --------------------



// -------------------- Black Castling --------------------



// -------------------- Castling Info Flags --------------------



// -------------------- Default Starting Position --------------------

/// Standard FEN string for the initial chess position.
pub const DEFAULT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// ==================== Helper Functions ====================

/// Combine all white piece positions and all black piece positions into a single bitboard.
pub fn get_pieces(pieces: &[u128; ARRAY_SIZE]) -> u128 {
    pieces[PAWNS] | pieces[KINGS] | pieces[QUEENS] | pieces[BISHOPS] | pieces[KNIGHTS] | pieces[ROOKS]
}