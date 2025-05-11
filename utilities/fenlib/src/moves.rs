/// Given a piece u64, the corresponding function returns a list of squares that they could move to.
/// 
/// **Note**: A square being in the list does not necessarily mean the piece can move there in a legal game.
///         For example, obstacles or other pieces on the board can restrict movement. This function
///         only returns the theoretical moves (unfiltered).

/// Possible moves for a white pawn. Moves are calculated using bitwise shifts.
///
/// A white pawn moves one square forward (8), captures diagonally (7, 9), and can advance two squares
/// from its starting position (16).
///
/// # Arguments
/// * `start` - The current position of the white pawn (bitboard representation).
pub fn white_pawn(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Diagonal captures (moving left or right)
    moves.push(start << 7);
    moves.push(start << 9);

    // Forward move (one square)
    moves.push(start << 8);

    // Double advance (only on starting square)
    moves.push(start << 16);

    moves
}

/// Possible moves for a black pawn. Similar to the white pawn, but moves are in the opposite direction.
///
/// A black pawn moves one square forward (8), captures diagonally (7, 9), and can advance two squares
/// from its starting position (16).
///
/// # Arguments
/// * `start` - The current position of the black pawn (bitboard representation).
pub fn black_pawn(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Diagonal captures (moving left or right)
    moves.push(start >> 7);
    moves.push(start >> 9);

    // Forward move (one square)
    moves.push(start >> 8);

    // Double advance (only on starting square)
    moves.push(start >> 16);

    moves
}

/// Possible moves for a knight. A knight moves in an "L" shape: two squares in one direction and then one square
/// perpendicular to that (or vice versa). It can jump over other pieces.
///
/// # Arguments
/// * `start` - The current position of the knight (bitboard representation).
pub fn knight(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Move 2 squares in one direction, 1 square in perpendicular direction
    moves.push(start << 6);
    moves.push(start << 10);
    moves.push(start << 15);
    moves.push(start << 17);

    moves.push(start >> 6);
    moves.push(start >> 10);
    moves.push(start >> 15);
    moves.push(start >> 17);

    moves
}

/// Possible moves for a king. The king can move one square in any direction (horizontally, vertically, or diagonally).
/// It can also castle to further squares in specific circumstances.
///
/// # Arguments
/// * `start` - The current position of the king (bitboard representation).
pub fn king(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Move one square in all 8 possible directions (vertically, horizontally, and diagonally)
    moves.push(start << 1);   // Right
    moves.push(start << 2);   // Right-Right
    moves.push(start << 7);   // Up-Left
    moves.push(start << 8);   // Up
    moves.push(start << 9);   // Up-Right

    moves.push(start >> 1);   // Left
    moves.push(start >> 2);   // Left-Left
    moves.push(start >> 7);   // Down-Left
    moves.push(start >> 8);   // Down
    moves.push(start >> 9);   // Down-Right

    moves
}

/// Possible moves for a rook. The rook moves horizontally or vertically across any number of squares.
///
/// # Arguments
/// * `start` - The current position of the rook (bitboard representation).
pub fn rook(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Horizontal moves (left and right)
    moves.push(start << 1);
    moves.push(start << 2);
    moves.push(start << 3);
    moves.push(start << 4);
    moves.push(start << 5);
    moves.push(start << 6);
    moves.push(start << 7);

    moves.push(start >> 1);
    moves.push(start >> 2);
    moves.push(start >> 3);
    moves.push(start >> 4);
    moves.push(start >> 5);
    moves.push(start >> 6);
    moves.push(start >> 7);

    // Vertical moves (up and down)
    moves.push(start << 8);
    moves.push(start << 16);
    moves.push(start << 24);
    moves.push(start << 32);
    moves.push(start << 40);
    moves.push(start << 48);
    moves.push(start << 56);

    moves.push(start >> 8);
    moves.push(start >> 16);
    moves.push(start >> 24);
    moves.push(start >> 32);
    moves.push(start >> 40);
    moves.push(start >> 48);
    moves.push(start >> 56);

    moves
}

/// Possible moves for a bishop. The bishop moves diagonally in any direction.
///
/// # Arguments
/// * `start` - The current position of the bishop (bitboard representation).
pub fn bishop(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Diagonal moves (top-right, top-left, bottom-right, bottom-left)
    moves.push(start << 9);
    moves.push(start << 18);
    moves.push(start << 27);
    moves.push(start << 36);
    moves.push(start << 45);
    moves.push(start << 54);
    moves.push(start << 63);

    moves.push(start << 7);
    moves.push(start << 14);
    moves.push(start << 21);
    moves.push(start << 28);
    moves.push(start << 35);
    moves.push(start << 42);
    moves.push(start << 49);

    moves.push(start >> 9);
    moves.push(start >> 18);
    moves.push(start >> 27);
    moves.push(start >> 36);
    moves.push(start >> 45);
    moves.push(start >> 54);
    moves.push(start >> 63);

    moves.push(start >> 7);
    moves.push(start >> 14);
    moves.push(start >> 21);
    moves.push(start >> 28);
    moves.push(start >> 35);
    moves.push(start >> 42);
    moves.push(start >> 49);

    moves
}

/// Possible moves for a queen. A queen combines the movement of a rook and a bishop, so its moves
/// are a combination of horizontal, vertical, and diagonal moves.
///
/// # Arguments
/// * `start` - The current position of the queen (bitboard representation).
pub fn queen(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    // Rook-like moves (horizontal and vertical)
    moves.push(start << 1);
    moves.push(start << 2);
    moves.push(start << 3);
    moves.push(start << 4);
    moves.push(start << 5);
    moves.push(start << 6);
    moves.push(start << 7);

    moves.push(start >> 1);
    moves.push(start >> 2);
    moves.push(start >> 3);
    moves.push(start >> 4);
    moves.push(start >> 5);
    moves.push(start >> 6);
    moves.push(start >> 7);

    moves.push(start << 8);
    moves.push(start << 16);
    moves.push(start << 24);
    moves.push(start << 32);
    moves.push(start << 40);
    moves.push(start << 48);
    moves.push(start << 56);

    moves.push(start >> 8);
    moves.push(start >> 16);
    moves.push(start >> 24);
    moves.push(start >> 32);
    moves.push(start >> 40);
    moves.push(start >> 48);
    moves.push(start >> 56);

    // Bishop-like moves (diagonal)
    moves.push(start << 9);
    moves.push(start << 18);
    moves.push(start << 27);
    moves.push(start << 36);
    moves.push(start << 45);
    moves.push(start << 54);
    moves.push(start << 63);

    moves.push(start << 7);
    moves.push(start << 14);
    moves.push(start << 21);
    moves.push(start << 28);
    moves.push(start << 35);
    moves.push(start << 42);
    moves.push(start << 49);

    moves.push(start >> 9);
    moves.push(start >> 18);
    moves.push(start >> 27);
    moves.push(start >> 36);
    moves.push(start >> 45);
    moves.push(start >> 54);
    moves.push(start >> 63);

    moves.push(start >> 7);
    moves.push(start >> 14);
    moves.push(start >> 21);
    moves.push(start >> 28);
    moves.push(start >> 35);
    moves.push(start >> 42);
    moves.push(start >> 49);

    moves
}
