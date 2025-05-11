use pyo3::prelude::*;

pub const EMPTY: u64 = 0b0000000000000000000000000000000000000000000000000000000000000000;
pub const FULL: u64 = 0b1111111111111111111111111111111111111111111111111111111111111111;
pub const FIRST: u64 = 0b1000000000000000000000000000000000000000000000000000000000000000;
pub const FILE: u64 = 0b1000000010000000100000001000000010000000100000001000000010000000;
pub const RANK: u64 = 0b1111111100000000000000000000000000000000000000000000000000000000;

const WHITE_KINGSIDE_BIT: u64 = FIRST >> (7 * 8) >> 6;
const WHITE_KINGSIDE_CLEAR: u64 = WHITE_KINGSIDE_BIT | (WHITE_KINGSIDE_BIT << 1);

const WHITE_QUEENSIDE_BIT: u64 = FIRST >> (7 * 8) >> 2;
const WHITE_QUEENSIDE_CLEAR: u64 = WHITE_QUEENSIDE_BIT | (WHITE_QUEENSIDE_BIT >> 1) | (WHITE_QUEENSIDE_BIT << 1);

const BLACK_KINGSIDE_BIT: u64 = FIRST >> 6;
const BLACK_KINGSIDE_CLEAR: u64 = BLACK_KINGSIDE_BIT | (BLACK_KINGSIDE_BIT << 1);

const BLACK_QUEENSIDE_BIT: u64 = FIRST >> 2;
const BLACK_QUEENSIDE_CLEAR: u64 = BLACK_QUEENSIDE_BIT | (BLACK_QUEENSIDE_BIT >> 1) | (BLACK_QUEENSIDE_BIT << 1);

