pub struct Fen {
    pub boards: [u64; 12],
    pub white_to_move: bool,
    pub castling: u8,
    pub en_passant: u64,
    pub halfmove: u16,
    pub fullmove: u16,
    pub white: u64,
    pub black: u64,
    pub full: u64,
}