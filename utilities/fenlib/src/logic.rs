/// The purpose of this file is to replace moves with a smarter system.
/// 
/// Checking if a move is legal then comes down to checking:
/// whether there is a piece of the same color on the target square,
/// whether we are in check (using attack patterns),
/// whether we can enpassant (only move where taking a piece =/= moving to that square),
/// whether we can castle.

use crate::utils_new::*;

/// This function determines the squares that the white pawn attacks,
/// and the squares that the white pawn may move to.
pub fn white_pawn_info(piece_info: &u128, team: &u128, opponents: &u128) -> (u128, u128) {
    let attacks: u128 = white_pawn_attack(piece_info);

    let stop_at: u128;
    let all_pieces: u128 = team | opponents;
    if piece_info & RANK_6 == 0 {
        stop_at = all_pieces | (piece_info << 32);
    } else {
        stop_at = all_pieces | RANK_3;
    }

    let moving_to: u128 = ray_up(piece_info, &stop_at);
    let pseudo_legals: u128 = (attacks & !team) & moving_to;

    (attacks, pseudo_legals)
}

/// This function determines the squares that the black pawn attacks,
/// and the squares that the black pawn may move to.
pub fn black_pawn_info(piece_info: &u128, team: &u128, opponents: &u128) -> (u128, u128) {
    let attacks: u128 = black_pawn_attack(piece_info);

    let stop_at: u128;
    let all_pieces: u128 = team | opponents;
    if piece_info & RANK_1 == 0 {
        stop_at = all_pieces | (piece_info >> 32);
    } else {
        stop_at = all_pieces | RANK_4;
    }

    let moving_to: u128 = ray_down(piece_info, &stop_at);
    let pseudo_legals: u128 = (attacks & !team) & moving_to;

    (attacks, pseudo_legals)
}

/// This function determines the squares that the knight attacks and the squares it may move to.
pub fn knight_info(piece_info: &u128, team: &u128) -> (u128, u128) {
    let attacks: u128 = knight_attack(piece_info);
    let pseudo_legal: u128 = attacks & !team;

    (attacks, pseudo_legal)
}

/// This function determines the squares that the king attacks and the squares it may move to.
pub fn king_info(piece_info: &u128, team: &u128) -> (u128, u128) {
    let attacks: u128 = king_attack(piece_info);
    let pseudo_legal: u128 = attacks & !team;

    (attacks, pseudo_legal)
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