fn piece_to_index(piece: char) -> Option<usize> {
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

fn index_to_piece(index: usize) -> Option<String> {
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

fn tile_to_pos(tile: &str) -> Option<[usize; 2]> {
    if tile.len() != 2 {
        return None;
    }

    let file_char = tile.chars().nth(0)?;
    let rank_char = tile.chars().nth(1)?;

    let file = match file_char {
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

    let rank = match rank_char {
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

    Some([rank, file])
}

fn pos_to_tile(pos: [usize; 2]) -> Option<String> {
    let rank: &str = match pos[0] {
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

    let file: &str = match pos[1] {
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

fn fen_get_castle(fen: &str) -> String {
    let parts: Vec<&str> = fen.split_whitespace().collect();

    parts[2].to_string()
}

fn fen_get_enpassant(fen: &str) -> String {
    let parts: Vec<&str> = fen.split_whitespace().collect();

    parts[3].to_string()
}

fn fen_board_to_bits(fen_board: &str) -> [u64; 12] {
    let mut boards: [u64; 12] = [0; 12];
    let rows: Vec<&str> = fen_board.split(' ').next().unwrap().split('/').collect();
    assert_eq!(rows.len(), 8, "Invalid FEN board representation");

    for (rank, row) in rows.iter().enumerate() {
        let mut file = 0;
        for ch in row.chars() {
            if ch.is_digit(10) {
                file += ch.to_digit(10).unwrap() as usize;
            } else if let Some(index) = piece_to_index(ch) {
                let sq = 63 - (rank * 8 + file);
                boards[index] |= 1u64 << sq;
                file += 1;
            }
        }
    }

    boards
}

fn bits_to_fen_board(boards: &[u64; 12]) -> String {
    let mut result = String::new();

    for rank in 0..8 {
        let mut empty = 0;

        for file in 0..8 {
            let sq_index = rank * 8 + file;
            let pos = FIRST >> sq_index;

            let mut piece_found = false;
            for (i, &bb) in boards.iter().enumerate() {
                if bb & pos != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }

                    let symbol = index_to_piece(i).unwrap_or("?".to_string());
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

fn _print_board(board: u64) {
    println!("{}", format!("{:b}", board));
}

fn _print_boards(board: &[u64; 12]) {
    for (i, board) in board.iter().enumerate() {
        println!("Piece {}: {:#066b}", i, board);
    }
}

fn pos_to_bit(pos: &[usize; 2]) -> u64 {
    (FIRST >> (pos[0] * 8)) >> pos[1]
}

fn bit_to_pos(bit: &u64) -> [usize; 2] {
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

    [rank, file]
}

pub fn get_file(index: u8) -> u64 {
    FILE >> index
}

pub fn get_rank(index: u8) -> u64 {
    RANK >> (index * 8)
}

pub fn get_pieces_on_file(index: u8, board: &[u64; 12]) -> [u64; 12] {
    let mut result: [u64; 12] = [0; 12];
    let file: u64 = get_file(index);
    for (i, bitboard) in board.iter().enumerate() {
        result[i] = bitboard & file;
    }
    result
}

pub fn get_pieces_on_rank(index: u8, board: &[u64; 12]) -> [u64; 12] {
    let mut result: [u64; 12] = [0; 12];
    let file: u64 = get_rank(index);
    for (i, bitboard) in board.iter().enumerate() {
        result[i] = bitboard & file;
    }
    result
}

pub fn get_white(board: &[u64; 12]) -> u64 {
    let mut result: u64 = EMPTY;
    for (i, bitboard) in board.iter().enumerate() {
        if i <= 5 {
            result |= bitboard;
        }
    }
    result
}

pub fn get_black(board: &[u64; 12]) -> u64 {
    let mut result: u64 = EMPTY;
    for (i, bitboard) in board.iter().enumerate() {
        if i > 5 {
            result |= bitboard;
        }
    }
    result
}

pub fn legal_move_white_pawn(start: &u64, end: &u64, full: &u64, black: &u64) -> bool {
    
    // if the pawn is on rank 8, it cannot move
    if start & get_rank(0) != 0 {
        return false;
    }

    // if we are not on file 1, the move is diagonally left, and there is a black piece there, the move is legal
    if (start & get_file(0) == 0) && (end & (start << 9) & black != 0) {
        return true;
    }

    // if we are not on file 8, the move is diagonally right, and there is a black piece there, the move is legal
    if (start & get_file(7) == 0) && (end & (start << 7) & black != 0) {
        return true;
    }

    // if the forward square is obstructed, no forward move is legal
    if start << 8 & full != 0 {
        return false;
    }

    // if the move is one forward, and there is no piece there, the move is legal
    if end & (start << 8) != 0 {
        return true;
    }

    // if we are on rank 2, the move is two forward and there is no piece there, the move is legal
    if (start & get_rank(6) != 0) && (end & (start << 16) & !full != 0) {
        return true;
    }

    false
}

pub fn legal_move_black_pawn(start: &u64, end: &u64, full: &u64, white: &u64) -> bool {
    
    // if the pawn is on rank 1, it cannot move
    if start & get_rank(7) != 0 {
        return false;
    }

    // if we are not on file 8, the move is diagonally right, and there is a white piece there, the move is legal
    if (start & get_file(7) == 0) && (end & (start >> 9) & white != 0) {
        return true;
    }

    // if we are not on file 1, the move is diagonally left, and there is a white piece there, the move is legal
    if (start & get_file(0) == 0) && (end & (start >> 7) & white != 0) {
        return true;
    }

    // if the forward square is obstructed, no forward move is legal
    if start >> 8 & full != 0 {
        return false;
    }

    // if the move is one forward, and there is no piece there, the move is legal
    if end & (start >> 8) != 0 {
        return true;
    }

    // if we are on rank 7, the move is two forward and there is no piece there, the move is legal
    if (start & get_rank(1) != 0) && (end & (start >> 16) & !full != 0) {
        return true;
    }

    false
}

pub fn legal_move_knight(start: &u64, end: &u64) -> bool {
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

pub fn legal_move_king(start: &u64, end: &u64) -> bool {

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

pub fn legal_move_rook(start: &u64, end: &u64, full: &u64) -> bool {

    // check for a move left
    if start & get_file(0) == 0 {
        for i in 1..8 {
            let pos: u64 = start << i;
    
            // if we cannot move furthur, we break
            if pos & full & !end != 0 {
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
            if pos & full & !end != 0 {
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
            if pos & full & !end != 0 {
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
            if pos & full & !end != 0 {
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

pub fn legal_move_bishop(start: &u64, end: &u64, full: &u64) -> bool {

    // check for a move left up
    if start & get_rank(0) == 0 && start & get_file(7) == 0 {
        for i in 1..8 {
            let pos: u64 = (start >> i) << (i * 8);
    
            // if we cannot move furthur, we break
            if pos & full & !end != 0 {
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
            if pos & full & !end != 0 {
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
            if pos & full & !end != 0 {
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
            if pos & full & !end != 0 {
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

pub fn check_standard_moves(boards: &[u64; 12], white_to_move: bool, white: &u64, black: &u64, full: &u64, start: &u64, end: &u64) -> bool {
    if start & full == 0 {
        return false;
    }

    let is_white: bool = start & white != 0;

    if is_white && !white_to_move {
        return false;
    }

    if !is_white && white_to_move {
        return false;
    }

    if white_to_move && (end & white != 0) {
        return false;
    }

    if !white_to_move && (end & black != 0) {
        return false;
    }

    // check white pawns
    if boards[0] & start != 0 {
        return legal_move_white_pawn(&start, &end, &full, &black);
    }

    // check black pawns
    if boards[6] & start != 0 {
        return legal_move_black_pawn(&start, &end, &full, &white);
    }

    // check knights
    if boards[1] & start != 0 || boards[7] & start != 0 {
        return legal_move_knight(&start, &end);
    }

    // check kings
    if boards[5] & start != 0 || boards[11] & start != 0 {
        return legal_move_king(&start, &end);
    }

    let mut is_legal_rook: bool = false;
    let mut is_legal_bishop: bool = false;
    let is_queen: bool = boards[4] & start != 0 || boards[10] & start != 0;

    // check rooks and queens
    if is_queen || boards[3] & start != 0 || boards[9] & start != 0 {
        is_legal_rook = legal_move_rook(&start, &end, &full);
        if !is_queen {
            return is_legal_rook;
        }
    }

    // check bishops and queens
    if is_queen || boards[2] & start != 0 || boards[8] & start != 0 {
        is_legal_bishop = legal_move_bishop(&start, &end, &full);
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

pub fn check_en_passant(boards: &[u64; 12], white_to_move: bool, en_passant: &u64, start: &u64, end: &u64) -> bool {

    // check if end is at the correct position for en passant
    if en_passant & end == 0 {
        return false
    }

    // check if the piece is a white pawn and in the right position
    if white_to_move && (start & boards[0] != 0) && (start & get_rank(3) != 0) && ((start << 7) & end != 0 || (start << 9) & end != 0) {
        return true
    }

    // check if the piece is a black pawn and in the right position
    if !white_to_move && (start & boards[6] != 0) && (start & get_rank(4) != 0) && ((start >> 7) & end != 0 || (start >> 9) & end != 0) {
        return true
    }

    false
}

pub fn check_castle(boards: &[u64; 12], full: &u64, start: &u64, end: &u64, white_to_move: bool, white_kingside: bool, white_queenside: bool, black_kingside: bool, black_queenside: bool) -> bool {
    if white_to_move && (start & boards[5] == 0) {
        return false
    }

    if !white_to_move && (start & boards[11] == 0) {
        return false
    }

    if white_to_move && white_kingside && (end & WHITE_KINGSIDE_BIT != 0) && (full & WHITE_KINGSIDE_CLEAR == 0) {
        return true
    }

    if white_to_move && white_queenside && (end & WHITE_QUEENSIDE_BIT != 0) && (full & WHITE_QUEENSIDE_CLEAR == 0) {
        return true
    }

    if !white_to_move && black_kingside && (end & BLACK_KINGSIDE_BIT != 0) && (full & BLACK_KINGSIDE_CLEAR == 0) {
        return true
    }

    if !white_to_move && black_queenside && (end & BLACK_QUEENSIDE_BIT != 0) && (full & BLACK_QUEENSIDE_CLEAR == 0) {
        return true
    }

    false
}

pub fn is_pseudo_legal(boards: &[u64; 12], white_to_move: bool, castle: &str, enpassant: &str, start: &u64, end: &u64) -> bool {
    let white: u64 = get_white(&boards);
    let black: u64 = get_black(&boards);
    let full: u64 = white | black;

    if check_standard_moves(&boards, white_to_move, &white, &black, &full, &start, &end) {
        return true;
    }

    if let Some(enpassant_pos) = tile_to_pos(enpassant) {
        let enpassant_bit: u64 = pos_to_bit(&enpassant_pos);
        if check_en_passant(&boards, white_to_move, &enpassant_bit, &start, &end) {
            return true;
        }
    }

    let is_castle = match castle {
        "_" => false,
        "K" => check_castle(&boards, &full, &start, &end, white_to_move, true, false, false, false),
        "k" => check_castle(&boards, &full, &start, &end, white_to_move, false, false, true, false),
        "Q" => check_castle(&boards, &full, &start, &end, white_to_move, false, true, false, false),
        "q" => check_castle(&boards, &full, &start, &end, white_to_move, false, false, false, true),
        "KQ" => check_castle(&boards, &full, &start, &end, white_to_move, true, true, false, false),
        "Kk" => check_castle(&boards, &full, &start, &end, white_to_move, true, false, true, false),
        "Kq" => check_castle(&boards, &full, &start, &end, white_to_move, true, false, false, true),
        "Qk" => check_castle(&boards, &full, &start, &end, white_to_move, false, true, true, false),
        "Qq" => check_castle(&boards, &full, &start, &end, white_to_move, false, true, false, true),
        "kq" => check_castle(&boards, &full, &start, &end, white_to_move, false, false, true, true),
        "KQk" => check_castle(&boards, &full, &start, &end, white_to_move, true, true, true, false),
        "KQq" => check_castle(&boards, &full, &start, &end, white_to_move, true, true, false, true),
        "Kkq" => check_castle(&boards, &full, &start, &end, white_to_move, true, false, true, true),
        "Qkq" => check_castle(&boards, &full, &start, &end, white_to_move, false, true, true, true),
        "KQkq" => check_castle(&boards, &full, &start, &end, white_to_move, true, true, true, true),
        _ => false
    };

    if is_castle {
        return true;
    }

    false

}

pub fn check_check(boards: &[u64; 12], white_to_move: bool, castle: &str, enpassant: &str) -> bool {
    let king: u64;
    let opponents: u64;
    
    if white_to_move {
        king = boards[5];
        opponents = get_black(boards);
    } else {
        king = boards[11];
        opponents = get_white(boards);
    }

    for i in 0..64 {
        let piece: u64 = FIRST >> i;
        if piece & opponents != 0 && is_pseudo_legal(boards, !white_to_move, castle, enpassant, &piece, &king) {
            return true;
        }
    }

    false
}

fn possible_moves_white_pawn(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 7);
    moves.push(start << 8);
    moves.push(start << 9);
    moves.push(start << 16);

    moves
}

fn possible_moves_black_pawn(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start >> 7);
    moves.push(start >> 8);
    moves.push(start >> 9);
    moves.push(start >> 16);

    moves
}

fn possible_moves_knight(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

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

fn possible_moves_king(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 1);
    moves.push(start << 2);
    moves.push(start << 7);
    moves.push(start << 8);
    moves.push(start << 9);

    moves.push(start >> 1);
    moves.push(start >> 2);
    moves.push(start >> 7);
    moves.push(start >> 8);
    moves.push(start >> 9);

    moves
}

fn possible_moves_rook(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

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

    moves
}

fn possible_moves_bishop(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

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

#[pyfunction]
fn lan_to_fen(board: &str, turn: &str, castle: &str, enpassant: &str, halfmove: &str, fullmove: &str, lan: &str) -> String {
    let mut boards: [u64; 12] = fen_board_to_bits(board);
    let white_to_move = match turn {
        "w" => true,
        "b" => false,
        _ => true,
    };

    let start: u64 = pos_to_bit(&tile_to_pos(&lan[0..2]).unwrap());
    let end: u64 = pos_to_bit(&tile_to_pos(&lan[2..4]).unwrap());

    let white: u64 = get_white(&boards);
    let black: u64 = get_black(&boards);
    let full: u64 = white | black;

    let new_turn: &str = match white_to_move {
        true => "b",
        false => "w"
    };

    let mut new_castle: String = castle.to_string();

    if start & boards[5] != 0 {
        new_castle = new_castle.chars().filter(|&c| c != 'K' && c != 'Q').collect();
    } else if start & boards[11] != 0 {
        new_castle = new_castle.chars().filter(|&c| c != 'k' && c != 'q').collect();
    } else if start & boards[3] != 0 {
        if start & (WHITE_QUEENSIDE_BIT << 2) != 0 {
            new_castle = new_castle.chars().filter(|&c| c != 'Q').collect();
        } else if start & (WHITE_KINGSIDE_BIT >> 1) != 0 {
            new_castle = new_castle.chars().filter(|&c| c != 'K').collect();
        }
    } else if start & boards[9] != 0 {
        if start & (BLACK_QUEENSIDE_BIT << 2) != 0 {
            new_castle = new_castle.chars().filter(|&c| c != 'q').collect();
        } else if start & (BLACK_KINGSIDE_BIT >> 1) != 0 {
            new_castle = new_castle.chars().filter(|&c| c != 'k').collect();
        }
    }

    if new_castle.len() == 0 {
        new_castle = "-".to_string();
    }

    // we check if an en passant is possible, thus if a pawn has moved two squares forward
    let mut new_enpassant: String = "-".to_string();
    if white_to_move && end & get_rank(4) != 0 && start & boards[0] & get_rank(6) != 0 {
        new_enpassant = pos_to_tile(bit_to_pos(&(start << 8))).unwrap()
    } else if !white_to_move && end & get_rank(3) != 0 && start & boards[6] & get_rank(1) != 0 {
        new_enpassant = pos_to_tile(bit_to_pos(&(start >> 8))).unwrap()
    }

    // increase halfmove if no pawn is moved and no piece is captured, else set it to 0
    let new_halfmove: String;
    if end & full == 0 && start & boards[0] == 0 && start & boards[6] == 0 {
        let halfmove_num: i32 = halfmove.parse().unwrap();
        new_halfmove = (halfmove_num + 1).to_string();
    } else {
        new_halfmove = "0".to_string()
    }

    let mut new_fullmove: String = fullmove.to_string();
    if !white_to_move {
        let fullmove_num: i32 = fullmove.parse().unwrap();
        new_fullmove = (fullmove_num + 1).to_string();
    }

    let enpassant_bit: u64 = match enpassant {
        "-" => EMPTY,
        _ => pos_to_bit(&tile_to_pos(&enpassant).unwrap())
    };

    // removing piece in case of enpassant
    if white_to_move && (enpassant_bit & end != 0) && (start & boards[0] != 0) {
        boards[6] &= !(enpassant_bit >> 8)
    } else if !white_to_move && (enpassant_bit & end != 0) && (start & boards[6] != 0) {
        boards[0] &= !(enpassant_bit << 8)
    }

    // moving rook in case of castling
    if white_to_move && (start & boards[5] != 0) && (end & WHITE_KINGSIDE_BIT != 0) {
        boards[3] &= !(WHITE_KINGSIDE_BIT >> 1);
        boards[3] |= WHITE_KINGSIDE_BIT << 1;
    } else if white_to_move && (start & boards[5] != 0) && (end & WHITE_QUEENSIDE_BIT != 0) {
        boards[3] &= !(WHITE_QUEENSIDE_BIT << 2);
        boards[3] |= WHITE_QUEENSIDE_BIT >> 1;
    } else if white_to_move && (start & boards[11] != 0) && (end & BLACK_KINGSIDE_BIT != 0) {
        boards[9] &= !(BLACK_KINGSIDE_BIT >> 1);
        boards[9] |= BLACK_KINGSIDE_BIT << 1;
    } else if white_to_move && (start & boards[11] != 0) && (end & BLACK_QUEENSIDE_BIT != 0) {
        boards[9] &= !(BLACK_QUEENSIDE_BIT << 2);
        boards[9] |= BLACK_QUEENSIDE_BIT >> 1;
    }

    for i in 0..12 {
        if boards[i] & end != 0 {
            boards[i] &= !end;
        }
    }

    for i in 0..12 {
        if boards[i] & start != 0 {
            boards[i] &= !start;
            boards[i] |= end;
        }
    }

    if lan.len() == 5 {
        let promoting: &Option<char> = &lan.chars().nth(4);
        if white_to_move {
            boards[0] &= !end;
            match promoting {
                Some('q') | Some('Q') => boards[4] |= end,
                Some('r') | Some('R') => boards[3] |= end,
                Some('n') | Some('N') => boards[1] |= end,
                Some('b') | Some('B') => boards[2] |= end,
                Some(_) => (),
                None => panic!("Something went wrong with promoting")
            }
        } else {
            boards[6] &= !end;
            match promoting {
                Some('q') | Some('Q') => boards[10] |= end,
                Some('r') | Some('R') => boards[9] |= end,
                Some('n') | Some('N') => boards[7] |= end,
                Some('b') | Some('B') => boards[8] |= end,
                Some(_) => (),
                None => panic!("Something went wrong with promoting")
            }
        }
    }

    let new_board: String = bits_to_fen_board(&boards);

    new_board + " " + new_turn + " " + &new_castle + " " + &new_enpassant + " " + &new_halfmove + " " + &new_fullmove
}

#[pyfunction]
fn move_to_lan(start_pos: [usize; 2], end_pos: [usize; 2], promoting_to: &str) -> String {

    // we assume that the move is legal

    let mut result: String = "".to_string();

    if let Some(start) = pos_to_tile(start_pos) {
        result += &start
    }

    if let Some(end) = pos_to_tile(end_pos) {
        result += &end
    }

    match promoting_to {
        "Q" | "q" | "R" | "r" | "B" | "b" | "N" | "n" => result += promoting_to,
        _ => result += ""
    };
    
    result
}

#[pyfunction]
fn is_legal_move(board: &str, turn: &str, castle: &str, enpassant: &str, halfmove: &str, fullmove: &str, start_pos: [usize; 2], end_pos: [usize; 2], promoting_to: &str) -> bool {
    let boards: [u64; 12] = fen_board_to_bits(board);
    let white_to_move = match turn {
        "w" => true,
        "b" => false,
        _ => true,
    };

    let start: u64 = pos_to_bit(&start_pos);
    let end: u64 = pos_to_bit(&end_pos);

    if !is_pseudo_legal(&boards, white_to_move, castle, enpassant, &start, &end) {
        return false
    }

    let lan: String = move_to_lan(start_pos, end_pos, promoting_to);
    let new_fen: String = lan_to_fen(board, turn, castle, enpassant, halfmove, fullmove, &lan);
    let new_boards: [u64; 12] = fen_board_to_bits(&new_fen);
    let new_castle: String = fen_get_castle(&new_fen);
    let new_enpassant: String = fen_get_enpassant(&new_fen);

    if !check_check(&new_boards, white_to_move, &new_castle, &new_enpassant) {
        return true
    }

    false
}

#[pyfunction]
fn get_all_possible_moves(board: &str, turn: &str, castle: &str, enpassant: &str, halfmove: &str, fullmove: &str) -> Vec<[usize; 2]> {
    let mut moves: Vec<[usize; 2]> = Vec::new();

    let boards: [u64; 12] = fen_board_to_bits(board);
    let white_to_move = match turn {
        "w" => true,
        "b" => false,
        _ => true,
    };

    let pieces = match white_to_move {
        true => get_white(&boards),
        false => get_black(&boards)
    };

    for i in 0..64 {
        let piece: u64 = FIRST >> i;
        
        if piece & pieces != 0 {
            // check white pawns
            if boards[0] & piece != 0 {
                for end in possible_moves_white_pawn(&piece) {
                    if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                        moves.push(bit_to_pos(&end));
                    }
                }
            }

            // check black pawns
            if boards[6] & piece != 0 {
                for end in possible_moves_black_pawn(&piece) {
                    if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                        moves.push(bit_to_pos(&end));
                    }
                }
            }
            
            // check knights
            if boards[1] & piece != 0 || boards[7] & piece != 0 {
                for end in possible_moves_knight(&piece) {
                    if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                        moves.push(bit_to_pos(&end));
                    }
                }
            }

            // check kings
            if boards[5] & piece != 0 || boards[11] & piece != 0 {
                for end in possible_moves_king(&piece) {
                    if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                        moves.push(bit_to_pos(&end));
                    }
                }
            }

            let is_queen: bool = boards[4] & piece != 0 || boards[10] & piece != 0;

            // check rooks and queens
            if is_queen || boards[3] & piece != 0 || boards[9] & piece != 0 {
                for end in possible_moves_rook(&piece) {
                    if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                        moves.push(bit_to_pos(&end));
                    }
                }
            }

            // check bishops and queens
            if is_queen || boards[2] & piece != 0 || boards[8] & piece != 0 {
                for end in possible_moves_bishop(&piece) {
                    if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                        moves.push(bit_to_pos(&end));
                    }
                }
            }
        }
    }

    moves
}

#[pyfunction]
fn get_possible_moves(board: &str, turn: &str, castle: &str, enpassant: &str, halfmove: &str, fullmove: &str, start: [usize; 2]) -> Vec<[usize; 2]> {
    let mut moves: Vec<[usize; 2]> = Vec::new();

    let boards: [u64; 12] = fen_board_to_bits(board);
    let piece: u64 = pos_to_bit(&start);

    // check white pawns
    if boards[0] & piece != 0 {
        for end in possible_moves_white_pawn(&piece) {
            if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                moves.push(bit_to_pos(&end));
            }
        }
    }

    // check black pawns
    if boards[6] & piece != 0 {
        for end in possible_moves_black_pawn(&piece) {
            if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                moves.push(bit_to_pos(&end));
            }
        }
    }
    
    // check knights
    if boards[1] & piece != 0 || boards[7] & piece != 0 {
        for end in possible_moves_knight(&piece) {
            if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                moves.push(bit_to_pos(&end));
            }
        }
    }

    // check kings
    if boards[5] & piece != 0 || boards[11] & piece != 0 {
        for end in possible_moves_king(&piece) {
            if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                moves.push(bit_to_pos(&end));
            }
        }
    }

    let is_queen: bool = boards[4] & piece != 0 || boards[10] & piece != 0;

    // check rooks and queens
    if is_queen || boards[3] & piece != 0 || boards[9] & piece != 0 {
        for end in possible_moves_rook(&piece) {
            if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                moves.push(bit_to_pos(&end));
            }
        }
    }

    // check bishops and queens
    if is_queen || boards[2] & piece != 0 || boards[8] & piece != 0 {
        for end in possible_moves_bishop(&piece) {
            if is_legal_move(board, turn, castle, enpassant, halfmove, fullmove, bit_to_pos(&piece), bit_to_pos(&end), "-") {
                moves.push(bit_to_pos(&end));
            }
        }
    }

    moves
}

#[pyfunction]
fn game_ended(board: &str, turn: &str, castle: &str, enpassant: &str, halfmove: &str, fullmove: &str) -> String {
    let boards: [u64; 12] = fen_board_to_bits(board);
    let white_to_move: bool = match turn {
        "w" => true,
        "b" => false,
        _ => true,
    };

    let halfmove_num: i32 = halfmove.parse().unwrap();
    if halfmove_num > 100 {
        return "½-½".to_string();
    }

    if white_to_move {
        let white_moves: usize = get_all_possible_moves(board, turn, castle, enpassant, halfmove, fullmove).len();
        let white_checked: bool = check_check(&boards, true, castle, enpassant);
        if white_moves == 0 && white_checked {
            return "0-1".to_string();
        } else if white_moves == 0 {
            return "½-½".to_string();
        }
    } else {
        let black_moves: usize = get_all_possible_moves(board, turn, castle, enpassant, halfmove, fullmove).len();
        let black_checked: bool = check_check(&boards, false, castle, enpassant);
        if black_moves == 0 && black_checked {
            return "1-0".to_string();
        } else if black_moves == 0 {
            return "½-½".to_string();
        }
    }

    "not ended".to_string()
}

#[pymodule]
fn rust_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(is_legal_move, m)?)?;
    m.add_function(wrap_pyfunction!(move_to_lan, m)?)?;
    m.add_function(wrap_pyfunction!(lan_to_fen, m)?)?;
    m.add_function(wrap_pyfunction!(get_all_possible_moves, m)?)?;
    m.add_function(wrap_pyfunction!(get_possible_moves, m)?)?;
    m.add_function(wrap_pyfunction!(game_ended, m)?)?;
    Ok(())
}