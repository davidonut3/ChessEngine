use pyo3::prelude::*;

mod moves;

pub fn piece_to_index(piece: char) -> Option<usize> {
    match piece {
        'P' => Some(0),
        'N' => Some(1),
        'B' => Some(2),
        'R' => Some(3),
        'Q' => Some(4),
        'K' => Some(5),
        'p' => Some(6),
        'n' => Some(7),
        'b' => Some(8),
        'r' => Some(9),
        'q' => Some(10),
        'k' => Some(11),
        _ => None,
    }
}

pub fn index_to_piece(index: usize) -> Option<String> {
    match index {
        0 => Some("P".to_string()),
        1 => Some("N".to_string()),
        2 => Some("B".to_string()),
        3 => Some("R".to_string()),
        4 => Some("Q".to_string()),
        5 => Some("K".to_string()),
        6 => Some("p".to_string()),
        7 => Some("n".to_string()),
        8 => Some("b".to_string()),
        9 => Some("r".to_string()),
        10 => Some("q".to_string()),
        11 => Some("k".to_string()),
        _ => None,
    }
}

pub fn string_to_boards(board: &str) -> [u64; 12] {
    let mut boards: [u64; 12] = [0; 12];
    let rows: Vec<&str> = board.split('/').collect();

    for (rank, row) in rows.iter().enumerate() {
        let mut file: usize = 0;
        for ch in row.chars() {
            if ch.is_digit(10) {
                file += ch.to_digit(10).unwrap() as usize;
            } else if let Some(index) = piece_to_index(ch) {
                let sq: usize = 63 - (rank * 8 + file);
                boards[index] |= 1u64 << sq;
                file += 1;
            }
        }
    }
    boards
}

pub fn string_to_turn(turn: &str) -> Option<bool> {
    match turn {
        "w" => Some(true),
        "b" => Some(false),
        _ => None,
    }
}

pub fn tile_to_bit_safe(tile: &str) -> Option<u64> {
    if tile.len() != 2 {
        return None;
    }

    let file_char: char = tile.chars().nth(0)?;
    let rank_char: char = tile.chars().nth(1)?;

    let file: usize = match file_char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => return None,
    };

    let rank: usize = match rank_char {
        '8' => 0,
        '7' => 1,
        '6' => 2,
        '5' => 3,
        '4' => 4,
        '3' => 5,
        '2' => 6,
        '1' => 7,
        _ => return None,
    };

    Some((FIRST >> (rank * 8)) >> file)
}

pub fn tile_to_bit(tile: &str) -> u64 {
    let bit: Option<u64> = tile_to_bit_safe(tile);
    match bit {
        Some(_) => bit.unwrap(),
        None => EMPTY
    }
}

pub fn bit_to_tile(bit: &u64) -> Option<String> {
    let mut rank: usize = 0;
    let mut file: usize = 0;
    for i in 0..8 {
        if bit & get_rank(i) != 0 {
            rank = i as usize;
        }

        if bit & get_file(i) != 0 {
            file = i as usize;
        }
    }

    let rank: &str = match rank {
        0 => "8",
        1 => "7",
        2 => "6",
        3 => "5",
        4 => "4",
        5 => "3",
        6 => "2",
        7 => "1",
        _ => return None,
    };

    let file: &str = match file {
        0 => "a",
        1 => "b",
        2 => "c",
        3 => "d",
        4 => "e",
        5 => "f",
        6 => "g",
        7 => "h",
        _ => return None,
    };

    Some(file.to_string() + rank)
}

fn move_to_lan(start: &u64, end: &u64, promoting_to: &str) -> String {

    // this function does not check whether the move is legal

    let mut result: String = "".to_string();

    if let Some(start) = bit_to_tile(start) {
        result += &start
    }

    if let Some(end) = bit_to_tile(end) {
        result += &end
    }

    match promoting_to {
        "Q" | "q" | "R" | "r" | "B" | "b" | "N" | "n" => result += promoting_to,
        _ => result += ""
    };
    
    result
}

const EMPTY: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000;
const FIRST: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;
const FILE: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
const RANK: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;

const WHITE_KINGSIDE_BIT: u64 = FIRST >> (7 * 8) >> 6;
const WHITE_KINGSIDE_CLEAR: u64 = WHITE_KINGSIDE_BIT | (WHITE_KINGSIDE_BIT << 1);

