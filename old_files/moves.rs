pub fn white_pawn(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 7);
    moves.push(start << 8);
    moves.push(start << 9);
    moves.push(start << 16);

    moves
}

pub fn black_pawn(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start >> 7);
    moves.push(start >> 8);
    moves.push(start >> 9);
    moves.push(start >> 16);

    moves
}

pub fn knight(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 6);
    moves.push(start << 10);
    moves.push(start << 15);
    moves.push(start << 17);

    moves.push(start >> 6);
    moves.push(start >> 10);
    moves.push(start >> 15);
    moves.push(start >> 17);

    moves
}

pub fn king(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 1);
    moves.push(start << 2);
    moves.push(start << 7);
    moves.push(start << 8);
    moves.push(start << 9);

    moves.push(start >> 1);
    moves.push(start >> 2);
    moves.push(start >> 7);
    moves.push(start >> 8);
    moves.push(start >> 9);

    moves
}

pub fn rook(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 1);
    moves.push(start << 2);
    moves.push(start << 3);
    moves.push(start << 4);
    moves.push(start << 5);
    moves.push(start << 6);
    moves.push(start << 7);

    moves.push(start >> 1);
    moves.push(start >> 2);
    moves.push(start >> 3);
    moves.push(start >> 4);
    moves.push(start >> 5);
    moves.push(start >> 6);
    moves.push(start >> 7);

    moves.push(start << 8);
    moves.push(start << 16);
    moves.push(start << 24);
    moves.push(start << 32);
    moves.push(start << 40);
    moves.push(start << 48);
    moves.push(start << 56);

    moves.push(start >> 8);
    moves.push(start >> 16);
    moves.push(start >> 24);
    moves.push(start >> 32);
    moves.push(start >> 40);
    moves.push(start >> 48);
    moves.push(start >> 56);

    moves
}

pub fn bishop(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 9);
    moves.push(start << 18);
    moves.push(start << 27);
    moves.push(start << 36);
    moves.push(start << 45);
    moves.push(start << 54);
    moves.push(start << 63);

    moves.push(start << 7);
    moves.push(start << 14);
    moves.push(start << 21);
    moves.push(start << 28);
    moves.push(start << 35);
    moves.push(start << 42);
    moves.push(start << 49);

    moves.push(start >> 9);
    moves.push(start >> 18);
    moves.push(start >> 27);
    moves.push(start >> 36);
    moves.push(start >> 45);
    moves.push(start >> 54);
    moves.push(start >> 63);

    moves.push(start >> 7);
    moves.push(start >> 14);
    moves.push(start >> 21);
    moves.push(start >> 28);
    moves.push(start >> 35);
    moves.push(start >> 42);
    moves.push(start >> 49);

    moves
}

pub fn queen(start: &u64) -> Vec<u64> {
    let mut moves: Vec<u64> = Vec::new();

    moves.push(start << 1);
    moves.push(start << 2);
    moves.push(start << 3);
    moves.push(start << 4);
    moves.push(start << 5);
    moves.push(start << 6);
    moves.push(start << 7);

    moves.push(start >> 1);
    moves.push(start >> 2);
    moves.push(start >> 3);
    moves.push(start >> 4);
    moves.push(start >> 5);
    moves.push(start >> 6);
    moves.push(start >> 7);

    moves.push(start << 8);
    moves.push(start << 16);
    moves.push(start << 24);
    moves.push(start << 32);
    moves.push(start << 40);
    moves.push(start << 48);
    moves.push(start << 56);

    moves.push(start >> 8);
    moves.push(start >> 16);
    moves.push(start >> 24);
    moves.push(start >> 32);
    moves.push(start >> 40);
    moves.push(start >> 48);
    moves.push(start >> 56);

    moves.push(start << 9);
    moves.push(start << 18);
    moves.push(start << 27);
    moves.push(start << 36);
    moves.push(start << 45);
    moves.push(start << 54);
    moves.push(start << 63);

    moves.push(start << 7);
    moves.push(start << 14);
    moves.push(start << 21);
    moves.push(start << 28);
    moves.push(start << 35);
    moves.push(start << 42);
    moves.push(start << 49);

    moves.push(start >> 9);
    moves.push(start >> 18);
    moves.push(start >> 27);
    moves.push(start >> 36);
    moves.push(start >> 45);
    moves.push(start >> 54);
    moves.push(start >> 63);

    moves.push(start >> 7);
    moves.push(start >> 14);
    moves.push(start >> 21);
    moves.push(start >> 28);
    moves.push(start >> 35);
    moves.push(start >> 42);
    moves.push(start >> 49);

    moves
}