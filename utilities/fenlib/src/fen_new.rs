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






THIS IS WHERE OLD INFO BEGINS, NO LONGER RELEVANT

The Fen struct only supports legal positions:

Both sides must have: exactly one king, at most 16 pieces, at most 8 pawns,
and 8 - number of pawns is the amount of extra bishops, knights, rooks and queens a side can have.
Also, the program may panic if the kings are adjacent to one another.

The older Fen struct (see git history) does support other positions.



For pins, we only need to look at sliding pieces.
We can adapt the ray functions in attacks to find the squares it is directly attacking,
the squares a piece would have to move to to block a check and the piece that may not be moved due to a pin.

Pieces:

left board stores position, right board stores attacks

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

THIS IS WHERE OLD INFO ENDS, NO LONGER RELEVANT

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
        array[PIECES] = get_pieces(&array);

        Self {
            
        }
    }
}