use crate::attacks::*;
use crate::parsing;
use crate::utils::*;


#[derive(Debug, Clone)]
pub struct Fen {
    pub array: Array,
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

        let mut array: Array = parsing::board_string_to_pieces(fen_str_split[0]);
        array[WHITE] = get_white_pieces(&array);
        array[BLACK] = get_black_pieces(&array);
        array[INFO] = parsing::get_info(fen_str_split);

        Self {
            array,
        }
    }

    pub fn to_string(&self) -> String {
        parsing::fen_to_string(self.array)
    }

    pub fn to_visual(&self) -> [[String; 8]; 8] {
        parsing::board_to_visual(self.array)
    }

    pub fn lan_to_fen(&mut self, lan: &str) {
        let move1: Move = parsing::lan_to_move(lan);
        self.move_to_fen(move1)
    }

    pub fn move_to_fen(&mut self, move1: Move) {

        // NOTE: This function does not check whether the move is legal

        let white_to_move: bool = self.white_to_move();
        let enpassant: u64 = parsing::compr_to_bin_enpassant(self.array[INFO]);

        let all_pieces: u64 = self.array[WHITE] | self.array[BLACK];

        let from: u64 = move1[0];
        let to: u64 = move1[1];
        let prom: u64 = move1[2];

        if white_to_move {

            // In case of castling, the respective rook must be moved, since the king is the only piece specified in the move
            
            let king_to_castle: bool = from & self.array[KING_W] & WHITE_KING_POS != 0;

            if king_to_castle && (to & WHITE_KINGSIDE_MOVE_TO != 0) && (WHITE_KINGSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case of kingside castle, we move the rook in the corner to the correct square
                self.array[ROOK_W] &= !(WHITE_KINGSIDE_MOVE_TO >> 1);
                self.array[ROOK_W] |= WHITE_KINGSIDE_MOVE_TO << 1;

            } else if king_to_castle && (to & WHITE_QUEENSIDE_MOVE_TO != 0) && (WHITE_QUEENSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case of queenside castle, we move the rook in the corner to the correct square
                self.array[ROOK_W] &= !(WHITE_QUEENSIDE_MOVE_TO << 2);
                self.array[ROOK_W] |= WHITE_QUEENSIDE_MOVE_TO >> 1;

            }

            // In case of enpassant, we remove the pawn that is captured
            if (enpassant & to != 0) && (self.array[PAWN_W] & from != 0) {
                self.array[PAWN_B] &= !(to >> 8);
            }

            // We set the enpassant flag to 0, since one can only do this move right after an opposing pawn is moved two squares forward
            self.array[INFO] &= !ENPASSANT;

            // In case a pawn has moved two squares forward, we update the enpassant flag accordingly
            if (to & RANK_4 != 0) && (from & self.array[PAWN_W] & RANK_6 != 0) {
                self.array[INFO] |= parsing::bin_to_compr_enpassant(from << 8);
            }

        } else {

            // In case of castling, the respective rook must be moved, since the king is the only piece specified in the move
            
            let king_to_castle: bool = from & self.array[KING_B] & BLACK_KING_POS != 0;

            if king_to_castle && (to & BLACK_KINGSIDE_MOVE_TO != 0) && (BLACK_KINGSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case of kingside castle, we move the rook in the corner to the correct square
                self.array[ROOK_B] &= !(BLACK_KINGSIDE_MOVE_TO >> 1);
                self.array[ROOK_B] |= BLACK_KINGSIDE_MOVE_TO << 1;

            } else if king_to_castle && (to & BLACK_QUEENSIDE_MOVE_TO != 0) && (BLACK_QUEENSIDE_RIGHTS & self.array[INFO] != 0) {

                // In case of queenside castle, we move the rook in the corner to the correct square
                self.array[ROOK_B] &= !(BLACK_QUEENSIDE_MOVE_TO << 2);
                self.array[ROOK_B] |= BLACK_QUEENSIDE_MOVE_TO >> 1;

            }

            // In case of enpassant, we remove the pawn that is captured
            if (enpassant & to != 0) && (self.array[PAWN_B] & from != 0) {
                self.array[PAWN_W] &= !(to << 8);
            }

            // We set the enpassant flag to 0, since one can only do this move right after an opposing pawn is moved two squares forward
            self.array[INFO] &= !ENPASSANT;

            // In case a pawn has moved two squares forward, we update the enpassant flag accordingly
            if (to & RANK_3 != 0) && (from & self.array[PAWN_B] & RANK_1 != 0) {
                self.array[INFO] |= parsing::bin_to_compr_enpassant(from >> 8);
            }

        }

        // If no pawn is moved and no piece is captured, we increase the halfmove, else we set it to 0
        let mut halfmove: u64 = parsing::compr_to_bin_halfmove(self.array[INFO]);

        if (to & all_pieces == 0) && (from & (self.array[PAWN_W] | self.array[PAWN_B]) == 0) {
            halfmove += 1;
        } else {
            halfmove = 0;
        }

        self.array[INFO] &= !HALFMOVE;
        self.array[INFO] |= parsing::bin_to_compr_halfmove(halfmove);

        // The fullmove counter is only increased when black is to move
        if !white_to_move {
            let mut fullmove: u64 = parsing::compr_to_bin_fullmove(self.array[INFO]);
            fullmove += 1;
            self.array[INFO] &= !FULLMOVE;
            self.array[INFO] |= parsing::bin_to_compr_fullmove(fullmove);
        }

        // If the move is a capture, we remove the captured piece from the board
        for i in 0..PIECE_SIZE {
            if self.array[i] & to != 0 {
                self.array[i] &= !to;
                break;
            }
        }

        // We move the piece on the board
        for i in 0..PIECE_SIZE {
            if self.array[i] & from != 0 {
                self.array[i] &= !from;
                self.array[i] |= to;
                break;
            }
        }

        // In case of promotion, we change the pieces according to the given info
        let promoting: bool = prom != NO_PROMOTION;

        if promoting && white_to_move {
            self.array[PAWN_W] &= !to;

            if prom & QUEEN_PROMOTION != 0 {
                self.array[QUEEN_W] |= to;
            } else if prom & ROOK_PROMOTION != 0 {
                self.array[ROOK_W] |= to;
            } else if prom & BISHOP_PROMOTION != 0 {
                self.array[BISHOP_W] |= to;
            } else if prom & KNIGHT_PROMOTION != 0 {
                self.array[KNIGHT_W] |= to;
            } else {
                panic!("move_to_fen: Found unknown flag for promotion")
            }
        } else if promoting {
            self.array[PAWN_B] &= !to;

            if prom & QUEEN_PROMOTION != 0 {
                self.array[QUEEN_B] |= to;
            } else if prom & ROOK_PROMOTION != 0 {
                self.array[ROOK_B] |= to;
            } else if prom & BISHOP_PROMOTION != 0 {
                self.array[BISHOP_B] |= to;
            } else if prom & KNIGHT_PROMOTION != 0 {
                self.array[KNIGHT_B] |= to;
            } else {
                panic!("move_to_fen: Found unknown flag for promotion")
            }
        }

        // The castling rights are updated if the rooks have moved or been captured and if the king moved
        if self.array[KING_W] & WHITE_KING_POS == 0 {
            self.array[INFO] &= !(WHITE_KINGSIDE_RIGHTS | WHITE_QUEENSIDE_RIGHTS);
        }

        if self.array[KING_B] & BLACK_KING_POS == 0 {
            self.array[INFO] &= !(BLACK_KINGSIDE_RIGHTS | BLACK_QUEENSIDE_RIGHTS);
        }

        if self.array[ROOK_W] & (WHITE_KING_POS >> 3) == 0 {
            self.array[INFO] &= !WHITE_KINGSIDE_RIGHTS;
        }

        if self.array[ROOK_W] & (WHITE_KING_POS << 4) == 0 {
            self.array[INFO] &= !WHITE_QUEENSIDE_RIGHTS;
        }

        if self.array[ROOK_B] & (BLACK_KING_POS >> 3) == 0 {
            self.array[INFO] &= !BLACK_KINGSIDE_RIGHTS;
        }

        if self.array[ROOK_B] & (BLACK_KING_POS << 4) == 0 {
            self.array[INFO] &= !BLACK_QUEENSIDE_RIGHTS;
        }


        // We switch the turn info
        if white_to_move {
            self.array[INFO] &= !TURN;
        } else {
            self.array[INFO] |= TURN;
        }

        // We update the positions of the pieces
        self.array[WHITE] = get_white_pieces(&self.array);
        self.array[BLACK] = get_black_pieces(&self.array);

    }

    pub fn player_in_check(&self, player_is_white: bool) -> bool {
        let king: u64;
        let attacks: u64;

        if player_is_white {
            king = self.array[KING_W];
            attacks = get_black_attacks(&self.array);
        } else {
            king = self.array[KING_B];
            attacks = get_white_attacks(&self.array);
        }

        king & attacks != 0
    }

    pub fn game_ended(&self) -> &str {
        let move_count: usize = self.get_legal_moves_array().1;
        let white_to_move: bool = self.white_to_move();
        let in_check: bool = self.player_in_check(white_to_move);
        let halfmove: u64 = parsing::compr_to_bin_halfmove(self.array[INFO]);
    
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
        // This function is not complete, it is just a quick test.
        // A board is valid if it can be reached from the starting position through legal moves only.
        // https://www.fide.com/FIDE/handbook/LawsOfChess.pdf 

        let white_king_count: u32 = self.array[KING_W].count_ones();
        let black_king_count: u32 = self.array[KING_B].count_ones();

        if white_king_count != 1 || black_king_count != 1 {
            panic!("is_valid_board: This board has too many or too few kings")
        }

        let white_piece_count: u32 = self.array[WHITE].count_ones();
        let black_piece_count: u32 = self.array[BLACK].count_ones();

        if white_piece_count > 20 || black_piece_count > 20 {
            panic!("is_valid_board: This board has too many pieces")
        }
    }

    pub fn get_legal_moves_lan(&self) -> Vec<String> {
        let mut result: Vec<String> = Vec::new();
        let legal_moves: MoveArray = self.get_legal_moves_array().0;

        for i in 0..MAX_MOVES {
            if legal_moves[i][0] == 0 {
                
                // If the from pos is 0, the array is empty from this point, so we break.
                break;

            } else {
                let lan: String = parsing::move_to_lan(&legal_moves[i]);
                result.push(lan);
            }
        }

        result
    }

    pub fn get_legal_moves_vec(&self) -> Vec<Move> {
        let mut result: Vec<Move> = Vec::new();
        let legal_moves: MoveArray = self.get_legal_moves_array().0;

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

    pub fn is_legal_move_lan(&self, lan: &str) -> bool {
        let move1: Move = parsing::lan_to_move(lan);
        self.is_legal_move(&move1)
    }

    pub fn is_legal_move(&self, move1: &Move) -> bool {
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
        let bit: u64 = parsing::tile_to_bit(tile);
        let moves: Vec<Move> = self.get_legal_moves_for_bit(bit);

        parsing::moves_to_lan_list(&moves)
    }

    pub fn get_legal_moves_for_bit(&self, bit: u64) -> Vec<Move> {
        let mut moves: Vec<Move> = Vec::new();
        let (legal_moves, move_count) = self.get_legal_moves_array();

        for i in 0..move_count {
            if legal_moves[i][0] == bit {
                moves.push(legal_moves[i])
            }
        }

        moves
    }

    pub fn get_legal_moves_array(&self) -> (MoveArray, usize) {

        // The clarity and speed of this function could benefit from other move gen implementations, such as magic bitboards etc,
        // if you are reading this and are unfamiliar with the code or have forgotten, good luck 💀💀💀,
        // I suggest you use something like lichess.org to visualize the logic

        let mut result: MoveArray = [[0; 3]; MAX_MOVES];
        let mut index: usize = 0;

        let white_to_move: bool = self.white_to_move();
        let all_pieces: u64 = self.array[WHITE] | self.array[BLACK];
        let enpassant: u64 = parsing::compr_to_bin_enpassant(self.array[INFO]);

        // We want to make sure the board is valid before we do all the calculations.
        self.is_valid_board();

        // We determine the pins, sliding checks, non-sliding checks, xray checks, attacks and enpassant permission
        let mut allow_enpassant: bool;
        let mut attacks: u64 = EMPTY;
        let mut xray_checks: u64 = EMPTY;
        let mut non_sliding_checks: u64 = EMPTY;
        let mut number_of_checks: usize;

        let mut pins: PinArray = [0; MAX_PINS];
        let mut number_of_pins: usize = 0;

        let mut sliding_checks: PinArray = [0; MAX_PINS];
        let mut number_of_sliding_checks: usize = 0;

        let opponent_queens: u64;
        let opponent_rooks: u64;
        let opponent_bishops: u64;
        let mut opponent_knights: u64;
        let mut opponent_pawns: u64;
        let opponent_pawn_attack: fn(u64) -> u64;

        // There is precisely one active king and precisely one opponent king, else self.is_valid_board(); would panic
        let active_king: u64;
        let opponent_king: u64;

        let team: u64;
        let opponents: u64;
        let can_enpassant: bool;
        let enpassant_attacks: u64;
        let mut pawns: u64;

        if white_to_move {
            opponent_queens = self.array[QUEEN_B];
            opponent_rooks = self.array[ROOK_B];
            opponent_bishops = self.array[BISHOP_B];
            opponent_knights = self.array[KNIGHT_B];
            opponent_pawns = self.array[PAWN_B];

            opponent_pawn_attack = black_pawn_attack;

            active_king = self.array[KING_W];
            opponent_king = self.array[KING_B];
            team = self.array[WHITE];
            opponents = self.array[BLACK];

            // We check if the enpassant flag is non-empty, and if there is a pawn that could do the en passant move
            enpassant_attacks = enpassant >> 8;
            can_enpassant = (enpassant != 0) && ((((enpassant >> 7) & !FILE_7) | ((enpassant >> 9) & !FILE_0)) & self.array[PAWN_W] != 0);
            pawns = self.array[PAWN_W];

        } else {
            opponent_queens = self.array[QUEEN_W];
            opponent_rooks = self.array[ROOK_W];
            opponent_bishops = self.array[BISHOP_W];
            opponent_knights = self.array[KNIGHT_W];
            opponent_pawns = self.array[PAWN_W];

            opponent_pawn_attack = white_pawn_attack;

            active_king = self.array[KING_B];
            opponent_king = self.array[KING_W];
            team = self.array[BLACK];
            opponents = self.array[WHITE];

            // We check if the enpassant flag is non-empty, and if there is a pawn that could do the en passant move
            enpassant_attacks = enpassant << 8;
            can_enpassant = (enpassant != 0) && ((((enpassant << 7) & !FILE_0) | ((enpassant << 9) & !FILE_7)) & self.array[PAWN_B] != 0);
            pawns = self.array[PAWN_B];
        }

        attacks |= king_attack(opponent_king);
        allow_enpassant = can_enpassant;

        let mut sliders: [u64; 3] = [opponent_queens, opponent_rooks, opponent_bishops];
        let slider_direction_counts: [usize; 3] = [8, 4, 4];
        let slider_directions: [[usize; 8]; 3] = [
            [UP, DOWN, LEFT, RIGHT, UPLEFT, UPRIGHT, DOWNLEFT, DOWNRIGHT],
            [UP, DOWN, LEFT, RIGHT, NO_DIR, NO_DIR, NO_DIR, NO_DIR],
            [UPLEFT, UPRIGHT, DOWNLEFT, DOWNRIGHT, NO_DIR, NO_DIR, NO_DIR, NO_DIR],
        ];
        
        // We now go through each sliding piece to determine it pins, checks, etc
        for slider_index in 0..3 {
            while sliders[slider_index] != 0 {
                let square: u32 = sliders[slider_index].trailing_zeros();
                let piece: u64 = 1u64 << square;
                let index: usize = piece.leading_zeros() as usize;

                let mut check_or_pin: u64 = EMPTY;
                let mut is_check: bool = false;

                for i in 0..slider_direction_counts[slider_index] {

                    let dir: usize = slider_directions[slider_index][i];
                    let ray: u64 = RAY_OCC[dir][index];
                    let attack_blockers: u64 = ray & all_pieces;
                    
                    if attack_blockers == 0 {
                        attacks |= ray;
                    } else {
                        attacks |= slider_attack(piece, attack_blockers, dir);
                    }

                    if ray & active_king != 0 {
                        let check_ray: u64 = check_ray(piece, active_king, dir);

                        let check_blockers: u64 = check_ray & !piece;
                        let number_of_blockers: u32 = (check_blockers & all_pieces).count_ones();

                        if number_of_blockers == 0 {

                            // If the only pieces in the ray are the piece itself and the opponent king, we have a check
                            is_check = true;
                            check_or_pin = check_ray & !active_king;

                            xray_checks |= check_xray(active_king, dir);

                        } else if number_of_blockers == 1 {

                            if check_blockers & team != 0 {
                                // If the attack is blocked by a piece of the current color, we have a pin
                                check_or_pin = check_ray & !active_king;
                            } else if can_enpassant && (enpassant_attacks & check_blockers != 0) && (enpassant & check_blockers == 0) {
                                // We prevent enpassant in a case like 8/8/K7/1pP5/8/8/4b3/7k w - - 0 1
                                allow_enpassant = false;
                            }

                        } else if number_of_blockers == 2 && can_enpassant && (enpassant_attacks & check_blockers != 0) && ((((enpassant_attacks >> 1) & !FILE_0) & pawns & check_blockers != 0) || (((enpassant_attacks << 1) & !FILE_7) & pawns & check_blockers != 0)) {
                            // We prevent enpassant in a case like 8/8/8/KpP4r/8/8/8/7k w - - 0 1
                            allow_enpassant = false;
                        }
                    }
                }

                if check_or_pin != 0 {
                    if is_check {
                        sliding_checks[number_of_sliding_checks] = check_or_pin;
                        number_of_sliding_checks += 1;
                    } else {
                        pins[number_of_pins] = check_or_pin;
                        number_of_pins += 1;
                    }
                }

                sliders[slider_index] &= !piece;
            }
        }

        number_of_checks = number_of_sliding_checks;

        while opponent_knights != 0 {
            let square: u32 = opponent_knights.trailing_zeros();
            let piece: u64 = 1u64 << square;

            let attack: u64 = knight_attack(piece);
            if attack & active_king != 0 {
                non_sliding_checks |= piece;
                number_of_checks += 1;
            }
            attacks |= attack;
            opponent_knights &= !piece;
        }

        while opponent_pawns != 0 {
            let square: u32 = opponent_pawns.trailing_zeros();
            let piece: u64 = 1u64 << square;

            let attack: u64 = opponent_pawn_attack(piece);
            if attack & active_king != 0 {
                non_sliding_checks |= piece;
                number_of_checks += 1;
            }
            attacks |= attack;
            opponent_pawns &= !piece;
        }

        // Now we start generating moves
        let in_check: bool = active_king & attacks != 0;
        let king_attacks: u64 = king_attack(active_king);
        let mut king_moves: u64 = king_attacks & !attacks & !team & !xray_checks;
        
        let mut knights: u64;
        let queens: u64;
        let rooks: u64;
        let bishops: u64;

        if white_to_move {

            queens = self.array[QUEEN_W];
            knights = self.array[KNIGHT_W];
            rooks = self.array[ROOK_W];
            bishops = self.array[BISHOP_W];

            // We allow castling if the king is not in check, the correct flag is set in info, and the squares between are empty and not attacked

            if !in_check && (WHITE_KINGSIDE_RIGHTS & self.array[INFO] != 0) && (WHITE_KINGSIDE_SQUARES & all_pieces == 0) && (WHITE_KINGSIDE_SQUARES & attacks == 0) {
                king_moves |= WHITE_KINGSIDE_MOVE_TO;
            }

            if !in_check && (WHITE_QUEENSIDE_RIGHTS & self.array[INFO] != 0) && (WHITE_QUEENSIDE_SQUARES & all_pieces == 0) && (WHITE_QUEENSIDE_ATTACKS & attacks == 0) {
                king_moves |= WHITE_QUEENSIDE_MOVE_TO;
            }

        } else {

            queens = self.array[QUEEN_B];
            knights = self.array[KNIGHT_B];
            rooks = self.array[ROOK_B];
            bishops = self.array[BISHOP_B];

            // We allow castling if the king is not in check, the correct flag is set in info, and the squares between are empty and not attacked

            if !in_check && (BLACK_KINGSIDE_RIGHTS & self.array[INFO] != 0) && (BLACK_KINGSIDE_SQUARES & all_pieces == 0) && (BLACK_KINGSIDE_SQUARES & attacks == 0) {
                king_moves |= BLACK_KINGSIDE_MOVE_TO;
            }

            if !in_check && (BLACK_QUEENSIDE_RIGHTS & self.array[INFO] != 0) && (BLACK_QUEENSIDE_SQUARES & all_pieces == 0) && (BLACK_QUEENSIDE_ATTACKS & attacks == 0) {
                king_moves |= BLACK_QUEENSIDE_MOVE_TO;
            }

        }

        // We add the kingmoves to the array of moves
        while king_moves != 0 {
            let square: u32 = king_moves.trailing_zeros();
            let pos: u64 = 1u64 << square;
            let move1: Move = [active_king, pos, EMPTY];
            result[index] = move1;
            index += 1;
            king_moves &= !pos;
        }

        // If there are more than two checks, the only piece that may move is the king
        if number_of_checks > 1 {
            return (result, index)
        }

        // Since pawn moves depend on the color, we have to handle the logic separatly
        if white_to_move {

            while pawns != 0 {
                let square: u32 = pawns.trailing_zeros();
                let pawn: u64 = 1u64 << square;
                let mut pawn_moves: u64 = EMPTY;

                let up: u64 = pawn << 8;
                if up & all_pieces == 0 {
                    pawn_moves |= up;

                    let upup: u64 = pawn << 16;
                    if pawn & RANK_6 != 0 && upup & all_pieces == 0 {
                        pawn_moves |= upup;
                    }
                }

                let upleft: u64 = (pawn << 9) & !FILE_7;
                let upright: u64 = (pawn << 7) & !FILE_0;

                if (upleft & opponents != 0) || (allow_enpassant && (upleft & enpassant != 0)) {
                    pawn_moves |= upleft;
                }

                if (upright & opponents != 0) || (allow_enpassant && (upright & enpassant != 0)) {
                    pawn_moves |= upright;
                }

                if number_of_sliding_checks == 1 {
                    pawn_moves &= sliding_checks[0];
                } else if number_of_checks == 1 {
                    pawn_moves &= non_sliding_checks;

                    if ((enpassant >> 8) & self.array[PAWN_B] & non_sliding_checks != 0) && (upleft & enpassant != 0 || upright & enpassant != 0) {
                        pawn_moves |= enpassant;
                    }
                }

                for i in 0..number_of_pins {
                    if pawn & pins[i] != 0 {
                        pawn_moves &= pins[i];
                    }
                }

                while pawn_moves != 0 {
                    let square: u32 = pawn_moves.trailing_zeros();
                    let pos: u64 = 1u64 << square;

                    if pos & RANK_0 != 0 {

                        let to_queen: Move = [pawn, pos, QUEEN_PROMOTION];
                        result[index] = to_queen;
                        index += 1;

                        let to_rook: Move = [pawn, pos, ROOK_PROMOTION];
                        result[index] = to_rook;
                        index += 1;

                        let to_bishop: Move = [pawn, pos, BISHOP_PROMOTION];
                        result[index] = to_bishop;
                        index += 1;

                        let to_knight: Move = [pawn, pos, KNIGHT_PROMOTION];
                        result[index] = to_knight;
                        index += 1;

                    } else {

                        let move1: Move = [pawn, pos, EMPTY];
                        result[index] = move1;
                        index += 1;

                    }

                    pawn_moves &= !pos;
                }

                pawns &= !pawn;
            }

        } else {

            while pawns != 0 {
                let square: u32 = pawns.trailing_zeros();
                let pawn: u64 = 1u64 << square;
                let mut pawn_moves: u64 = EMPTY;

                let down: u64 = pawn >> 8;
                if down & all_pieces == 0 {
                    pawn_moves |= down;

                    let downdown: u64 = pawn >> 16;
                    if pawn & RANK_1 != 0 && downdown & all_pieces == 0 {
                        pawn_moves |= downdown;
                    }
                }

                let downleft: u64 = (pawn >> 7) & !FILE_7;
                let downright: u64 = (pawn >> 9) & !FILE_0;

                if (downleft & opponents != 0) || (allow_enpassant && (downleft & enpassant != 0)) {
                    pawn_moves |= downleft;
                }

                if (downright & opponents != 0) || (allow_enpassant && (downright & enpassant != 0)) {
                    pawn_moves |= downright;
                }

                if number_of_sliding_checks == 1 {
                    pawn_moves &= sliding_checks[0];
                } else if number_of_checks == 1 {
                    pawn_moves &= non_sliding_checks;

                    if ((enpassant << 8) & self.array[PAWN_W] & non_sliding_checks != 0) && (downleft & enpassant != 0 || downright & enpassant != 0) {
                        pawn_moves |= enpassant;
                    }
                }

                for i in 0..number_of_pins {
                    if pawn & pins[i] != 0 {
                        pawn_moves &= pins[i];
                    }
                }

                while pawn_moves != 0 {
                    let square: u32 = pawn_moves.trailing_zeros();
                    let pos: u64 = 1u64 << square;

                    if pos & RANK_7 != 0 {

                        let to_queen: Move = [pawn, pos, QUEEN_PROMOTION];
                        result[index] = to_queen;
                        index += 1;

                        let to_rook: Move = [pawn, pos, ROOK_PROMOTION];
                        result[index] = to_rook;
                        index += 1;

                        let to_bishop: Move = [pawn, pos, BISHOP_PROMOTION];
                        result[index] = to_bishop;
                        index += 1;

                        let to_knight: Move = [pawn, pos, KNIGHT_PROMOTION];
                        result[index] = to_knight;
                        index += 1;

                    } else {

                        let move1: Move = [pawn, pos, EMPTY];
                        result[index] = move1;
                        index += 1;

                    }

                    pawn_moves &= !pos;
                }

                pawns &= !pawn;
            }

        }

        // We generate the moves for the knights
        while knights != 0 {
            let square: u32 = knights.trailing_zeros();
            let knight: u64 = 1u64 << square;

            let mut knight_moves: u64 = knight_attack(knight) & !team;

            if number_of_sliding_checks == 1 {
                knight_moves &= sliding_checks[0];
            } else if number_of_checks == 1 {
                knight_moves &= non_sliding_checks;
            }

            for i in 0..number_of_pins {
                if knight & pins[i] != 0 {
                    knight_moves &= pins[i];
                }
            }

            while knight_moves != 0 {
                let square: u32 = knight_moves.trailing_zeros();
                let pos: u64 = 1u64 << square;
                let move1: Move = [knight, pos, EMPTY];
                result[index] = move1;
                index += 1;
                knight_moves &= !pos;
            }
            
            knights &= !knight;
        }

        // We generate the moves for the sliding pieces
        let mut other_pieces: [u64; 3] = [queens, rooks, bishops];
        let other_pieces_attacks: [fn(u64, u64) -> u64; 3] = [queen_attack, rook_attack, bishop_attack];
        
        for i in 0..3 {
            while other_pieces[i] != 0 {
                let square: u32 = other_pieces[i].trailing_zeros();
                let piece: u64 = 1u64 << square;

                let mut moves: u64 = other_pieces_attacks[i](piece, all_pieces) & !team;

                if number_of_sliding_checks == 1 {
                    moves &= sliding_checks[0];
                } else if number_of_checks == 1 {
                    moves &= non_sliding_checks;
                }

                for i in 0..number_of_pins {
                    if piece & pins[i] != 0 {
                        moves &= pins[i];
                    }
                }

                while moves != 0 {
                    let square: u32 = moves.trailing_zeros();
                    let pos: u64 = 1u64 << square;
                    let move1: Move = [piece, pos, EMPTY];
                    result[index] = move1;
                    index += 1;
                    moves &= !pos;
                }
                
                other_pieces[i] &= !piece;
            }
        }

        // println!("Total: {:?} moves", index);
        // for i in 0..index {
        //     print!("{:?} ", parsing::move_to_lan(&result[i]))
        // }
        // print!("\n");

        (result, index)
    }
}