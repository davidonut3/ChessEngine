/*

The new Fen struct will work with u128 instead of u64 to efficiently check whether a piece has moved off the board.

The struct will only contain one array of 8 u128:

Per piece type, we will store the positions of the white pieces on the left board and the positions of the black pieces on the right board.
We will also have a u128 which stores all white pieces on the left board and all the black pieces on the right board.
We will use the last u128 for the rest of the info:

64 bits for en passant info,
16 bits for the number of halfmoves,
16 bits for the number of fullmoves,
4 bits for castling info,
1 bit for turn info

*/

use crate::logic::*;
use crate::parsing_new;
use crate::parsing_new::move_to_lan;
use crate::utils_new::*;


#[derive(Debug, Clone)]
pub struct Fen {
    pub array: [u128; ARRAY_SIZE],
}

impl Fen {
    pub fn new() -> Self {
        Self {
            array: DEFAULT_FEN,
        }
    }

    pub fn from_str(fen_str: &str) -> Self {
        let fen_str_split: Vec<&str> = fen_str.trim().split_whitespace().collect();
        if fen_str_split.len() != 6 {
            panic!("Found incorrect fen notation");
        }

        let mut array: [u128; ARRAY_SIZE] = parsing_new::board_string_to_pieces(fen_str_split[0]);
        array[ALL_PIECES] = get_pieces(&array);
        array[INFO] = parsing_new::get_info(fen_str_split);

        Self {
            array,
        }
    }

    pub fn to_string(&self) -> String {
        parsing_new::fen_to_string(self.array)
    }

    pub fn to_visual(&self) -> [[String; 8]; 8] {
        parsing_new::board_to_visual(self.array)
    }

    pub fn lan_to_fen(&mut self, lan: &str) {
        let move1: [u128; 3] = parsing_new::lan_to_move(lan);
        self.move_to_fen(move1)
    }

