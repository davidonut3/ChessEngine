/// The purpose of this file is to replace moves with a smarter system.
/// 
/// THIS DOCUMENTATION NEEDS SOME WORK
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
    let piece: u128 = piece_info & BOARD1;

    let mut one_up: u128 = piece << 16;
    let mut two_up: u128 = piece << 32;
    let all: u128 = team | opponents;

    if one_up & all != 0 {

        // If any piece blocks the way, we cannot move forward
        one_up = EMPTY

    } else if piece & RANK_6 == 0 || two_up & all != 0 {

        // If we can move one forward, but we are not on rank 2 or any piece blocks the way, we cannot move two forward
        two_up = EMPTY;

    }

    // The pseudo legal moves are the places we can go and the places we attack that do not have our own pieces
    let moving_to: u128 = one_up | two_up;
    let pseudo_legal: u128 = (attacks & !team) & moving_to;

    (attacks, pseudo_legal)
}

/// This function determines the squares that the black pawn attacks,
/// and the squares that the black pawn may move to.
pub fn black_pawn_info(piece_info: &u128, team: &u128, opponents: &u128) -> (u128, u128) {
    let attacks: u128 = black_pawn_attack(piece_info);
    let piece: u128 = piece_info & BOARD1;

    let mut one_down: u128 = piece >> 16;
    let mut two_down: u128 = piece >> 32;
    let all: u128 = team | opponents;

    if one_down & all != 0 {

        // If any piece blocks the way, we cannot move forward
        one_down = EMPTY

    } else if piece & RANK_6 == 0 || two_down & all != 0 {

        // If we can move one forward, but we are not on rank 2 or any piece blocks the way, we cannot move two forward
        two_down = EMPTY;

    }

    // The pseudo legal moves are the places we can go and the places we attack that do not have our own pieces
    let moving_to: u128 = one_down | two_down;
    let pseudo_legal: u128 = (attacks & !team) & moving_to;

    (attacks, pseudo_legal)
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

    // We do not check for castling here, since in order to do so, we need to know if the king is in check,
    // or if any squares involved in the castle are being attacked

    (attacks, pseudo_legal)
}

/// This function determines the squares that the bishop attack and the squares it may move to,
/// as well as additional information on pins and checks.
pub fn bishop_info(piece_info: &u128, team: &u128, opponents: &u128, opponent_king: &u128) -> (u128, u128, u128, u128) {
    
    // This function repeats itself, however I do not see a good fix right now,
    // and as it is not a big deal, I will leave it like this for now.

    let mut attacks: u128 = EMPTY;
    let mut pseudo_legal: u128 = EMPTY;
    let mut check: u128 = EMPTY;
    let mut pin: u128 = EMPTY;

    let piece: u128 = piece_info & BOARD1;
    let mut result: u128;
    let mut blocked: bool;

    // Shoot ray up left to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece << 17 * i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // Shoot ray up right to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece << 15 * i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // Shoot ray down left to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece >> 15 * i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // Shoot ray right to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece >> 17 * i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // We move the info to the other board, since that is where we expect them
    (attacks >> 8, pseudo_legal >> 8, check >> 8, pin >> 8)
}

/// This function determines the squares that the rook attack and the squares it may move to,
/// as well as additional information on pins and checks.
pub fn rook_info(piece_info: &u128, team: &u128, opponents: &u128, opponent_king: &u128) -> (u128, u128, u128, u128) {

    // This function repeats itself, however I do not see a good fix right now,
    // and as it is not a big deal, I will leave it like this for now.

    let mut attacks: u128 = EMPTY;
    let mut pseudo_legal: u128 = EMPTY;
    let mut check: u128 = EMPTY;
    let mut pin: u128 = EMPTY;

    let piece: u128 = piece_info & BOARD1;
    let mut result: u128;
    let mut blocked: bool;

    // Shoot ray up to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece << 16 * i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // Shoot ray down to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece >> 16 * i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // Shoot ray left to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece << i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // Shoot ray right to check attack, checks and pins
    result = piece;
    blocked = false;

    for i in 1..8 {
        let pos: u128 = piece >> i;

        if pos & BOARD1 == 0 {

            // If the ray goes off the board, we can exit the loop.
            // Since we have not reached the opponent king, we do not have to add anything to check or pin
            break;

        }

        result |= pos;

        if pos & opponent_king != 0 {

            if blocked {

                // If we have been blocked before, this is a pin, so we add it
                pin |= result;
                
            } else {

                // If we were not blocked yet, we can attack the king. This is a check, an attack and a pseudo legal move, but not a pin
                attacks |= pos;
                pseudo_legal |= pos;
                check |= result;

            }

            // Since we are blocked and we do not have to look further for pins and checks,
            // we can exit out of the loop
            break;

        }

        if !blocked {

            if pos & team != 0 {

                // If we reach a teammate whilst not having been blocked yet, we add the pos to the attacks,
                // and we set blocked to true
                attacks |= pos;
                blocked = true;

            } else {

                if pos & opponents != 0 {

                    // If we reach an opponent, we set blocked to true
                    blocked = true;

                }

                // If we reach an opponent or an empty square whilst not having been blocked yet, we add the pos to the attacks and the pseudo legal moves
                attacks |= pos;
                pseudo_legal |= pos;
            }
        }
    }

    // We move the info to the other board, since that is where we expect them
    (attacks >> 8, pseudo_legal >> 8, check >> 8, pin >> 8)
}

/// This function determines the squares that the queen attack and the squares it may move to,
/// as well as additional information on pins and checks.
pub fn queen_info(piece_info: &u128, team: &u128, opponents: &u128, opponent_king: &u128) -> (u128, u128, u128, u128) {

    // The queen combines the patterns of the rook and the bishop
    
    let bishop_info: (u128, u128, u128, u128) = bishop_info(piece_info, team, opponents, opponent_king);
    let rook_info: (u128, u128, u128, u128) = rook_info(piece_info, team, opponents, opponent_king);

    let attacks: u128 = bishop_info.0 | rook_info.0;
    let pseudo_legal: u128 = bishop_info.1 | rook_info.1;
    let check: u128 = bishop_info.2 | rook_info.2;
    let pin: u128 = bishop_info.3 | rook_info.3;

    (attacks, pseudo_legal, check, pin)
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
