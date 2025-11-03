use crate::utils::*;

const COMPR_QUEEN_PROM: u16 = 0b0001000000000000;
const COMPR_ROOK_PROM: u16 = 0b0010000000000000;
const COMPR_BISHOP_PROM: u16 = 0b0100000000000000;
const COMPR_KNIGHT_PROM: u16 = 0b1000000000000000;

/// Converts a tile in algebraic notation (e.g., "e4") to a bitboard representation.
pub fn tile_to_bit(tile: &str) -> u64 {
    if tile == "-" {
        return EMPTY;
    } 
    
    if tile.len() != 2 {
        panic!("Found string of wrong length when attempting to parse tile");
    }

    let file_char: char = tile.chars().nth(0).unwrap();
    let rank_char: char = tile.chars().nth(1).unwrap();

    let rank: usize = match rank_char {
        '8' => 0,
        '7' => 1,
        '6' => 2,
        '5' => 3,
        '4' => 4,
        '3' => 5,
        '2' => 6,
        '1' => 7,
        _ => panic!("Found unknown char when attempting to parse tile rank"),
    };

    let file: usize = match file_char {
        'a' => 0,
        'b' => 1,
        'c' => 2,
        'd' => 3,
        'e' => 4,
        'f' => 5,
        'g' => 6,
        'h' => 7,
        _ => panic!("Found unknown char when attempting to parse tile file"),
    };

    FIRST >> (rank * 8 + file)
}

