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
}