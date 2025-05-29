/// First (left) board in u128 representation
pub const BOARD1: u128 = 0xFF00FF00FF00FF00FF00FF00FF00FF00;

/// Second (right) board in u128 representation
pub const BOARD2: u128 = 0xFF00FF00FF00FF00FF00FF00FF00FF;

/// Empty bitboard
pub const EMPTY: u64 = 0x0;

/// Bitboard with only the top-left square (A8) set.
pub const FIRST: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;

/// Bitboard with the top (8th) rank set.
pub const RANK: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;

/// Bitboard with the leftmost (A-file) set.
pub const FILE: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;

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

// -------------------- Chess Constants --------------------

/// Since an array of moves in the current implementation is 16 * 3 * 218 = 10464 bytes > 10 kB,
/// we might want to consider creating a struct for moves in u16 notation:
/// 6 bits for the from pos, 6 bits for the to pos, 4 bits for promotion.
pub const MAX_MOVES: usize = 218;

/// Max number of sliding pieces per side, this is 13 in a legal chess game.
/// We may want to increase it to allow for any number of sliding pieces.
pub const MAX_PINS: usize = 8;

pub const WHITE_WINS: &str = "1-0";
pub const BLACK_WINS: &str = "0-1";
pub const DRAW: &str = "½-½";
pub const NOT_ENDED: &str = "not ended";

// -------------------- Array Index --------------------

pub const UP: usize = 0;
pub const DOWN:usize = 1;
pub const LEFT: usize = 2;
pub const RIGHT: usize = 3;
pub const UPLEFT: usize = 4;
pub const UPRIGHT: usize = 5;
pub const DOWNLEFT: usize = 6;
pub const DOWNRIGHT: usize = 7;

pub const ROOK_DIRS: [usize; 4] = [UP, DOWN, LEFT, RIGHT];
pub const BISHOP_DIRS: [usize; 4] = [UPLEFT, UPRIGHT, DOWNLEFT, DOWNRIGHT];
pub const QUEEN_DIRS: [usize; 8] = [UP, DOWN, LEFT, RIGHT, UPLEFT, UPRIGHT, DOWNLEFT, DOWNRIGHT];

pub const PAWN_W: usize = 0;
pub const PAWN_B: usize = 1;
pub const KING_W: usize = 2;
pub const KING_B: usize = 3;
pub const QUEEN_W: usize = 4;
pub const QUEEN_B: usize = 5;
pub const BISHOP_W: usize = 6;
pub const BISHOP_B: usize = 7;
pub const KNIGHT_W: usize = 8;
pub const KNIGHT_B: usize = 9;
pub const ROOK_W: usize = 10;
pub const ROOK_B: usize = 11;
pub const WHITE: usize = 12;
pub const BLACK: usize = 13;
pub const INFO: usize = 14;

pub const PIECE_SIZE: usize = 12;
pub const ARRAY_SIZE: usize = 15;

pub type Array = [u64; ARRAY_SIZE];

pub type PinArray = [u64; MAX_PINS];

pub type Move = [u64; 3];
pub type MoveArray = [[u64; 3]; MAX_MOVES];

pub type OccupancyArray = [u64; 64];

// -------------------- Info Position --------------------

pub const HALFMOVE: u64 =                   0b1111111111111111000000000000000000000000000000000000000000000000;
pub const FULLMOVE: u64 =                   0b0000000000000000111111111111111100000000000000000000000000000000;
pub const ENPASSANT: u64 =                  0b0000000000000000000000000000000011111111000000000000000000000000;
pub const CASTLING: u64 =                   0b0000000000000000000000000000000000000000111100000000000000000000;
pub const TURN: u64 =                       0b0000000000000000000000000000000000000000000010000000000000000000;

// -------------------- Castling Info Flags --------------------

pub const WHITE_KINGSIDE_RIGHTS: u64 =      0b0000000000000000000000000000000000000000100000000000000000000000;
pub const WHITE_QUEENSIDE_RIGHTS: u64 =     0b0000000000000000000000000000000000000000010000000000000000000000;
pub const BLACK_KINGSIDE_RIGHTS: u64 =      0b0000000000000000000000000000000000000000001000000000000000000000;
pub const BLACK_QUEENSIDE_RIGHTS: u64 =     0b0000000000000000000000000000000000000000000100000000000000000000;

// -------------------- White Castling --------------------

pub const WHITE_KING_POS: u64 =             0b0000000000000000000000000000000000000000000000000000000000001000;

pub const WHITE_KINGSIDE_MOVE_TO: u64 =     0b0000000000000000000000000000000000000000000000000000000000000010;
pub const WHITE_QUEENSIDE_MOVE_TO: u64 =    0b0000000000000000000000000000000000000000000000000000000000100000;

