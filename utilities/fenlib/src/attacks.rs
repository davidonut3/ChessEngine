/// The purpose of this file is to replace moves with a smarter system

/// This represents the left board of the two boards
pub const BOARD1: u128 = 0xFF00FF00FF00FF00FF00FF00FF00FF00;

/// This represents the right board of the two boards
pub const BOARD2: u128 = BOARD1 >> 8;

/// Determine the squares where the knight may move to
pub fn knight(info: &u128) -> u128 {
    let piece: u128 = info & BOARD1;
    let ull: u128 = piece << 18;
    let urr: u128 = piece << 14;
    let uul: u128 = piece << 33;
    let uur: u128 = piece << 31;
    let drr: u128 = piece >> 18;
    let dll: u128 = piece >> 14;
    let ddr: u128 = piece >> 33;
    let ddl: u128 = piece >> 31;

    let board: u128 = ull | urr | uul | uur | drr | dll | ddr | ddl;
    board >> 8
}