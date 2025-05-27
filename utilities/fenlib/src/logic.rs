use crate::utils::*;
use crate::parsing::{bit_64_to_128, bit_128_to_64};

/// This function determines the patterns for pins and checks by the other player, along with other information.
pub fn get_pins_and_checks(array: &[u128; ARRAY_SIZE], white_to_move: bool) -> ([u128; MAX_SLIDERS], usize, usize, usize, u128, u128, u128, bool) {
    let mut pins_and_checks: [u128; MAX_SLIDERS] = [0; MAX_SLIDERS];
    let mut number_of_sliding_checks: usize = 0;
    let mut pins: [u128; MAX_SLIDERS] = [0; MAX_SLIDERS];
    let mut number_of_pins: usize = 0;

    let mut allow_enpassant: bool = true;
    let mut attacks: u128 = EMPTY;
    let mut xray_checks: u128 = EMPTY;

    let mut queens: u128;
    let mut rooks: u128;
    let mut bishops: u128;
    let mut knights: u128;
    let mut pawns: u128;
    let pawn_attack: fn(u128) -> u128;

    let active_king: u128;

    if white_to_move {
        queens = (array[QUEENS] & BOARD2) << 8;
        rooks = (array[ROOKS] & BOARD2) << 8;
        bishops = (array[BISHOPS] & BOARD2) << 8;
        knights = (array[KNIGHTS] & BOARD2) << 8;
        pawns = (array[PAWNS] & BOARD2) << 8;

        active_king = array[KINGS] & BOARD1;
        pawn_attack = black_pawn_attack;
        attacks |= king_attack(array[KINGS] << 8);
    } else {
        queens = array[QUEENS] & BOARD1;
        rooks = array[ROOKS] & BOARD1;
        bishops = array[BISHOPS] & BOARD1;
        knights = array[KNIGHTS] & BOARD1;
        pawns = array[PAWNS] & BOARD1;

        active_king = (array[KINGS] & BOARD2) << 8;
        pawn_attack = white_pawn_attack;
        attacks |= king_attack(array[KINGS]);
    }

    while queens != 0 {
        let square: u32 = queens.trailing_zeros();
        let piece: u128 = 1u128 << square;
        let (piece_attacks, check_or_pin, xray_check, is_check, may_enpassant) = queen_pins_or_checks(piece, array, white_to_move);

        attacks |= piece_attacks;
        xray_checks |= xray_check;
            
        if check_or_pin != 0 {

            if is_check {
                pins_and_checks[number_of_sliding_checks] = check_or_pin;
                number_of_sliding_checks += 1;
            } else {
                pins[number_of_pins] = check_or_pin;
                number_of_pins += 1;
            }

        }

        if !may_enpassant {
            allow_enpassant = false;
        }

        queens &= !piece;
    }

    while rooks != 0 {
        let square: u32 = rooks.trailing_zeros();
        let piece: u128 = 1u128 << square;
        let (piece_attacks, check_or_pin, xray_check, is_check, may_enpassant) = rook_pins_or_checks(piece, array, white_to_move);

        attacks |= piece_attacks;
        xray_checks |= xray_check;
        
        if check_or_pin != 0 {

            if is_check {
                pins_and_checks[number_of_sliding_checks] = check_or_pin;
                number_of_sliding_checks += 1;
            } else {
                pins[number_of_pins] = check_or_pin;
                number_of_pins += 1;
            }

        }

        if !may_enpassant {
            allow_enpassant = false;
        }

        rooks &= !piece;
    }

    while bishops != 0 {
        let square: u32 = bishops.trailing_zeros();
        let piece: u128 = 1u128 << square;
        let (piece_attacks, check_or_pin, xray_check, is_check, may_enpassant) = bishop_pins_or_checks(piece, array, white_to_move);

        attacks |= piece_attacks;
        xray_checks |= xray_check;
        
        if check_or_pin != 0 {

            if is_check {
                pins_and_checks[number_of_sliding_checks] = check_or_pin;
                number_of_sliding_checks += 1;
            } else {
                pins[number_of_pins] = check_or_pin;
                number_of_pins += 1;
            }

        }

        if !may_enpassant {
            allow_enpassant = false;
        }

        bishops &= !piece;
    }

    let mut number_of_checks: usize = number_of_sliding_checks;
    let mut non_sliding_checks: u128 = EMPTY;

    while knights != 0 {
        let square: u32 = knights.trailing_zeros();
        let piece: u128 = 1u128 << square;

        let attack: u128 = knight_attack(piece);
        if attack & active_king != 0 {
            non_sliding_checks |= piece;
            number_of_checks += 1;
        }
        attacks |= attack;
        knights &= !piece;
    }

    while pawns != 0 {
        let square: u32 = pawns.trailing_zeros();
        let piece: u128 = 1u128 << square;

        let attack: u128 = pawn_attack(piece);
        if attack & active_king != 0 {
            non_sliding_checks |= piece;
            number_of_checks += 1;
        }
        attacks |= attack;
        pawns &= !piece;
    }

    // We add the pins to the array of pins and checks, so that the checks come first and the pins after that
    for i in 0..number_of_pins {
        pins_and_checks[number_of_sliding_checks + i] = pins[i];
    }

    (pins_and_checks, number_of_sliding_checks, number_of_checks, number_of_pins + number_of_sliding_checks, non_sliding_checks, attacks, xray_checks, allow_enpassant)
}

