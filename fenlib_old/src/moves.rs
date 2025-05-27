/// Given a piece u64, the corresponding function returns a list of squares that they could move to.
/// 
/// **Note**: A square being in the list does not necessarily mean the piece can move there in a legal game.
///         For example, obstacles or other pieces on the board can restrict movement. This function
///         only returns the theoretical moves (unfiltered).

pub const PAWN_GUESS: usize = 4;
pub const KNIGHT_GUESS: usize = 8;
pub const KING_GUESS: usize = 10;
pub const ROOK_GUESS: usize = 28;
pub const BISHOP_GUESS: usize = 28;
pub const QUEEN_GUESS: usize = 54;

/// Possible moves for a white pawn. Moves are calculated using bitwise shifts.
///
/// A white pawn moves one square forward (8), captures diagonally (7, 9), and can advance two squares
/// from its starting position (16).
///
/// # Arguments
/// * `start` - The current position of the white pawn (bitboard representation).
pub fn white_pawn(start: &u64) -> [u64; PAWN_GUESS] {
    let mut moves: [u64; PAWN_GUESS] = [0; PAWN_GUESS];

    // Diagonal captures (moving left or right)
    moves[0] = start << 7;
    moves[1] = start << 9;

    // Forward move (one square)
    moves[2] = start << 8;

    // Double advance (only on starting square)
    moves[3] = start << 16;

    moves
}

/// Possible moves for a black pawn. Similar to the white pawn, but moves are in the opposite direction.
///
/// A black pawn moves one square forward (8), captures diagonally (7, 9), and can advance two squares
/// from its starting position (16).
///
/// # Arguments
/// * `start` - The current position of the black pawn (bitboard representation).
pub fn black_pawn(start: &u64) -> [u64; PAWN_GUESS] {
    let mut moves: [u64; PAWN_GUESS] = [0; PAWN_GUESS];

    // Diagonal captures (moving left or right)
    moves[0] = start >> 7;
    moves[1] = start >> 9;

    // Forward move (one square)
    moves[2] = start >> 8;

    // Double advance (only on starting square)
    moves[3] = start >> 16;

    moves
}

/// Possible moves for a knight. A knight moves in an "L" shape: two squares in one direction and then one square
/// perpendicular to that (or vice versa). It can jump over other pieces.
///
/// # Arguments
/// * `start` - The current position of the knight (bitboard representation).
pub fn knight(start: &u64) -> [u64; KNIGHT_GUESS] {
    let mut moves: [u64; KNIGHT_GUESS] = [0; KNIGHT_GUESS];

    // Move 2 squares in one direction, 1 square in perpendicular direction
    moves[0] = start << 6;
    moves[1] = start << 10;
    moves[2] = start << 15;
    moves[3] = start << 17;

    moves[4] = start >> 6;
    moves[5] = start >> 10;
    moves[6] = start >> 15;
    moves[7] = start >> 17;

    moves
}

/// Possible moves for a king. The king can move one square in any direction (horizontally, vertically, or diagonally).
/// It can also castle to further squares in specific circumstances.
///
/// # Arguments
/// * `start` - The current position of the king (bitboard representation).
pub fn king(start: &u64) -> [u64; KING_GUESS] {
    let mut moves: [u64; KING_GUESS] = [0; KING_GUESS];

    // Move one square in all 8 possible directions (vertically, horizontally, and diagonally)
    moves[0] = start << 1;   // Right
    moves[1] = start << 2;   // Right-Right
    moves[2] = start << 7;   // Up-Left
    moves[3] = start << 8;   // Up
    moves[4] = start << 9;   // Up-Right

    moves[5] = start >> 1;   // Left
    moves[6] = start >> 2;   // Left-Left
    moves[7] = start >> 7;   // Down-Left
    moves[8] = start >> 8;   // Down
    moves[9] = start >> 9;   // Down-Right

    moves
}

/// Possible moves for a rook. The rook moves horizontally or vertically across any number of squares.
///
/// # Arguments
/// * `start` - The current position of the rook (bitboard representation).
pub fn rook(start: &u64) -> [u64; ROOK_GUESS] {
    let mut moves: [u64; ROOK_GUESS] = [0; ROOK_GUESS];

    // Horizontal moves (left and right)
    moves[0] = start << 1;
    moves[1] = start << 2;
    moves[2] = start << 3;
    moves[3] = start << 4;
    moves[4] = start << 5;
    moves[5] = start << 6;
    moves[6] = start << 7;

    moves[7] = start >> 1;
    moves[8] = start >> 2;
    moves[9] = start >> 3;
    moves[10] = start >> 4;
    moves[11] = start >> 5;
    moves[12] = start >> 6;
    moves[13] = start >> 7;

    // Vertical moves (up and down)
    moves[14] = start << 8;
    moves[15] = start << 16;
    moves[16] = start << 24;
    moves[17] = start << 32;
    moves[18] = start << 40;
    moves[19] = start << 48;
    moves[20] = start << 56;

    moves[21] = start >> 8;
    moves[22] = start >> 16;
    moves[23] = start >> 24;
    moves[24] = start >> 32;
    moves[25] = start >> 40;
    moves[26] = start >> 48;
    moves[27] = start >> 56;

    moves
}

