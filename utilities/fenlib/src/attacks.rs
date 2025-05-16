use crate::utils_new::*;

/// The purpose of this file is to replace moves with a smarter system.
/// The functions below determine the places where the piece may move to.
/// We do not know any info on the target square here,
/// since we need all the info to get the attack, defend and pin patters,
/// and for the legal moves per piece.
/// 
/// Checking if a move is legal then comes down to checking:
/// whether there is a piece of the same color on the target square,
/// whether we are in check (using attack patterns),
/// whether we can enpassant (only move where taking a piece =/= moving to that square),
/// whether we can castle.

/// This function determines the squares that the knight attacks
pub fn knight(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
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
pub fn king(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
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
/// This does not include the enpassant logic
pub fn white_pawn(info: &u128, ) -> u128 {
    let piece: u128 = info & BOARD1;
    let ul: u128 = piece << 17;
    let ur: u128 = piece << 15;

    let board: u128 = ul | ur;
    (board & BOARD1) >> 8
}

/// This function determines the squares that the black pawn attacks
/// This does not include the enpassant logic
pub fn black_pawn(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
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

/// This function shoots a ray up from the piece, regardless of any piece in the way
pub fn xray_up(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let u1: u128 = piece << 16;
    let u2: u128 = u1 << 16;
    let u3: u128 = u2 << 16;
    let u4: u128 = u3 << 16;
    let u5: u128 = u4 << 16;
    let u6: u128 = u5 << 16;
    let u7: u128 = u6 << 16;

    let board: u128 = u1 | u2 | u3 | u4 | u5 | u6 | u7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray down from the piece, regardless of any piece in the way
pub fn xray_down(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let d1: u128 = piece >> 16;
    let d2: u128 = d1 >> 16;
    let d3: u128 = d2 >> 16;
    let d4: u128 = d3 >> 16;
    let d5: u128 = d4 >> 16;
    let d6: u128 = d5 >> 16;
    let d7: u128 = d6 >> 16;

    let board: u128 = d1 | d2 | d3 | d4 | d5 | d6 | d7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray left from the piece, regardless of any piece in the way
pub fn xray_left(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let l1: u128 = piece << 1;
    let l2: u128 = l1 << 1;
    let l3: u128 = l2 << 1;
    let l4: u128 = l3 << 1;
    let l5: u128 = l4 << 1;
    let l6: u128 = l5 << 1;
    let l7: u128 = l6 << 1;

    let board: u128 = l1 | l2 | l3 | l4 | l5 | l6 | l7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray right from the piece, regardless of any piece in the way
pub fn xray_right(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let r1: u128 = piece >> 1;
    let r2: u128 = r1 >> 1;
    let r3: u128 = r2 >> 1;
    let r4: u128 = r3 >> 1;
    let r5: u128 = r4 >> 1;
    let r6: u128 = r5 >> 1;
    let r7: u128 = r6 >> 1;

    let board: u128 = r1 | r2 | r3 | r4 | r5 | r6 | r7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray up left from the piece, regardless of any piece in the way
pub fn xray_upleft(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let ul1: u128 = piece << 17;
    let ul2: u128 = ul1 << 17;
    let ul3: u128 = ul2 << 17;
    let ul4: u128 = ul3 << 17;
    let ul5: u128 = ul4 << 17;
    let ul6: u128 = ul5 << 17;
    let ul7: u128 = ul6 << 17;

    let board: u128 = ul1 | ul2 | ul3 | ul4 | ul5 | ul6 | ul7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray up right from the piece, regardless of any piece in the way
pub fn xray_upright(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let ur1: u128 = piece << 15;
    let ur2: u128 = ur1 << 15;
    let ur3: u128 = ur2 << 15;
    let ur4: u128 = ur3 << 15;
    let ur5: u128 = ur4 << 15;
    let ur6: u128 = ur5 << 15;
    let ur7: u128 = ur6 << 15;

    let board: u128 = ur1 | ur2 | ur3 | ur4 | ur5 | ur6 | ur7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray down left from the piece, regardless of any piece in the way
pub fn xray_downleft(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let dl1: u128 = piece >> 15;
    let dl2: u128 = dl1 >> 15;
    let dl3: u128 = dl2 >> 15;
    let dl4: u128 = dl3 >> 15;
    let dl5: u128 = dl4 >> 15;
    let dl6: u128 = dl5 >> 15;
    let dl7: u128 = dl6 >> 15;

    let board: u128 = dl1 | dl2 | dl3 | dl4 | dl5 | dl6 | dl7;
    (board & BOARD1) >> 8
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

/// This function shoots a ray right from the piece, regardless of any piece in the way
pub fn xray_downright(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let dr1: u128 = piece >> 17;
    let dr2: u128 = dr1 >> 17;
    let dr3: u128 = dr2 >> 17;
    let dr4: u128 = dr3 >> 17;
    let dr5: u128 = dr4 >> 17;
    let dr6: u128 = dr5 >> 17;
    let dr7: u128 = dr6 >> 17;

    let board: u128 = dr1 | dr2 | dr3 | dr4 | dr5 | dr6 | dr7;
    (board & BOARD1) >> 8
}