const WHITE_QUEENSIDE_BIT: u64 = FIRST >> (7 * 8) >> 2;
const WHITE_QUEENSIDE_CLEAR: u64 = WHITE_QUEENSIDE_BIT | (WHITE_QUEENSIDE_BIT >> 1) | (WHITE_QUEENSIDE_BIT << 1);

const WHITE_KING_BIT: u64 = WHITE_KINGSIDE_BIT << 2;

const BLACK_KINGSIDE_BIT: u64 = FIRST >> 6;
const BLACK_KINGSIDE_CLEAR: u64 = BLACK_KINGSIDE_BIT | (BLACK_KINGSIDE_BIT << 1);

const BLACK_QUEENSIDE_BIT: u64 = FIRST >> 2;
const BLACK_QUEENSIDE_CLEAR: u64 = BLACK_QUEENSIDE_BIT | (BLACK_QUEENSIDE_BIT >> 1) | (BLACK_QUEENSIDE_BIT << 1);

const BLACK_KING_BIT: u64 = BLACK_KINGSIDE_BIT << 2;

const DEFAULT: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1";

pub fn get_file(index: u8) -> u64 {
    FILE >> index
}

pub fn get_rank(index: u8) -> u64 {
    RANK >> (index * 8)
}

fn get_white(boards: &[u64; 12]) -> u64 {
    boards[0] | boards[1] | boards[2] | boards[3] | boards[4] | boards[5]
}