/// Possible moves for a bishop. The bishop moves diagonally in any direction.
///
/// # Arguments
/// * `start` - The current position of the bishop (bitboard representation).
pub fn bishop(start: &u64) -> [u64; BISHOP_GUESS] {
    let mut moves: [u64; BISHOP_GUESS] = [0; BISHOP_GUESS];

    // Diagonal moves (top-right, top-left, bottom-right, bottom-left)
    moves[0] = start << 9;
    moves[1] = start << 18;
    moves[2] = start << 27;
    moves[3] = start << 36;
    moves[4] = start << 45;
    moves[5] = start << 54;
    moves[6] = start << 63;

    moves[7] = start << 7;
    moves[8] = start << 14;
    moves[9] = start << 21;
    moves[10] = start << 28;
    moves[11] = start << 35;
    moves[12] = start << 42;
    moves[13] = start << 49;

    moves[14] = start >> 9;
    moves[15] = start >> 18;
    moves[16] = start >> 27;
    moves[17] = start >> 36;
    moves[18] = start >> 45;
    moves[19] = start >> 54;
    moves[20] = start >> 63;

    moves[21] = start >> 7;
    moves[22] = start >> 14;
    moves[23] = start >> 21;
    moves[24] = start >> 28;
    moves[25] = start >> 35;
    moves[26] = start >> 42;
    moves[27] = start >> 49;

    moves
}

/// Possible moves for a queen. A queen combines the movement of a rook and a bishop, so its moves
/// are a combination of horizontal, vertical, and diagonal moves.
///
/// # Arguments
/// * `start` - The current position of the queen (bitboard representation).
pub fn queen(start: &u64) -> [u64; QUEEN_GUESS] {
    let mut moves: [u64; QUEEN_GUESS] = [0; QUEEN_GUESS];

    // one would think that QUEEN_GUESS should be 56, (num for rooks + num for bishops)
    // however, start << 7 and start >> 7 are checked by both rooks and bishops
    // so we have removed them to avoid duplicates

    // Rook-like moves (horizontal and vertical)
    moves[0] = start << 1;
    moves[1] = start << 2;
    moves[2] = start << 3;
    moves[3] = start << 4;
    moves[4] = start << 5;
    moves[5] = start << 6;
    moves[6] = start << 7;

    moves[7] = start >> 1;
    moves[8] = start >> 2;
    moves[9] = start >> 3;
    moves[10] = start >> 4;
    moves[11] = start >> 5;
    moves[12] = start >> 6;
    moves[13] = start >> 7;

    moves[14] = start << 8;
    moves[15] = start << 16;
    moves[16] = start << 24;
    moves[17] = start << 32;
    moves[18] = start << 40;
    moves[19] = start << 48;
    moves[20] = start << 56;

    moves[21] = start >> 8;
    moves[22] = start >> 16;
    moves[23] = start >> 24;
    moves[24] = start >> 32;
    moves[25] = start >> 40;
    moves[26] = start >> 48;
    moves[27] = start >> 56;

    // Bishop-like moves (diagonal)
    moves[28] = start << 9;
    moves[29] = start << 18;
    moves[30] = start << 27;
    moves[31] = start << 36;
    moves[32] = start << 45;
    moves[33] = start << 54;
    moves[34] = start << 63;

    moves[35] = start << 14;
    moves[36] = start << 21;
    moves[37] = start << 28;
    moves[38] = start << 35;
    moves[39] = start << 42;
    moves[40] = start << 49;

    moves[41] = start >> 9;
    moves[42] = start >> 18;
    moves[43] = start >> 27;
    moves[44] = start >> 36;
    moves[45] = start >> 45;
    moves[46] = start >> 54;
    moves[47] = start >> 63;

    moves[48] = start >> 14;
    moves[49] = start >> 21;
    moves[50] = start >> 28;
    moves[51] = start >> 35;
    moves[52] = start >> 42;
    moves[53] = start >> 49;

    moves
}