/// This function determines the squares that the rook pins/checks.
/// This function is very complex (sorry) and may contain sneaky errors.
/// The function also determines whether we may en passant or not.
pub fn rook_pins_or_checks(piece: u128, array: &[u128; ARRAY_SIZE], white_to_move: bool) -> (u128, u128, u128, bool, bool) {
    let team: u128;
    let opponents: u128;
    let opponent_king: u128;
    let opponent_pawns: u128;

    let enpassant: u128 = array[INFO] & BOARD1;
    let can_enpassant: bool;
    let enpassant_attacks: u128;

    // The variables are named from the perspective of the sliding pieces of the opposing color.
    if white_to_move {
        team = (array[ALL_PIECES] & BOARD2) << 8;
        opponents = array[ALL_PIECES] & BOARD1;
        opponent_king = array[KINGS] & BOARD1;
        opponent_pawns = array[PAWNS] & BOARD1;
        enpassant_attacks = enpassant >> 16;
        can_enpassant = enpassant != 0 && (((enpassant >> 15 | enpassant >> 17) & BOARD1) & (array[PAWNS] & BOARD1) != 0);
    } else {
        team = array[ALL_PIECES] & BOARD1;
        opponents = (array[ALL_PIECES] & BOARD2) << 8;
        opponent_king = (array[KINGS] & BOARD2) << 8;
        opponent_pawns = (array[PAWNS] & BOARD2) << 8;
        enpassant_attacks = enpassant << 16;
        can_enpassant = enpassant != 0 && (((enpassant << 15 | enpassant << 17) & BOARD1) & ((array[PAWNS] & BOARD2) << 8) != 0);
    }

    let all_pieces: u128 = team | opponents;

    let mut check_or_pin: u128 = EMPTY;
    let mut attacks: u128 = EMPTY;
    let mut is_check: bool = false;
    let mut may_enpassant: bool = can_enpassant;
    let mut xray_check: u128 = EMPTY;

    let directions: [fn(u128, usize) -> u128; 4] = [up, down, left, right];

    for direction in directions {

        // First, we shoot a ray in the given direction until we hit the end of the board or the opponent king.
        // We also add the squares to the attack if no piece has been found yet.
        let mut ray: u128 = piece;
        let mut blocked: bool = false;
        let mut xray_blocked: bool = false;
        let mut found_king: bool = false;

        for i in 1..8 {
            let pos: u128 = direction(piece, i);

            if pos & BOARD1 == 0 {
                // If we reach the end of the board, we stop the ray
                break
            }

            if !found_king {
                ray |= pos;
            } else if !xray_blocked {
                xray_check |= pos;

                if pos & all_pieces != 0 {
                    xray_blocked = true;
                }
            }

            if pos & opponent_king != 0 {
                found_king = true;

                if blocked {
                    xray_blocked = true;
                }
            }

            if !blocked {
                // If we have not been blocked yet, we add the position to the attacks.
                attacks |= pos;

                if pos & all_pieces != 0 {
                    // If we find a piece, we cannot attack furthur, so we set blocked to true.
                    blocked = true;
                    
                    if pos & opponent_king == 0 {
                        xray_blocked = true;
                    }
                }
            }
        }

        if found_king {

            let number_of_blockers: u32 = (ray & all_pieces).count_ones() - 2;
            let blockers: u128 = ray & !(piece | opponent_king);

            if number_of_blockers == 0 {

                // If the only pieces in the ray are the piece itself and the opponent king, we have a check.
                is_check = true;
                check_or_pin = ray & !opponent_king;

            } else if number_of_blockers == 1 {

                if blockers & opponents != 0 {
                    // If the attack is blocked by one piece of the opposing color, we have a pin.
                    check_or_pin = ray & !opponent_king;
                } else if can_enpassant && (enpassant_attacks & blockers != 0) && (enpassant & blockers == 0) {
                    // If doing the enpassant reveals the king, we may not en passant.
                    // This can be caused by a bishop that attacks the opponent king through the piece that could be captured with en passant.
                    may_enpassant = false;
                }

            // I would like to express my ... and repeat, regret and ape ape apologies.
            } else if number_of_blockers == 2 && can_enpassant && (enpassant_attacks & blockers != 0) && ((((enpassant_attacks >> 1) & BOARD1) & opponent_pawns & blockers != 0) || (((enpassant_attacks << 1) & BOARD1) & opponent_pawns & blockers != 0)) {
                // If doing the enpassant reveals the king, we may not en passant.
                // This can be caused by a rook that attacks the opponent king through the piece that could be captured with en passant and the piece that does the en passant.
                may_enpassant = false;
            }
        }
    }

    (attacks, check_or_pin, xray_check, is_check, may_enpassant)
}