fn get_black(boards: &[u64; 12]) -> u64 {
    boards[6] | boards[7] | boards[8] | boards[9] | boards[10] | boards[11]
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct Fen {

    pub boards: [u64; 12],

    #[pyo3(get)]
    pub white_to_move: bool,

    #[pyo3(get)]
    pub castling: String,

    #[pyo3(get)]
    pub en_passant: String,

    #[pyo3(get)]
    pub halfmove: i32,

    #[pyo3(get, set)]
    pub fullmove: i32,

    #[pyo3(get)]
    pub all_moves: Vec<String>,

    pub white: u64,

    pub black: u64,

    pub full: u64,
}

#[pymethods]
impl Fen {
    #[new]
    pub fn new() -> Self {
        Self::from_str(DEFAULT).unwrap()
    }

    #[staticmethod]
    pub fn from_str(fen_str: &str) -> Option<Self> {
        let fen_parts: Vec<&str> = fen_str.trim().split_whitespace().collect();
        if fen_parts.len() != 6 {
            return None;
        }

        let boards: [u64; 12] = string_to_boards(fen_parts[0]);
        let white: u64 = get_white(&boards);
        let black: u64 = get_black(&boards);
        let full: u64 = white | black;

        Some(Self {
            boards,
            white_to_move: string_to_turn(fen_parts[1]).unwrap(),
            castling: fen_parts[2].to_string(),
            en_passant: fen_parts[3].to_string(),
            halfmove: fen_parts[4].parse().ok()?,
            fullmove: fen_parts[5].parse().ok()?,
            all_moves: Vec::new(),
            white,
            black,
            full,
        })
    }

    pub fn to_string(&self) -> String {
        format!(
            "{} {} {} {} {} {}",
            self.bits_to_fen_board(),
            self.turn_to_string(),
            self.castling,
            self.en_passant,
            self.halfmove,
            self.fullmove
        )
    }

    pub fn to_visual(&self) -> [[String; 8]; 8] {
        let mut board: [[String; 8]; 8] = std::array::from_fn(|_| {
            std::array::from_fn(|_| "-".to_string())
        });

        for rank in 0..8 {
            for file in 0..8 {
                let bit: u64 = (FIRST >> (rank * 8)) >> file;
                for i in 0..12 {
                    if self.boards[i] & bit != 0 {
                        board[rank][file] = index_to_piece(i).unwrap();
                    }
                }
            }
        }

        board
    }
    
    pub fn bits_to_fen_board(&self) -> String {
        let mut result: String = String::new();
        for rank in 0..8 {
            let mut empty: i32 = 0;
            for file in 0..8 {
                let sq_index: i32 = rank * 8 + file;
                let pos: u64 = FIRST >> sq_index;
                let mut piece_found: bool = false;
                for (i, &board) in self.boards.iter().enumerate() {
                    if board & pos != 0 {
                        if empty > 0 {
                            result.push_str(&empty.to_string());
                            empty = 0;
                        }
                        let symbol: String = index_to_piece(i).unwrap();
                        result += &symbol;
                        piece_found = true;
                        break;
                    }
                }
                if !piece_found {
                    empty += 1;
                }
            }
            if empty > 0 {
                result.push_str(&empty.to_string());
            }
            if rank != 7 {
                result += "/";
            }
        }
        result
    }

    pub fn turn_to_string(&self) -> &str {
        match self.white_to_move {
            true => "w",
            false => "b",
        }
    }

    pub fn is_legal_move_lan(&mut self, lan: &str) -> bool {
        let start_tile: &str = &lan[0..2];
        let end_tile: &str = &lan[2..4];
        let promoting_to: String = if lan.len() == 5 {
            lan[4..5].to_string()
        } else {
            "-".to_string()
        };

        let start: u64 = tile_to_bit(start_tile);
        let end: u64 = tile_to_bit(end_tile);
    
        self.is_legal_move(&start, &end, &promoting_to)
    }

    pub fn get_possible_moves_tile(&mut self, tile: &str) -> Vec<String> {
        self.get_possible_moves(&tile_to_bit(tile))
    }
    
    pub fn in_check(&mut self) -> bool {
        let king: u64 = match self.white_to_move {
            true => self.boards[5],
            false => self.boards[11]
        };

        let opponents= match self.white_to_move {
            true => self.black,
            false => self.white
        };

        self.white_to_move = !self.white_to_move;
        let mut in_check: bool = false;
    
        for i in 0..64 {
            let piece: u64 = FIRST >> i;
            if piece & opponents != 0 && self.is_pseudo_legal(&piece, &king) {
                in_check = true;
                break;
            }
        }

        self.white_to_move = !self.white_to_move;
    
        in_check
    }

    pub fn lan_to_fen(&mut self, lan: &str) {
        self.all_moves.push(lan.to_string());

        let start: u64 = tile_to_bit(&lan[0..2]);
        let end: u64 = tile_to_bit(&lan[2..4]);

        // moving rook in case of castling
        if self.white_to_move && self.castling.contains('K') && (start & self.boards[5] != 0) && (end & WHITE_KINGSIDE_BIT != 0) {
            self.boards[3] &= !(WHITE_KINGSIDE_BIT >> 1);
            self.boards[3] |= WHITE_KINGSIDE_BIT << 1;
        } else if self.white_to_move && self.castling.contains('Q') && (start & self.boards[5] != 0) && (end & WHITE_QUEENSIDE_BIT != 0) {
            self.boards[3] &= !(WHITE_QUEENSIDE_BIT << 2);
            self.boards[3] |= WHITE_QUEENSIDE_BIT >> 1;
        } else if !self.white_to_move && self.castling.contains('k') && (start & self.boards[11] != 0) && (end & BLACK_KINGSIDE_BIT != 0) {
            self.boards[9] &= !(BLACK_KINGSIDE_BIT >> 1);
            self.boards[9] |= BLACK_KINGSIDE_BIT << 1;
        } else if !self.white_to_move && self.castling.contains('q') && (start & self.boards[11] != 0) && (end & BLACK_QUEENSIDE_BIT != 0) {
            self.boards[9] &= !(BLACK_QUEENSIDE_BIT << 2);
            self.boards[9] |= BLACK_QUEENSIDE_BIT >> 1;
        }

        // removing piece in case of enpassant, enpassant is only legal from certain tiles, so we dont have to check whether the pawn is on the correct tile
        let enpassant_bit: u64 = tile_to_bit(&self.en_passant);
        if self.white_to_move && (enpassant_bit & end != 0) && (start & self.boards[0] != 0) {
            self.boards[6] &= !(enpassant_bit >> 8)
        } else if !self.white_to_move && (enpassant_bit & end != 0) && (start & self.boards[6] != 0) {
            self.boards[0] &= !(enpassant_bit << 8)
        }
    
        // if the rooks or kings are not on their starting position, the castling information is changed accordingly
        if self.boards[5] & WHITE_KING_BIT == 0 {
            self.castling = self.castling.chars().filter(|&c| c != 'K' && c != 'Q').collect();
        }

        if self.boards[11] & BLACK_KING_BIT == 0 {
            self.castling = self.castling.chars().filter(|&c| c != 'k' && c != 'q').collect();
        }

        if self.boards[3] & (WHITE_KINGSIDE_BIT >> 1) == 0 {
            self.castling = self.castling.chars().filter(|&c| c != 'K').collect();
        }

        if self.boards[3] & (WHITE_QUEENSIDE_BIT << 2) == 0 {
            self.castling = self.castling.chars().filter(|&c| c != 'Q').collect();
        }

        if self.boards[9] & (BLACK_KINGSIDE_BIT >> 1) == 0 {
            self.castling = self.castling.chars().filter(|&c| c != 'k').collect();
        }

        if self.boards[9] & (BLACK_QUEENSIDE_BIT << 2) == 0 {
            self.castling = self.castling.chars().filter(|&c| c != 'q').collect();
        }
    
        if self.castling.len() == 0 {
            self.castling = "-".to_string();
        }
    
        // we check if an en passant is possible, thus if a pawn has moved two squares forward
        self.en_passant = "-".to_string();
        if self.white_to_move && end & get_rank(4) != 0 && start & self.boards[0] & get_rank(6) != 0 {
            self.en_passant = bit_to_tile(&(start << 8)).unwrap();
        } else if !self.white_to_move && end & get_rank(3) != 0 && start & self.boards[6] & get_rank(1) != 0 {
            self.en_passant = bit_to_tile(&(start >> 8)).unwrap();
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
    
        for i in 0..12 {
            if self.boards[i] & end != 0 {
                self.boards[i] &= !end;
            }
        }
    
        for i in 0..12 {
            if self.boards[i] & start != 0 {
                self.boards[i] &= !start;
                self.boards[i] |= end;
            }
        }
    
        if lan.len() == 5 {
            let promoting: &Option<char> = &lan.chars().nth(4);
            if self.white_to_move {
                self.boards[0] &= !end;
                match promoting {
                    Some('q') | Some('Q') => self.boards[4] |= end,
                    Some('r') | Some('R') => self.boards[3] |= end,
                    Some('n') | Some('N') => self.boards[1] |= end,
                    Some('b') | Some('B') => self.boards[2] |= end,
                    Some(_) => (),
                    None => panic!("Something went wrong with promoting")
                }
            } else {
                self.boards[6] &= !end;
                match promoting {
                    Some('q') | Some('Q') => self.boards[10] |= end,
                    Some('r') | Some('R') => self.boards[9] |= end,
                    Some('n') | Some('N') => self.boards[7] |= end,
                    Some('b') | Some('B') => self.boards[8] |= end,
                    Some(_) => (),
                    None => panic!("Something went wrong with promoting")
                }
            }
        }

        self.white_to_move = match self.white_to_move {
            true => false,
            false => true,
        };

        self.white = get_white(&self.boards);
        self.black = get_black(&self.boards);
        self.full = self.white | self.black;
    }

    pub fn get_all_possible_moves(&mut self) -> Vec<String> {
        let mut moves: Vec<String> = Vec::new();
    
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

    pub fn game_ended(&mut self) -> String {
        let result: String;
    
        if self.halfmove > 100 {
            result = "½-½".to_string();
        } else {
            let moves: usize = self.get_all_possible_moves().len();
            let in_check: bool = self.in_check();
    
            if moves == 0 && in_check {
                result = if self.white_to_move {
                    "0-1".to_string()
                } else {
                    "1-0".to_string()
                };
            } else if moves == 0 {
                result = "½-½".to_string();
            } else {
                result = "not ended".to_string();
            }
        }

        if result != "not ended" {
            self.all_moves.push(result.clone());
        }
    
        result
    }
}

impl Fen {
    pub fn is_legal_move(&mut self, start: &u64, end: &u64, promoting_to: &str) -> bool {
        if !self.is_pseudo_legal(&start, &end) {
            return false
        }

        let lan: String = move_to_lan(start, end, promoting_to);
        let mut new_fen: Self = Self {
            boards: self.boards,
            white_to_move: self.white_to_move,
            castling: self.castling.clone(),
            en_passant: self.en_passant.clone(),
            halfmove: self.halfmove,
            fullmove: self.fullmove,
            all_moves: Vec::new(),
            white: self.white,
            black: self.black,
            full: self.full,
        };

        new_fen.lan_to_fen(&lan);
        new_fen.white_to_move = !new_fen.white_to_move;

        if !new_fen.in_check() {
            return true;
        }

        false
    }

    pub fn get_possible_moves(&mut self, start: &u64) -> Vec<String> {
        let mut moves: Vec<String> = Vec::new();
        
        let possible_moves: Vec<u64>;

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
            possible_moves = Vec::new();
        }

        for end in possible_moves {
            if self.is_legal_move(start, &end, "-") {
                if (self.boards[0] & start != 0 && get_rank(0) & end != 0) || (self.boards[6] & start != 0 && get_rank(7) & end != 0) {
                    moves.push(move_to_lan(start, &end, "q"));
                    moves.push(move_to_lan(start, &end, "r"));
                    moves.push(move_to_lan(start, &end, "b"));
                    moves.push(move_to_lan(start, &end, "n"));
                } else {
                    moves.push(move_to_lan(start, &end, "-"))
                }
            }
        }
    
        moves
    }

    pub fn legal_move_white_pawn(&self, start: &u64, end: &u64) -> bool {
    
        // if the pawn is on rank 8, it cannot move
        if start & get_rank(0) != 0 {
            return false;
        }
    
        // if we are not on file 1, the move is diagonally left, and there is a black piece there, the move is legal
        if (start & get_file(0) == 0) && (end & (start << 9) & self.black != 0) {
            return true;
        }
    
        // if we are not on file 8, the move is diagonally right, and there is a black piece there, the move is legal
        if (start & get_file(7) == 0) && (end & (start << 7) & self.black != 0) {
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
        if (start & get_rank(6) != 0) && (end & (start << 16) & !self.full != 0) {
            return true;
        }
    
        false
    }
    
    pub fn legal_move_black_pawn(&self, start: &u64, end: &u64) -> bool {
        
        // if the pawn is on rank 1, it cannot move
        if start & get_rank(7) != 0 {
            return false;
        }
    
        // if we are not on file 8, the move is diagonally right, and there is a white piece there, the move is legal
        if (start & get_file(7) == 0) && (end & (start >> 9) & self.white != 0) {
            return true;
        }
    
        // if we are not on file 1, the move is diagonally left, and there is a white piece there, the move is legal
        if (start & get_file(0) == 0) && (end & (start >> 7) & self.white != 0) {
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
        if (start & get_rank(1) != 0) && (end & (start >> 16) & !self.full != 0) {
            return true;
        }
    
        false
    }
    
    pub fn legal_move_knight(&self, start: &u64, end: &u64) -> bool {
        let one_left: u64 = start << 1 & !get_file(7);
        let one_right: u64 = start >> 1 & !get_file(0);
    
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
        if end & (one_up << 2) & !(get_file(6) | get_file(7)) != 0 {
            return true;
        }
    
        // check if end is one up two right from start
        if end & (one_up >> 2) & !(get_file(0) | get_file(1)) != 0 {
            return true;
        }
    
        // check if end is one down two left from start
        if end & (one_down << 2) & !(get_file(6) | get_file(7)) != 0 {
            return true;
        }
    
        // check if end is one down two right from start
        if end & (one_down >> 2) & !(get_file(0) | get_file(1)) != 0 {
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
        let one_left: u64 = start << 1 & !get_file(7);
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
        let one_right: u64 = start >> 1 & !get_file(0);
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
        if start & get_file(0) == 0 {
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
                if pos & get_file(0) != 0 {
                    break;
                }
            }
        }
    
        // check for a move right
        if start & get_file(7) == 0 {
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
                if pos & get_file(7) != 0 {
                    break;
                }
            }
        }
    
        // check for a move up
        if start & get_rank(0) == 0 {
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
                if pos & get_rank(0) != 0 {
                    break;
                }
            }
        }
    
        // check for a move down
        if start & get_rank(7) == 0 {
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
                if pos & get_rank(7) != 0 {
                    break;
                }
            }
        }
    
        false
    }
    
    pub fn legal_move_bishop(&self, start: &u64, end: &u64) -> bool {
    
        // check for a move left up
        if start & get_rank(0) == 0 && start & get_file(7) == 0 {
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
                if pos & get_file(7) != 0 {
                    break;
                }
        
                // if we reach rank 0, we cannot move furthur, thus we break
                if pos & get_rank(0) != 0 {
                    break;
                }
            }
        }
    
        // check for a move right up
        if start & get_rank(0) == 0 && start & get_file(0) == 0 {
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
                if pos & get_file(0) != 0 {
                    break;
                }
        
                // if we reach rank 0, we cannot move furthur, thus we break
                if pos & get_rank(0) != 0 {
                    break;
                }
            }
        }
    
        // check for a move left down
        if start & get_rank(7) == 0 && start & get_file(7) == 0 {
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
                if pos & get_file(7) != 0 {
                    break;
                }
        
                // if we reach rank 7, we cannot move furthur, thus we break
                if pos & get_rank(7) != 0 {
                    break;
                }
            }
        }
    
        // check for a move right down
        if start & get_rank(7) == 0 && start & get_file(0) == 0 {
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
                if pos & get_file(0) != 0 {
                    break;
                }
        
                // if we reach rank 7, we cannot move furthur, thus we break
                if pos & get_rank(7) != 0 {
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
        if tile_to_bit(&self.en_passant) & end == 0 {
            return false
        }
    
        // check if the piece is a white pawn and in the right position
        if self.white_to_move && (start & self.boards[0] != 0) && (start & get_rank(3) != 0) && ((start << 7) & end != 0 || (start << 9) & end != 0) {
            return true
        }
    
        // check if the piece is a black pawn and in the right position
        if !self.white_to_move && (start & self.boards[6] != 0) && (start & get_rank(4) != 0) && ((start >> 7) & end != 0 || (start >> 9) & end != 0) {
            return true
        }
    
        false
    }
    
    pub fn check_castle(&self, start: &u64, end: &u64, white_kingside: bool, white_queenside: bool, black_kingside: bool, black_queenside: bool) -> bool {
        
        // check if the white king wants to move
        if self.white_to_move && (start & self.boards[5] == 0) {
            return false
        }
    
        // check if the black king wants to move
        if !self.white_to_move && (start & self.boards[11] == 0) {
            return false
        }
        
        // check if white king side position is clear and there are no piece in the way
        if self.white_to_move && white_kingside && (end & WHITE_KINGSIDE_BIT != 0) && (self.full & WHITE_KINGSIDE_CLEAR == 0) {
            return true
        }
    
        // check if white queen side position is clear and there are no piece in the way
        if self.white_to_move && white_queenside && (end & WHITE_QUEENSIDE_BIT != 0) && (self.full & WHITE_QUEENSIDE_CLEAR == 0) {
            return true
        }
    
        // check if black king side position is clear and there are no piece in the way
        if !self.white_to_move && black_kingside && (end & BLACK_KINGSIDE_BIT != 0) && (self.full & BLACK_KINGSIDE_CLEAR == 0) {
            return true
        }
    
        // check if black queen side position is clear and there are no piece in the way
        if !self.white_to_move && black_queenside && (end & BLACK_QUEENSIDE_BIT != 0) && (self.full & BLACK_QUEENSIDE_CLEAR == 0) {
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
    
        let is_castle: bool = match self.castling.as_str() {
            "_" => false,
            "K" => self.check_castle(&start, &end, true, false, false, false),
            "k" => self.check_castle(&start, &end, false, false, true, false),
            "Q" => self.check_castle(&start, &end, false, true, false, false),
            "q" => self.check_castle(&start, &end, false, false, false, true),
            "KQ" => self.check_castle(&start, &end, true, true, false, false),
            "Kk" => self.check_castle(&start, &end, true, false, true, false),
            "Kq" => self.check_castle(&start, &end, true, false, false, true),
            "Qk" => self.check_castle(&start, &end, false, true, true, false),
            "Qq" => self.check_castle(&start, &end, false, true, false, true),
            "kq" => self.check_castle(&start, &end, false, false, true, true),
            "KQk" => self.check_castle(&start, &end, true, true, true, false),
            "KQq" => self.check_castle(&start, &end, true, true, false, true),
            "Kkq" => self.check_castle(&start, &end, true, false, true, true),
            "Qkq" => self.check_castle(&start, &end, false, true, true, true),
            "KQkq" => self.check_castle(&start, &end, true, true, true, true),
            _ => false
        };
    
        if is_castle {
            return true;
        }
    
        false
    
    }
}

#[pymodule]
fn rust_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<Fen>()?;
    Ok(())
}