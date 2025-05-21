use crate::utils_new::*;

/// Converts a tile in algebraic notation (e.g., "e4") to a bitboard representation.
pub fn tile_to_bit(tile: &str) -> u128 {
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

    FIRST >> (rank * 16 + file)
}

/// Converts a single-bit bitboard to its algebraic tile notation (e.g., 0b1 -> "h1").
pub fn bit_to_tile(bit: &u128) -> String {
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

/// Converts board string into array of pieces
pub fn board_string_to_pieces(board: &str) -> [u128; ARRAY_SIZE] {
    let mut pieces: [u128; ARRAY_SIZE] = [0; ARRAY_SIZE];

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
                let bit: u128 = FIRST >> (rank * 16 + file);
                match piece {
                    'P' => pieces[PAWNS] |= bit,
                    'p' => pieces[PAWNS] |= bit >> 8,
                    'K' => pieces[KINGS] |= bit,
                    'k' => pieces[KINGS] |= bit >> 8,
                    'Q' => pieces[QUEENS] |= bit,
                    'q' => pieces[QUEENS] |= bit >> 8,
                    'B' => pieces[BISHOPS] |= bit,
                    'b' => pieces[BISHOPS] |= bit >> 8,
                    'N' => pieces[KNIGHTS] |= bit,
                    'n' => pieces[KNIGHTS] |= bit >> 8,
                    'R' => pieces[ROOKS] |= bit,
                    'r' => pieces[ROOKS] |= bit >> 8,
                    _ => panic!("board_string_to_pieces: Found unknown string in board string")
                }
            }
        }
    }

    pieces
}

/// Converts string info into bit info
pub fn get_info(info: Vec<&str>) -> u128 {
    string_to_turn(info[1]) | string_to_castling(info[2]) | string_to_enpassant(info[3]) | string_to_halfmove(info[4]) | string_to_fullmove(info[5])
}

/// Converts enpassant string into bit represenation
pub fn string_to_enpassant(tile: &str) -> u128 {
    tile_to_bit(tile)
}

