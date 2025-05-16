use crate::attacks::*;

/*

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

Pawn_info:

left board stores en passant square (empty if no en passant),
right board stores promotion info:

first 32 bits of second board are for white pawns,
other 32 bits of second board are for black pawns,
4 bits per pawn:

0000 - not promoted
1000 - is queen
0100 - is rook
0010 - is biship
0001 - is knight

All:

white_all and black_all store all their respective piece on the left board,
and all the squares they attack on the right board.

Full_info:

left board stores all pieces, right board contains other info:

the first 8 moves store whether white is to move or not
the next 16 bits store the number of halfmoves,
the next 16 bits store the number of fullmoves,
and the next 8 store the castling information.

Check_pin:

white_check_pin and black_check_pin store the rays that attack the king on the left board,
and the rays that pin pieces to the king on the right board.

*/

#[derive(Debug, Clone)]
pub struct Fen {
    pub white_pieces: [u128; 16],
    pub black_pieces: [u128; 16],

    pub white_all: u128,
    pub black_all: u128,
    
    pub white_check_pin: u128,
    pub black_check_pin: u128,

    pub full_info: u128,
    pub pawn_info: u128,
}