/// This function determines the squares that the bishop pins/checks.
/// This function is very complex (sorry) and may contain sneaky errors.
/// The function also determines whether we may en passant or not.
pub fn bishop_pins_or_checks(piece: u128, array: &[u128; ARRAY_SIZE], white_to_move: bool) -> (u128, u128, u128, bool, bool) {
    let team: u128;
    let opponents: u128;
    let opponent_king: u128;
    let opponent_pawns: u128;

    let enpassant: u128 = array[INFO] & BOARD1;
    let can_enpassant: bool;
    let enpassant_attacks: u128;

    // The variables are named from the perspective of the sliding pieces of the opposing color.
    if white_to_move {
        team = (array[ALL_PIECES] & BOARD2) << 8;
        opponents = array[ALL_PIECES] & BOARD1;
        opponent_king = array[KINGS] & BOARD1;
        opponent_pawns = array[PAWNS] & BOARD1;
        enpassant_attacks = enpassant >> 16;
        can_enpassant = enpassant != 0 && (((enpassant >> 15 | enpassant >> 17) & BOARD1) & (array[PAWNS] & BOARD1) != 0);
    } else {
        team = array[ALL_PIECES] & BOARD1;
        opponents = (array[ALL_PIECES] & BOARD2) << 8;
        opponent_king = (array[KINGS] & BOARD2) << 8;
        opponent_pawns = (array[PAWNS] & BOARD2) << 8;
        enpassant_attacks = enpassant << 16;
        can_enpassant = enpassant != 0 && (((enpassant << 15 | enpassant << 17) & BOARD1) & ((array[PAWNS] & BOARD2) << 8) != 0);
    }

    let all_pieces: u128 = team | opponents;

    let mut check_or_pin: u128 = EMPTY;
    let mut attacks: u128 = EMPTY;
    let mut is_check: bool = false;
    let mut may_enpassant: bool = can_enpassant;
    let mut xray_check: u128 = EMPTY;

    let directions: [fn(u128, usize) -> u128; 4] = [upleft, upright, downleft, downright];

    for direction in directions {

        // First, we shoot a ray in the given direction until we hit the end of the board or the opponent king.
        // We also add the squares to the attack if no piece has been found yet.
        let mut ray: u128 = piece;
        let mut blocked: bool = false;
        let mut xray_blocked: bool = false;
        let mut found_king: bool = false;

        for i in 1..8 {
            let pos: u128 = direction(piece, i);

            if pos & BOARD1 == 0 {
                // If we reach the end of the board, we stop the ray
                break
            }

            if !found_king {
                ray |= pos;
            } else if !xray_blocked {
                xray_check |= pos;

                if pos & all_pieces != 0 {
                    xray_blocked = true;
                }
            }

            if pos & opponent_king != 0 {
                found_king = true;

                if blocked {
                    xray_blocked = true;
                }
            }

            if !blocked {
                // If we have not been blocked yet, we add the position to the attacks.
                attacks |= pos;

                if pos & all_pieces != 0 {
                    // If we find a piece, we cannot attack furthur, so we set blocked to true.
                    blocked = true;
                    
                    if pos & opponent_king == 0 {
                        xray_blocked = true;
                    }
                }
            }
        }

        if found_king {

            let number_of_blockers: u32 = (ray & all_pieces).count_ones() - 2;
            let blockers: u128 = ray & !(piece | opponent_king);

            if number_of_blockers == 0 {

                // If the only pieces in the ray are the piece itself and the opponent king, we have a check.
                is_check = true;
                check_or_pin = ray & !opponent_king;

            } else if number_of_blockers == 1 {

                if blockers & opponents != 0 {
                    // If the attack is blocked by one piece of the opposing color, we have a pin.
                    check_or_pin = ray & !opponent_king;
                } else if can_enpassant && (enpassant_attacks & blockers != 0) && (enpassant & blockers == 0) {
                    // If doing the enpassant reveals the king, we may not en passant.
                    // This can be caused by a bishop that attacks the opponent king through the piece that could be captured with en passant.
                    may_enpassant = false;
                }

            // I would like to express my ... and repeat, regret and ape ape apologies.
            } else if number_of_blockers == 2 && can_enpassant && (enpassant_attacks & blockers != 0) && ((((enpassant_attacks >> 1) & BOARD1) & opponent_pawns & blockers != 0) || (((enpassant_attacks << 1) & BOARD1) & opponent_pawns & blockers != 0)) {
                // If doing the enpassant reveals the king, we may not en passant.
                // This can be caused by a rook that attacks the opponent king through the piece that could be captured with en passant and the piece that does the en passant.
                may_enpassant = false;
            }
        }
    }

    (attacks, check_or_pin, xray_check, is_check, may_enpassant)
}