/// Parses the turn string from FEN ("w" or "b").
pub fn string_to_turn(turn: &str) -> u128 {
    match turn {
        "w" => TURN,
        "b" => EMPTY,
        _ => panic!("Found unknown string when attempting to parse turn string")
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
pub fn string_to_castling(castling: &str) -> u128 {
    let mut result: u128 = EMPTY;
    
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

/// Converts halfmove string to bit representation
pub fn string_to_halfmove(halfmove: &str) -> u128 {
    let halfmove_binary: u128 = halfmove.parse().unwrap();
    let first_part: u128 = halfmove_binary & 0xFF00;
    let second_part: u128 = halfmove_binary & 0xFF;
    
    (first_part << 5 * 16) & (second_part << 6 * 16)
}

/// Converts fullmove string to bit representation
pub fn string_to_fullmove(fullmove: &str) -> u128 {
    let fullmove_binary: u128 = fullmove.parse().unwrap();
    let first_part: u128 = fullmove_binary & 0xFF00;
    let second_part: u128 = fullmove_binary & 0xFF;
    
    (first_part << 3 * 16) & (second_part << 4 * 16)
}
 
/*

/// Parses the promotion information from a LAN move string (e.g., "e7e8q").
pub fn string_to_promotion(lan: &str) -> u64 {
    if lan.len() == 5 {
        match &lan[4..5] {
            "q" | "Q" => QUEEN_PROM,
            "r" | "R" => ROOK_PROM,
            "b" | "B" => BISHOP_PROM,
            "n" | "N" => KNIGHT_PROM,
            _ => panic!("Found unknown char when attempting to parse promotion info")
        }
    } else {
        NO_PROM
    }
}

/// Converts a move (start bit, end bit, promotion) to a long algebraic notation (LAN) string.
/// 
/// **NOTE:** this function does not check whether the move is legal
/// 
/// # Arguments
/// * `start` - Bitboard with starting position.
/// * `end` - Bitboard with ending position.
/// * `promoting_to` - Bitboard mask for promotion piece.
///
/// # Returns
/// * `String` - Move in LAN (e.g., "e7e8q").
pub fn move_to_lan(move1: &[u64; 3]) -> String {

    let mut result: String = "".to_string();

    result += &bit_to_tile(&move1[0]);
    result += &bit_to_tile(&move1[1]);

    let promoting_to: u64 = move1[2];

    if promoting_to & QUEEN_PROM != 0 {
        result += "q"
    } else if promoting_to & ROOK_PROM != 0 {
        result += "r"
    } else if promoting_to & BISHOP_PROM != 0 {
        result += "b"
    } else if promoting_to & KNIGHT_PROM != 0 {
        result += "n"
    }
    
    result
}

/// Converts a vector of moves in [start, end, promotion] bitboard format into a list of LAN strings.
///
/// # Arguments
/// * `moves` - Vector of moves, each represented by `start`, `end`, `promotion`.
///
/// # Returns
/// * `Vec<String>` - Vector of moves in LAN format.
pub fn moves_to_lan_list(moves: &Vec<[u64; 3]>) -> Vec<String> {
    moves.iter().map(|move1: &[u64; 3]| move_to_lan(move1)).collect()
}

*/

/// Converts array into a visual 8x8 board of piece strings.
pub fn board_to_visual(array: [u128; ARRAY_SIZE]) -> [[String; 8]; 8] {
    let mut board: [[String; 8]; 8] = std::array::from_fn(|_| {
        std::array::from_fn(|_| "-".to_string())
    });

    for rank in 0..8 {
        for file in 0..8 {
            let white_bit: u128 = FIRST >> rank * 16 + file;
            let black_bit: u128 = white_bit >> 8;

            if array[PAWNS] & white_bit != 0 {
                board[rank][file] = "P".to_string();
            } else if array[PAWNS] & black_bit != 0 {
                board[rank][file] = "p".to_string();
            } else if array[KINGS] & white_bit != 0 {
                board[rank][file] = "K".to_string();
            } else if array[KINGS] & black_bit != 0 {
                board[rank][file] = "k".to_string();
            } else if array[QUEENS] & white_bit != 0 {
                board[rank][file] = "Q".to_string();
            } else if array[QUEENS] & black_bit != 0 {
                board[rank][file] = "q".to_string();
            } else if array[BISHOPS] & white_bit != 0 {
                board[rank][file] = "B".to_string();
            } else if array[BISHOPS] & black_bit != 0 {
                board[rank][file] = "b".to_string();
            } else if array[KNIGHTS] & white_bit != 0 {
                board[rank][file] = "N".to_string();
            } else if array[KNIGHTS] & black_bit != 0 {
                board[rank][file] = "n".to_string();
            } else if array[ROOKS] & white_bit != 0 {
                board[rank][file] = "R".to_string();
            } else if array[ROOKS] & black_bit != 0 {
                board[rank][file] = "r".to_string();
            }
        }
    }

    board
}

/// Converts array into a FEN board string.
pub fn board_to_string(array: [u128; ARRAY_SIZE]) -> String {
    let mut result: String = String::new();
        for rank in 0..8 {
            let mut empty: i32 = 0;
            for file in 0..8 {
                let white_bit: u128 = FIRST >> rank * 16 + file;
                let black_bit: u128 = white_bit >> 8;

                if array[PAWNS] & white_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "P";
                } else if array[PAWNS] & black_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "p";
                } else if array[KINGS] & white_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "K";
                } else if array[KINGS] & black_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "k";
                } else if array[QUEENS] & white_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "Q";
                } else if array[QUEENS] & black_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "q";
                } else if array[BISHOPS] & white_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "B";
                } else if array[BISHOPS] & black_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "b";
                } else if array[KNIGHTS] & white_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "N";
                } else if array[KNIGHTS] & black_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "n";
                } else if array[ROOKS] & white_bit != 0 {
                    if empty > 0 {
                        result.push_str(&empty.to_string());
                        empty = 0;
                    }
                    result += "R";
                } else if array[ROOKS] & black_bit != 0 {
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

/// Converts a boolean turn value into FEN turn string.
pub fn turn_to_string(info: u128) -> String {
    match info & TURN != 0 {
        true => "b".to_string(),
        false => "w".to_string(),
    }
}

/// Converts a castling rights bitmask into a FEN-style castling string.
pub fn castling_to_string(info: u128) -> String {
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

/// Converts a bitboard en passant square into algebraic notation.
pub fn enpassant_to_string(info: u128) -> String {
    let enpassant: u128 = info & BOARD1;

    if enpassant == 0 {
        "-".to_string()
    } else {
        bit_to_tile(&enpassant)
    }
}
