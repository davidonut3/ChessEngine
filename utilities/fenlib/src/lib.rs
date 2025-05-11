pub mod parsing;
pub mod moves;
pub mod utils;

use std::time::Instant;

use crate::utils::*;

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
}

impl Fen {
    pub fn new() -> Self {
        Self::from_str(utils::DEFAULT)
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

        if !self.is_pseudo_legal(&start, &end) {
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
        let moves: Vec<[u64; 3]> = self.get_possible_moves(&parsing::tile_to_bit(tile));
        parsing::moves_to_lan_list(&moves)
    }
    
    pub fn get_possible_moves(&self, start: &u64) -> Vec<[u64; 3]> {
        // this function takes 500-3000 ns

        let time: Instant = Instant::now();

        let mut moves: Vec<[u64; 3]> = Vec::new();
        
        let possible_moves: Vec<u64>;
    
        // guessing moves takes 0-2000 ns
        if self.boards[0] & start != 0 {
            possible_moves = moves::white_pawn(&start);
        } else if self.boards[6] & start != 0 {
            possible_moves = moves::black_pawn(&start);
        } else if self.boards[1] & start != 0 || self.boards[7] & start != 0 {
            possible_moves = moves::knight(&start);
        } else if self.boards[5] & start != 0 || self.boards[11] & start != 0 {
            possible_moves = moves::king(&start);
        } else if self.boards[4] & start != 0 || self.boards[10] & start != 0 {
            possible_moves = moves::queen(&start);
        } else if self.boards[3] & start != 0 || self.boards[9] & start != 0 {
            possible_moves = moves::rook(&start);
        } else if self.boards[2] & start != 0 || self.boards[8] & start != 0 {
            possible_moves = moves::bishop(&start);
        } else {
            return Vec::new();
        }
    
        // creating list takes 400-2000 ns
        for end in possible_moves {

            // checking whether a move is legal takes 0-200 ns on average, with jumps to 300-1000 ns
            let is_legal: bool = self.is_legal_move(&[*start, end, NO_PROM]);

            // adding move to list takes 0-300 ns on average, with jumps to 400-1200 ns
            if is_legal {
                if (self.boards[0] & start != 0 && RANK_0 & end != 0) || (self.boards[6] & start != 0 && RANK_7 & end != 0) {
                    moves.push([*start, end, QUEEN_PROM]);
                    moves.push([*start, end, ROOK_PROM]);
                    moves.push([*start, end, BISHOP_PROM]);
                    moves.push([*start, end, KNIGHT_PROM]);
                } else {
                    moves.push([*start, end, NO_PROM]);
                }
            }
        }

        println!("Generating moves took {:?}", time.elapsed());
    
        moves
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
            if piece & opponents != 0 && new_fen.is_pseudo_legal(&piece, &king) {
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
        let mut moves: Vec<[u64; 3]> = Vec::new();
    
        let pieces = match self.white_to_move {
            true => self.white,
            false => self.black
        };
    
        for i in 0..64 {
            let piece: u64 = FIRST >> i;
            
            if piece & pieces != 0 {
                moves.append(&mut self.get_possible_moves(&piece));
            }
        }
    
        parsing::moves_to_lan_list(&moves)
    }

    pub fn get_all_possible_moves(&self) -> Vec<[u64; 3]> {
        let mut moves: Vec<[u64; 3]> = Vec::new();
    
        let pieces = match self.white_to_move {
            true => self.white,
            false => self.black
        };
    
        for i in 0..64 {
            let piece: u64 = FIRST >> i;
            
            if piece & pieces != 0 {
                moves.append(&mut self.get_possible_moves(&piece));
            }
        }
    
        moves
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
    
    pub fn check_castle(&self, start: &u64, end: &u64) -> bool {
        
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
    
    pub fn is_pseudo_legal(&self, start: &u64, end: &u64) -> bool {
        if self.check_standard_moves(&start, &end) {
            return true;
        }
    
        if self.check_en_passant(&start, &end) {
            return true;
        }
    
        if self.check_castle(&start, &end) {
            return true;
        }
    
        false
    
    }
}