/// This function determines the squares that the queen pins/checks.
/// This function is very complex (sorry) and may contain sneaky errors.
/// The function also determines whether we may en passant or not.
pub fn queen_pins_or_checks(piece: u128, array: &[u128; ARRAY_SIZE], white_to_move: bool) -> (u128, u128, u128, bool, bool) {
    let team: u128;
    let opponents: u128;
    let opponent_king: u128;
    let opponent_pawns: u128;

    let enpassant: u128 = array[INFO] & BOARD1;
    let can_enpassant: bool;
    let enpassant_attacks: u128;

    // The variables are named from the perspective of the sliding pieces of the opposing color.
    if white_to_move {
        team = (array[ALL_PIECES] & BOARD2) << 8;
        opponents = array[ALL_PIECES] & BOARD1;
        opponent_king = array[KINGS] & BOARD1;
        opponent_pawns = array[PAWNS] & BOARD1;
        enpassant_attacks = enpassant >> 16;
        can_enpassant = enpassant != 0 && (((enpassant >> 15 | enpassant >> 17) & BOARD1) & (array[PAWNS] & BOARD1) != 0);
    } else {
        team = array[ALL_PIECES] & BOARD1;
        opponents = (array[ALL_PIECES] & BOARD2) << 8;
        opponent_king = (array[KINGS] & BOARD2) << 8;
        opponent_pawns = (array[PAWNS] & BOARD2) << 8;
        enpassant_attacks = enpassant << 16;
        can_enpassant = enpassant != 0 && (((enpassant << 15 | enpassant << 17) & BOARD1) & ((array[PAWNS] & BOARD2) << 8) != 0);
    }

    let all_pieces: u128 = team | opponents;

    let mut check_or_pin: u128 = EMPTY;
    let mut attacks: u128 = EMPTY;
    let mut is_check: bool = false;
    let mut may_enpassant: bool = can_enpassant;
    let mut xray_check: u128 = EMPTY;

    let directions: [fn(u128, usize) -> u128; 8] = [up, down, left, right, upleft, upright, downleft, downright];

    for direction in directions {

        // First, we shoot a ray in the given direction until we hit the end of the board or the opponent king.
        // We also add the squares to the attack if no piece has been found yet.
        let mut ray: u128 = piece;
        let mut blocked: bool = false;
        let mut xray_blocked: bool = false;
        let mut found_king: bool = false;

        for i in 1..8 {
            let pos: u128 = direction(piece, i);

            if pos & BOARD1 == 0 {
                // If we reach the end of the board, we stop the ray
                break
            }

            if !found_king {
                ray |= pos;
            } else if !xray_blocked {
                xray_check |= pos;

                if pos & all_pieces != 0 {
                    xray_blocked = true;
                }
            }

            if pos & opponent_king != 0 {
                found_king = true;

                if blocked {
                    xray_blocked = true;
                }
            }

            if !blocked {
                // If we have not been blocked yet, we add the position to the attacks.
                attacks |= pos;

                if pos & all_pieces != 0 {
                    // If we find a piece, we cannot attack furthur, so we set blocked to true.
                    blocked = true;
                    
                    if pos & opponent_king == 0 {
                        xray_blocked = true;
                    }
                }
            }
        }

        if found_king {

            let number_of_blockers: u32 = (ray & all_pieces).count_ones() - 2;
            let blockers: u128 = ray & !(piece | opponent_king);

            if number_of_blockers == 0 {

                // If the only pieces in the ray are the piece itself and the opponent king, we have a check.
                is_check = true;
                check_or_pin = ray & !opponent_king;

            } else if number_of_blockers == 1 {

                if blockers & opponents != 0 {
                    // If the attack is blocked by one piece of the opposing color, we have a pin.
                    check_or_pin = ray & !opponent_king;
                } else if can_enpassant && (enpassant_attacks & blockers != 0) && (enpassant & blockers == 0) {
                    // If doing the enpassant reveals the king, we may not en passant.
                    // This can be caused by a bishop that attacks the opponent king through the piece that could be captured with en passant.
                    may_enpassant = false;
                }

            // I would like to express my ... and repeat, regret and ape ape apologies.
            } else if number_of_blockers == 2 && can_enpassant && (enpassant_attacks & blockers != 0) && ((((enpassant_attacks >> 1) & BOARD1) & opponent_pawns & blockers != 0) || (((enpassant_attacks << 1) & BOARD1) & opponent_pawns & blockers != 0)) {
                // If doing the enpassant reveals the king, we may not en passant.
                // This can be caused by a rook that attacks the opponent king through the piece that could be captured with en passant and the piece that does the en passant.
                may_enpassant = false;
            }
        }
    }

    (attacks, check_or_pin, xray_check, is_check, may_enpassant)
}

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
    let mut attacks: u128 = EMPTY;

    let piece: u128 = piece_info & BOARD1;

    let directions: [fn(u128, usize) -> u128; 4] = [upleft, upright, downleft, downright];

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
    let mut attacks: u128 = EMPTY;

    let piece: u128 = piece_info & BOARD1;

    let directions: [fn(u128, usize) -> u128; 4] = [up, down, left, right];

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

pub fn up(piece: u128, index: usize) -> u128 {
    piece << 16 * index
}

pub fn down(piece: u128, index: usize) -> u128 {
    piece >> 16 * index
}

pub fn left(piece: u128, index: usize) -> u128 {
    piece << 1 * index
}

pub fn right(piece: u128, index: usize) -> u128 {
    piece >> 1 * index
}

pub fn upleft(piece: u128, index: usize) -> u128 {
    piece << 17 * index
}

pub fn upright(piece: u128, index: usize) -> u128 {
    piece << 15 * index
}

pub fn downleft(piece: u128, index: usize) -> u128 {
    piece >> 15 * index
}

pub fn downright(piece: u128, index: usize) -> u128 {
    piece >> 17 * index
}