pub const WHITE_KINGSIDE_SQUARES: u64 =     0b0000000000000000000000000000000000000000000000000000000000000110;
pub const WHITE_QUEENSIDE_SQUARES: u64 =    0b0000000000000000000000000000000000000000000000000000000001110000;
pub const WHITE_QUEENSIDE_ATTACKS: u64 =    0b0000000000000000000000000000000000000000000000000000000000110000;

// -------------------- Black Castling --------------------

pub const BLACK_KING_POS: u64 =             0b0000100000000000000000000000000000000000000000000000000000000000;

pub const BLACK_KINGSIDE_MOVE_TO: u64 =     0b0000001000000000000000000000000000000000000000000000000000000000;
pub const BLACK_QUEENSIDE_MOVE_TO: u64 =    0b0010000000000000000000000000000000000000000000000000000000000000;

pub const BLACK_KINGSIDE_SQUARES: u64 =     0b0000011000000000000000000000000000000000000000000000000000000000;
pub const BLACK_QUEENSIDE_SQUARES: u64 =    0b0111000000000000000000000000000000000000000000000000000000000000;
pub const BLACK_QUEENSIDE_ATTACKS: u64 =    0b0011000000000000000000000000000000000000000000000000000000000000;

// -------------------- Promotion Info --------------------

pub const QUEEN_PROMOTION: u64 = 1u64 << 0;
pub const ROOK_PROMOTION: u64 = 1u64 << 1;
pub const BISHOP_PROMOTION: u64 = 1u64 << 2;
pub const KNIGHT_PROMOTION: u64 = 1u64 << 3;
pub const NO_PROMOTION: u64 = EMPTY;

// -------------------- Default Starting Position --------------------

/// Standard FEN string for the initial chess position.
pub const DEFAULT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

/// Default array for Fen object
pub const DEFAULT_FEN: [u64; ARRAY_SIZE] = [
    0b0000000000000000000000000000000000000000000000001111111100000000,
    0b0000000011111111000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000001000,
    0b0000100000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000010000,
    0b0001000000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000000100100,
    0b0010010000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000001000010,
    0b0100001000000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000000000000010000001,
    0b1000000100000000000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000000000000000000001111111111111111,
    0b1111111111111111000000000000000000000000000000000000000000000000,
    0b0000000000000000000000000000000100000000111110000000000000000000,
];

// -------------------- u128 constants --------------------

pub const FIRST_128: u128 = 0x80000000000000000000000000000000;
pub const EMPTY_128: u128 = 0x0;

// ==================== Helper Functions ====================

pub fn get_white_pieces(pieces: &Array) -> u64 {
    pieces[PAWN_W] | pieces[KING_W] | pieces[QUEEN_W] | pieces[BISHOP_W] | pieces[KNIGHT_W] | pieces[ROOK_W]
}

pub fn get_black_pieces(pieces: &Array) -> u64 {
    pieces[PAWN_B] | pieces[KING_B] | pieces[QUEEN_B] | pieces[BISHOP_B] | pieces[KNIGHT_B] | pieces[ROOK_B]
}

/// Helper function for printing u64 bitboards.
pub fn print_bitboard(bitboard: u64) {
    let mut result: String = "".to_string();
    for i in 0..64 {
        if (FIRST >> i) & bitboard != 0 {
            result += "1";
        } else {
            result += "0";
        }
    }
    println!("{:?}", result);
}

/// Helper function for printing u128 bitboards.
pub fn print_bitboard_128(bitboard: u128) {
    let mut result: String = "".to_string();
    for i in 0..128 {
        if (FIRST_128 >> i) & bitboard != 0 {
            result += "1";
        } else {
            result += "0";
        }
    }
    println!("{:?}", result);
}

pub fn up(piece: u64, index: usize) -> u64 {
    piece << 8 * index
}

pub fn down(piece: u64, index: usize) -> u64 {
    piece >> 8 * index
}

pub fn left(piece: u64, index: usize) -> u64 {
    piece << 1 * index
}

pub fn right(piece: u64, index: usize) -> u64 {
    piece >> 1 * index
}

pub fn upleft(piece: u64, index: usize) -> u64 {
    piece << 9 * index
}

pub fn upright(piece: u64, index: usize) -> u64 {
    piece << 7 * index
}

pub fn downleft(piece: u64, index: usize) -> u64 {
    piece >> 7 * index
}

pub fn downright(piece: u64, index: usize) -> u64 {
    piece >> 9 * index
}

pub fn none(piece: u64, index: usize) -> u64 {
    (piece << index) >> index
}
