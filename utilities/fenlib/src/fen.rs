use crate::utils::*;
use crate::parsing;
use crate::moves;
use crate::default;

#[derive(Debug, Clone)]
pub struct Fen {
    pub boards: [u64; 12],
    pub white_to_move: bool,
    pub castling: u8,
    pub en_passant: u64,
    pub halfmove: u16,
    pub fullmove: u16,
    pub white: u64,
    pub black: u64,
    pub full: u64,
    pub white_attack: u64,
    pub black_attack: u64,
    pub white_check: u64,
    pub black_check: u64,
    pub white_pin: u64,
    pub black_pin: u64,
    pub legal_moves: [[u64; 3]; MAX_MOVES],
}

impl Fen {
    pub fn new() -> Self {
        
        Self {
            boards: default::BOARDS,
            white_to_move: default::WHITE_TO_MOVE,
            castling: default::CASTLING,
            en_passant: default::ENPASSANT,
            halfmove: default::HALFMOVE,
            fullmove: default::FULLMOVE,
            white: default::WHITE,
            black: default::BLACK,
            full: default::FULL,
            white_attack: default::WHITE_ATTACK,
            black_attack: default::BLACK_ATTACK,
            white_check: default::WHITE_CHECK,
            black_check: default::BLACK_CHECK,
            white_pin: default::WHITE_PIN,
            black_pin: default::BLACK_PIN,
            legal_moves: default::LEGAL_MOVES,
        }
    }