    pub fn move_to_fen(&mut self, move1: [u128; 3]) {

        // This function does not check whether the move is legal

        let white_to_move: bool = self.white_to_move();

        let white_from: u128 = move1[0];
        let white_to: u128 = move1[1];

        let black_from: u128 = white_from >> 8;
        let black_to: u128 = white_to >> 8;

        let prom_to: u128 = move1[2];

        // A lot of the computation relies on which color is making the move.
        // Depending on the color, we move the start and end positions to the corresponding board.
        // Namely: left for white and right for black.
        // I am not sure if this is the best way to do this.
        if white_to_move {

            // In case of castling, we move the respective rook, since the king is the piece that is moved in the move
            let king_to_move: bool = white_from & self.array[KINGS] != 0;

            if king_to_move && (white_to & WHITE_KINGSIDE_MOVE_TO != 0) {

                // In case the king wants to move to the kingside castle square, we remove the rook to the right of the king,
                // and place it to the left of the king.
                self.array[ROOKS] &= !(WHITE_KINGSIDE_MOVE_TO >> 1);
                self.array[ROOKS] |= WHITE_KINGSIDE_MOVE_TO << 1;

            } else if king_to_move && (white_to & WHITE_QUEENSIDE_MOVE_TO != 0) {

                // In case the king wants to move to the queenside castle square, we remove the rook to the left of the king,
                // and place it to the right of the king.
                self.array[ROOKS] &= !(WHITE_QUEENSIDE_MOVE_TO << 2);
                self.array[ROOKS] |= WHITE_QUEENSIDE_MOVE_TO >> 1;

            }

            // In case of en passant, we remove the piece that is captured.
            if self.array[INFO] & white_to != 0 && self.array[PAWNS] & white_from != 0 {
                self.array[PAWNS] &= !(white_to >> 24);
            }

            // In case a pawn has moved to squares forward, we update the enpassant flag accordingly.
            self.array[INFO] &= !BOARD1;

            if (white_to & RANK_4 != 0) && (white_from & self.array[PAWNS] & RANK_6 != 0) {
                self.array[INFO] |= white_from << 16;
            }

        } else {

            // In case of castling, we move the respective rook, since the king is the piece that is moved in the move
            let king_to_move: bool = black_from & self.array[KINGS] != 0;

            if king_to_move && (black_to & BLACK_KINGSIDE_MOVE_TO != 0) {

                // In case the king wants to move to the kingside castle square, we remove the rook to the right of the king,
                // and place it to the left of the king.
                self.array[ROOKS] &= !(BLACK_KINGSIDE_MOVE_TO >> 1);
                self.array[ROOKS] |= BLACK_KINGSIDE_MOVE_TO << 1;

            } else if king_to_move && (black_to & BLACK_QUEENSIDE_MOVE_TO != 0) {

                // In case the king wants to move to the queenside castle square, we remove the rook to the left of the king,
                // and place it to the right of the king.
                self.array[ROOKS] &= !(BLACK_QUEENSIDE_MOVE_TO << 2);
                self.array[ROOKS] |= BLACK_QUEENSIDE_MOVE_TO >> 1;

            }

            // In case of en passant, we remove the piece that is captured.
            if (self.array[INFO] >> 8) & black_to != 0 && self.array[PAWNS] & black_from != 0 {
                self.array[PAWNS] &= !(black_to >> 8);
            }

            // In case a pawn has moved to squares forward, we update the enpassant flag accordingly.
            self.array[INFO] &= !BOARD1;

            if (black_to & RANK_3 != 0) && (black_from & self.array[PAWNS] & RANK_1 != 0) {
                self.array[INFO] |= black_from >> 16;
            }

        }

        // If no pawn is moved and no piece is captured, we increase the halfmove, else we set it to 0.
        let mut halfmove: u16 = parsing_new::compr_to_bin_halfmove(self.array[INFO]);
        let all_pieces: u128 = (self.array[ALL_PIECES] & BOARD1) | ((self.array[ALL_PIECES] & BOARD2) << 8);

        if (white_to & all_pieces == 0) && (white_from & self.array[PAWNS] == 0) && (black_from & self.array[PAWNS] == 0) {
            halfmove += 1;
        } else {
            halfmove = 0;
        }

        self.array[INFO] &= !(HALFMOVE1 | HALFMOVE2);
        self.array[INFO] |= parsing_new::bin_to_compr_halfmove(halfmove);

        // If black is to move, we increase the fullmove by 1.
        let mut fullmove: u16 = parsing_new::compr_to_bin_fullmove(self.array[INFO]);

        if !white_to_move {
            fullmove += 1;
        }

        self.array[INFO] &= !(FULLMOVE1 | FULLMOVE2);
        self.array[INFO] |= parsing_new::bin_to_compr_fullmove(fullmove);

        // If the move is a capture, we remove the taken piece from the board.
        for i in 0..6 {
            if self.array[i] & white_to != 0 {
                self.array[i] &= !white_to;
                break;
            } else if self.array[i] & black_to != 0 {
                self.array[i] &= !black_to;
                break;
            }
        }

        // We apply the move to the board.
        for i in 0..6 {
            if self.array[i] & white_from != 0 {
                self.array[i] &= !white_from;
                self.array[i] |= white_to;
                break;
            } else if self.array[i] & black_from != 0 {
                self.array[i] &= !black_from;
                self.array[i] |= black_to;
                break;
            }
        }

        // In case of promotion, we change the pieces according to the promotion info
        let promoting: bool = prom_to != NO_PROMOTION;

        if promoting && white_to_move {
            self.array[PAWNS] &= !white_to;
            if prom_to & QUEEN_PROMOTION != 0 {
                self.array[QUEENS] |= white_to;
            } else if prom_to & ROOK_PROMOTION != 0 {
                self.array[ROOKS] |= white_to;
            } else if prom_to & BISHOP_PROMOTION != 0 {
                self.array[BISHOPS] |= white_to;
            } else if prom_to & KNIGHT_PROMOTION != 0 {
                self.array[KNIGHTS] |= white_to;
            } else {
                panic!("move_to_fen: Found unknown flag for promotion")
            }
        } else if promoting {
            self.array[PAWNS] &= !black_to;
            if prom_to & QUEEN_PROMOTION != 0 {
                self.array[QUEENS] |= black_to;
            } else if prom_to & ROOK_PROMOTION != 0 {
                self.array[ROOKS] |= black_to;
            } else if prom_to & BISHOP_PROMOTION != 0 {
                self.array[BISHOPS] |= black_to;
            } else if prom_to & KNIGHT_PROMOTION != 0 {
                self.array[KNIGHTS] |= black_to;
            } else {
                panic!("move_to_fen: Found unknown flag for promotion")
            }
        }

        // We update the castling rights based on whether the rooks have moved or have been captured and whether the king has moved.
        if self.array[KINGS] & WHITE_KING_POS == 0 {
            self.array[INFO] &= !(WHITE_KINGSIDE_RIGHTS | WHITE_QUEENSIDE_RIGHTS);
        }

        if self.array[KINGS] & BLACK_KING_POS == 0 {
            self.array[INFO] &= !(BLACK_KINGSIDE_RIGHTS | BLACK_QUEENSIDE_RIGHTS);
        }

        if self.array[ROOKS] & (WHITE_KING_POS >> 3) == 0 {
            self.array[INFO] &= !WHITE_KINGSIDE_RIGHTS;
        }

        if self.array[ROOKS] & (WHITE_KING_POS << 4) == 0 {
            self.array[INFO] &= !WHITE_QUEENSIDE_RIGHTS;
        }

        if self.array[ROOKS] & (BLACK_KING_POS >> 3) == 0 {
            self.array[INFO] &= !BLACK_KINGSIDE_RIGHTS;
        }

        if self.array[ROOKS] & (BLACK_KING_POS << 4) == 0 {
            self.array[INFO] &= !BLACK_QUEENSIDE_RIGHTS;
        }

        // We switch turn info.
        if white_to_move {
            self.array[INFO] &= !TURN;
        } else {
            self.array[INFO] |= TURN;
        }

        // We update the positions of the pieces in ALL_PIECES
        self.array[ALL_PIECES] = get_pieces(&self.array);

    }

