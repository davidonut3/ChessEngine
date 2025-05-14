/// Bitboard representing an empty set (no pieces).
pub const EMPTY: u64 = 0x0;

/// Bitboard with only the top-left square (A8) set.
pub const FIRST: u64 = 0x8000000000000000;

/// Bitboard with the top (8th) rank set.
pub const RANK: u64 = 0xFF00000000000000;

/// Bitboard with the leftmost (A-file) set.
pub const FILE: u64 = 0x8080808080808080;

pub const RANK_0: u64 = RANK >> 0 * 8; // Rank 1
pub const RANK_1: u64 = RANK >> 1 * 8; // Rank 2
pub const RANK_2: u64 = RANK >> 2 * 8; // Rank 3
pub const RANK_3: u64 = RANK >> 3 * 8; // Rank 4
pub const RANK_4: u64 = RANK >> 4 * 8; // Rank 5
pub const RANK_5: u64 = RANK >> 5 * 8; // Rank 6
pub const RANK_6: u64 = RANK >> 6 * 8; // Rank 7
pub const RANK_7: u64 = RANK >> 7 * 8; // Rank 8

pub const FILE_0: u64 = FILE >> 0; // File a
pub const FILE_1: u64 = FILE >> 1; // File b
pub const FILE_2: u64 = FILE >> 2; // File c
pub const FILE_3: u64 = FILE >> 3; // File d
pub const FILE_4: u64 = FILE >> 4; // File e
pub const FILE_5: u64 = FILE >> 5; // File f
pub const FILE_6: u64 = FILE >> 6; // File g
pub const FILE_7: u64 = FILE >> 7; // File h

pub const RANKS: [u64; 8] = [RANK_0, RANK_1, RANK_2, RANK_3, RANK_4, RANK_5, RANK_6, RANK_7];
pub const FILES: [u64; 8] = [FILE_0, FILE_1, FILE_2, FILE_3, FILE_4, FILE_5, FILE_6, FILE_7];

// -------------------- White Castling --------------------

/// H1 (white kingside rook starting square).
pub const WHITE_KINGSIDE_BIT: u64 = FIRST >> (7 * 8) >> 6;

/// Squares that must be empty for white kingside castling (F1 and G1).
pub const WHITE_KINGSIDE_CLEAR: u64 = WHITE_KINGSIDE_BIT | (WHITE_KINGSIDE_BIT << 1);

/// E1 (white king starting square).
pub const WHITE_KING_BIT: u64 = WHITE_KINGSIDE_BIT << 2;

/// A1 (white queenside rook starting square).
pub const WHITE_QUEENSIDE_BIT: u64 = FIRST >> (7 * 8) >> 2;

/// Squares that must be empty for white queenside castling (B1, C1, D1).
pub const WHITE_QUEENSIDE_CLEAR: u64 = WHITE_QUEENSIDE_BIT | (WHITE_QUEENSIDE_BIT >> 1) | (WHITE_QUEENSIDE_BIT << 1);

// -------------------- Black Castling --------------------

/// H8 (black kingside rook starting square).
pub const BLACK_KINGSIDE_BIT: u64 = FIRST >> 6;

/// Squares that must be empty for black kingside castling (F8 and G8).
pub const BLACK_KINGSIDE_CLEAR: u64 = BLACK_KINGSIDE_BIT | (BLACK_KINGSIDE_BIT << 1);

/// E8 (black king starting square).
pub const BLACK_KING_BIT: u64 = BLACK_KINGSIDE_BIT << 2;

/// A8 (black queenside rook starting square).
pub const BLACK_QUEENSIDE_BIT: u64 = FIRST >> 2;

/// Squares that must be empty for black queenside castling (B8, C8, D8).
pub const BLACK_QUEENSIDE_CLEAR: u64 = BLACK_QUEENSIDE_BIT | (BLACK_QUEENSIDE_BIT >> 1) | (BLACK_QUEENSIDE_BIT << 1);

// -------------------- Castling Info Flags --------------------

/// Bit flag indicating white can castle kingside.
pub const WHITE_KINGSIDE_INFO: u8 = 1u8 << 3;

/// Bit flag indicating white can castle queenside.
pub const WHITE_QUEENSIDE_INFO: u8 = 1u8 << 2;

/// Bit flag indicating black can castle kingside.
pub const BLACK_KINGSIDE_INFO: u8 = 1u8 << 1;

/// Bit flag indicating black can castle queenside.
pub const BLACK_QUEENSIDE_INFO: u8 = 1u8;

// -------------------- Promotion Flags --------------------

/// Promotion to queen.
pub const QUEEN_PROM: u64 = 1u64 << 3;

/// Promotion to rook.
pub const ROOK_PROM: u64 = 1u64 << 2;

/// Promotion to bishop.
pub const BISHOP_PROM: u64 = 1u64 << 1;

/// Promotion to knight.
pub const KNIGHT_PROM: u64 = 1u64;

/// No promotion.
pub const NO_PROM: u64 = 0u64;

// -------------------- Default Starting Position --------------------

/// Standard FEN string for the initial chess position.
pub const DEFAULT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

// -------------------- Main constants --------------------

/// Max number of moves a piece can make
pub const MAX_MOVES_PIECE: usize = 28;

/// Maximum number of legal moves in a chess position.
/// The actually largest number is unknown, the known largest is 218.
/// However, the max number rarely exceeds 80-100, therefore we use 120.
pub const MAX_MOVES: usize = 120;

// ==================== Helper Functions ====================

/// Combines all white pieces from the board array into a single bitboard.
///
/// # Arguments
/// * `boards` - Array of 12 bitboards representing piece positions.
///               The first 6 entries are white pieces.
pub fn get_white(boards: &[u64; 12]) -> u64 {
    boards[0] | boards[1] | boards[2] | boards[3] | boards[4] | boards[5]
}

/// Combines all black pieces from the board array into a single bitboard.
///
/// # Arguments
/// * `boards` - Array of 12 bitboards representing piece positions.
///               The last 6 entries are black pieces.
pub fn get_black(boards: &[u64; 12]) -> u64 {
    boards[6] | boards[7] | boards[8] | boards[9] | boards[10] | boards[11]
}