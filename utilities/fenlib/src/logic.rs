use crate::utils::*;
use crate::parsing::{bit_64_to_128, bit_128_to_64};

/// This function provides the attack patterns for white.
pub fn get_white_attacks(array: &Array) -> u64 {
    let all_pieces: u128 = bit_64_to_128(array[WHITE] | array[BLACK]);

    let pawns: u128 = bit_64_to_128(array[PAWN_W]);
    let kings: u128 = bit_64_to_128(array[KING_W]);
    let knights: u128 = bit_64_to_128(array[KNIGHT_W]);

    let mut attacks: u128 = white_pawn_attack(pawns) | king_attack(kings) | knight_attack(knights);

    let mut queens: u128 = bit_64_to_128(array[QUEEN_W]);
    let mut bishops: u128 = bit_64_to_128(array[BISHOP_W]);
    let mut rooks: u128 = bit_64_to_128(array[ROOK_W]);

    while queens != 0 {
        let square: u32 = queens.trailing_zeros();
        let queen: u128 = 1u128 << square;
        attacks |= queen_attack(queen, all_pieces);
        queens &= !queen;
    }

    while bishops != 0 {
        let square: u32 = bishops.trailing_zeros();
        let bishop: u128 = 1u128 << square;
        attacks |= bishop_attack(bishop, all_pieces);
        bishops &= !bishop;
    }

    while rooks != 0 {
        let square: u32 = rooks.trailing_zeros();
        let rook: u128 = 1u128 << square;
        attacks |= rook_attack(rook, all_pieces);
        rooks &= !rook;
    }

    bit_128_to_64(attacks)
}

/// This function provides the attack patterns for black.
pub fn get_black_attacks(array: &Array) -> u64 {
    let all_pieces: u128 = bit_64_to_128(array[WHITE] | array[BLACK]);

    let pawns: u128 = bit_64_to_128(array[PAWN_B]);
    let kings: u128 = bit_64_to_128(array[KING_B]);
    let knights: u128 = bit_64_to_128(array[KNIGHT_B]);

    let mut attacks: u128 = black_pawn_attack(pawns) | king_attack(kings) | knight_attack(knights);

    let mut queens: u128 = bit_64_to_128(array[QUEEN_B]);
    let mut bishops: u128 = bit_64_to_128(array[BISHOP_B]);
    let mut rooks: u128 = bit_64_to_128(array[ROOK_B]);

    while queens != 0 {
        let square: u32 = queens.trailing_zeros();
        let queen: u128 = 1u128 << square;
        attacks |= queen_attack(queen, all_pieces);
        queens &= !queen;
    }

    while bishops != 0 {
        let square: u32 = bishops.trailing_zeros();
        let bishop: u128 = 1u128 << square;
        attacks |= bishop_attack(bishop, all_pieces);
        bishops &= !bishop;
    }

    while rooks != 0 {
        let square: u32 = rooks.trailing_zeros();
        let rook: u128 = 1u128 << square;
        attacks |= rook_attack(rook, all_pieces);
        rooks &= !rook;
    }

    bit_128_to_64(attacks)
}

/// This function determines the squares that the white pawn attacks
pub fn white_pawn_attack(piece_info: u128) -> u128 {
    let piece: u128 = piece_info & BOARD1;
    let ul: u128 = piece << 17;
    let ur: u128 = piece << 15;

    let board: u128 = ul | ur;
    board & BOARD1
}

/// This function determines the squares that the black pawn attacks
pub fn black_pawn_attack(piece_info: u128) -> u128 {
    let piece: u128 = piece_info & BOARD1;
    let dl: u128 = piece >> 15;
    let dr: u128 = piece >> 17;

    let board: u128 = dl | dr;
    board & BOARD1
}

/// This function determines the squares that the knight attacks
pub fn knight_attack(piece_info: u128) -> u128 {
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
    board & BOARD1
}

/// This function determines the squares that the king attacks
pub fn king_attack(piece_info: u128) -> u128 {
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
    board & BOARD1
}

/// This function determines the squares that the bishop attacks
pub fn bishop_attack(piece_info: u128, all_pieces: u128) -> u128 {
    let mut attacks: u128 = EMPTY_128;

    let piece: u128 = piece_info & BOARD1;

    let directions: [fn(u128, usize) -> u128; 4] = [upleft_128, upright_128, downleft_128, downright_128];

    for direction in directions {
        for i in 1..8 {
            let pos: u128 = direction(piece, i);

            if pos & BOARD1 == 0 {
                break
            }

            attacks |= pos;

            if pos & all_pieces != 0 {
                break
            }
        }
    }

    attacks
}

/// This function determines the squares that the rook attacks
pub fn rook_attack(piece_info: u128, all_pieces: u128) -> u128 {
    let mut attacks: u128 = EMPTY_128;

    let piece: u128 = piece_info & BOARD1;

    let directions: [fn(u128, usize) -> u128; 4] = [up_128, down_128, left_128, right_128];

    for direction in directions {
        for i in 1..8 {
            let pos: u128 = direction(piece, i);

            if pos & BOARD1 == 0 {
                break
            }

            attacks |= pos;

            if pos & all_pieces != 0 {
                break
            }
        }
    }

    attacks
}

/// This function determines the squares that the queen attacks
pub fn queen_attack(piece_info: u128, all_pieces: u128) -> u128 {

    // The queen combines the patterns of the rook and the bishop

    bishop_attack(piece_info, all_pieces) | rook_attack(piece_info, all_pieces)
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

pub fn up_128(piece: u128, index: usize) -> u128 {
    piece << 16 * index
}

pub fn down_128(piece: u128, index: usize) -> u128 {
    piece >> 16 * index
}

pub fn left_128(piece: u128, index: usize) -> u128 {
    piece << 1 * index
}

pub fn right_128(piece: u128, index: usize) -> u128 {
    piece >> 1 * index
}

pub fn upleft_128(piece: u128, index: usize) -> u128 {
    piece << 17 * index
}

pub fn upright_128(piece: u128, index: usize) -> u128 {
    piece << 15 * index
}

pub fn downleft_128(piece: u128, index: usize) -> u128 {
    piece >> 15 * index
}

pub fn downright_128(piece: u128, index: usize) -> u128 {
    piece >> 17 * index
}