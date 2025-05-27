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

            if king_to_move && (white_to & WHITE_KINGSIDE_MOVE_TO != 0) && (white_from & WHITE_KING_POS != 0) && (WHITE_KINGSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case the king wants to move to the kingside castle square, we remove the rook to the right of the king,
                // and place it to the left of the king.
                self.array[ROOKS] &= !(WHITE_KINGSIDE_MOVE_TO >> 1);
                self.array[ROOKS] |= WHITE_KINGSIDE_MOVE_TO << 1;

            } else if king_to_move && (white_to & WHITE_QUEENSIDE_MOVE_TO != 0) && (white_from & WHITE_KING_POS != 0) && (WHITE_QUEENSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case the king wants to move to the queenside castle square, we remove the rook to the left of the king,
                // and place it to the right of the king.
                self.array[ROOKS] &= !(WHITE_QUEENSIDE_MOVE_TO << 2);
                self.array[ROOKS] |= WHITE_QUEENSIDE_MOVE_TO >> 1;

            }

            // In case of en passant, we remove the piece that is captured.
            if self.array[INFO] & white_to != 0 && self.array[PAWNS] & white_from != 0 {
                self.array[PAWNS] &= !(white_to >> 24);
            }

            // In case a pawn has moved two squares forward, we update the enpassant flag accordingly.
            self.array[INFO] &= !BOARD1;

            if (white_to & RANK_4 != 0) && (white_from & self.array[PAWNS] & RANK_6 != 0) {
                self.array[INFO] |= white_from << 16;
            }

        } else {

            // In case of castling, we move the respective rook, since the king is the piece that is moved in the move
            let king_to_move: bool = black_from & self.array[KINGS] != 0;

            if king_to_move && (black_to & (BLACK_KINGSIDE_MOVE_TO >> 8) != 0) && (white_from & BLACK_KING_POS != 0) && (BLACK_KINGSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case the king wants to move to the kingside castle square, we remove the rook to the right of the king,
                // and place it to the left of the king.
                self.array[ROOKS] &= !(BLACK_KINGSIDE_MOVE_TO >> 9);
                self.array[ROOKS] |= BLACK_KINGSIDE_MOVE_TO >> 7;

            } else if king_to_move && (black_to & (BLACK_QUEENSIDE_MOVE_TO >> 8) != 0) && (white_from & BLACK_KING_POS != 0) && (BLACK_QUEENSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case the king wants to move to the queenside castle square, we remove the rook to the left of the king,
                // and place it to the right of the king.
                self.array[ROOKS] &= !(BLACK_QUEENSIDE_MOVE_TO >> 6);
                self.array[ROOKS] |= BLACK_QUEENSIDE_MOVE_TO >> 9;

            }

            // In case of en passant, we remove the piece that is captured.
            if (self.array[INFO] >> 8) & black_to != 0 && self.array[PAWNS] & black_from != 0 {
                self.array[PAWNS] &= !(black_to << 24);
            }

            // In case a pawn has moved two squares forward, we update the enpassant flag accordingly.
            self.array[INFO] &= !BOARD1;

            if (black_to & RANK_3 != 0) && (black_from & self.array[PAWNS] & RANK_1 != 0) {
                self.array[INFO] |= black_from >> 8;
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

        if (self.array[KINGS] << 8) & BLACK_KING_POS == 0 {
            self.array[INFO] &= !(BLACK_KINGSIDE_RIGHTS | BLACK_QUEENSIDE_RIGHTS);
        }

        if self.array[ROOKS] & (WHITE_KING_POS >> 3) == 0 {
            self.array[INFO] &= !WHITE_KINGSIDE_RIGHTS;
        }

        if self.array[ROOKS] & (WHITE_KING_POS << 4) == 0 {
            self.array[INFO] &= !WHITE_QUEENSIDE_RIGHTS;
        }

        if (self.array[ROOKS] << 8) & (BLACK_KING_POS >> 3) == 0 {
            self.array[INFO] &= !BLACK_KINGSIDE_RIGHTS;
        }

        if (self.array[ROOKS] << 8) & (BLACK_KING_POS << 4) == 0 {
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
        let (white_attacks, black_attacks): (u128, u128) = get_attacks(&self.array);

        if player_is_white {

            // The white king is in check if it is attacked by any black piece
            let king: u128 = self.array[KINGS] & BOARD1;
            return king & black_attacks != 0

        } else {

            // The black king is in check if it is attacked by any white piece
            let king: u128 = (self.array[KINGS] & BOARD2) << 8;
            return king & white_attacks != 0

        }
    }

    pub fn game_ended(&self) -> &str {
        let move_count: usize = self.get_legal_moves_array().1;
        let white_to_move: bool = self.white_to_move();
        let in_check: bool = self.player_in_check(white_to_move);
        let halfmove: u16 = parsing_new::compr_to_bin_halfmove(self.array[INFO]);
    
        if move_count == 0 && in_check {
            if white_to_move {
                BLACK_WINS
            } else {
                WHITE_WINS
            }
        } else if move_count == 0 || halfmove > 99 {
            DRAW
        } else {
            NOT_ENDED
        }
    }

    pub fn white_to_move(&self) -> bool {
        self.array[INFO] & TURN != 0
    }

    pub fn is_valid_board(&self) {
        let white_king_count: u32 = (self.array[KINGS] & BOARD1).count_ones();
        let black_king_count: u32 = (self.array[KINGS] & BOARD2).count_ones();

        if white_king_count != 1 || black_king_count != 1 {
            panic!("is_valid_board: This board has too many or too few kings")
        }

        let white_piece_count: u32 = (self.array[ALL_PIECES] & BOARD1).count_ones();
        let black_piece_count: u32 = (self.array[ALL_PIECES] & BOARD2).count_ones();

        if white_piece_count > 20 || black_piece_count > 20 {
            panic!("is_valid_board: This board has too many pieces")
        }
    }

    pub fn get_legal_moves_lan(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let legal_moves: [[u128; 3]; MAX_MOVES] = self.get_legal_moves_array().0;

        for i in 0..MAX_MOVES {
            if legal_moves[i][0] == 0 {
                
                // If the from pos is 0, the array is empty from this point, so we break.
                break;

            } else {
                let lan: String = parsing_new::move_to_lan(&legal_moves[i]);
                result.push(lan);
            }
        }

        result
    }

    pub fn get_legal_moves_vec(&self) -> Vec<[u128; 3]> {
        let mut result: Vec<[u128; 3]> = Vec::new();
        let legal_moves: [[u128; 3]; MAX_MOVES] = self.get_legal_moves_array().0;

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

    pub fn get_legal_moves_array(&self) -> ([[u128; 3]; MAX_MOVES], usize) {

        // This function is very large and should maybe be cut up into pieces (pun intended).
        // It relies heavily on the logic in get_pins_and_checks, which is also a massive function.

        let mut result: [[u128; 3]; MAX_MOVES] = [[0; 3]; MAX_MOVES];
        let mut index: usize = 0;

        let white_to_move: bool = self.white_to_move();
        let white_pieces: u128 = self.array[ALL_PIECES] & BOARD1;
        let black_pieces: u128 = (self.array[ALL_PIECES] & BOARD2) << 8;
        let all_pieces: u128 = white_pieces | black_pieces;
        let enpassant: u128 = self.array[INFO] & BOARD1;

        // We want to make sure the board is valid before we do all the calculations.
        self.is_valid_board();

        let (checks_and_pins, number_of_sliding_checks, number_of_checks, number_of_pins_and_checks, non_sliding_checks, attacks, xray_checks, allow_enpassant) = get_pins_and_checks(&self.array, white_to_move);

        let king: u128;
        let in_check: bool;
        let mut king_moves: u128;
        let team: u128;
        let opponents: u128;

        let mut queens: u128;
        let mut knights: u128;
        let mut rooks: u128;
        let mut bishops: u128;

        if white_to_move {

            king = self.array[KINGS] & BOARD1;
            in_check = king & attacks != 0;
            team = white_pieces;
            opponents = black_pieces;

            queens = self.array[QUEENS] & BOARD1;
            knights = self.array[KNIGHTS] & BOARD1;
            rooks = self.array[ROOKS] & BOARD1;
            bishops = self.array[BISHOPS] & BOARD1;

            // The king may move to a square that it attacks, but that is not attack by any other opponent piece.
            king_moves = king_attack(king) & !attacks & !team & !xray_checks;

            // If castling is allowed, and the squares between are empty and not attacked, we can castle.
            if !in_check && (WHITE_KINGSIDE_RIGHTS & self.array[INFO] != 0) && (WHITE_KINGSIDE_SQUARES & all_pieces == 0) && (WHITE_KINGSIDE_SQUARES & attacks == 0) {
                king_moves |= WHITE_KINGSIDE_MOVE_TO;
            }
            
            if !in_check && (WHITE_QUEENSIDE_RIGHTS & self.array[INFO] != 0) && (WHITE_QUEENSIDE_SQUARES & all_pieces == 0) && (WHITE_QUEENSIDE_ATTACKS & attacks == 0) {
                king_moves |= WHITE_QUEENSIDE_MOVE_TO;
            }

        } else {

            king = (self.array[KINGS] & BOARD2) << 8;
            in_check = king & attacks != 0;
            team = black_pieces;
            opponents = white_pieces;

            queens = (self.array[QUEENS] & BOARD2) << 8;
            knights = (self.array[KNIGHTS] & BOARD2) << 8;
            rooks = (self.array[ROOKS] & BOARD2) << 8;
            bishops = (self.array[BISHOPS] & BOARD2) << 8;

            // The king may move to a square that it attacks, but that is not attack by any other opponent piece.
            king_moves = king_attack(king) & !attacks & !team & !xray_checks;

            // If castling is allowed, and the squares between are empty and not attacked, we can castle.
            if !in_check && (BLACK_KINGSIDE_RIGHTS & self.array[INFO] != 0) && (BLACK_KINGSIDE_SQUARES & all_pieces == 0) && (BLACK_KINGSIDE_SQUARES & attacks == 0) {
                king_moves |= BLACK_KINGSIDE_MOVE_TO;
            }
            
            if !in_check && (BLACK_QUEENSIDE_RIGHTS & self.array[INFO] != 0) && (BLACK_QUEENSIDE_SQUARES & all_pieces == 0) && (BLACK_QUEENSIDE_ATTACKS & attacks == 0) {
                king_moves |= BLACK_QUEENSIDE_MOVE_TO;
            }
        }
        
        while king_moves != 0 {
            let square: u32 = king_moves.trailing_zeros();
            let pos: u128 = 1u128 << square;
            let move1: [u128; 3] = [king, pos, EMPTY];
            result[index] = move1;
            index += 1;
            king_moves &= !pos;
        }

        // If there are more than two checks, the only piece that may move is the king.
        if number_of_checks > 1 {
            return (result, index)
        }

        if white_to_move {

            let mut pawns: u128 = self.array[PAWNS] & BOARD1;

            while pawns != 0 {
                let square: u32 = pawns.trailing_zeros();
                let pawn: u128 = 1u128 << square;
                let mut pawn_moves: u128 = EMPTY;

                let up: u128 = pawn << 16;
                if up & all_pieces == 0 {
                    pawn_moves |= up;

                    let upup: u128 = pawn << 32;
                    if pawn & RANK_6 != 0 && upup & all_pieces == 0 {
                        pawn_moves |= upup;
                    }
                }

                let upleft: u128 = (pawn << 17) & BOARD1;
                let upright: u128 = (pawn << 15) & BOARD1;

                if (upleft & opponents != 0) || (allow_enpassant && (upleft & enpassant != 0)) {
                    pawn_moves |= upleft;
                }

                if (upright & opponents != 0) || (allow_enpassant && (upright & enpassant != 0)) {
                    pawn_moves |= upright;
                }

                if number_of_sliding_checks == 1 {
                    pawn_moves &= checks_and_pins[0];
                } else if number_of_checks == 1 {
                    pawn_moves &= non_sliding_checks;

                    if (enpassant >> 16 & ((self.array[PAWNS] & BOARD2) << 8) & non_sliding_checks) != 0 && ((enpassant >> 15) & pawn != 0 || (enpassant >> 17) & pawn != 0) {
                        pawn_moves |= enpassant;
                    }
                }

                for i in number_of_sliding_checks..number_of_pins_and_checks {
                    if pawn & checks_and_pins[i] != 0 {
                        pawn_moves &= checks_and_pins[i];
                    }
                }

                while pawn_moves != 0 {
                    let square: u32 = pawn_moves.trailing_zeros();
                    let pos: u128 = 1u128 << square;

                    if pos & RANK_0 != 0 {

                        let to_queen: [u128; 3] = [pawn, pos, QUEEN_PROMOTION];
                        result[index] = to_queen;
                        index += 1;

                        let to_rook: [u128; 3] = [pawn, pos, ROOK_PROMOTION];
                        result[index] = to_rook;
                        index += 1;

                        let to_bishop: [u128; 3] = [pawn, pos, BISHOP_PROMOTION];
                        result[index] = to_bishop;
                        index += 1;

                        let to_knight: [u128; 3] = [pawn, pos, KNIGHT_PROMOTION];
                        result[index] = to_knight;
                        index += 1;

                    } else {

                        let move1: [u128; 3] = [pawn, pos, EMPTY];
                        result[index] = move1;
                        index += 1;

                    }

                    pawn_moves &= !pos;
                }

                pawns &= !pawn;
            }

        } else {

            let mut pawns: u128 = (self.array[PAWNS] & BOARD2) << 8;

            while pawns != 0 {
                let square: u32 = pawns.trailing_zeros();
                let pawn: u128 = 1u128 << square;
                let mut pawn_moves: u128 = EMPTY;

                let down: u128 = pawn >> 16;
                if down & all_pieces == 0 {
                    pawn_moves |= down;

                    let downdown: u128 = pawn >> 32;
                    if pawn & RANK_1 != 0 && downdown & all_pieces == 0 {
                        pawn_moves |= downdown;
                    }
                }

                let downleft: u128 = (pawn >> 15) & BOARD1;
                let downright: u128 = (pawn >> 17) & BOARD1;

                if (downleft & opponents != 0) || (allow_enpassant && (downleft & enpassant != 0)) {
                    pawn_moves |= downleft;
                }

                if (downright & opponents != 0) || (allow_enpassant && (downright & enpassant != 0)) {
                    pawn_moves |= downright;
                }

                if number_of_sliding_checks == 1 {
                    pawn_moves &= checks_and_pins[0];
                } else if number_of_checks == 1 {
                    pawn_moves &= non_sliding_checks;

                    if (enpassant << 16 & (self.array[PAWNS] & BOARD1) & non_sliding_checks) != 0 && ((enpassant << 15) & pawn != 0 || (enpassant << 17) & pawn != 0) {
                        pawn_moves |= enpassant;
                    }
                }

                for i in number_of_sliding_checks..number_of_pins_and_checks {
                    if pawn & checks_and_pins[i] != 0 {
                        pawn_moves &= checks_and_pins[i];
                    }
                }

                while pawn_moves != 0 {
                    let square: u32 = pawn_moves.trailing_zeros();
                    let pos: u128 = 1u128 << square;

                    if pos & RANK_7 != 0 {

                        let to_queen: [u128; 3] = [pawn, pos, QUEEN_PROMOTION];
                        result[index] = to_queen;
                        index += 1;

                        let to_rook: [u128; 3] = [pawn, pos, ROOK_PROMOTION];
                        result[index] = to_rook;
                        index += 1;

                        let to_bishop: [u128; 3] = [pawn, pos, BISHOP_PROMOTION];
                        result[index] = to_bishop;
                        index += 1;

                        let to_knight: [u128; 3] = [pawn, pos, KNIGHT_PROMOTION];
                        result[index] = to_knight;
                        index += 1;

                    } else {

                        let move1: [u128; 3] = [pawn, pos, EMPTY];
                        result[index] = move1;
                        index += 1;
                        
                    }

                    pawn_moves &= !pos;
                }

                pawns &= !pawn;
            }

        }

        while queens != 0 {
            let square: u32 = queens.trailing_zeros();
            let queen: u128 = 1u128 << square;

            let mut queen_moves: u128 = queen_attack(queen, all_pieces) & !(team);

            if number_of_sliding_checks == 1 {
                queen_moves &= checks_and_pins[0];
            } else if number_of_checks == 1 {
                queen_moves &= non_sliding_checks;
            }

            for i in number_of_sliding_checks..number_of_pins_and_checks {
                if queen & checks_and_pins[i] != 0 {
                    queen_moves &= checks_and_pins[i];
                }
            }

            while queen_moves != 0 {
                let square: u32 = queen_moves.trailing_zeros();
                let pos: u128 = 1u128 << square;
                let move1: [u128; 3] = [queen, pos, EMPTY];
                result[index] = move1;
                index += 1;
                queen_moves &= !pos;
            }
            
            queens &= !queen;
        }

        while knights != 0 {
            let square: u32 = knights.trailing_zeros();
            let knight: u128 = 1u128 << square;

            let mut knight_moves: u128 = knight_attack(knight) & !(team);

            if number_of_sliding_checks == 1 {
                knight_moves &= checks_and_pins[0];
            } else if number_of_checks == 1 {
                knight_moves &= non_sliding_checks;
            }

            for i in number_of_sliding_checks..number_of_pins_and_checks {
                if knight & checks_and_pins[i] != 0 {
                    knight_moves &= checks_and_pins[i];
                }
            }

            while knight_moves != 0 {
                let square: u32 = knight_moves.trailing_zeros();
                let pos: u128 = 1u128 << square;
                let move1: [u128; 3] = [knight, pos, EMPTY];
                result[index] = move1;
                index += 1;
                knight_moves &= !pos;
            }
            
            knights &= !knight;
        }

        while rooks != 0 {
            let square: u32 = rooks.trailing_zeros();
            let rook: u128 = 1u128 << square;

            let mut rook_moves: u128 = rook_attack(rook, all_pieces) & !(team);

            if number_of_sliding_checks == 1 {
                rook_moves &= checks_and_pins[0];
            } else if number_of_checks == 1 {
                rook_moves &= non_sliding_checks;
            }

            for i in number_of_sliding_checks..number_of_pins_and_checks {
                if rook & checks_and_pins[i] != 0 {
                    rook_moves &= checks_and_pins[i];
                }
            }

            while rook_moves != 0 {
                let square: u32 = rook_moves.trailing_zeros();
                let pos: u128 = 1u128 << square;
                let move1: [u128; 3] = [rook, pos, EMPTY];
                result[index] = move1;
                index += 1;
                rook_moves &= !pos;
            }
            
            rooks &= !rook;
        }

        while bishops != 0 {
            let square: u32 = bishops.trailing_zeros();
            let bishop: u128 = 1u128 << square;

            let mut bishop_moves: u128 = bishop_attack(bishop, all_pieces) & !(team);

            if number_of_sliding_checks == 1 {
                bishop_moves &= checks_and_pins[0];
            } else if number_of_checks == 1 {
                bishop_moves &= non_sliding_checks;
            }


            for i in number_of_sliding_checks..number_of_pins_and_checks {
                if bishop & checks_and_pins[i] != 0 {
                    bishop_moves &= checks_and_pins[i];
                }
            }

            while bishop_moves != 0 {
                let square: u32 = bishop_moves.trailing_zeros();
                let pos: u128 = 1u128 << square;
                let move1: [u128; 3] = [bishop, pos, EMPTY];
                result[index] = move1;
                index += 1;
                bishop_moves &= !pos;
            }
            
            bishops &= !bishop;
        }

        // for i in 0..index {
        //     print!("{:?} ", parsing_new::move_to_lan(&result[i]))
        // }
        // print!("\n");

        (result, index)
    }

    pub fn is_legal_move_lan(&self, lan: &str) -> bool {
        let move1: [u128; 3] = parsing_new::lan_to_move(lan);
        self.is_legal_move(&move1)
    }

    pub fn is_legal_move(&self, move1: &[u128; 3]) -> bool {
        let (moves, move_count) = self.get_legal_moves_array();
        let mut result: bool = false;

        for i in 0..move_count {
            if &moves[i] == move1 {
                result = true;
            }
        }

        result
    }

    pub fn get_legal_moves_for_tile(&self, tile: &str) -> Vec<String> {
        let bit: u128 = parsing_new::tile_to_bit(tile);
        let moves: Vec<[u128; 3]> = self.get_legal_moves_for_bit(bit);

        parsing_new::moves_to_lan_list(&moves)
    }

    pub fn get_legal_moves_for_bit(&self, bit: u128) -> Vec<[u128; 3]> {
        let mut moves: Vec<[u128; 3]> = Vec::new();
        let (legal_moves, move_count) = self.get_legal_moves_array();

        for i in 0..move_count {
            if legal_moves[i][0] == bit {
                moves.push(legal_moves[i])
            }
        }

        moves
    }
}