/// The purpose of this file is to replace moves with a smarter system.
/// 
/// THIS DOCUMENTATION NEEDS SOME WORK
/// 
/// Checking if a move is legal comes down to checking:
/// whether there is a piece of the same color on the target square,
/// whether we are in check (using attack patterns),
/// whether we can enpassant (only move where taking a piece != moving to that square),
/// whether we can castle.

use crate::utils_new::*;

/// This function provides the attack patterns for white and black,
/// in the form of two u128, which both have the respective attack patterns on the left board.
pub fn get_attacks(array: [u128; ARRAY_SIZE]) -> (u128, u128) {
    let all_pieces: u128 = (array[ALL_PIECES] & BOARD1) | ((array[ALL_PIECES] & BOARD2) << 8);

    let white_pawns: u128 = array[PAWNS] & BOARD1;
    let white_kings: u128 = array[KINGS] & BOARD1;
    let white_knights: u128 = array[KNIGHTS] & BOARD1;

    let mut white_attack: u128 = white_pawn_attack(&white_pawns) | knight_attack(&white_knights) | king_attack(&white_kings);

    let mut white_queens: u128 = array[QUEENS] & BOARD1;
    let mut white_bishops: u128 = array[BISHOPS] & BOARD1;
    let mut white_rooks: u128 = array[ROOKS] & BOARD1;

    while white_queens != 0 {
        let square: u32 = white_queens.trailing_zeros();
        let piece: u128 = 1u128 << square;
        white_attack |= queen_attack(&piece, &all_pieces);
        white_queens &= !piece;
    }

    while white_bishops != 0 {
        let square: u32 = white_bishops.trailing_zeros();
        let piece: u128 = 1u128 << square;
        white_attack |= bishop_attack(&piece, &all_pieces);
        white_bishops &= !piece;
    }

    while white_rooks != 0 {
        let square: u32 = white_rooks.trailing_zeros();
        let piece: u128 = 1u128 << square;
        white_attack |= rook_attack(&piece, &all_pieces);
        white_rooks &= !piece;
    }

    let black_pawns: u128 = (array[PAWNS] & BOARD2) << 8;
    let black_kings: u128 = (array[KINGS] & BOARD2) << 8;
    let black_knights: u128 = (array[KNIGHTS] & BOARD2) << 8;

    let mut black_attack: u128 = black_pawn_attack(&black_pawns) | knight_attack(&black_knights) | king_attack(&black_kings);

    let mut black_queens: u128 = (array[QUEENS] & BOARD2) << 8;
    let mut black_bishops: u128 = (array[BISHOPS] & BOARD2) << 8;
    let mut black_rooks: u128 = (array[ROOKS] & BOARD2) << 8;

    while black_queens != 0 {
        let square: u32 = black_queens.trailing_zeros();
        let piece: u128 = 1u128 << square;
        black_attack |= queen_attack(&piece, &all_pieces);
        black_queens &= !piece;
    }

    while black_bishops != 0 {
        let square: u32 = black_bishops.trailing_zeros();
        let piece: u128 = 1u128 << square;
        black_attack |= bishop_attack(&piece, &all_pieces);
        black_bishops &= !piece;
    }

    while black_rooks != 0 {
        let square: u32 = black_rooks.trailing_zeros();
        let piece: u128 = 1u128 << square;
        black_attack |= rook_attack(&piece, &all_pieces);
        black_rooks &= !piece;
    }

    (white_attack, black_attack)
}

/// This function determines the squares that the white pawn attacks
pub fn white_pawn_attack(piece_info: &u128) -> u128 {
    let piece: u128 = piece_info & BOARD1;
    let ul: u128 = piece << 17;
    let ur: u128 = piece << 15;

    let board: u128 = ul | ur;
    (board & BOARD1) >> 8
}

/// This function determines the squares that the black pawn attacks
pub fn black_pawn_attack(piece_info: &u128) -> u128 {
    let piece: u128 = piece_info & BOARD1;
    let dl: u128 = piece >> 15;
    let dr: u128 = piece >> 17;

    let board: u128 = dl | dr;
    (board & BOARD1) >> 8
}

/// This function determines the squares that the knight attacks
pub fn knight_attack(piece_info: &u128) -> u128 {
    let piece: u128 = piece_info & BOARD1;
    let ull: u128 = piece << 18;
    let urr: u128 = piece << 14;
    let uul: u128 = piece << 33;
    let uur: u128 = piece << 31;
    let drr: u128 = piece >> 18;
    let dll: u128 = piece >> 14;
    let ddr: u128 = piece >> 33;
    let ddl: u128 = piece >> 31;

    let board: u128 = ull | urr | uul | uur | drr | dll | ddr | ddl;
    (board & BOARD1) >> 8
}

/// This function determines the squares that the king attacks
pub fn king_attack(piece_info: &u128) -> u128 {
    let piece: u128 = piece_info & BOARD1;
    let l: u128 = piece << 1;
    let r: u128 = piece >> 1;
    let u: u128 = piece << 16;
    let d: u128 = piece >> 16;
    let ul: u128 = piece << 17;
    let ur: u128 = piece << 15;
    let dl: u128 = piece >> 15;
    let dr: u128 = piece >> 17;

    let board: u128 = l | r | u | d | ul | ur | dl | dr;
    (board & BOARD1) >> 8
}

/// This function determines the squares that the bishop attacks,
pub fn bishop_attack(piece_info: &u128, all_pieces: &u128) -> u128 {
    let mut attacks: u128 = EMPTY;

    let piece: u128 = piece_info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece << 17 * i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    for i in 1..8 {
        let pos: u128 = piece << 15 * i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    for i in 1..8 {
        let pos: u128 = piece >> 15 * i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    for i in 1..8 {
        let pos: u128 = piece >> 17 * i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    attacks
}

/// This function determines the squares that the rook attacks
pub fn rook_attack(piece_info: &u128, all_pieces: &u128) -> u128 {
    let mut attacks: u128 = EMPTY;

    let piece: u128 = piece_info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece << 16 * i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    for i in 1..8 {
        let pos: u128 = piece >> 16 * i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    for i in 1..8 {
        let pos: u128 = piece << i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    for i in 1..8 {
        let pos: u128 = piece >> i;

        if pos & BOARD1 == 0 {
            break;
        }

        attacks |= pos;

        if pos & all_pieces != 0 {
            break
        }
    }

    attacks
}

/// This function determines the squares that the queen attacks
pub fn queen_attack(piece_info: &u128, all_pieces: &u128) -> u128 {

    // The queen combines the patterns of the rook and the bishop

    bishop_attack(piece_info, all_pieces) | rook_attack(piece_info, all_pieces)
}

/// This functions shoots a ray up from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_up(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece << 16 * i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray down from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_down(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece >> 16 * i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray left from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_left(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece << i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray right from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_right(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece >> i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray up left from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_upleft(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece << 17 * i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray up right from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_upright(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece << 15 * i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray down left from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_downleft(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece >> 15 * i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}

/// This functions shoots a ray down right from the piece, and stops when it reaches a piece
/// 
/// `stop_at` should be the positions in BOARD1 where you do not want the piece to go
pub fn ray_downright(info: &u128, stop_at: &u128) -> u128 {
    let mut result: u128 = 0x0;
    let piece: u128 = info & BOARD1;

    for i in 1..8 {
        let pos: u128 = piece >> 17 * i;
        if pos & stop_at == 0 {
            result |= pos;
        } else {
            break;
        }
    }

    (result & BOARD1) >> 8
}