/// Converts a single-bit bitboard to its algebraic tile notation (e.g., 0b1 -> "h1").
pub fn bit_to_tile(bit: u64) -> String {
    let ones: u32 = bit.count_ones();
    if ones > 1 || ones == 0 {
        panic!("Found wrong format when attempting to parse bit")
    }

    let mut rank: usize = 0;
    let mut file: usize = 0;
    for i in 0..8 as usize{
        if bit & RANKS[i] != 0 {
            rank = i;
        }

        if bit & FILES[i] != 0 {
            file = i;
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
        _ => panic!("Found unknown rank index when attempting to parse bit"),
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
        _ => panic!("Found unknown file index when attempting to parse bit"),
    };

    file.to_string() + rank
}

/// Converts single-bit bitboard to binary search bitboard
pub fn bit_to_compr(bit: u64) -> u8 {
    let ones: u32 = bit.count_ones();

    if ones == 0 { return 0b01000000 }

    if ones == 1 { return bit.leading_zeros() as u8 }

    panic!("bit_to_compr: Bitboard has to many 1's");
}

/// Converts binary search bitboard to single-bit bitboard
pub fn compr_to_bit(bit: u8) -> u64 {
    if bit & 0b01000000 != 0 { return EMPTY }

    FIRST >> (bit & 0b00111111)
}

/// Converts board string into array of pieces
pub fn board_string_to_pieces(board: &str) -> Array {
    let mut pieces: Array = [0; ARRAY_SIZE];

    // We loop over the 8 rows of the board string
    let rows: Vec<&str> = board.split('/').collect();
    for (rank, char_pieces) in rows.iter().enumerate() {
        let mut file: usize = 0;
        
        // For each piece in each row, we check its value
        for piece in char_pieces.chars() {
            if piece.is_digit(10) {

                // If the piece is a number, we skip that number of pieces
                file += piece.to_digit(10).unwrap() as usize;
            } else {
                let bit: u64 = FIRST >> (rank * 8 + file);
                match piece {
                    'P' => pieces[PAWN_W] |= bit,
                    'p' => pieces[PAWN_B] |= bit,
                    'K' => pieces[KING_W] |= bit,
                    'k' => pieces[KING_B] |= bit,
                    'Q' => pieces[QUEEN_W] |= bit,
                    'q' => pieces[QUEEN_B] |= bit,
                    'B' => pieces[BISHOP_W] |= bit,
                    'b' => pieces[BISHOP_B] |= bit,
                    'N' => pieces[KNIGHT_W] |= bit,
                    'n' => pieces[KNIGHT_B] |= bit,
                    'R' => pieces[ROOK_W] |= bit,
                    'r' => pieces[ROOK_B] |= bit,
                    _ => panic!("board_string_to_pieces: Found unknown string in board string")
                }
                file += 1;
            }
        }
    }

    pieces
}

/// Converts array into a FEN board string.
pub fn board_to_string(array: Array) -> String {
    let mut result: String = String::new();
        for rank in 0..8 {
            let mut empty: i32 = 0;
            for file in 0..8 {
                let bit: u64 = FIRST >> rank * 8 + file;

                if array[PAWN_W] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "P";
                } else if array[PAWN_B] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "p";
                } else if array[KING_W] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "K";
                } else if array[KING_B] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "k";
                } else if array[QUEEN_W] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "Q";
                } else if array[QUEEN_B] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "q";
                } else if array[BISHOP_W] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "B";
                } else if array[BISHOP_B] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "b";
                } else if array[KNIGHT_W] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "N";
                } else if array[KNIGHT_B] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "n";
                } else if array[ROOK_W] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "R";
                } else if array[ROOK_B] & bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "r";
                } else {
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

/// Converts string info into bit info
pub fn get_info(info: Vec<&str>) -> u64 {
    string_to_turn(info[1]) | string_to_castling(info[2]) | string_to_compr_enpassant(info[3]) | string_to_compr_halfmove(info[4]) | string_to_compr_fullmove(info[5])
}

/// Converts enpassant string to compressed bit representation
pub fn string_to_compr_enpassant(enpassant: &str) -> u64 {
    (bit_to_compr(tile_to_bit(enpassant)) as u64) << 24
}

/// Converts enpassant compressed bit representation to string
pub fn compr_to_string_enpassant(info: u64) -> String {
    let enpassant: u8 = ((info & ENPASSANT) >> 24) as u8;

    if (enpassant & 0b01000000) == 0 {
        return "-".to_string()
    } else {
        return bit_to_tile(compr_to_bit(enpassant))
    }
}

/// Converts enpassant binary represenation to compressed bit representation
pub fn bin_to_compr_enpassant(enpassant: u64) -> u64 {
    (bit_to_compr(enpassant) as u64) << 24
}

/// Converts enpassant compressed bit representation to binary representation
pub fn compr_to_bin_enpassant(info: u64) -> u64 {
    let enpassant: u8 = ((info & ENPASSANT) >> 24) as u8;

    compr_to_bit(enpassant)
}

/// Parses the turn string from FEN ("w" or "b").
pub fn string_to_turn(turn: &str) -> u64 {
    match turn {
        "w" => TURN,
        "b" => EMPTY,
        _ => panic!("Found unknown string when attempting to parse turn string")
    }
}

/// Converts a boolean turn value into FEN turn string.
pub fn turn_to_string(info: u64) -> String {
    match info & TURN != 0 {
        true => "w".to_string(),
        false => "b".to_string(),
    }
}

/// Parses the castling rights string from FEN format.
///
/// Possible values:
/// * "-" means no castling rights.
/// * "KQkq" format where:
///   - K = White kingside
///   - Q = White queenside
///   - k = Black kingside
///   - q = Black queenside
pub fn string_to_castling(castling: &str) -> u64 {
    let mut result: u64 = EMPTY;
    
    if castling.contains("K") {
        result |= WHITE_KINGSIDE_RIGHTS;
    }

    if castling.contains("Q") {
        result |= WHITE_QUEENSIDE_RIGHTS;
    }

    if castling.contains("k") {
        result |= BLACK_KINGSIDE_RIGHTS;
    }

    if castling.contains("q") {
        result |= BLACK_QUEENSIDE_RIGHTS;
    }

    result
}

/// Converts a castling rights bitmask into a FEN-style castling string.
pub fn castling_to_string(info: u64) -> String {
    let mut result: String = "".to_string();

    if info & WHITE_KINGSIDE_RIGHTS != 0 {
        result += "K"
    }

    if info & WHITE_QUEENSIDE_RIGHTS != 0 {
        result += "Q"
    }

    if info & BLACK_KINGSIDE_RIGHTS != 0 {
        result += "k"
    }

    if info & BLACK_QUEENSIDE_RIGHTS != 0 {
        result += "q"
    }

    if result.len() == 0 {
        result = "-".to_string()
    }

    result
}

/// Converts halfmove string to compressed bit representation
pub fn string_to_compr_halfmove(halfmove: &str) -> u64 {
    let bit: u64 = halfmove.parse().unwrap();
    bit << 48
}

/// Converts halfmove compressed bit representation to string
pub fn compr_to_string_halfmove(info: u64) -> String {
    let halfmove: u64 = (info & HALFMOVE) >> 48;
    halfmove.to_string()
}

/// Converts halfmove binary representation to compressed bit representation
pub fn bin_to_compr_halfmove(halfmove: u64) -> u64 {
    halfmove << 48
}

/// Converts halfmove compressed bit representation to binary representation
pub fn compr_to_bin_halfmove(info: u64) -> u64 {
    (info & HALFMOVE) >> 48
}

/// Converts fullmove string to compressed bit representation
pub fn string_to_compr_fullmove(fullmove: &str) -> u64 {
    let bit: u64 = fullmove.parse().unwrap();
    bit << 32
}

/// Converts fullmove compressed bit representation to string
pub fn compr_to_string_fullmove(info: u64) -> String {
    let fullmove: u64 = (info & FULLMOVE) >> 32;
    fullmove.to_string()
}

/// Converts fullmove binary representation to compressed bit representation
pub fn bin_to_compr_fullmove(fullmove: u64) -> u64 {
    fullmove << 32
}

/// Converts fullmove compressed bit representation to binary representation
pub fn compr_to_bin_fullmove(info: u64) -> u64 {
    (info & FULLMOVE) >> 32
}

/// Converts array into FEN string.
pub fn fen_to_string(array: Array) -> String {
    format!(
        "{} {} {} {} {} {}",
        board_to_string(array),
        turn_to_string(array[INFO]),
        castling_to_string(array[INFO]),
        compr_to_string_enpassant(array[INFO]),
        compr_to_string_halfmove(array[INFO]),
        compr_to_string_fullmove(array[INFO]),
    )
}

/// Converts array into a visual 8x8 board of piece strings.
pub fn board_to_visual(array: Array) -> [[String; 8]; 8] {
    let mut board: [[String; 8]; 8] = std::array::from_fn(|_| {
        std::array::from_fn(|_| "-".to_string())
    });

    for rank in 0..8 {
        for file in 0..8 {
            let bit: u64 = FIRST >> rank * 8 + file;

            if array[PAWN_W] & bit != 0 {
                board[rank][file] = "P".to_string();
            } else if array[PAWN_B] & bit != 0 {
                board[rank][file] = "p".to_string();
            } else if array[KING_W] & bit != 0 {
                board[rank][file] = "K".to_string();
            } else if array[KING_B] & bit != 0 {
                board[rank][file] = "k".to_string();
            } else if array[QUEEN_W] & bit != 0 {
                board[rank][file] = "Q".to_string();
            } else if array[QUEEN_B] & bit != 0 {
                board[rank][file] = "q".to_string();
            } else if array[BISHOP_W] & bit != 0 {
                board[rank][file] = "B".to_string();
            } else if array[BISHOP_B] & bit != 0 {
                board[rank][file] = "b".to_string();
            } else if array[KNIGHT_W] & bit != 0 {
                board[rank][file] = "N".to_string();
            } else if array[KNIGHT_B] & bit != 0 {
                board[rank][file] = "n".to_string();
            } else if array[ROOK_W] & bit != 0 {
                board[rank][file] = "R".to_string();
            } else if array[ROOK_B] & bit != 0 {
                board[rank][file] = "r".to_string();
            }
        }
    }

    board
}

/// Parses the promotion information from a LAN move string (e.g., "e7e8q").
pub fn string_to_promotion(lan: &str) -> u64 {
    if lan.len() == 5 {
        match &lan[4..5] {
            "q" | "Q" => QUEEN_PROMOTION,
            "r" | "R" => ROOK_PROMOTION,
            "b" | "B" => BISHOP_PROMOTION,
            "n" | "N" => KNIGHT_PROMOTION,
            _ => panic!("Found unknown char when attempting to parse promotion info")
        }
    } else {
        NO_PROMOTION
    }
}

/// Converts binary move to LAN move string
pub fn move_to_lan(move1: &Move) -> String {
    let mut result: String = "".to_string();

    result += &bit_to_tile(move1[0]);
    result += &bit_to_tile(move1[1]);

    let promoting_to: u64 = move1[2];

    if promoting_to & QUEEN_PROMOTION != 0 {
        result += "q"
    } else if promoting_to & ROOK_PROMOTION != 0 {
        result += "r"
    } else if promoting_to & BISHOP_PROMOTION != 0 {
        result += "b"
    } else if promoting_to & KNIGHT_PROMOTION != 0 {
        result += "n"
    }
    
    result
}

/// Converts LAN move string to binary move
pub fn lan_to_move(lan: &str) -> Move {
    let start: u64 = tile_to_bit(&lan[0..2]);
    let end: u64 = tile_to_bit(&lan[2..4]);
    let promoting_to: u64 = string_to_promotion(lan);

    [start, end, promoting_to]
}

/// Converts a vector of moves in [start, end, promotion] bitboard format into a list of LAN strings.
pub fn moves_to_lan_list(moves: &Vec<Move>) -> Vec<String> {
    moves.iter().map(|move1: &Move| move_to_lan(move1)).collect()
}

pub fn move_to_compact(move1: &Move) -> CompactMove {
    // We don't use bit_to_compr since we can assume the bitboards have exactly one 1

    let from: u16 = move1[0].leading_zeros() as u16;
    let to: u16 = (move1[1].leading_zeros() as u16) << 6;
    let promotion: u16 = match move1[2] {
        QUEEN_PROMOTION => COMPR_QUEEN_PROM,
        ROOK_PROMOTION => COMPR_ROOK_PROM,
        BISHOP_PROMOTION => COMPR_BISHOP_PROM,
        KNIGHT_PROMOTION => COMPR_KNIGHT_PROM,
        _ => 0
    };

    from | to | promotion
}

pub fn compact_to_move(move1: &CompactMove) -> Move {
    let from = move1 & 0b0000000000111111;
    let to = (move1 & 0b0000111111000000) >> 6;
    let flag = move1 & 0b1111000000000000;
    let promotion = match flag {
        COMPR_QUEEN_PROM => QUEEN_PROMOTION,
        COMPR_ROOK_PROM => ROOK_PROMOTION,
        COMPR_BISHOP_PROM => BISHOP_PROMOTION,
        COMPR_KNIGHT_PROM => KNIGHT_PROMOTION,
        _ => 0
    };

    [FIRST >> from, FIRST >> to, promotion]
}