    pub fn player_in_check(&self, player_is_white: bool) -> bool {
        let attacks:(u128, u128) = get_attacks(self.array);

        if player_is_white {

            // The white king is in check if it is attacked by any black piece
            let king: u128 = self.array[KINGS] & BOARD1;
            let black_attacks: u128 = attacks.1;
            return king & black_attacks != 0

        } else {

            // The black king is in check if it is attacked by any white piece
            let king: u128 = (self.array[KINGS] & BOARD2) << 8;
            let white_attacks: u128 = attacks.0;
            return king & white_attacks != 0

        }
    }

    pub fn white_to_move(&self) -> bool {
        self.array[INFO] & TURN != 0
    }

    pub fn get_legal_moves_lan(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let legal_moves: [[u128; 3]; MAX_MOVES] = self.get_legal_moves_array();

        for i in 0..MAX_MOVES {
            if legal_moves[i][0] == 0 {
                
                // If the from pos is 0, the array is empty from this point, so we break.
                break;

            } else {
                result.push(move_to_lan(&legal_moves[i]));
            }
        }

        result
    }

    pub fn get_legal_moves_vec(&self) -> Vec<[u128; 3]> {
        let mut result: Vec<[u128; 3]> = Vec::new();
        let legal_moves: [[u128; 3]; MAX_MOVES] = self.get_legal_moves_array();

        for i in 0..MAX_MOVES {
            if legal_moves[i][0] == 0 {
                
                // If the from pos is 0, the array is empty from this point, so we break.
                break;

            } else {
                result.push(legal_moves[i]);
            }
        }

        result
    }

    pub fn get_legal_moves_array(&self) -> [[u128; 3]; MAX_MOVES] {
        let mut result: [[u128; 3]; MAX_MOVES] = [[0; 3]; MAX_MOVES];
        let mut index: usize = 0;

        result
    }
}