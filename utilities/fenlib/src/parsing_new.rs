use crate::utils_new::*;

/// Converts a tile in algebraic notation (e.g., "e4") to a bitboard representation.
///
/// `"-"` is interpreted as an empty square and returns the EMPTY bitboard.
///
/// # Arguments
/// * `tile` - A string slice representing the tile.
///
/// # Returns
/// * `u128` - Bitboard representation of the tile.
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
///
/// # Arguments
/// * `bit` - A reference to a bitboard with only one bit set.
///
/// # Returns
/// * `String` - Tile in algebraic notation.
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

/// Converts a bitboard index back to its corresponding piece character.
///
/// # Arguments
/// * `index` - Index of the piece.
///
/// # Returns
/// * `String` - Character representation of the piece.
/// 
/// Piece mappings:
/// * 0-7 = pawn, 8 = king, 9 = queen, 10-11 = bishop, 12-13 = knight, 14-15 = rook
pub fn index_to_piece(index: usize) -> String {
    let result = match index {
        0 => "P",
        1 => "N",
        2 => "B",
        3 => "R",
        4 => "Q",
        5 => "K",
        6 => "p",
        7 => "n",
        8 => "b",
        9 => "r",
        10 => "q",
        11 => "k",
        _ => panic!("Found unknown index while attempting to parse piece index"),
    };

    result.to_string()
}

/// Parses a FEN-style board string into an array of bitboards (one for each piece type).
///
/// # Arguments
/// * `board` - FEN board representation string.
///
/// # Returns
/// * `[u64; 12]` - Array of bitboards for all pieces.
pub fn string_to_board(board: &str) -> [u64; 12] {
    let mut boards: [u64; 12] = [0; 12];
    let rows: Vec<&str> = board.split('/').collect();

    for (rank, row) in rows.iter().enumerate() {
        let mut file: usize = 0;
        for ch in row.chars() {
            if ch.is_digit(10) {
                file += ch.to_digit(10).unwrap() as usize;
            } else {
                let index: usize = piece_to_index(ch);
                boards[index] |= FIRST >> (rank * 8 + file);
                file += 1;
            }
        }
    }
    boards
}

/// Parses the turn string from FEN ("w" or "b").
///
/// # Arguments
/// * `turn` - "w" if white to move, "b" if black.
///
/// # Returns
/// * `bool` - `true` if white to move, `false` if black.
pub fn string_to_turn(turn: &str) -> bool {
    let result: bool;
    match turn {
        "w" => result = true,
        "b" => result = false,
        _ => panic!("Found unknown string when attempting to parse turn string")
    }

    result
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
///
/// # Arguments
/// * `castling` - Castling string.
///
/// # Returns
/// * `u8` - Bitmask representing castling rights.
pub fn string_to_castling(castling: &str) -> u8 {
    let mut result: u8 = 0x0;
    
    if castling.contains("K") {
        result |= WHITE_KINGSIDE_INFO;
    }

    if castling.contains("Q") {
        result |= WHITE_QUEENSIDE_INFO;
    }

    if castling.contains("k") {
        result |= BLACK_KINGSIDE_INFO;
    }

    if castling.contains("q") {
        result |= BLACK_QUEENSIDE_INFO;
    }

    result
}

/// Parses the en passant tile from FEN format.
///
/// # Arguments
/// * `enpassant` - Tile in algebraic notation or "-" for none.
///
/// # Returns
/// * `u64` - Bitboard representation of en passant square.
pub fn string_to_enpassant(enpassant: &str) -> u64 {
    tile_to_bit(enpassant)
}

/// Parses the promotion information from a LAN move string (e.g., "e7e8q").
///
/// # Arguments
/// * `lan` - LAN string with optional promotion character at the end.
///
/// # Returns
/// * `u64` - Bitboard constant representing promotion piece.
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

/// Converts an array of 12 bitboards into a visual 8x8 board of piece strings.
///
/// # Arguments
/// * `boards` - Array of bitboards representing the board state.
///
/// # Returns
/// * `[[String; 8]; 8]` - 2D array visual representation of the board.
pub fn board_to_visual(boards: &[u64; 12]) -> [[String; 8]; 8] {
    let mut board: [[String; 8]; 8] = std::array::from_fn(|_| {
        std::array::from_fn(|_| "-".to_string())
    });

    for rank in 0..8 {
        for file in 0..8 {
            let bit: u64 = (FIRST >> (rank * 8)) >> file;
            for i in 0..12 {
                if boards[i] & bit != 0 {
                    board[rank][file] = index_to_piece(i);
                }
            }
        }
    }

    board
}

/// Converts an array of 12 bitboards into a FEN-style board string.
///
/// # Arguments
/// * `boards` - Array of bitboards representing the board state.
///
/// # Returns
/// * `String` - FEN-format string of piece placement.
pub fn board_to_string(boards: &[u64; 12]) -> String {
    let mut result: String = String::new();
        for rank in 0..8 {
            let mut empty: i32 = 0;
            for file in 0..8 {
                let sq_index: i32 = rank * 8 + file;
                let pos: u64 = FIRST >> sq_index;
                let mut piece_found: bool = false;
                for (i, &board) in boards.iter().enumerate() {
                    if board & pos != 0 {
                        if empty > 0 {
                            result.push_str(&empty.to_string());
                            empty = 0;
                        }
                        let symbol: String = index_to_piece(i);
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

/// Converts a boolean turn value into FEN turn string.
///
/// # Arguments
/// * `white_to_move` - `true` if white to move, `false` if black to move.
///
/// # Returns
/// * `String` - "w" or "b"
pub fn turn_to_string(white_to_move: bool) -> String {
    match white_to_move {
        true => "w".to_string(),
        false => "b".to_string(),
    }
}

/// Converts a castling rights bitmask into a FEN-style castling string.
///
/// # Arguments
/// * `castling` - Bitmask representing castling rights.
///
/// # Returns
/// * `String` - FEN-style castling string.
pub fn castling_to_string(castling: &u8) -> String {
    let mut result: String = "".to_string();

    if castling & WHITE_KINGSIDE_INFO != 0 {
        result += "K"
    }

    if castling & WHITE_QUEENSIDE_INFO != 0 {
        result += "Q"
    }

    if castling & BLACK_KINGSIDE_INFO != 0 {
        result += "k"
    }

    if castling & BLACK_QUEENSIDE_INFO != 0 {
        result += "q"
    }

    if result.len() == 0 {
        result = "-".to_string()
    }

    result
}

/// Converts a bitboard en passant square into algebraic notation.
///
/// # Arguments
/// * `enpassant` - Bitboard with en passant square.
///
/// # Returns
/// * `String` - Tile in algebraic notation.
pub fn enpassant_to_string(enpassant: &u64) -> String {
    if *enpassant == EMPTY {
        "-".to_string()
    } else {
        bit_to_tile(enpassant)
    }
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