    pub fn from_str(fen_str: &str) -> Self {
        let fen_parts: Vec<&str> = fen_str.trim().split_whitespace().collect();
        if fen_parts.len() != 6 {
            panic!("Found incorrect fen notation");
        }

        let boards: [u64; 12] = parsing::string_to_board(fen_parts[0]);
        let white: u64 = get_white(&boards);
        let black: u64 = get_black(&boards);
        let full: u64 = white | black;

        Self {
            boards,
            white_to_move: parsing::string_to_turn(fen_parts[1]),
            castling: parsing::string_to_castling(fen_parts[2]),
            en_passant: parsing::string_to_enpassant(fen_parts[3]),
            halfmove: fen_parts[4].parse().ok().unwrap(),
            fullmove: fen_parts[5].parse().ok().unwrap(),
            white,
            black,
            full,
            white_attack: EMPTY,
            black_attack: EMPTY,
            white_check: EMPTY,
            black_check: EMPTY,
            white_pin: EMPTY,
            black_pin: EMPTY,
            legal_moves: [[EMPTY, EMPTY, EMPTY]; MAX_MOVES],
        }
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            parsing::board_to_string(&self.boards),
            parsing::turn_to_string(self.white_to_move),
            parsing::castling_to_string(&self.castling),
            parsing::enpassant_to_string(&self.en_passant),
            self.halfmove,
            self.fullmove
        )
    }

    pub fn to_visual(&self) -> [[String; 8]; 8] {
        parsing::board_to_visual(&self.boards)
    }

    pub fn is_legal_move_lan(&self, lan: &str) -> bool {
        let start: u64 = parsing::tile_to_bit(&lan[0..2]);
        let end: u64 = parsing::tile_to_bit(&lan[2..4]);

        let promoting_to: u64 = parsing::string_to_promotion(lan);

        self.is_legal_move(&[start, end, promoting_to])
    }

    pub fn is_legal_move(&self, move1: &[u64; 3]) -> bool {
        let start: u64 = move1[0];
        let end: u64 = move1[1];

        if !self.is_pseudo_legal(&start, &end, true) {
            return false
        }

        let mut new_fen: Self = self.clone();
    
        new_fen.move_to_fen(&move1);
        new_fen.white_to_move = !new_fen.white_to_move;
    
        if !new_fen.in_check() {
            return true;
        }
    
        false
    }

    pub fn get_possible_moves_tile(&self, tile: &str) -> Vec<String> {
        let moves_info: ([[u64; 3]; MAX_MOVES_PIECE], usize) = self.get_possible_moves(&parsing::tile_to_bit(tile));
        let moves:[[u64; 3]; MAX_MOVES_PIECE]  = moves_info.0;
        let count: usize = moves_info.1;

        let mut vec_moves: Vec<[u64; 3]> = Vec::new();

        for i in 0..count {
            vec_moves.push(moves[i]);
        }

        parsing::moves_to_lan_list(&vec_moves)
    }
    
    pub fn get_possible_moves(&self, start: &u64) -> ([[u64; 3]; MAX_MOVES_PIECE], usize) {
        let mut moves: [[u64; 3]; MAX_MOVES_PIECE] = [[0; 3]; MAX_MOVES_PIECE];
        let mut count: usize = 0;

        if self.boards[0] & start != 0 {
            let possible_moves: [u64; moves::PAWN_GUESS] = moves::white_pawn(&start);
            for i in 0..moves::PAWN_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    if self.boards[0] & start != 0 && RANK_0 & end != 0 {
                        moves[count + 0] = [*start, end, QUEEN_PROM];
                        moves[count + 1] = [*start, end, ROOK_PROM];
                        moves[count + 2] = [*start, end, BISHOP_PROM];
                        moves[count + 3] = [*start, end, KNIGHT_PROM];
                        count += 4;
                    } else {
                        moves[count] = [*start, end, NO_PROM];
                        count += 1;
                    }
                }
            }
        } else if self.boards[6] & start != 0 {
            let possible_moves: [u64; moves::PAWN_GUESS] = moves::black_pawn(&start);
            for i in 0..moves::PAWN_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    if self.boards[6] & start != 0 && RANK_7 & end != 0 {
                        moves[count + 0] = [*start, end, QUEEN_PROM];
                        moves[count + 1] = [*start, end, ROOK_PROM];
                        moves[count + 2] = [*start, end, BISHOP_PROM];
                        moves[count + 3] = [*start, end, KNIGHT_PROM];
                        count += 4;
                    } else {
                        moves[count] = [*start, end, NO_PROM];
                        count += 1;
                    }
                }
            }
        } else if self.boards[1] & start != 0 || self.boards[7] & start != 0 {
            let possible_moves: [u64; moves::KNIGHT_GUESS] = moves::knight(&start);
            for i in 0..moves::KNIGHT_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    moves[count] = [*start, end, NO_PROM];
                    count += 1;
                }
            }
        } else if self.boards[5] & start != 0 || self.boards[11] & start != 0 {
            let possible_moves: [u64; moves::KING_GUESS] = moves::king(&start);
            for i in 0..moves::KING_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    moves[count] = [*start, end, NO_PROM];
                    count += 1;
                }
            }
        } else if self.boards[4] & start != 0 || self.boards[10] & start != 0 {
            let possible_moves: [u64; moves::QUEEN_GUESS] = moves::queen(&start);
            for i in 0..moves::QUEEN_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    moves[count] = [*start, end, NO_PROM];
                    count += 1;
                }
            }
        } else if self.boards[3] & start != 0 || self.boards[9] & start != 0 {
            let possible_moves: [u64; moves::ROOK_GUESS] = moves::rook(&start);
            for i in 0..moves::ROOK_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    moves[count] = [*start, end, NO_PROM];
                    count += 1;
                }
            }
        } else if self.boards[2] & start != 0 || self.boards[8] & start != 0 {
            let possible_moves: [u64; moves::BISHOP_GUESS] = moves::bishop(&start);
            for i in 0..moves::BISHOP_GUESS {
                let end: u64 = possible_moves[i];
                if self.is_legal_move(&[*start, end, NO_PROM]) {
                    moves[count] = [*start, end, NO_PROM];
                    count += 1;
                }
            }
        } else {
            return (moves, 0);
        }
    
        (moves, count)
    }
    
    pub fn in_check(&self) -> bool {
        let king: u64 = match self.white_to_move {
            true => self.boards[5],
            false => self.boards[11]
        };

        let opponents: u64= match self.white_to_move {
            true => self.black,
            false => self.white
        };

        let mut new_fen: Self = self.clone();

        new_fen.white_to_move = !new_fen.white_to_move;
    
        for i in 0..64 {
            let piece: u64 = FIRST >> i;
            if piece & opponents != 0 && new_fen.is_pseudo_legal(&piece, &king, false) {
                return true;
            }
        }
    
        false
    }

    pub fn lan_to_fen(&mut self, lan: &str) {
        let start: u64 = parsing::tile_to_bit(&lan[0..2]);
        let end: u64 = parsing::tile_to_bit(&lan[2..4]);

        let promoting_to: u64 = parsing::string_to_promotion(lan);

        self.move_to_fen(&[start, end, promoting_to])
    }

    pub fn move_to_fen(&mut self, move1: &[u64; 3]) {
        let start: u64 = move1[0];
        let end: u64 = move1[1];
        let promoting_to: u64 = move1[2];

        // moving rook in case of castling
        if self.white_to_move && (start & self.boards[5] != 0) && (end & WHITE_KINGSIDE_BIT != 0) && (self.castling & WHITE_KINGSIDE_INFO != 0) {
            self.boards[3] &= !(WHITE_KINGSIDE_BIT >> 1);
            self.boards[3] |= WHITE_KINGSIDE_BIT << 1;
        } else if self.white_to_move && (start & self.boards[5] != 0) && (end & WHITE_QUEENSIDE_BIT != 0) && (self.castling & WHITE_QUEENSIDE_INFO != 0) {
            self.boards[3] &= !(WHITE_QUEENSIDE_BIT << 2);
            self.boards[3] |= WHITE_QUEENSIDE_BIT >> 1;
        } else if !self.white_to_move && (start & self.boards[11] != 0) && (end & BLACK_KINGSIDE_BIT != 0) && (self.castling & BLACK_KINGSIDE_INFO != 0) {
            self.boards[9] &= !(BLACK_KINGSIDE_BIT >> 1);
            self.boards[9] |= BLACK_KINGSIDE_BIT << 1;
        } else if !self.white_to_move && (start & self.boards[11] != 0) && (end & BLACK_QUEENSIDE_BIT != 0) && (self.castling & BLACK_QUEENSIDE_INFO != 0) {
            self.boards[9] &= !(BLACK_QUEENSIDE_BIT << 2);
            self.boards[9] |= BLACK_QUEENSIDE_BIT >> 1;
        }

        // removing piece in case of enpassant, enpassant is only legal from certain tiles, so we dont have to check whether the pawn is on the correct tile
        if self.white_to_move && (self.en_passant & end != 0) && (start & self.boards[0] != 0) {
            self.boards[6] &= !(self.en_passant >> 8)
        } else if !self.white_to_move && (self.en_passant & end != 0) && (start & self.boards[6] != 0) {
            self.boards[0] &= !(self.en_passant << 8)
        }
    
        // we check if an en passant is possible, thus if a pawn has moved two squares forward
        self.en_passant = EMPTY;
        if self.white_to_move && end & RANK_4 != 0 && start & self.boards[0] & RANK_6 != 0 {
            self.en_passant = start << 8;
        } else if !self.white_to_move && end & RANK_3 != 0 && start & self.boards[6] & RANK_1 != 0 {
            self.en_passant = start >> 8;
        }
    
        // increase halfmove if no pawn is moved and no piece is captured, else set it to 0
        if end & self.full == 0 && start & self.boards[0] == 0 && start & self.boards[6] == 0 {
            self.halfmove += 1;
        } else {
            self.halfmove = 0;
        }

        // the fullmove is only increased after a move by black
        if !self.white_to_move {
            self.fullmove += 1;
        }
    
        // apply capture to the board
        for i in 0..12 {
            if self.boards[i] & end != 0 {
                self.boards[i] &= !end;
            }
        }
    
        // apply move to the board
        for i in 0..12 {
            if self.boards[i] & start != 0 {
                self.boards[i] &= !start;
                self.boards[i] |= end;
            }
        }
    
        // promote to the given piece
        if promoting_to != NO_PROM {
            if self.white_to_move {
                self.boards[0] &= !end;
                if promoting_to & QUEEN_PROM != 0 {
                    self.boards[4] |= end
                } else if promoting_to & ROOK_PROM != 0 {
                    self.boards[3] |= end
                } else if promoting_to & BISHOP_PROM != 0 {
                    self.boards[2] |= end
                } else if promoting_to & KNIGHT_PROM != 0 {
                    self.boards[1] |= end
                } else {
                    panic!("Something went wrong with promoting")
                }
            } else {
                self.boards[6] &= !end;
                if promoting_to & QUEEN_PROM != 0 {
                    self.boards[10] |= end
                } else if promoting_to & ROOK_PROM != 0 {
                    self.boards[9] |= end
                } else if promoting_to & BISHOP_PROM != 0 {
                    self.boards[8] |= end
                } else if promoting_to & KNIGHT_PROM != 0 {
                    self.boards[7] |= end
                } else {
                    panic!("Something went wrong with promoting")
                }
            }
        }

        // if the rooks or kings are not on their starting position, the castling information is changed accordingly
        if self.boards[5] & WHITE_KING_BIT == 0 {
            self.castling &= !(WHITE_KINGSIDE_INFO & WHITE_QUEENSIDE_INFO);
        }

        if self.boards[11] & BLACK_KING_BIT == 0 {
            self.castling &= !(BLACK_KINGSIDE_INFO & BLACK_QUEENSIDE_INFO);
        }

        if self.boards[3] & (WHITE_KINGSIDE_BIT >> 1) == 0 {
            self.castling &= !WHITE_KINGSIDE_INFO;
        }

        if self.boards[3] & (WHITE_QUEENSIDE_BIT << 2) == 0 {
            self.castling &= !WHITE_QUEENSIDE_INFO;
        }

        if self.boards[9] & (BLACK_KINGSIDE_BIT >> 1) == 0 {
            self.castling &= !BLACK_KINGSIDE_INFO;
        }

        if self.boards[9] & (BLACK_QUEENSIDE_BIT << 2) == 0 {
            self.castling &= !BLACK_QUEENSIDE_INFO;
        }

        self.white_to_move = match self.white_to_move {
            true => false,
            false => true,
        };

        self.white = get_white(&self.boards);
        self.black = get_black(&self.boards);
        self.full = self.white | self.black;
    }

    pub fn get_all_possible_moves_lan(&self) -> Vec<String> {
        let moves: Vec<[u64; 3]> = self.get_all_possible_moves();
        parsing::moves_to_lan_list(&moves)
    }

    pub fn get_all_possible_moves(&self) -> Vec<[u64; 3]> {

        let mut vec_moves: Vec<[u64; 3]> = Vec::new();
    
        let pieces = match self.white_to_move {
            true => self.white,
            false => self.black
        };
    
        for i in 0..64 {
            let piece: u64 = FIRST >> i;
            
            if piece & pieces != 0 {
                let moves_info: ([[u64; 3]; MAX_MOVES_PIECE], usize) = self.get_possible_moves(&piece);
                let moves:[[u64; 3]; MAX_MOVES_PIECE]  = moves_info.0;
                let count: usize = moves_info.1;

                for i in 0..count {
                    vec_moves.push(moves[i])
                }
            }
        }
    
        vec_moves
    }

    pub fn game_ended(&self) -> String {
        let moves: usize = self.get_all_possible_moves().len();
        let in_check: bool = self.in_check();
    
        if moves == 0 && in_check {
            if self.white_to_move {
                "0-1".to_string()
            } else {
                "1-0".to_string()
            }
        } else if moves == 0 || self.halfmove > 100 {
            "½-½".to_string()
        } else {
            "not ended".to_string()
        }
    }
    
    pub fn legal_move_white_pawn(&self, start: &u64, end: &u64) -> bool {
    
        // if the pawn is on rank 8, it cannot move
        if start & RANK_0 != 0 {
            return false;
        }
    
        // if we are not on file 1, the move is diagonally left, and there is a black piece there, the move is legal
        if (start & FILE_0 == 0) && (end & (start << 9) & self.black != 0) {
            return true;
        }
    
        // if we are not on file 8, the move is diagonally right, and there is a black piece there, the move is legal
        if (start & FILE_7 == 0) && (end & (start << 7) & self.black != 0) {
            return true;
        }
    
        // if the forward square is obstructed, no forward move is legal
        if start << 8 & self.full != 0 {
            return false;
        }
    
        // if the move is one forward, and there is no piece there, the move is legal
        if end & (start << 8) != 0 {
            return true;
        }
    
        // if we are on rank 2, the move is two forward and there is no piece there, the move is legal
        if (start & RANK_6 != 0) && (end & (start << 16) & !self.full != 0) {
            return true;
        }
    
        false
    }
    
    pub fn legal_move_black_pawn(&self, start: &u64, end: &u64) -> bool {
        
        // if the pawn is on rank 1, it cannot move
        if start & RANK_7 != 0 {
            return false;
        }
    
        // if we are not on file 8, the move is diagonally right, and there is a white piece there, the move is legal
        if (start & FILE_7 == 0) && (end & (start >> 9) & self.white != 0) {
            return true;
        }
    
        // if we are not on file 1, the move is diagonally left, and there is a white piece there, the move is legal
        if (start & FILE_0 == 0) && (end & (start >> 7) & self.white != 0) {
            return true;
        }
    
        // if the forward square is obstructed, no forward move is legal
        if start >> 8 & self.full != 0 {
            return false;
        }
    
        // if the move is one forward, and there is no piece there, the move is legal
        if end & (start >> 8) != 0 {
            return true;
        }
    
        // if we are on rank 7, the move is two forward and there is no piece there, the move is legal
        if (start & RANK_1 != 0) && (end & (start >> 16) & !self.full != 0) {
            return true;
        }
    
        false
    }
    
    pub fn legal_move_knight(&self, start: &u64, end: &u64) -> bool {
        let one_left: u64 = start << 1 & !FILE_7;
        let one_right: u64 = start >> 1 & !FILE_0;
    
        // check if end is one left two up from start
        if end & (one_left << 16) != 0 {
            return true;
        }
    
        // check if end is one left two down from start
        if end & (one_left >> 16) != 0 {
            return true;
        }
    
        // check if end is one right two up from start
        if end & (one_right << 16) != 0 {
            return true;
        }
    
        // check if end is one right two down from start
        if end & (one_right >> 16) != 0 {
            return true;
        }
    
        let one_up: u64 = start << 8;
        let one_down: u64 = start >> 8;
    
        // check if end is one up two left from start
        if end & (one_up << 2) & !(FILE_6 | FILE_7) != 0 {
            return true;
        }
    
        // check if end is one up two right from start
        if end & (one_up >> 2) & !(FILE_0 | FILE_1) != 0 {
            return true;
        }
    
        // check if end is one down two left from start
        if end & (one_down << 2) & !(FILE_6 | FILE_7) != 0 {
            return true;
        }
    
        // check if end is one down two right from start
        if end & (one_down >> 2) & !(FILE_0 | FILE_1) != 0 {
            return true;
        }
    
        false
    }
    
    pub fn legal_move_king(&self, start: &u64, end: &u64) -> bool {
    
        // check if end is one up from start
        if end & (start << 8) != 0 {
            return true;
        }
    
        // check if end is one down from start
        if end & (start >> 8) != 0 {
            return true;
        }
    
        // check if end is one left from start
        let one_left: u64 = start << 1 & !FILE_7;
        if end & one_left != 0 {
            return true;
        }
    
        // check if end is one left one up from start
        if end & (one_left << 8) != 0 {
            return true;
        }
    
        // check if end is one left one down from start
        if end & (one_left >> 8) != 0 {
            return true;
        }
    
        // check if end is one right from start
        let one_right: u64 = start >> 1 & !FILE_0;
        if end & one_right != 0 {
            return true;
        }
    
        // check if end is one right one up from start
        if end & (one_right << 8) != 0 {
            return true;
        }
    
        // check if end is one right one down from start
        if end & (one_right >> 8) != 0 {
            return true;
        }
    
        false
    }
    
    pub fn legal_move_rook(&self, start: &u64, end: &u64) -> bool {
    
        // check for a move left
        if start & FILE_0 == 0 {
            for i in 1..8 {
                let pos: u64 = start << i;
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach file 0, we cannot move furthur, thus we break
                if pos & FILE_0 != 0 {
                    break;
                }
            }
        }
    
        // check for a move right
        if start & FILE_7 == 0 {
            for i in 1..8 {
                let pos: u64 = start >> i;
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach file 7, we cannot move furthur, thus we break
                if pos & FILE_7 != 0 {
                    break;
                }
            }
        }
    
        // check for a move up
        if start & RANK_0 == 0 {
            for i in 1..8 {
                let pos: u64 = start << (i * 8);
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach rank 0, we cannot move furthur, thus we break
                if pos & RANK_0 != 0 {
                    break;
                }
            }
        }
    
        // check for a move down
        if start & RANK_7 == 0 {
            for i in 1..8 {
                let pos: u64 = start >> (i * 8);
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach rank 7, we cannot move furthur, thus we break
                if pos & RANK_7 != 0 {
                    break;
                }
            }
        }
    
        false
    }
    
    pub fn legal_move_bishop(&self, start: &u64, end: &u64) -> bool {
    
        // check for a move left up
        if start & RANK_0 == 0 && start & FILE_7 == 0 {
            for i in 1..8 {
                let pos: u64 = (start >> i) << (i * 8);
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach file 7, we cannot move furthur, thus we break
                if pos & FILE_7 != 0 {
                    break;
                }
        
                // if we reach rank 0, we cannot move furthur, thus we break
                if pos & RANK_0 != 0 {
                    break;
                }
            }
        }
    
        // check for a move right up
        if start & RANK_0 == 0 && start & FILE_0 == 0 {
            for i in 1..8 {
                let pos: u64 = (start << i) << (i * 8);
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach file 0, we cannot move furthur, thus we break
                if pos & FILE_0 != 0 {
                    break;
                }
        
                // if we reach rank 0, we cannot move furthur, thus we break
                if pos & RANK_0 != 0 {
                    break;
                }
            }
        }
    
        // check for a move left down
        if start & RANK_7 == 0 && start & FILE_7 == 0 {
            for i in 1..8 {
                let pos: u64 = (start >> i) >> (i * 8);
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach file 7, we cannot move furthur, thus we break
                if pos & FILE_7 != 0 {
                    break;
                }
        
                // if we reach rank 7, we cannot move furthur, thus we break
                if pos & RANK_7 != 0 {
                    break;
                }
            }
        }
    
        // check for a move right down
        if start & RANK_7 == 0 && start & FILE_0 == 0 {
            for i in 1..8 {
                let pos: u64 = (start << i) >> (i * 8);
        
                // if we cannot move furthur, we break
                if pos & self.full & !end != 0 {
                    break;
                }
        
                // check if end is the pos
                if pos & end != 0 {
                    return true;
                }
        
                // if we reach file 0, we cannot move furthur, thus we break
                if pos & FILE_0 != 0 {
                    break;
                }
        
                // if we reach rank 7, we cannot move furthur, thus we break
                if pos & RANK_7 != 0 {
                    break;
                }
            }
        }
    
        false
    }
    
    pub fn check_standard_moves(&self, start: &u64, end: &u64) -> bool {
        if start & self.full == 0 {
            return false;
        }
    
        let is_white: bool = start & self.white != 0;
    
        if is_white && !self.white_to_move {
            return false;
        }
    
        if !is_white && self.white_to_move {
            return false;
        }
    
        if self.white_to_move && (end & self.white != 0) {
            return false;
        }
    
        if !self.white_to_move && (end & self.black != 0) {
            return false;
        }
    
        // check white pawns
        if self.boards[0] & start != 0 {
            return self.legal_move_white_pawn(&start, &end);
        }
    
        // check black pawns
        if self.boards[6] & start != 0 {
            return self.legal_move_black_pawn(&start, &end);
        }
    
        // check knights
        if self.boards[1] & start != 0 || self.boards[7] & start != 0 {
            return self.legal_move_knight(&start, &end);
        }
    
        // check kings
        if self.boards[5] & start != 0 || self.boards[11] & start != 0 {
            return self.legal_move_king(&start, &end);
        }
    
        let mut is_legal_rook: bool = false;
        let mut is_legal_bishop: bool = false;
        let is_queen: bool = self.boards[4] & start != 0 || self.boards[10] & start != 0;
    
        // check rooks and queens
        if is_queen || self.boards[3] & start != 0 || self.boards[9] & start != 0 {
            is_legal_rook = self.legal_move_rook(&start, &end);
            if !is_queen {
                return is_legal_rook;
            }
        }
    
        // check bishops and queens
        if is_queen || self.boards[2] & start != 0 || self.boards[8] & start != 0 {
            is_legal_bishop = self.legal_move_bishop(&start, &end);
            if !is_queen {
                return is_legal_bishop;
            }
        }
    
        // final check queens
        if is_legal_rook || is_legal_bishop {
            return true;
        }
    
        false
    }
    
    pub fn check_en_passant(&self, start: &u64, end: &u64) -> bool {
    
        // check if end is at the correct position for en passant
        if self.en_passant & end == 0 {
            return false
        }
    
        // check if the piece is a white pawn and in the right position
        if self.white_to_move && (start & self.boards[0] != 0) && (start & RANK_3 != 0) && ((start << 7) & end != 0 || (start << 9) & end != 0) {
            return true
        }
    
        // check if the piece is a black pawn and in the right position
        if !self.white_to_move && (start & self.boards[6] != 0) && (start & RANK_4 != 0) && ((start >> 7) & end != 0 || (start >> 9) & end != 0) {
            return true
        }
    
        false
    }
    
    pub fn check_castle(&self, start: &u64, end: &u64, check_check_castle: bool) -> bool {

        // check if the king is in check
        if check_check_castle && self.in_check() {
            return false
        }
        
        // check if the white king wants to move
        if self.white_to_move && (start & self.boards[5] == 0) {
            return false
        }
    
        // check if the black king wants to move
        if !self.white_to_move && (start & self.boards[11] == 0) {
            return false
        }
        
        // check if white king side position is clear and there are no piece in the way
        if self.white_to_move && (self.castling & (1u8 << 3) != 0) && (end & WHITE_KINGSIDE_BIT != 0) && (self.full & WHITE_KINGSIDE_CLEAR == 0) {
            return true
        }
    
        // check if white queen side position is clear and there are no piece in the way
        if self.white_to_move && (self.castling & (1u8 << 2) != 0) && (end & WHITE_QUEENSIDE_BIT != 0) && (self.full & WHITE_QUEENSIDE_CLEAR == 0) {
            return true
        }
    
        // check if black king side position is clear and there are no piece in the way
        if !self.white_to_move && (self.castling & (1u8 << 1) != 0) && (end & BLACK_KINGSIDE_BIT != 0) && (self.full & BLACK_KINGSIDE_CLEAR == 0) {
            return true
        }
    
        // check if black queen side position is clear and there are no piece in the way
        if !self.white_to_move && (self.castling & 1u8 != 0) && (end & BLACK_QUEENSIDE_BIT != 0) && (self.full & BLACK_QUEENSIDE_CLEAR == 0) {
            return true
        }
    
        false
    }
    
    pub fn is_pseudo_legal(&self, start: &u64, end: &u64, check_check_castle: bool) -> bool {
        if self.check_standard_moves(&start, &end) {
            return true;
        }
    
        if self.check_en_passant(&start, &end) {
            return true;
        }
    
        if self.check_castle(&start, &end, check_check_castle) {
            return true;
        }
    
        false
    
    }
}