/*

The Fen struct only supports legal positions:

Both sides must have: exactly one king, at most 16 pieces, at most 8 pawns,
and 8 - number of pawns is the amount of extra bishops, knights, rooks and queens a side can have.
Also, the program may panic if the kings are adjacent to one another.

The older Fen struct (see git history) does support other positions.



For pins, we only need to look at sliding pieces.
We can adapt the ray functions in attacks to find the squares it is directly attacking,
the squares a piece would have to move to to block a check and the piece that may not be moved due to a pin.

Pieces:

left board stores position, right board stores legal moves

a pawn = 0
b pawn = 1
c pawn = 2
d pawn = 3
e pawn = 4
f pawn = 5
g pawn = 6
h pawn = 7

king = 8
queen = 9
k bishop = 10
q bishop = 11
k knight = 12
q knight = 13
k rook = 14
q rook = 15

Attack:

white_all and black_all store all their respective piece on the left board,
and all the squares they attack on the right board.

Full_enpassant:

left board stores all pieces, right board stores en passant square (empty if no en passant).

Game_info

first 32 bits are for white pawns promotion info,
next 32 bits are for black pawns promotion info,
4 bits per pawn:

0000 - not promoted
1000 - is queen
0100 - is rook
0010 - is biship
0001 - is knight

the next 16 bits store the number of halfmoves,
the next 16 bits store the number of fullmoves,
the next 4 store the castling information,
and the next bit stores whether white is to move or not.

castling:
0000 - no castle rights
1000 - white kingside
0100 - white queenside
0010 - black kingside
0001 - black queenside

Check_pin:

white_check_pin and black_check_pin store the rays that attack the king on the left board,
and the rays that pin pieces to the king on the right board.

*/

use crate::logic::*;
use crate::parsing_new;
use crate::utils_new::*;
use crate::default_new;


#[derive(Debug, Clone)]
pub struct Fen {
    pub white_pieces: [u128; 16],
    pub black_pieces: [u128; 16],

    pub white_attack: u128,
    pub black_attack: u128,
    
    pub white_check_pin: u128,
    pub black_check_pin: u128,

    pub full_enpassant: u128,
    pub game_info: u128,
}

impl Fen {
    pub fn new() -> Self {
        Self {
            white_pieces: default_new::WHITE_PIECES,
            black_pieces: default_new::BLACK_PIECES,
            white_attack: default_new::WHITE_ATTACK,
            black_attack: default_new::BLACK_ATTACK,
            white_check_pin: default_new::WHITE_CHECK_PIN,
            black_check_pin: default_new::BLACK_CHECK_PIN,
            full_enpassant: default_new::FULL_ENPASSANT,
            game_info: default_new::GAME_INFO,
        }
    }

    pub fn from_str(fen_str: &str) -> Self {
        let fen_str_split: Vec<&str> = fen_str.trim().split_whitespace().collect();
        if fen_str_split.len() != 6 {
            panic!("Found incorrect fen notation");
        }

        let white_info: ([u128; 16], u128) = parsing_new::string_to_white_pieces(fen_str_split[0]);
        let black_info: ([u128; 16], u128) = parsing_new::string_to_black_pieces(fen_str_split[0]);

        let white_pieces_pos_only: [u128; 16] = white_info.0;
        let black_pieces_pos_only: [u128; 16] = black_info.0;
        let game_info_promotion_only: u128 = white_info.1 | black_info.1;

        let white_attack_pos_only: u128 = get_all_pieces(&white_pieces_pos_only);
        let black_attack_pos_only: u128 = get_all_pieces(&black_pieces_pos_only);

        Self {
            
        }
    }
}