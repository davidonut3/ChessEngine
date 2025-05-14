use crate::fen::*;

pub fn perft(max_depth: usize) {
    // see https://www.chessprogramming.org/Perft

    // this function checks the number of legal moves after every so many moves, up to max_depth

    let fen: Fen = Fen::new();

    for i in 1..max_depth + 1 {
        println!("Depth {:?} yields {:?} moves", i, recursive_perft_check(&fen, i))
    }
}

pub fn recursive_perft_check(fen: &Fen, depth: usize) -> usize {
    let possible_moves: Vec<[u64; 3]> = fen.get_all_possible_moves();

    if depth == 1 {
        // if we reach a depth of 1, we return the number of legal moves from the current fen
        return possible_moves.len()
    } else {
        // if we are not at a depth of 1, we recursively call the function to determine the number of legal moves after <depth> moves
        let mut total: usize = 0;
        for move1 in possible_moves {
            let mut new_fen: Fen = fen.clone();
            new_fen.move_to_fen(&move1);
            total += recursive_perft_check(&new_fen, depth - 1)
        }
        return total;
    }
}