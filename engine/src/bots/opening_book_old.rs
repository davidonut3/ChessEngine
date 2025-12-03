use std::collections::HashSet;

use crate::bots::opening_book::get_opening_move;
use crate::fenlib::fen::Fen;
use crate::parsing;

use rand::Rng;

pub fn _check() {
    // // Create fen list
    // for opening in OPENING_BOOK {
    //     println!("{:?},", opening.0)
    // }

    // // Check for collisions in fen list
    // for i in 0..OPENING_FENS.len() {
    //     for j in 0..OPENING_FENS.len() {
    //         if (i != j) && (OPENING_FENS[i] == OPENING_FENS[j]) {
    //             println!("{:?} with {:?} = {:?} with {:?}", i, OPENING_FENS[i], j, OPENING_FENS[j]);
    //             panic!()
    //         }
    //     }
    // }

    // // Create hash list
    // for opening in OPENING_FENS {
    //     let fen: Fen = Fen::from_str(opening);
    //     let hash: u64 = fen.get_partial_zobrist();

    //     println!("{:?},", hash)
    // }

    // // Check for collisions in hash list when shifted n bits
    // let shift = 39;
    // for i in 0..OPENING_HASHES.len() {
    //     for j in 0..OPENING_HASHES.len() {
    //         if (i != j) && ((OPENING_HASHES[i] >> shift) == (OPENING_HASHES[j] >> shift)) {
    //             println!("{:?} with {:?} = {:?} with {:?}", i, OPENING_HASHES[i], j, OPENING_HASHES[j]);
    //             panic!()
    //         }
    //     }
    // }

}

pub fn compute_hash_function() {
    let unique: HashSet<u64> = OPENING_HASHES.iter().cloned().collect();
    if unique.len() != OPENING_HASHES.len() {
        panic!("Zobrist collision detected BEFORE magic numbers.");
    }

    let mut rng = rand::rng();
    let max_range: u64 = u64::MAX;

    let magic = rng.random_range(0..max_range);
    let shift = 58;

    println!("Magic {:?}", magic);

    let mut buckets: [Vec<u64>; 64] = [
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
        Vec::new(),
    ];

    for hash in OPENING_HASHES {
        
        let index: usize = (hash.wrapping_mul(magic) >> shift) as usize;

        buckets[index].push(hash.clone())

    }

    let mut magics: Vec<u64> = Vec::new();

    for i in 0..64 {
        let (magic, collisions) = compute_perfect_hash_function(buckets[i].clone(), buckets[i].len());

        if collisions > 0 {
            println!("Magic {:?} has {:?} collisions", i, collisions)
        }

        magics.push(magic);
    }

    println!("DONE");

    for magic in magics {
        println!("{:?}", magic)
    }
}

pub fn compute_perfect_hash_function(hashes: Vec<u64>, hash_count: usize) -> (u64, i32) {
    
    let shift = 56;
    let num_candidates = 10000000;

    let mut best_magic: u64 = 0;
    let mut best_index_range: u64 = u64::MAX;
    let mut best_collisions = i32::MAX;

    let mut rng = rand::rng();
    let max_range: u64 = u64::MAX;

    for _ in 0..num_candidates {

        // We generate odd numbers to reach all indices, not just the even ones
        let magic: u64 = rng.random_range(..max_range) | 1;

        let mut used: HashSet<u64> = HashSet::with_capacity(hash_count);
        let mut max_index = 0;
        let mut min_index = 0;
        let mut collisions = 0;

        for h in &hashes {

            let index = h.wrapping_mul(magic) >> shift;

            if index > max_index {
                max_index = index;
            }

            if index < min_index {
                min_index = index
            }

            if !used.insert(index) {
                collisions += 1;
            }
        }

        // If we find a perfect magic, we return it
        if collisions == 0 {
            return (magic, 0)
        }

        let index_range = max_index - min_index;

        if collisions < best_collisions {
            best_index_range = index_range;
            best_magic = magic;
            best_collisions = collisions;
        } else if collisions == best_collisions && index_range < best_index_range {
            best_index_range = index_range;
            best_magic = magic;
        }
    }

    // If we do not find a perfect magic, we return the one with the least collisions
    (best_magic, best_collisions)
}

const MAGIC_MAIN: u64 = 3134906545090246530;
const SHIFT_MAIN: u64 = 58;
const SHIFT_BUCKET: u64 = 56;
const MAGICS: [u64; 64] = [
    5971743464145005705,
    4789389687790637853,
    4830851653383174343,
    17576381440065381957,
    2257047376531186683,
    16435209512755860999,
    18010425809965335729,
    13996604805239217483,
    11540830937986860933,
    3366478013427932181,
    6133089226353628885,
    15719494879288879875,
    13293983775105728105,
    1882114105419877011,
    17322165572361751815,
    9812387299420180999,
    14521731177465168125,
    1837259003675807137,
    14153006288766649249,
    11220322089818829755,
    9004862841342034475,
    7593964137706131031,
    9206783584401818867,
    2400641029628885173,
    17787942289066206359,
    13130250316891133235,
    14048868102368663273,
    1680465528569681659,
    12817720024889776927,
    17771450426619087979,
    2118129954877424799,
    7386963526798867679,
    7115265518187030935,
    10557535270006462329,
    18057236720728026559,
    12125259948922308187,
    1692579925580276543,
    42001621190276079,
    10509976698191439487,
    1982040010944013485,
    12668790950050737907,
    13610988839704689057,
    1798194978568995779,
    8122859190286452211,
    10664651084674651997,
    17042280482227533927,
    6904310229529690043,
    1086015164560916961,
    3259562283020004009,
    16530126357980233083,
    13310085533160670455,
    14571959803983305081,
    775747746035931751,
    11086426803621188317,
    6952513603162432635,
    15685308612382161327,
    13190298392467232425,
    11209623470189188225,
    14883493664010061959,
    7436771661637041193,
    18058990361671796451,
    14872515613561403979,
    7445651362894462807,
    5281351752662894999,
];

pub fn create_opening_table() {
    let mut hash_book: [[Option<&[(&str, u32)]>; 256]; 64] = [[None; 256]; 64];

    for opening in OPENING_BOOK {
        let fen_str = opening.0;
        let fen = Fen::from_str(fen_str);
        let partial_zobrist = fen.get_partial_zobrist();

        let index_main: usize = (partial_zobrist.wrapping_mul(MAGIC_MAIN) >> SHIFT_MAIN) as usize;
        let magic: u64 = MAGICS[index_main];
        let index: usize = (partial_zobrist.wrapping_mul(magic) >> SHIFT_BUCKET) as usize;

        hash_book[index_main][index] = Some(opening.1)
    }

    println!("pub const OPENING_BOOK: [[Option<(u32, &[(u16, u32)])>; 256]; 64] = [");

    for book in hash_book {
        println!("\t[");

        for opening in book {
            if let Some(info) = opening {

                print!("\t\tSome(");

                let mut total_moves: u32 = 0;
                for move1 in info {
                    total_moves += move1.1;
                }

                print!("({:?}, &[", total_moves);

                for move1 in info {
                    print!("({:?}, {:?}), ", parsing::lan_to_compact(move1.0), move1.1)
                }

                println!("])),")
            } else {
                println!("\t\t{:?},", opening)
            }
        }

        println!("\t],")
    }

    println!("];")
}

pub fn check_openings() {
    for opening in OPENING_FENS {
        let fen = Fen::from_str(opening);
        let partial_zobrist = fen.get_partial_zobrist();

        get_opening_move(partial_zobrist);
    }
}


const OPENING_FENS: &[&str] = &[
    "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/3P4/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/8/3Pn3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/8/3Pn3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/4P3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/1ppp1ppp/p1n5/4p3/B3P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/1ppp1ppp/p1B5/4p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3p4/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3pN3/5N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PPQ1PPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4p3/3p1b2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4p3/3p1b2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/4Pp2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/4Pp2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1p1p/8/6p1/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1p1p/8/6p1/4Pp1P/5N2/PPPP2P1/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/2PP4/5N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3PN3/8/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2Pp4/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/2N1P3/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/2N1PN2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4Pn2/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PP1BPPPP/R2QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2Pp4/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4P3/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1b1kbnr/ppppqppp/2n5/4P3/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3PN3/8/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3Pp3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/3Pp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3p4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2P5/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1PpP4/8/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/4P3/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2Pp4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/3nP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/3nP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p4/3nP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p4/3nP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4P3/3p4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/3nP3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p4/3nP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/3nP3/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/4P3/8/2n5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/4P3/8/2P5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3P1p2/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/1P2P3/8/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/1p2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/1p1PP3/8/P1P2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/1p1PP3/8/P1P2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3pP3/1p1P4/8/P1P2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/q7/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/q7/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/3p2p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/3p2p1/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPPQPPP/RNB1K2R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/5N2/PPPNPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5N2/PPPNPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3pP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3pP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/4P3/2p5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2B1P3/2p5/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/8/1p6/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/8/1p6/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/3p4/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4p3/3nP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4p3/3nP3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p1p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3pP3/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/4P3/3p4/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/4P3/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbp1pppp/1p1p4/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3Pp3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3PB3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rn1qkbnr/pp1bpppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rn1qkbnr/pp1Bpppp/3p4/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4P3/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4P3/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3P4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/6B1/3Pn3/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3PnB2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3PnB2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3PnB2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/3p1p2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppbppp/8/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppbppp/8/8/2B1Pp2/5N2/PPPP2PP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/5P2/PPPPPKPP/RNBQ1BNR w kq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2ppP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2PpP3/8/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbQkbnr/ppp2ppp/8/4p3/4P3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/3PP3/3B4/PPPN1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/3P1N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/3P1N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/3B4/PPPN1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/4P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/4P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbq1bnr/pppppkpp/5p2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQ - 0 0",
    "rnbqkb1r/pppppppp/8/4P3/8/2P5/P1PP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/P4N2/1P2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3Pn2B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2p5/3Pn2B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4p3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4N3/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5N2/8/8/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/4P1P1/PP1P1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pB2/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2B5/4PN2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/4P1P1/PP1P1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2pP4/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2pP4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3Pp3/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2p1p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/P4N2/1P2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2P5/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3pP3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqk1nr/ppppbppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqk1nr/ppppbppp/2n5/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/3p4/4p3/3PP1b1/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/3p4/4P3/4P1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/8/PPQ1PPPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3PPP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PPP2/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PPP2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/3Pp3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/2P1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/3q4/8/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/3q4/8/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/3Pp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3pP3/1b1P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2P5/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3Pp3/5P2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2p5/3Pn2B/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP3P/2N5/PP2PPP1/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq  - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3QP3/5N2/PPP2PPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/6B1/3P4/5N2/PPPNPPPP/R2QKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4p3/3nP3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/5Pb1/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/7p/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/2PP4/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/3P4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2NP4/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/1B6/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r2qkbnr/pppbpppp/2np4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p3B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2pP2B1/8/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnb1kb1r/pp1ppppp/1q3n2/2pP2B1/8/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnb1kb1r/pp1ppppp/1q3n2/2pP2B1/8/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/7n/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/3nP3/2B5/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4P3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P3n1/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P1P1n1/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3N4/8/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3pP3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3pP3/8/PPPPNPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/6B1/3Pn2P/8/PPP1PPP1/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p2B1/3Pn2P/8/PPP1PPP1/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2PB1N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1bB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1bB1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1bB1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3P4/3n4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3PP3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/4B3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/q7/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/7P/8/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/P6P/8/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pP3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4P3/3pP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3Pp3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/pppppp1p/8/6p1/3Pn2B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3Pp3/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPPNPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3Pp3/8/PPPNPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3Pp3/4P3/PPPN1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/5P2/1P3N2/P1PPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/2P5/PP2PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/q7/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2np1n2/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2np1n2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4p3/1b2P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4p3/1bB1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4P3/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4P3/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3P4/8/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3b4/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3b4/8/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "r1b1kbnr/ppppqppp/2n5/4P3/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnb1kb1r/ppppqppp/5n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kb1r/ppppqppp/5n2/4N3/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/5p2/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/5p2/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3p4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3N4/2N5/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3Pp3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pN3/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3b4/3pN3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3b4/3pN3/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2BPp3/2N5/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2P2N1P/PP1P1PP1/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4P3/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/8/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/4P3/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2P5/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/3Pp3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "r1bqk1nr/ppppbppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppnpppp/8/8/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/5P2/4P3/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/4P3/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/3P4/4P3/PPP1NPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/3nP3/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/1n6/4P3/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/1n6/4P3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/6B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/6B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3pP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3pP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/4P3/2p5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/4P3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/3P1N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P3n1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/8/4p3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/8/1P2PN2/PBPP1PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/4P3/3p4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/8/3Pp3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/8/3Pp3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/3PP3/8/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/6B1/3PP3/8/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/6B1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p1N1/2B1P3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1p1pp/3p4/5p2/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2pP4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pp4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pP4/8/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/p1pppppp/1p6/8/2P5/5bP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3PPP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5pB1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4p3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/8/4P3/8/PPP2PPP/RNBqKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/8/4P3/8/PPP2PPP/RNBK1BNR b kq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3PnB2/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/5P1N/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/5P1N/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/1B2p3/8/1P2P3/PBPP1PPP/RN1QK1NR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2B2n2/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/5P2/5NP1/PPPPP2P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P3P/8/PPP2PP1/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/5n2/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1pp5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1pp5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5pB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5B2/5p2/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/5p2/5p2/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2NP4/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2Pp4/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2Pp4/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/2Pp4/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPP1NPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4p3/5P2/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4P3/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/3P1NP1/PPP2P1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2PP4/5P2/PP2P1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/5P2/PP2P1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3P4/3P4/5P2/PP2P1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pP4/8/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/pp2pppp/8/2pq4/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/pp2pppp/8/2pq4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/6b1/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "r2qkbnr/pppnpppp/8/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bP5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bP5/2N2N2/PPQPPPPP/R1B1KB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1p1pp/8/3p1p2/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2PPp3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1b1PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2p1p3/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2p1p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3Pp3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/6B1/3P4/2P5/PP2PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2nP4/8/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnb1kb1r/ppp1pppp/5n2/3q4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kb1r/ppp1pppp/5n2/3q4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2BPP3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p3/2BPn3/8/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2B1n3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pP2B1/4n3/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p2B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/1p2pppp/p2p4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p2pppp/p2p4/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1bB1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3P4/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2P5/8/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/8/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/1P4P1/P1PPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P4P1/P1PPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P4P1/PBPPPPBP/RN1QK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1ppp1/5n1p/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/8/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2P5/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pP4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/pp2pppp/8/2pq4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3P4/2p5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2pP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2pP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3p4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/1P3N2/P1P1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/7p/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/7p/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/8/7p/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/3QP3/5N2/PPP2PPP/RNB1KB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/3P4/2P3P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/2PP2b1/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P1B2/8/PP2PPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/1B2p3/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3pp3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2Pp4/8/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/5P2/4Pb2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/5P2/4PQ2/PPPP2PP/RNB1KB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppb1pp/4p3/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p3B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3p4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5B2/2p5/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5p2/2p5/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/3q4/8/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rn1qkbnr/pb1ppppp/1p6/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rn1qkbnr/pb1ppppp/1p6/2p5/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2p5/3PnB2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pP4/4nB2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/2P5/PP4PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/4P3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/6b1/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/3QP3/5N2/PPP2PPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p1P3/8/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p3/2B1n3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/1Q3N2/PP2PPPP/RNB1KB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3pPb2/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5B2/3p4/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5p2/3p4/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5p2/3p4/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/7p/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5Bp1/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1p1p/5pp1/8/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1p1p/5pp1/8/3P3P/8/PPP1PPP1/RN1QKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5Bp1/8/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5p2/3p4/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnb1kb1r/pppp1ppp/4pq2/8/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnb1kb1r/pppp1ppp/4pq2/8/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1p1p/4p1p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2pPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2pPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnb1kb1r/pp1ppppp/1q3n2/2p5/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnb1kb1r/pp1ppppp/1q3n2/2p5/3P1B2/N3P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pB2/3p4/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2pP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2pP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2BP4/4P3/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2BpP3/8/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3pPb2/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3pPb2/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4pP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4NP2/8/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3pP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P1B2/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3PP3/6P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppn1ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2Pp4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2PQ4/8/PP2PPPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/2PQ4/8/PP2PPPP/RNB1KBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1npppp/3p4/1Bp5/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "r1b1kbnr/ppppqppp/2n5/4P3/8/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/5p2/2p5/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/3p2p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rn2kbnr/ppp1pppp/8/3q4/6b1/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kb1r/ppp1pppp/5n2/3q4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kb1r/ppp1pppp/5n2/3q4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn2kbnr/ppp1pppp/8/3q4/6b1/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3P4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/5p2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/6b1/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P2Bn1/8/PP2PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppp1p/2n3p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/5P2/2N2N2/PPPPP1PP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/p1ppppbp/1p4p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2pP4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3Pn3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3Pn3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3pP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pn1p/6B1/3PP3/8/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pB1p/8/3PP3/8/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/7p/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/7p/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/8/3PP2p/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3pP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pP3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4P3/2Pp4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4P3/2Pp4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1p1p/5pp1/8/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p1P3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3P1b2/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3P4/3P4/8/PP2PPPP/RbBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3P4/Q2P4/8/PP2PPPP/RbB1KBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pp4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pP4/8/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p1P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p1P3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/1n6/4P3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/5b2/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/5b2/8/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2P5/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2P5/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/6B1/3Pp3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3PnB2/8/PPPNPPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/1p1ppppp/p1n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/1p1ppppp/p1B5/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p4/3nP3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P1NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/2P5/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/8/5bP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p2B1/3Pn3/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3Pn2B/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N4P/PPP2PP1/R1BQKBNR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/5N2/PP1BPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P1N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1B2/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1p2/8/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1ppp1/7p/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1ppp1/7p/3p4/3P3B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3QP3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3QP3/8/PPP2PPP/RNB1KBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/4P3/4Q3/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5P2/8/3p4/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/8/1B2p3/3nP3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/4p3/8/1bPP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/4p3/8/1bPP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1p1ppp/4p3/2p5/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1p1p/5pp1/8/3P4/6P1/PPP1PP1P/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pP4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1p1pp/3p1n2/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1p1pp/3p1n2/5p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2pP4/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2pP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2BP4/4P3/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p2B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2pP4/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/4p3/8/1bPP4/8/PP1NPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/1P6/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2ppP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/1P2PN2/PBPP1PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4p3/1P6/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/3P4/PPPNPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4p3/1PN5/PBPP1PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/1n6/4P3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/4p3/1p6/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/8/p7/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1ppppppp/8/p7/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4P3/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2p5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/4pP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/4pP2/3P4/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/P1N2N2/1PPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/3p4/8/3P2b1/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2B3p1/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p2pppp/p2p4/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2B1Pp2/8/PPPP2PP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/2B1Pp2/8/PPPP2PP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3P4/3p4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p2pppp/p7/2pp4/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3Pp3/8/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/6b1/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/8/1P3b2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
    "rnb1kbnr/pppp1ppp/8/4p3/4PP1q/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/pppp1ppp/8/4p3/4PP1q/6P1/PPPP3P/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppppqppp/8/4p3/4PP2/6P1/PPPP3P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/8/8/p2PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/8/1p6/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4p3/1bP5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4p3/1bP5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P2P1/8/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1Bppp/4p3/3p4/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p1b2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/5P1N/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4P3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbQkbnr/ppp2ppp/8/4p3/2P5/8/PP2PPPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N1P3/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4p3/2PP4/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3p4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rn1qkbnr/pbppp1pp/1p6/5p2/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1N3/4P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/4P3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/8/7p/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnb1kbnr/pp2pppp/1qp5/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/5P2/3P1N2/PPP1P1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/4p3/8/1bPP4/8/PP1BPPPP/RN1QKBNR b KQkq - 0 0",
    "rnb1k1nr/ppppqppp/4p3/8/1bPP4/8/PP1BPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3p4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3p4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppppp1pp/2n5/5p2/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppppp1pp/2n5/5P2/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1p1pp/2n5/3p1P2/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1p1pp/2n5/3p1P2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3pP3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3N4/8/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3N4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/2N2P2/PPP1P1PP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N2Q2/PPPP1PPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3P4/3P2b1/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3P4/3P2b1/8/PPP1BPPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PPQ1PPPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "r1b1kbnr/ppppqppp/2n5/4P3/5B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/3p4/8/2PP2b1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/3p4/8/2PP4/5b2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2P5/PP1N1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1p1pp/3p4/5p2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3p4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/6P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/1Bp5/4P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/8/1Bp5/3nP3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/8/2p5/2BnP3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3Pn3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3PN3/8/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/3pP3/8/PPPPNPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/3pP3/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p1P3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p1P3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5N2/PPPPQPPP/RNB1KB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5N2/PPPPQPPP/RNB1KB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5NP1/PPPPQP1P/RNB1KB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/n7/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/B7/8/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/p1pppppp/p7/8/4P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/1B2pP2/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/6B1/3Pp3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/3p1n2/8/2PP2b1/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/1P2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2P5/4P3/8/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1p1ppp/4p3/2b5/4P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1p1ppp/4p3/2b5/3PP3/8/P1P2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2p5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2p5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/4n3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3pp3/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2p5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/1p1PP3/5N2/P1P2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PPP3/5P2/PP4PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/2B5/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq  - 0 0",
    "rnbqkbnr/pp2pppp/2pP4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2pp4/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3P4/3P2b1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/4q3/8/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/4q3/8/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp2B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4p3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4N3/5N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4p3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3pP3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pn1p/6B1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3Pn3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pP4/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pn1p/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pn1p/8/3P3B/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/6P1/8/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/6b1/8/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/1Bpp4/5P2/4PN2/PPPP2PP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3p4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PPQ1PPPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/2PPp3/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/8/1B2p3/3NP3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3Pp3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3Pp3/2P2P2/PP4PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3Q4/5N2/PPP1PPPP/RNB1KB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2pP4/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2pP4/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pp4/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pN4/8/8/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5pB1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5pB1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5pB1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/3Pp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/3p4/3Ppp2/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/2P1P3/PP3PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3PPP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2p5/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pp2pppp/8/3P4/3p4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/8/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/1B2pp2/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2Pp4/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/2Pp4/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2p5/3PnB2/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/2N4P/PPP2PP1/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/3PPp2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/3pP3/8/PPPPNPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/3pP3/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p3P/8/8/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp2P/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3P4/2n5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/3P4/2P5/P1P2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3p4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3N4/6P1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/5P2/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/5n2/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1ppppppp/8/p7/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/1B6/8/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/P7/1P2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/1P6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/1P6/8/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/8/3P1NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/8/3P1NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4p3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/6N1/4p3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/6N1/4p3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/6N1/4p3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2pP4/2P1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/P1N5/1PP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pP4/2p5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/2p1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P1P2/8/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P2P1/PPPNPPBP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P2P1/8/PPP2P1P/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/P1N5/1P2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pPp3/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2pPp3/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2pPp3/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2p1p3/1P2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPPQPPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4Np2/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/5n2/4Np2/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3P4/1b1P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P1n3/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P1n3/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/2PPp3/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/6N1/2PPp3/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/2NP4/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/8/2NP4/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2pp4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/8/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/4P3/2pP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4P3/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pppp1pbp/6p1/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/pppp1pbp/6p1/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p3/3Pn3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p3/3Pn3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2BpP3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPPQPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2B2/3p4/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2p3B1/3Pn2P/8/PPP1PPP1/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/6PN/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/4P3/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1pp5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1pp5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/1p3n2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/1p3n2/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3PnB2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppn1ppp/3p4/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/5p2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/P7/1P2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3Pp3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3PP3/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/n7/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/n7/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/3P2b1/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/3p4/8/3PP1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6pB/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3pp3/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2pp1/4p2p/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/P1N5/1PP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P3P/8/PPP1PPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P3P/8/PPP1PPP1/RNBQKBNR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P1Q1/2N5/PPPP1PPP/R1B1K1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnb1kbnr/pp2pppp/8/2pq4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/2PP4/5N2/PP2PPPP/RbBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/3Pn2B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqk1nr/ppppbppp/2n5/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1nppp/4p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/1PB1P3/5N2/P1PP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P1n3/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppppp2p/6p1/5pB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/Q1p5/6P1/PP1PPPBP/RNB1K1NR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/3P4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/1P2P3/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/1P2P3/PB1P1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/5P2/4P3/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2pP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1p1/7p/5pB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1p1/7p/5p2/3P3B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp3/7p/5pp1/3P3B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp3/7p/5pp1/3P4/6B1/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pP3/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pP4/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/2N1P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/1P6/P7/2PPPPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PPP3/3B4/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4p3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp3pp/3p4/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp3pp/3p4/4pp2/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2PB4/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pP4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/8/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/3p2p1/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p2pppp/p2p4/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppn1ppp/3p4/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P1P2/8/PPP1P1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/2Pp4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/2PQ4/8/PP2PPPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/2PQ4/8/PP2PPPP/RNB1KBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/2pppppp/p7/1p6/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P3n1/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/3p2p1/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/6P1/PP1BPP1P/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4P3/2Pp4/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/1pppppp1/8/p6p/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pP4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/8/6p1/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/1p2P3/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/1p2P3/P7/2PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3pP3/1p6/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2B1p3/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3Pp3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/3Pp3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/2PP2P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3Pp3/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3pP3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/6P1/8/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/6P1/7P/PPPPPPB1/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2ppP3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2ppP3/5P2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2ppP3/5P2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2ppp1/2p4p/3p4/3P3B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2ppp1/2p4p/3p4/3P3B/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2BPP3/2N5/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3p4/2b1p3/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/1ppppppp/8/p7/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/1PP5/5N2/P2PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/1PP5/5N2/P2PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/1PP5/5N2/PB1PPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pb1ppppp/1p6/2p5/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/6B1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/3Pp3/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/8/4n3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/8/4n3/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/1B1pp3/8/1P2P3/PBPP1PPP/RN1QK1NR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/4p3/2N2P2/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4p3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4N3/5N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/2PPp3/8/PP1NPPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/8/4n3/4PP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/4B3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/1P3NP1/P1PPPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/1P3NP1/P1PPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/1P3NP1/PBPPPP1P/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5pB1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/2Pp4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/5n2/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/4P3/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2BPP3/2N5/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/P3P3/2N5/1PPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2pP4/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/3pb3/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP1B2/5N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/8/PPPNPPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/8/6p1/3PnB2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2B5/4pp2/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/1P2P3/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/1p2P3/P4N2/2PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/5N1P/PPP2PP1/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/2pppppp/p7/1p6/P2PP3/8/1PP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2p5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2NP1N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3p4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4pP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/4NP2/8/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/4P1P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/8/4P1P1/PPPPNPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3Q4/2N5/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2P2n2/8/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1p1pp/5n2/3p1pB1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3p4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3Q4/2N5/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/8/PPPPQPPP/RNB1KBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/8/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3Q4/2N5/PPP1PPPP/R1B1KBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/8/2NQ4/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4P3/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4b3/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3ppn2/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/7P/8/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p2P/8/8/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4N3/8/8/PPPPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p3B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/6B1/3p4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/6B1/3Q4/2N5/PPP1PPPP/R3KBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1bB1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/1p2pppp/p7/2pP4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1pp1pppp/p7/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/1pp1pppp/p7/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5Q2/PPPP1PPP/RNB1KBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppb1/6pp/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppb1/6pp/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/1p1PP3/5N2/P1P2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/1P6/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2pPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/1P6/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/8/1p6/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/1p3n2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2P2n2/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/6B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p1P3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/3Pp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3n4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2P5/4P1P1/PP1P1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/3P4/5P2/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/1QN5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/8/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/6B1/3P4/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/5N2/PPPNPPPP/R2QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/P1N5/1P2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/3PP3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2pp1n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/P3P3/2N5/1PPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/4p3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/8/1P6/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/P1P5/8/1P2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/1pp2ppp/p3p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P2P1/8/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/4p3/3N4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/3P1N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3Pp3/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/8/3Pn3/5P2/8/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/6n1/3P4/5P2/8/PPP1P1PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3P4/4p3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5Bp1/8/8/1P6/P1PPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1p1p/5pp1/8/8/1P6/P1PPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1p1p/5pp1/8/2P5/1P6/P2PPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/4P3/4n3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3Pp3/8/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2B5/4PN2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/6B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/2p2n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pPp3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2pPp3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4P3/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N1P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/4P3/p7/2PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/3p4/8/5Pb1/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp3pp/3p4/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqkbnr/ppp1p1pp/5p2/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnb1kbnr/pp1ppppp/8/q1P5/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/8/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N1P/PPPP1PP1/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/5N1P/PPPP1PP1/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3P4/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3b4/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2Np4/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/7Q/2N5/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/2P5/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/P1pP4/5N2/1P2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p3B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/6N1/2B1p3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/1B1P4/8/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rn1qkb1r/pppbpppp/5n2/1B1P4/8/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rn1qkb1r/pppbpppp/5n2/3P4/8/8/PPPPBPPP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/2PP2b1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbq1bnr/pppppkpp/5p2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQ - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/3p4/2p5/2P1P1b1/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/5p2/4P3/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/2Pp4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/4p3/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/4P3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2ppp1/2p4p/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2ppp1/2p4p/3p4/3P3B/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4P3/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2B5/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/2PP2b1/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1p1pp/5B2/3p1p2/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/3P1NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2pPp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2pPp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2pPp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/P6P/8/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pP2/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pppppp/5n2/1p6/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pppppp/5n2/1p6/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/5n2/1p6/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rn1qkb1r/pbpppppp/5n2/1p6/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pP3/6b1/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2pP4/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/8/PP1NPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/4pn2/1p6/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/4pn2/1p6/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p4/2P3b1/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnb1kbnr/ppp2ppp/8/3qp3/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2P5/8/4B3/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/6B1/3P4/5N2/PPPNPPPP/R2QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/1PPp4/4PN2/P2P1PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppppp/7n/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/7n/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/3P4/6PN/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2P5/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/2PB4/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2PP1N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/5bN1/4p3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/2pp4/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/5n2/2pPp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq  - 0 0",
    "rnbqkb1r/pp1p1ppp/5n2/2pPp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2P2N2/PP1PBPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2P5/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N2NP1/PPPP1P1P/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4P3/6P1/PPPP1P1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP1BPPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3p4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pP4/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/2NP4/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/3pb3/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/3pB3/4p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2ppP3/8/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2ppP3/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pP3/4P3/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/4P3/2PP4/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/1P6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/4n3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2pP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2pP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/1Bp1P3/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3P4/1p6/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/4P3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/P7/5N2/1PP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/1pppppp1/p6p/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1pppppp1/p6p/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3pN3/3P2b1/8/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/4BP2/PPP3PP/RN1QKBNR b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/4p3/1b6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/4B3/1b6/8/P1PPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4B3/1b6/8/P1PPPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pP3/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/2N1P3/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1pp5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1pp5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p3B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p3B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2P5/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/4pn2/6B1/1bPP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/1P6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3Q4/5N2/PPP1PPPP/RNB1KB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rn1qkbnr/pbp1pppp/1p1p4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/4p3/1bP5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/3Np3/1bP5/8/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/4p3/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/2PP2b1/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4pp2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/6B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5NP1/PPPP1PBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/1ppp1ppp/4p3/p7/1bPP4/8/PP1BPPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4p3/1b2P3/2NP1N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/3QP3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3QP3/8/PPP2PPP/RNB1KBNR w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/2B5/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/2B5/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/8/1B6/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/5N2/PPPNPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/3pP3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2pPp3/2P1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/3B4/PPP1QPPP/RNB1K1NR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3bp3/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/4Pp2/8/PPPPB1PP/RNBQK1NR b KQkq - 0 0",
    "rnb1kbnr/pppp1ppp/8/8/4Pp1q/8/PPPPB1PP/RNBQK1NR w KQkq - 0 0",
    "rnb1kbnr/pppp1ppp/8/8/4Pp1q/8/PPPPB1PP/RNBQ1KNR b kq - 0 0",
    "rnbqkbnr/p1pp1ppp/1p2p3/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpp1ppp/1p2p3/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/1Bp5/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rn1qkb1r/pppbpppp/5n2/3P4/2B5/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/1pp2ppp/p3p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3pPP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3pPP2/8/PPPPN1PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1p1pp/5p2/3p4/3P3B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/p1pppp1p/1p4p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/3p1n2/8/3PP1b1/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/3p1n2/8/3PP1b1/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P1P2/8/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4p3/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4p3/3p1b2/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4p3/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppppp1/7p/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp2/7p/6p1/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/3P4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5P2/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/4PP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/2Pp4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/2PQ4/2N5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3p4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3N4/6P1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p4/3P1Pb1/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/8/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/8/1P4P1/PBPPPP1P/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3P4/8/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppnpppp/8/3pP3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4Pn2/8/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pP2/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3pP3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/5P2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/5P2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4p3/3NP3/8/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/1P6/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/1P3NP1/P2PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppbppp/5n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/q7/2B5/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2B1Pp2/8/PPPP2PP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2B1Pp2/2N5/PPPP2PP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3pP3/4n3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/4P3/2Pp4/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/4P3/2PB4/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/4P3/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppnpppp/3p4/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppb1/6pp/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppppb1/6pp/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/4p3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/4P1P1/PPPPNPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/3P1N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pppp1pbp/6p1/4p3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3P4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/5P2/1P2PN2/P1PP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2Pp4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3pN3/8/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/6PP/PPPPPPB1/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3ppn2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3pP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "r1bqkb1r/pppnpppp/3p1n2/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p1b2/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/ppp2ppp/4p3/3p1b2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/3p4/3PN3/8/8/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5p2/2pP4/8/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N2NP1/PPPP1P1P/R1BQKB1R b KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2N1P3/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3Pp3/4B3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3Pp3/4B3/PPPN1PPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/P7/1PPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1p1pp/2n2p2/3pP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p2B1/3PN3/8/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pP4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2N3P1/PP1P1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp2Q/8/2N1P3/PPPP1PPP/R1B1KBNR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/2N1P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/3QP3/4n3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/1n6/2P1P3/8/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pP4/8/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2NP1N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/2pppppp/p7/1p6/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3pNb2/3P4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3pNb2/2PP4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/8/PPPPQPPP/RNB1KBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/8/8/PPPPQPPP/RNB1KBNR b KQkq - 0 0",
    "rnb1kbnr/ppp2ppp/4p3/3q4/8/8/PPPPQPPP/RNB1KBNR w KQkq - 0 0",
    "rnb1kbnr/ppp2ppp/4p3/3q4/8/2N5/PPPPQPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3B4/4Pp2/8/PPPP2PP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPPBPPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp3pp/4p3/3p1p2/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4PP2/3P4/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/4P3/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppppp2p/6p1/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppppp2p/6p1/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppp2p/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/4P3/1B3N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/1n6/4P3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqk1nr/ppppbppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2p5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/4p3/8/2PP4/8/PP1bPPPP/RN1QKBNR w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/4P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/4P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/1B1p4/3P4/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/pppp1pbp/4p1p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/2P3b1/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/4p3/1bP5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/P1N2N2/1P1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/P1pP4/2N5/1P2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/4p3/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3pP3/3Pn3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/6p1/6B1/3Pn3/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/4p3/2PP4/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/8/6p1/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4P3/5p2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3N4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2P1PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/2pp4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1npppp/2pp4/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/5P2/1P2PN2/P1PP2PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/5Q2/PPPP1PPP/RNB1K1NR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p1P3/2P5/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/4Pp2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/4Pp2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p2B1/3P4/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
    "rnbqk1nr/pppp1pbp/4p1p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1p1pp/3p4/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2ppP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2pP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2pP4/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnb1kb1r/ppp1pppp/5n2/3q4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1p1p/8/6p1/2B1Pp2/5N2/PPPP2PP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/p1pppppp/1p3n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkb1r/pbpppppp/1p3n2/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/2b1p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/4n3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/2p2np1/8/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2np4/3P4/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/3p4/3Pn3/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
    "rnb1kbnr/pppp2pp/5q2/4Np2/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/pppp2pp/5q2/4Np2/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/4n3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pP2/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/4P2P/8/PPPP1PP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3p1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3P1B2/8/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/8/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/3p1n2/5b2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/8/6P1/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/8/1ppP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N1P3/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/6n1/3P4/4PP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/Q1p5/5N2/PP1PPPPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/Q1p5/5N2/PP1PPPPP/RNB1KB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2Q5/5N2/PP1PPPPP/RNB1KB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/3p1n2/5b2/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/8/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/5P2/1P2P3/PBPP2PP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/6P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/2Pp4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/2P5/4p3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/8/2P5/4B3/PP3PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/3BP3/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2PBP3/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/P7/1PPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/1P2P3/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/1p2P3/P7/2PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/8/1P2P3/8/2PP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/3Pp3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/4P1P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/3PPp2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1p1pp/2n2p2/3pP3/3P1P2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/2P1P3/2N3P1/PP1P1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3pp3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NPP/PPPPPPB1/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppppbppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp2ppp/8/3qp3/8/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppp2ppp/8/3qp3/8/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pB1p/8/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/4P3/6P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/4P3/6P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/q7/1P6/2N5/P1PP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP2P/2N5/PPP2PP1/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/6P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n2n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP1Q1/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/1pp2ppp/p3p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N2N2/PPPPBPPP/R1BQK2R b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3pp3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/3pp3/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2B2n2/4p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/3p1n2/4P3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/8/8/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/8/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/4p3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4P3/8/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4P3/4P3/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1ppppp/p7/2p5/P3P3/2N5/1PPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2p1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/2Pp4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/2PQ4/2N5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/4p3/8/2PP4/8/PP1QPPPP/RN2KBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N4P/PPP2PP1/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/6P1/PPPP1P1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/6P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/3P2Q1/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqk2r/ppppbppp/4pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2Pp4/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pppp1p1p/2n3p1/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/4PN2/PPPPB1PP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/4P3/3P1N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/8/5p2/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/3P4/5P2/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppn1ppp/3p4/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/4P3/7P/PPPP1PP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/8/PPPPQPPP/RNB1K1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2BP4/4P3/PP3PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/4P3/6P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/2p5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/1P2P3/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/3p4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pP3/3P2b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p4/3P2b1/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3pN3/5Pb1/8/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/5P2/4PN2/PPPPB1PP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N2P2/PPP1P1PP/R2QKBNR b KQkq - 0 0",
    "rn1qkbnr/pbp1pppp/1p1p4/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp3pp/3p4/4pp2/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/1pp2ppp/p3p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "r1b1kbnr/ppqppppp/2n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3Pp3/2p5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pP4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/1P3N2/PBPP1PPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/2pp4/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/8/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/8/2B1Pp2/5N2/PPPP2PP/RNBQK2R b KQkq - 0 0",
    "rnbqk2r/pppp1ppp/5n2/4p3/1bB1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/5N2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppppp1pp/5p2/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/3p4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1p1ppp/p3p3/2p5/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/1P4P1/PBPPPP1P/RN1QKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/1P4P1/PBPPPP1P/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/8/1P4P1/PBPPPPBP/RN1QK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3p4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3N4/6P1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbp1pppp/1p1p4/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/4p3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/4p3/5N2/PPPPQPPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3P4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/3Pp3/4BP2/PPP3PP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2Pp4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/2PQ4/2N5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/3P1Bb1/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4N3/4n3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4N3/3Pn3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/8/3p4/2PP4/5b2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/pppp1ppp/8/2b1p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/4P3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/4p3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4N3/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rnb1k1nr/ppppqppp/4p3/8/1bPP4/5N2/PP1BPPPP/RN1QKB1R b KQkq - 0 0",
    "rnb1kbnr/ppp1pppp/8/3q4/8/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3pP3/1p6/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/1P3N2/PB1PPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/2P1p3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3P4/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/5n2/4pp2/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pn1p/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2nP4/8/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3pP3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/8/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pb1ppppp/1p6/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pb1ppppp/1p6/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3QP3/2P5/PP3PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3P4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPPQPPP/RNB1K2R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/6P1/2N5/PPPPPP1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/6n1/2N5/PPPPPP1P/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/8/8/4P1n1/2N5/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n5/3nP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkb1r/pppppppp/2n5/3nP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2pP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/p2ppppp/1p3n2/2p5/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp1ppp1p/5np1/2pP4/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4N3/4n3/8/PPPPQPPP/RNB1KB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/6p1/3p4/5P2/5NP1/PPPPP2P/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/5NP1/PPPPP2P/RNBQKB1R w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/5NP1/PPPPP1BP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1npppp/2pp4/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p1P3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/2pppppp/p7/1p6/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/1bpppppp/p7/1p6/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/1bpppppp/p7/1p6/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pP2/2B5/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
    "rn1qkbnr/pp2pppp/3p4/2p5/2P1P1b1/5N2/PP1PBPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3pP3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/4p3/2pP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2p5/3pP3/3P4/PPP1NPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3pp3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/2P5/PP1N1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1p1pppp/8/1p6/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppp1p/2n3p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp2pp/4p3/5p2/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/1P2PN2/P2P1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/8/1p6/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/8/1p6/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2p1p3/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/3p4/2p1p3/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5P2/PPP1P1PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppppbppp/4p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/2PP2b1/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3P1b2/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/8/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3pP3/8/8/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1pppppp1/p6p/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/8/6p1/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p1p3/3p4/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/3P4/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppppp1p/2n3p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/8/P6P/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/P6P/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1pp1pp/8/2pP1p2/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppppppp/2n5/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppp1bppp/3pp3/8/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/8/1p6/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/1bpppppp/p7/1p6/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/8/6p1/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp2pp/4pn2/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/7P/PPPP1PP1/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppn1ppp/3p4/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnb1kbnr/pppp1ppp/8/8/2B1Pp1q/8/PPPP2PP/RNBQK1NR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2P1p3/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2pp4/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/6B1/2PP4/8/PP2PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1pp1/4pn1p/6B1/2PP4/8/PP2PPPP/RN1QKBNR w KQkq - 0 0",
    "rn1qkbnr/ppp1pppp/3p4/8/3P2b1/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2ppP3/5P2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/3P1Bb1/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2P5/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/pp1p1ppp/4p3/2b5/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/8/1p6/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/2PP4/5P2/PP2P1PP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pP4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnb1kbnr/pp2pppp/8/2pq4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnb1kbnr/pp2pppp/8/2pq4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppppp1pp/2n5/4Pp2/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp1bpppp/3p4/1Bp5/P3P3/5N2/1PPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2pp2p1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/6P1/PPPP1P1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/6P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P2P1/PPPN1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/2pp4/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/3P4/PPPNPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/1p6/2p5/1P2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/3P4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2Npp/3p1n2/8/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/3p1n2/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/3P1N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppppp1pp/5n2/5p2/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2pp4/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/3P4/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/8/1Bp5/3nP3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
    "r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/3p2p1/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1ppp1/5B1p/3p4/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p1p3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
    "rnbq1bnr/pppppkpp/5p2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQ - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/pppnpppp/8/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/8/1p6/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/3p4/1Bp5/4P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p3/2B1n3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p2Q/2B1n3/2N5/PPPP1PPP/R1B1K1NR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/4p3/2P5/P7/1P1PPPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppppp/5n2/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2P2n2/8/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/3p4/3p4/2P1P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2pP4/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/8/P6P/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n5/3pp3/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/3P4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3P4/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n5/4p3/2P5/1P6/PB1PPPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3p4/4n3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppppnppp/8/3Pp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/1pp2ppp/p3p3/3p4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pppp2pp/8/4pp2/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbq1bnr/pppppkpp/5p2/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQ - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "r1b1kbnr/ppp1pppp/2n5/3q4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/5p2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3P1p2/5N2/PPP1P1PP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/8/4p3/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2Bp4/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/3P2P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "r2qkbnr/ppp1pppp/2n5/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "r1bqkbnr/pp1npppp/3p4/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/4P3/3P2P1/PPP2P1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/6P1/7P/PPPPPP2/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/8/3pP3/4N3/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppppp1p/5np1/8/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqk2r/ppppppbp/5np1/8/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1ppp1p/2p3p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/2p3p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2n5/2pp4/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
    "rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/pp3ppp/2p5/3pp3/8/4P1P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
    "r1bqkbnr/pppp2pp/2n5/4pp2/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/3P1N2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppppp2p/5p2/6p1/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnb1kbnr/pp2pppp/2p5/3q4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbBr/pppp2pp/2n5/4pp2/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/4PP2/PPPP1KPP/RNBQ1BNR b kq - 0 0",
    "r1bqkb1r/pppp1ppp/2n1pn2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pp1p/2p3p1/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1p1pp/2n5/1B1p1P2/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5B2/2pp4/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
    "r1bqkb1r/pp1ppppp/2n2n2/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
    "rnbqkbnr/2pppppp/p7/1p6/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/8/2Pp4/3P1N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3P4/8/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
    "rnb1kb1r/ppppqppp/5n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p2ppppp/8/1ppP4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1ppP4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/3ppppp/p7/1ppP4/P3P3/8/1PP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkb1r/p1pp1ppp/1p2pn2/6B1/2PP4/5N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/8/1Bp5/3NP3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/3p4/2b1p3/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 0 0",
    "r1bqkb1r/pppp1ppp/2n2n2/4p3/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/3p4/4P3/3P4/PPP1QPPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3pP3/8/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/4p3/8/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P3NP1/P1PPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/4p3/2N2P2/PPPP2PP/R1BQKBNR w KQkq - 0 0",
    "rnbqkbnr/2pppppp/p7/1p6/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "rn1qkbnr/1bpppppp/p7/1p6/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/5n2/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkb1r/pppnpppp/8/3pP3/8/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/2PNp3/8/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/5P2/PPPPPKPP/RNBQ1BNR w kq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/3P4/PPPNPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/2n1p3/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/2p5/4P3/P7/1PPP1PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppp1ppp/8/8/3PPp2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1P2/3BP3/PPP3PP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/4p3/1BppP3/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
    "rnb1kbnr/ppq1pppp/2pp4/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1ppppppp/p7/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4P3/2P1n3/8/PP1NPPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/8/4p3/2B1n3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/4p3/8/2p5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/8/4N3/8/PPPPQPPP/R1B1KBNR b KQkq - 0 0",
    "rnbqk1nr/ppp1ppbp/6p1/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
    "r1bqkbnr/ppp1pppp/2n5/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/p1pppppp/1p6/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
    "rn1qkbnr/pbpppppp/1p6/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rn1qkb1r/ppp1pppp/5n2/3P4/3P2b1/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/5n2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp1pppp/3p1n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/2p5/8/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/3p4/4p3/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/4p3/3Pn3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/4p3/3Pn3/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pppp1ppp/4pn2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/3BP3/PPPN1PPP/R1BQK1NR b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/1pppppp1/p6p/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4P3/PPPN1PPP/R2QKBNR b KQkq - 0 0",
    "rnbqk1nr/ppp2ppp/8/3pp3/Pb5P/8/2PPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p2pp/2p1p3/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pppppp1p/6p1/8/3PnB2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/8/2pp4/8/P6P/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp3ppp/8/2ppp3/8/P1P4P/1P1PPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pppppp1p/6p1/8/8/P6P/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqk1nr/ppppppbp/6p1/8/8/P6P/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1pp2ppp/p3p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/3PP3/PPP1NPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/P1P4P/1P1PPPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/5NPP/PPPPPP2/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NPP/PPPPPP2/RNBQKB1R w KQkq - 0 0",
    "rnbqkbnr/1p1pppp1/p1p4p/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
    "r1bqkbnr/pp1ppppp/2n5/8/4P3/3Q4/PPP2PPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/3P4/P1P4P/1P2PPP1/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/5n2/3pp3/8/P1P4P/1P1PPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/1pppppp1/p6p/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
    "r1bqk1nr/pppp1ppp/2n5/3Np3/1bP5/5N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp1ppppp/8/2p5/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p2pp/2p1p3/5p2/2PP4/5N2/PP1BPPPP/RN1QKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p4/6P1/4P3/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/8/3p2P1/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/5n2/2pp4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p2P1/8/8/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/4P3/3P4/PPP1QPPP/RNB1KBNR b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3pp3/8/4P1N1/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkb1r/ppp2ppp/4pn2/3pP3/8/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
    "rnbqkb1r/ppp1nppp/4p3/3p4/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
    "rnb1kbnr/pppp1ppp/8/8/3PPp1q/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/pp2pppp/2np4/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR w KQkq - 0 0",
    "r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/ppppnppp/4p3/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
    "rnbqkb1r/ppp1pp1p/5np1/3p4/8/1P3NP1/P1PPPPBP/RNBQK2R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3P4/3P1p2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
    "r1bqkbnr/pppp1ppp/4p3/3Pn3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1ppppp/8/2pnP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp1pppp/3p4/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/pp2pppp/2p5/3p4/8/4P1N1/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
    "rnbqkbnr/ppp2ppp/8/3p4/3PPp2/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
    "rnbqkbnr/pp1p1ppp/2p5/4p3/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
    "rnbqkb1r/pp1p1ppp/2p2n2/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
    "r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/2N2NP1/PPP1PP1P/R1BQKB1R b KQkq - 0 0",
    "rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P3NPP/P1PPPP2/RNBQKB1R b KQkq - 0 0",
    "rn1qkbnr/pp2pppp/2p5/3p1b2/8/5NPP/PPPPPP2/RNBQKB1R w KQkq - 0 0",
];

const OPENING_HASHES: &[u64] = &[
    12284723483594076938,
    15853533859685008450,
    14731950262543316068,
    6944240636507662184,
    15923776501048014390,
    6613955743142175900,
    106363626911716165,
    3327699412853650658,
    13718246379493067280,
    9761768955445735370,
    16571915770375398980,
    7366630971983264782,
    9899526059726771884,
    2688277500064056736,
    14121050968606070028,
    8058221415573325312,
    15237636748097050462,
    6361384781991507767,
    4191229111478516069,
    521131662060101808,
    10621141211114133200,
    14543604496447365385,
    973855878450810676,
    1331034441185287802,
    4301256851948121826,
    3442905045153777550,
    16017703657992905372,
    8481599552371847416,
    273516975833827248,
    10234268507600403212,
    11418343878374705962,
    1239053831077161229,
    5441083332379827924,
    3410443422567939561,
    5571402843459307296,
    8379048387802047226,
    6323694336500520543,
    4939876184343084056,
    7124580424207531434,
    5764837800185513869,
    15507356449306334503,
    7752747516062762105,
    15398220689448177388,
    7571395875446185906,
    14213220562591878334,
    11497439540309506919,
    1210069717240944960,
    6962962229122946332,
    14739694327241836048,
    10825938229072212425,
    15996027055794189123,
    10519407063047170626,
    15725184092394729348,
    13283651836408078806,
    13943431861479466788,
    1844754492699837609,
    10763343498222596803,
    17126684895167375631,
    10361696704929064797,
    2585247493153567825,
    10321173922318197575,
    33837849173950816,
    6596592581588879988,
    16265728118661117644,
    18320754944538617730,
    17430723977245122398,
    15392504227593934680,
    13229575065981136273,
    1996331349608857245,
    4685142579452692804,
    775634238345251970,
    9455581441439891266,
    3392470138082962510,
    16277503423927921937,
    6504458013236620107,
    4711649594089356767,
    7542067864500620615,
    14126462866219803291,
    15040947415235910992,
    17975492768025980360,
    7348119354812543493,
    14999297710603576211,
    17605489982240205717,
    12440823354589903865,
    2585828467920722398,
    3649345848276413010,
    8746685188713033620,
    2581384218598021197,
    13258377813686311178,
    2005149414702942726,
    766816139176704025,
    16565746994892739564,
    5175574275825165175,
    4047288502292965085,
    10304768872579709237,
    2534792209805306425,
    6465106392992428230,
    15327984188431371477,
    3361458686231543603,
    455033227426717702,
    7265180298980682120,
    6758460198158240850,
    15916292014664159348,
    17913473591514142834,
    11167625496916956704,
    15076929329528198118,
    13478118201830949141,
    1668414048827249177,
    7573106431335014999,
    1573552416816550775,
    7651597646004115165,
    9817687116393844938,
    3791210821143970196,
    13535864998945907635,
    1011335001273102885,
    7700649157970333976,
    8011694300270678460,
    15282569069401685033,
    8663262387975876389,
    10536173696408298912,
    1663596466448884465,
    8260877350592233615,
    13692874071826759307,
    9736528582533815121,
    16545268020699828959,
    12563911538086729497,
    4287995662871498220,
    14613677639157255687,
    17422753716470679274,
    6766252269179033062,
    16232370380686886072,
    15590674215325688162,
    9770934314563102742,
    12554952707570246530,
    4316094806532422007,
    16549147637728302270,
    6415406623111022048,
    6922559388129878074,
    185315694503193012,
    17511270826953246470,
    12317648093199798234,
    17312275941829752835,
    6659996579460508431,
    10317101993068585089,
    1184997888710957353,
    8220164604323059579,
    7018046202691342600,
    16386308601104089677,
    11002740855179416216,
    9854123624416371390,
    1149828385050357780,
    548263733342256215,
    13412067661847894680,
    16181891859223555393,
    5529611436534070861,
    17972975640396296155,
    4411911242393420262,
    4769808678088006940,
    12945337734731248558,
    9445359113212176118,
    9201092494874970825,
    14056473939408575383,
    8015879803648936091,
    3958523640102710116,
    15583348684592288970,
    14603461029918898844,
    15577869257842208710,
    13424567367262249364,
    11400531455840659858,
    9353359541906503900,
    1315239458223502051,
    6451345104205248293,
    18293413408764401961,
    887972100305583163,
    12312070095999293038,
    642692015688623861,
    16997057999091572514,
    12777049286368831739,
    17461908405188540481,
    271340232034801395,
    277215830922845523,
    4860298989863844972,
    320808168357140703,
    17034159227735848286,
    12079251173392380056,
    14204808714378672406,
    6520612577411481790,
    13052689294936844828,
    7061567837111155727,
    988149534140893266,
    3162920940712921568,
    5289323160971935854,
    191204837087986404,
    6692356589459363807,
    15250346656123549176,
    7383065724225479790,
    190034538840491859,
    6509617183078897471,
    7442088988636909290,
    9862823064166669936,
    10891064359635341118,
    12925228971603281720,
    16672833303892475455,
    13240567332519689138,
    17081992055536127083,
    15282587130870163889,
    8511986333929838890,
    10748478989540991229,
    9706257339377234843,
    14804375628766671121,
    17532794080754245132,
    14576245516088376278,
    17698415158037002843,
    6463200326486714711,
    16417676512861078991,
    15766689491158671381,
    9258860002823182284,
    14679895840810586634,
    9108672281816777628,
    15151236741943267472,
    4246466199113928881,
    285907937730098539,
    11655792747402167792,
    16861909194187917178,
    6880515026012032256,
    6964855544374070413,
    13301271764249828298,
    15444384234197043373,
    15520041893456352143,
    9372589351384523983,
    4018290623281837401,
    4736904927767298577,
    18174376354980011212,
    2988190774584522729,
    13124737783578103361,
    8690663929296412633,
    7449073402198298263,
    3217557682901863249,
    5675976718145275139,
    17621664382862824539,
    13784536362591770321,
    6377030520133580953,
    1421061707576655199,
    5426077966143778099,
    4409664836417608315,
    16597765500237577621,
    6018759206112979971,
    3985727265900007230,
    4254182588267464382,
    15243903851432313058,
    9196835793825944558,
    7849220152439273897,
    4311803742373554842,
    202485975125741376,
    6501705664535502636,
    657875029714540242,
    13079374260522108812,
    1850914796192142464,
    14854912195979386872,
    7755969766427205468,
    12813638392680151681,
    18020119655337115463,
    10957713354040526222,
    11402435203324667397,
    6515303070101604408,
    12443871736469165917,
    44195750355551313,
    6498025065349313416,
    9103720219221818254,
    8624500630289279428,
    4356954189903997954,
    5226089808757640923,
    7986188288475058025,
    4312967848318582420,
    2681895005650489645,
    5133483636417752036,
    2588167238590859481,
    4713163066709049687,
    7283431955936577873,
    5670829036908937447,
    10787747379177454366,
    10250767466078600509,
    13212382563072514279,
    15409153270482768233,
    11391987127536407523,
    17368077822448785538,
    15383373640726564996,
    16028070599307573057,
    16678764600960897691,
    9616079580991760466,
    17513415421632945946,
    17206526647449687226,
    5424450910704428750,
    13727131912693812684,
    4413619991141914502,
    11943498540288040594,
    17183686430860581716,
    2405134579019460417,
    10181584931041606733,
    10442148924468494523,
    17599855483728618374,
    12450995434649564138,
    13315813593034349276,
    5590369989995671906,
    13252742042647875911,
    1492404080387257503,
    11630515019276248329,
    957968665625277957,
    969077584811778438,
    7621947874717185451,
    64830254397448422,
    9092125836031123257,
    2901296742547691060,
    5386580794659676090,
    7136252837609893666,
    7729070521926686848,
    14370894140095384460,
    2418197428136891638,
    1771864074299289900,
    7230979203600957452,
    17700944266078965718,
    17205897733526397623,
    4820360640961628086,
    17684408641603735998,
    6295378654549860645,
    8146944219022123159,
    11309175376738809624,
    2001745575603955122,
    12110230720110985225,
    15262208561514231791,
    11939405680816554151,
    538816968038955915,
    2522355803521889165,
    6884520384560096997,
    15618300132064949327,
    9302780224595269442,
    13259679292091658904,
    10934793869603988556,
    2454962374143145434,
    10249427359824520918,
    12117827031707547012,
    10110683878704741762,
    16536382487283340547,
    14716052264654035275,
    14804720655575905885,
    12679724553519333331,
    16444708086056009049,
    3082256973144472582,
    9778728580203360674,
    4258010065690812668,
    3186204431188878745,
    5683786491553157002,
    16342539504582810758,
    8007258153672028547,
    2654904679063778373,
    8948072519135962153,
    14988384355821975333,
    4462671654950568284,
    12874247201529766779,
    16888338826335449879,
    10270923907898773546,
    10896635138478014445,
    4597914468516787867,
    1845769734586070825,
    8934979627642091899,
    14640220606025617066,
    10973683124217824548,
    17674362299402274826,
    10899344113990709955,
    4591797720462016966,
    2000198168208034240,
    10925837655139602189,
    15881728672412469963,
    16407564332519705414,
    8057020967850419300,
    18017132230166544017,
    3974922454120485250,
    15798074769500530660,
    8483985437819734322,
    15158511499806899425,
    18185402312251238487,
    15166551152004376706,
    11078022246984707396,
    3201211446163209426,
    6490803333863426970,
    12000060644757547797,
    7124220081979213096,
    18005918357045826315,
    11606957570319713127,
    3116277771932942663,
    4639978782931143492,
    17025863589857531976,
    5830610069322320150,
    3127198128595472602,
    5261360277754814088,
    1341613895264989916,
    10474982386845608086,
    18019728129875319897,
    5747465113016208847,
    10463759407170432439,
    17561009459649029177,
    15883516360189415002,
    15436723637297065052,
    2683284308760149764,
    5169974518401939082,
    7959719790989723154,
    17317200531933404305,
    2422746753898941480,
    12009605690645126786,
    5835289927731580977,
    18219204408143102781,
    1963443649377473109,
    8771900651833138139,
    12716824298332555511,
    11532113271873206481,
    12564094321969352789,
    15507575045155584999,
    2784120650186003797,
    5084942882921759534,
    17528201953969885880,
    10923030647297028485,
    14943941581161292265,
    14609676331640639539,
    3898868205910031372,
    4935325965970077350,
    2124818586584346661,
    3846308769820939197,
    9124554390704247095,
    13810931319198570049,
    5556017427085859088,
    3607689805134148873,
    14565746622701947943,
    10630369833148198698,
    1850316973974613258,
    9595947257035003053,
    16692635458933923107,
    5441658035122097711,
    1239681448922249718,
    5501142075720065187,
    10417795647355911166,
    1925426848791017950,
    7375871286148925589,
    6345922601848223359,
    8965058185873473145,
    12241962757148709948,
    13372673820764398618,
    12765345158101433462,
    15416197037046472523,
    13255387699500639618,
    10008628080235494487,
    15377968611281719727,
    16172480350713031325,
    10812925925425749461,
    8745318615734688775,
    11104318082259223062,
    17913620314821888920,
    7477582938474394160,
    9843777271377614994,
    7143102869113411449,
    117909998360076023,
    7310519546770554314,
    4594993614977911315,
    494682495079165897,
    17708645263947938637,
    10132751364500752286,
    12307401806540244524,
    15421408171136710412,
    8361567315363373917,
    1624042005697669843,
    6723490484436266773,
    3796351448347962760,
    5215217011607215272,
    4006411668331993346,
    5387255947144076268,
    8348167373879803446,
    14063849750064682422,
    15409331553451023721,
    6203764727105062691,
    8390115621407180742,
    3377472289247522847,
    16892774193676759684,
    10853751081121399716,
    16814693921790903822,
    9490756219032028050,
    16273723262442524096,
    7229780606551333583,
    4624670392319677129,
    2498830983904041799,
    820863627193674719,
    5896574816396663814,
    1879044400830519744,
    573641893278432540,
    5735134719617107990,
    5523650842278962361,
    16228282334138845724,
    14399320312959073928,
    2788900230813534022,
    3190159586169038948,
    1410161031943383529,
    6365423128703904815,
    7391391683530556223,
    14169857068168344484,
    13500947254902519310,
    5889217303592112556,
    5638981626968228066,
    6962016858407277311,
    10507529353367326996,
    14655455177412159181,
    2220519563518414760,
    6625898751778533106,
    17159310088224882532,
    10012862974640610393,
    15132438767992540213,
    18142982505546284205,
    1849719722507022664,
    10876085271609709423,
    17612484524871752417,
    10428882164954003932,
    10984558939337331211,
    7986280881396347334,
    17006332837835162053,
    7153609822704690146,
    127572524604254828,
    536460161268188931,
    3548136592448894611,
    7554734756490458509,
    4659093581776281998,
    1880517268588156503,
    3733795162595436239,
    8596388765738371267,
    1589023876783782539,
    9012605377614897804,
    4811929172295164673,
    3798896172699948105,
    5609185555529512039,
    15171382909177236943,
    7219986147183357524,
    4632512099743032914,
    2506954163889237980,
    12506045503919279611,
    17815952823918426658,
    10657824576214440223,
    14576573252210352034,
    1940028400060977312,
    9095747059440104349,
    7947694985435005883,
    814863606320390771,
    14017958901429694815,
    17392922443957185561,
    16966907536218115187,
    9940869550918714877,
    17087739452462671552,
    13227992311982166297,
    9980564170236840140,
    2632927176412333124,
    9189768621047260916,
    1972107765324924361,
    8781409191099583559,
    2496684192381216719,
    10271164490694653123,
    5840127921469864017,
    18239804724097313629,
    11475977418424050959,
    6690382550769066737,
    5821784449729458190,
    594139997176766562,
    11822599421710857070,
    13540445631247819296,
    5368154562800340114,
    16367729688370435011,
    6883649927396557469,
    11488844312329275667,
    18235781202889492289,
    10046837550361374729,
    8779333870718251215,
    17767384444789871336,
    1318851839365208348,
    8128434862120489106,
    4255401222411522584,
    2663075278570275670,
    174578116432856977,
    3259343910983114806,
    15623263912408904159,
    2626877377237769696,
    9515350929108148798,
    16202712370372034819,
    12028863495256792974,
    13648476465921662479,
    8393486497310199691,
    16112537663714108276,
    1987884969461776132,
    455875678367446180,
    12285564866094573480,
    5317022433974239456,
    13277786815818868210,
    13825415530358813111,
    11215872084449147313,
    5180219261666240266,
    9149643991396827856,
    12230855309995485379,
    784080618488139035,
    5955753608875033357,
    15836908932426965548,
    8639171138427547936,
    15148475879719138399,
    2460486254312900129,
    4144655871596044683,
    13064086679501624775,
    14065865857393386127,
    11224680323166810851,
    13816310661738117861,
    621517698281028155,
    2407729912624794275,
    7042168835555414785,
    14814115548230156301,
    6082832404794173274,
    2456014580021514258,
    8854192377045844094,
    4898135518081213860,
    14701210947106833508,
    1930687470382087575,
    773028963840757169,
    18102129392820938915,
    8159151066309997777,
    6409407589596382281,
    400884957527538639,
    14792783944149257807,
    9695658459138657161,
    16030572558305614821,
    7551712828640999900,
    15931378191024449501,
    13441921271545584713,
    17387118883243040451,
    3365219960964500243,
    5535408635129742657,
    14337970988274911073,
    9493223992356768773,
    13449843809464248799,
    15574838519547715665,
    16997532103936023809,
    1116263164788914142,
    6307342261383227314,
    16312145003130084592,
    9287515097223615870,
    9578159415608354565,
    16242673043706270826,
    6630055441019712746,
    12507736304588380760,
    17599432234149602675,
    6021771515660378379,
    18427922552593740295,
    11330671143581285257,
    9856960637886766939,
    16919804030488372626,
    11603625038726406228,
    13604707229940984820,
    14605924293451914428,
    1705893310084939473,
    985374859591407666,
    10262581462319710840,
    17035373634901227697,
    8596377889642483217,
    18375311666390196669,
    2080287736962158874,
    13751515520003458379,
    5054634196121320949,
    7088345947563980275,
    4102288354775515690,
    8298202162043488236,
    14337443291227100794,
    4888268987300499755,
    13474765623251195168,
    14736041959431430760,
    10822269223629608369,
    14177974731486437935,
    8148065836090504121,
    11490423249143425549,
    7625411518246032852,
    11365744733159498642,
    11290581412989260506,
    9328583904834476410,
    11263199725067670236,
    13988729528399536654,
    11539416335872252103,
    17259812653811753147,
    17328778445238940149,
    4481822567907552427,
    1005725039290782203,
    7881936438839218858,
    201410748924614038,
    16424515370155171523,
    13482216544865619631,
    10719372445666092236,
    9817381590963919276,
    16878248877694247781,
    10298990876368502872,
    5527716500176318938,
    16242416719264767244,
    14194026195152791169,
    9986918399749887050,
    8149934668647709690,
    3021791482856467350,
    10807937413421838,
    13134456721387834367,
    15777672281390929304,
    7905515280634426372,
    14379498606938999888,
    8776875343731539632,
    5765824731726827048,
    16021758598593016044,
    10315197025830579990,
    5999282827792448497,
    12062269562668274453,
    3196168567082447740,
    13977564178192374929,
    3773029731546022151,
    6260001551858124937,
    9157529614314649617,
    11420019254186424015,
    7380538612650570059,
    10944037067314271321,
    16682312797082115338,
    6566533404290616404,
    1217766540973984146,
    11905149499842058244,
    9971017675435147171,
    5763425999947110083,
    1534410233460662554,
    6812875977056806108,
    16959928141619295562,
    7811536578187585504,
    4282570429189317071,
    9596391587638252335,
    14785670320794959061,
    16978484159166772597,
    17043006690558030466,
    12052634434453108548,
    3290886281624663181,
    9450998569227475547,
    17055044727977669291,
    17178805401807719801,
    16567040179144051991,
    1469304040187734021,
    1577128329768918518,
    6783677491563337776,
    6229665423932502443,
    8387141467224968358,
    106734389832889622,
    9292925837851093817,
    17764467659015511153,
    10891361639264929368,
    4254040244723193172,
    6416695397077872541,
    5836106574336841219,
    4302842396662450517,
    3396177806285817521,
    3012321922967175252,
    7464240042575778466,
    3193383801394763620,
    7393058808575917786,
    18138467320176121741,
    10216358625106993349,
    9717933925297207683,
    13414667382994851087,
    15228373003019472913,
    15758240775568638555,
    8546711528577601879,
    5986418974748535565,
    3573475348327129731,
    3841950065377556159,
    6328359608434966321,
    8526086492663334463,
    3234185890658578918,
    12018188311424840646,
    17268515516255073947,
    15804056190839667060,
    867152523432266699,
    10897582160542905328,
    14811039207089734697,
    5095182569823609920,
    7489714243360464991,
    4830440040182234664,
    7692360418037026480,
    631924982743282809,
    3697403346760135649,
    15355978266024846367,
    16369353523535510438,
    15924724487281687083,
    16200411699075490679,
    15543101027114639084,
    6241119321880937162,
    3409876793996014683,
    5607492064978075093,
    1809072470152453198,
    10946037672871696432,
    7731646516418589711,
    11942812873952572752,
    11005533877979934589,
    3789500241632803953,
    14574828637189847228,
    6889890331914691145,
    10758936690987554373,
    3486780319837796435,
    13526745055842786089,
    3402514277091800570,
    16001413651502837548,
    17504089840730046426,
    10405712393694934612,
    13376094271177557708,
    15926304183205463675,
    1606567893618844826,
    9689020806012098933,
    9500740207549282462,
    13456946602040139076,
    13471077933899985240,
    11423824735760103774,
    10302234473033877880,
    13249914357175397538,
    2380459476430531609,
    8915310711813839808,
    15503698855740130077,
    7131948796557677195,
    3419491932984527987,
    9464588873324142463,
    2448939131484544877,
    1763445588131105435,
    4644871117665407587,
    10101140729620279992,
    10008102812363101935,
    16782137468508531750,
    4352926884311763600,
    5748181958630452945,
    14726490944117089462,
    2114509712328283677,
    15534335132283750604,
    1573771222275021881,
    7651235903971252627,
    5577692268876194266,
    4258428834110494354,
    13221025483697442911,
    9329378526794192373,
    2544131547760387438,
    3770495565994484623,
    17113346624945057,
    5475162320015048295,
    1246146399983438270,
    18263607695584244698,
    5861961356783214806,
    3736402717762742616,
    18172535717873360152,
    11997954889483037377,
    11236294895381374549,
    14058755472277321273,
    13071047535362703729,
    17535245790609131421,
    14573771982934221383,
    10444108069417325982,
    15796252086646067288,
    12382577934068736844,
    8154077062853121399,
    7322416731076766396,
    16333307698709542934,
    10583050012836245083,
    1824039641469609716,
    17430935341157789948,
    14858293913860436337,
    640062580004141259,
    14638646978223626995,
    7424865111080143359,
    9018708013089304226,
    15721322923169824735,
    9510753136429095968,
    13467239346033113594,
    4857983370392805963,
    17292342686646794902,
    10194248210848674584,
    7886729403206697600,
    3856954479850377964,
    6839008707705128325,
    4341042953002168214,
    10381636292104092826,
    10647784705220490164,
    236758899109057676,
    14109009130175877738,
    15867968243872564004,
    17726084155001402300,
    3529222743724229947,
    16821130902033405287,
    11461558288637567535,
    7583869667045639686,
    2519819136234374080,
    4655071980786371986,
    17057000861863485086,
    14274021503301618087,
    10218915557507484769,
    15162850761583445030,
    17732416100815752224,
    14538304314762759597,
    9368909854121412715,
    8821212353387467257,
    401581181276394579,
    16887793817486073496,
    1874097510656146723,
    4587143634163892736,
    1612609999252845208,
    14060608493725374909,
    17862961743075358608,
    15614614101573275703,
    9368292449092904942,
    6875921772410831284,
    547683623843320280,
    7155950521546453733,
    9412286952873728031,
    14311710262294574074,
    9998433336591985843,
    206434009972158182,
    16293539418804748680,
    3044288001100365810,
    1379999851012184616,
    13598591307048427334,
    6713545872139127694,
    9830015256439988395,
    7277352390313221660,
    7070584565915263613,
    1187120833840130263,
    7351717358863425439,
    2488955326174419196,
    1748818877343803280,
    16571284164712791634,
    13432251053911166574,
    6865067583773044047,
    4510878545161009334,
    1296422865329871594,
    3326345360717290343,
    231746962652086524,
    12859118138876584355,
    4574930481807056452,
    17539583469830913603,
    5076119337430502804,
    18073939167309176010,
    14108162084382832912,
    17023790542112845118,
    4670548792845088227,
    7500211640634152914,
    6126945430170324487,
    5768487273191646197,
    18167881778414612729,
    5327967726447944047,
    7720560358929787931,
    14341836267358666519,
    17932257123476373146,
    10278101927458228690,
    14689703808795013971,
    12180898299520522560,
    12554766074872505550,
    11928892656099669657,
    16725007740628082116,
    12566611526535114507,
    16547054040497957581,
    8473415127162885902,
    15509228194897451710,
    13094313143054226224,
    12721272011366945980,
    8166274366521884217,
    15516486006216808752,
    17802029985695293272,
    10967591826142561118,
    12398494279515613357,
    17771433137277050740,
    13648074182689807026,
    16100605875683754481,
    4716039780131012121,
    771767351027121119,
    9716411219861355794,
    4157767257031075366,
    1943692082394517899,
    17621895419073764669,
    10401983405587660288,
    3337792263561792719,
    11408675169303958346,
    12934738474176213392,
    10951094084208621974,
    14920415931945724410,
    16856180077029629294,
    11685236950640941224,
    15848267312216110037,
    3061792842046084807,
    18298991402266869917,
    7110994072586484021,
    33815279567213350,
    12417728699130396714,
    12808344252869440896,
    10921807641177748530,
    11155622121779021283,
    1555044317357664196,
    16887453930127300606,
    4786015769226768324,
    1254584602543401383,
    78911631127271809,
    4035679323955575899,
    10338250268034488710,
    15404645312399470175,
    12762782447536573794,
    9849834665655352300,
    13215242118027278945,
    15504637062599317002,
    17044810750908267875,
    15389521555748493497,
    16976038810462702511,
    7692105998755485577,
    2213637255231064165,
    11680669576364595515,
    14325065688154798598,
    15473713980641413664,
    11347610365183021030,
    7256172747166681525,
    289490902650374153,
    6697300423649794149,
    18077950596589891798,
    9668874303582552566,
    11564349811256311876,
    5584836742762584011,
    16166915560841589242,
    3094405937411943055,
    3839730952922980619,
    5881629346346369590,
    17095038525919712104,
    9913210295627231590,
    17096794975324983899,
    6383274273971160013,
    7315148673788025368,
    8261436743904736058,
    16035634192306921526,
    3232346983585495093,
    3817659568285212137,
    4792963333716437665,
    1219104467914005405,
    8927579820967434189,
    5921327645723223893,
    2086275624907153481,
    3576932888828070042,
    6157856436366125991,
    18071905658671036317,
    12757702547963688215,
    11389366151249275801,
    4268054209398028616,
    13329510241421697794,
    2307085091828886038,
    10099578411108120858,
    3484274154918333117,
    7076711021829929857,
    3022729326734179911,
    8305333556848740766,
    13979718251878017217,
    6284061998391827185,
    3739027109643549132,
    2563329810975754730,
    17154199776949577027,
    13234585775574042190,
    18189426209423721352,
    3240034672399190004,
    3269719776254526222,
    10830575509910736505,
    17514149518613891800,
    2042219050032963706,
    5773508306119852119,
    13011604175091861693,
    17034703130668947665,
    11080644551154420211,
    1121110585866947376,
    4650647198418198551,
    10989866263065607812,
    7570038990103763862,
    9680354409970839860,
    402866891685243774,
    2511852875401079834,
    12510998966330199613,
    2449256233491204336,
    10218950359366589436,
    14263900460605728737,
    17800362835417014973,
    12768863807170797123,
    2096598653618818383,
    13556572034930111062,
    16614345152847955343,
    14539237220521271848,
    12616552424520513551,
    10352641177705714630,
    14153858344252462592,
    12349969812848936378,
    10716232703236391991,
    4078911582423652155,
    14596096499164131340,
    10554798566760964192,
    108914574210167521,
    2161640540796561805,
    976937140258199979,
    5064759883935456365,
    16870494819596957142,
    8494360077800276133,
    15208918600038927139,
    16864058252891565817,
    12734410850268616992,
    4611031351478663943,
    6746319672201524565,
    4127959935889430120,
    9617553539672401861,
    3506509355940083283,
    9104236066516083619,
    2079324669526256173,
    2001764336978854475,
    8765591907934243865,
    4664715630133111235,
    10508917069280029291,
    292403887320723614,
    6056421654640363136,
    4420257129314322252,
    8232019912704269494,
    12605148523155489731,
    17797353942037370837,
    5728728592851080294,
    14737159555976521085,
    12540106291929410803,
    1463064808009311011,
    12173126429071372925,
    10517704749334763431,
    10719342418808738086,
    2383309589446787248,
    7697870095789639030,
    3564705694548515903,
    11378864380222571746,
    2399187769071097276,
    613133882233726244,
    5923032404031738621,
    15639891778710942356,
    16727626227889745072,
    4176568207583368251,
    6710626121798694662,
    4585630157991624328,
    10632979967524892036,
    214115948633866478,
    4677724050337451571,
    1331582217525746094,
    12275180637495200546,
    17120956878607672993,
    15026411090253594484,
    13749971462392528694,
    13987832494789479958,
    6924103722251262535,
    14155504434956911144,
    11575583683140395285,
    18020063833008674508,
    14295773750182330940,
    7294121976266942436,
    2707352980000179931,
    6713002040110769665,
    1700143708872971956,
    9925638446234424791,
    10290058549826419961,
    11607193842927425178,
    13564696393271252239,
    10971909504543726857,
    10959265764062914192,
    10739148065438144483,
    3409959319321917308,
    15705914341186557105,
    17893884198125981985,
    2002602083938068019,
    12545724195210327973,
    10257329063059284177,
    13168768482446082988,
    6577252539108888617,
    4724013276711134604,
    12111426070618678721,
    304254054717159117,
    13285998318263463983,
    1962582236376978935,
    12074345193555202121,
    13288905331177013511,
    16080989820499439560,
    12017871160803871149,
    1135759011302943891,
    11104462029226866278,
    15734307029106867381,
    38350856266368378,
    2266194483135812660,
    17221435779120394534,
    2513326486595645429,
    11404056892846112427,
    1227023043706543244,
    16806400319826221510,
    10070001185079480392,
    1505209896814733792,
    12389901792402933320,
    9739564889313533232,
    353850242484848506,
    11247361495005040238,
    11893833936793562036,
    6184714463884458319,
    3531470021836102258,
    4099618888352581688,
    2856311600317355349,
    13117059400463830213,
    18288638045106112771,
    5854119615285400653,
    5010650531606817670,
    13516157052380498405,
    4454324299379751246,
    9800120908398373080,
    14107074390513446174,
    11183432225071218034,
    1474810115980894037,
    1000582406689568819,
    8025776102344536509,
    12506628829038655571,
    14503474158323948398,
    10661758044822072503,
    13943201102797237826,
    6078113658491647159,
    9419366635601224222,
    3868812953107837283,
    6319963721614019498,
    17715777341246973080,
    6482814040596969364,
    16525375305305597642,
    7088681894128759904,
    7113871460208849331,
    15638025043257553995,
    8442538773656741703,
    13904892556670973649,
    9614979019381082977,
    13255356988158547613,
    9475040678543484016,
    6915860851148101330,
    18263189918208594116,
    15790057236302696822,
    7041940431546720909,
    3060696424200036359,
    1219477642189517943,
    10824624796278489260,
    3395499776866903001,
    17754053480952774574,
    7970411228753983162,
    14028737494014010806,
    9839927678237675581,
    14071797710315509243,
    1294531792378286359,
    2949537305147618509,
    11898135551173530886,
    6465722041309772089,
    8787773856042995301,
    4056803336250005504,
    10748042713495511774,
    6168829909000442045,
    7819888724450784615,
    11916372594689379923,
    5944422119717260343,
    8347611700030507818,
    1287292795797795299,
    8070770771114697754,
    16810203090726818024,
    5168867925739509923,
    2552777589822061470,
    14563947908458323524,
    10425012793707014557,
    4340535224596984128,
    6635111146235080610,
    5887061782577326237,
    10362279090601528018,
    16940568866985470447,
    99596731149508525,
    12092492413006187588,
    864032946528837448,
    17248716800469036896,
    7035167882110228641,
    14584402770732451350,
    6141350616521031614,
    14827523992877594282,
    16412039558549720450,
    8979904782930310186,
    17767523128026077923,
    15711090623213334825,
    10506443621245334767,
    16757667554455446279,
    4945991971130126347,
    1646665843463440284,
    6964018832793497812,
    200193930373332614,
    12604374644549856650,
    12089283163497366531,
    14215122177580273037,
    6492504675958699045,
    7230361776833202895,
    11058300783361317575,
    1599589713987762400,
    11914036671954417414,
    18439906939596783839,
    8678790066091418765,
    12653832122900993845,
    10079064031534767923,
    673591710848305782,
    17930206080859385700,
    18319534126972460782,
    5913384222903766498,
    16148968932965421457,
    2350814344949175257,
    9409810848404096904,
    15535586035867981178,
    11666142551409107506,
    6419962416572783413,
    1029708352127344162,
    7682166819233955103,
    7027248983119311937,
    19702247200561938,
    8644968380766284368,
    4138926243600574332,
    6748260527311176769,
    11601042299501348640,
    10243281952181066733,
    18290678185492419284,
    15710099204706441110,
    16365164873448092236,
    11516771890805801836,
    9857856837116298637,
    9568060537964784700,
    13090753854661282451,
    9327667400345741328,
    1024168363215964727,
    13539902819098909601,
    16082826712165234844,
    14511073688397111250,
    10651343185010354187,
    17800447285245971254,
    4816056838522542752,
    7243051814189472530,
    3048345011299369521,
    5664171449127248140,
    18107526865487835290,
    14293168926010403151,
    7064950811997687028,
    8735933220696443803,
    4587524999703788582,
    14200924823007221002,
    8885696281797535722,
    14360596152848541963,
    2937653973463340089,
    15194958644330644718,
    5841133517731394056,
    4148372321627316384,
    6721396951790535581,
    15622464111745148954,
    4331258753861510431,
    11690645466839500143,
    15313856975636734153,
    7216938200219676543,
    67975386897353794,
    13307180205379137095,
    3551288138787953245,
    13897653332238726383,
    9403156232168480707,
    3028430365500278296,
    14163945482480233360,
    13523349473645256187,
    16287781360028499702,
    8975864681557495388,
    12531752144704911712,
    17823319600052080313,
    9124931374932000121,
    2064770525496313776,
    8791835273707283275,
    5315782343972331035,
    16261064592013179254,
    15679383807564973606,
    4220645023880134641,
    1391037559245975258,
    17458167154866019606,
    4379037861034251191,
    7219440122712294363,
    16991893772784792582,
    4193719817675048057,
    6838115405525903172,
    15170273556140188041,
    17264252009389897820,
    17602710457011439420,
    7940482966039633178,
    3144723374231091420,
    11641595347081050876,
    3309371251538574385,
    9349964558965043005,
    4235614396095884874,
    14648551125460814680,
    12025841415615287321,
    14161092285507313227,
    1705295233172483785,
    3345224817990934992,
    1277048666461790301,
    12890951193381913989,
    15981121917721237523,
    10810997674899544729,
    12473098020126695082,
    10471258017223498028,
    16938403853579532520,
    5982891509647646907,
    7932759751648180590,
    15256117984044610885,
    13770598684053398375,
    17643632358256746989,
    14625875497272266480,
    7409842930908798460,
    3071847863470938245,
    10767986419925668106,
    16044339726798329036,
    12605567177559967810,
    12809939457391295402,
    15352987038842853527,
    18132810390693524670,
    13680696975784837659,
    3471130553952609232,
    6114073526475588461,
    3240687956045705054,
    4512787240658335408,
    10082926647982050641,
    5358841476061512039,
    10323174051085299303,
    7911640123786105555,
    14456921387020430740,
    8566630667810234228,
    3800456806711150299,
    9098205735735539661,
    17672310839136669510,
    8388079580906378080,
    10993691244361225687,
    7941370060062160410,
    5733131532974453642,
    7375143929130615570,
    8913361007974002410,
    2028719171414796643,
    13359681040740544002,
    11007459540579478418,
    2156116096892382403,
    5602040923929734498,
    689987528756891038,
    10322041659794523606,
    14979958160128750630,
    16585185128696628414,
    1095932654220426634,
    7244478404912208720,
    3650505556182597212,
    1835040756513337559,
    14494074011638048019,
    5090517263550270766,
    11407915794111552777,
    18172903641823335259,
    9818696961148578214,
    10966954916283488640,
    3755707149101025932,
    2650147455830999932,
    11957039340025110362,
    4194596934476782353,
    7886386796981803391,
    11564109355568704324,
    2591145172661417338,
    12124317198582083592,
    4084528520122757984,
    4604938738987163832,
    9460513809831637478,
    13426279891126408252,
    6675857964384783481,
    17332641747189948277,
    11504601518509982153,
    4440854956884219288,
    4856974337364623357,
    2662553128643972476,
    4622243228414586433,
    15561596626398817910,
    10179675838851029423,
    16319294553268043285,
    10004978516471366742,
    3455923673276099474,
    5581764199183295004,
    8141122870471982018,
    17425496652917860324,
    15528873257768085078,
    9098350305297398437,
    3780987966638908288,
    11646655162818345333,
    5831326014010505886,
    14189389941560386887,
    16736112342272725325,
    18118312446602490409,
    13899997999699620317,
    3847218937880981977,
    9125263013806076959,
    13281716254405664418,
    653503408401268733,
    13338677304963880609,
    8031597629217050997,
    13073306858386643692,
    11140986515164652219,
    13039974583830285073,
    6945446039173184794,
    12065106026517081840,
    385568488629952107,
    1316247270337370250,
    12602839386061244884,
    713465963676532960,
    14606839850079333846,
    9179493995387206720,
    17302836919340998247,
    15193035882897896878,
    16987935002137858164,
    1206383425569155798,
    13682233710466353269,
    15578319535930780017,
    17699745202635500062,
    6453272169806936338,
    406922766080362878,
    12216346855541283442,
    3668375909535066954,
    10195832205708845814,
    8562474221635992269,
    15778506793913657793,
    2432522331565800903,
    11595316381499269601,
    18129668155468552358,
    4483634877819481225,
    13245315563037884286,
    11287729203434077000,
    14273751601075260561,
    7654445705016369053,
    8530800523180241736,
    8202909464363247363,
    15995121312960705551,
    13797224568343068033,
    4375621856615065547,
    1925736958586381995,
    11058964704108575969,
    4866869449119409559,
    17229417398089940298,
    5861623502939328426,
    8133387775849861972,
    5316037933490730638,
    14241545867595411523,
    10260116816811668869,
    17005962215251786711,
    7361798786944760311,
    4945389071801353476,
    10875818228516329993,
    1587228932001728003,
    12125234041179731171,
    6976055707707694852,
    9505734599721374850,
    3907665502332601620,
    5362898468574552977,
    120388684384675415,
    17278573942102731899,
    8764058510879311762,
    15387868008646022302,
    6480344219301057107,
    1308768323841087381,
    10161436445069404469,
    2389208251520727609,
    17232219506053116594,
    6258440646449764891,
    3628282196144879498,
    13290460160270316970,
    14717394545816002055,
    7011364861420919536,
    10946378498804169021,
    1476796074816654766,
    5515589055677832642,
    7296576388483018074,
    15255835197718779853,
    17830122605782993741,
    14873304113296567959,
    14227382474359678921,
    10270750428488978361,
    3027049345308562170,
    12750299709868458518,
    7928867941991555984,
    2724567836130633302,
    11623445114110368740,
    9837883775201160060,
    14261878245482145662,
    4127600175614921537,
    17850326082440235550,
    6599348348929399058,
    725348145318294044,
    1475392405271668436,
    11340194021229063321,
    422410045306031998,
    6498829153885656273,
    5601255310949645428,
    11409312286617562059,
    10120342961911318418,
    4681135377637996190,
    5802831402057238200,
    15880054537749151223,
    12757484661513689866,
    4221095405853080755,
    5799391996025455912,
    7593721018421158130,
    1561944995514339794,
    13232908999923074435,
    299670530282740552,
    11201828960264544992,
    6104050911280647038,
    14693470019022484182,
    12198168111408114787,
    15476521588534675995,
    3919264363565208068,
    2204092088341624577,
    2209232068717052613,
    4104012636403101315,
    6917965047907204847,
    3631963607896785492,
    13692400268956009625,
    3379489882048833664,
    5345935946027813821,
    2265633585423510750,
    4254304803765085400,
    15299848316601798266,
    5408746391810815954,
    9815654380588668806,
    3790427705523848920,
    179494913323156298,
    15744090449552774451,
    16386342409447469289,
    6652010764190835449,
    4817855034591366862,
    656143185311140616,
    9455096735446118563,
    1223543939061255474,
    15175212498056176703,
    9514436667058252359,
    7104655642266353304,
    10017672294028996451,
    2821904584876506223,
    4167104295672255165,
    13482871960634273201,
    14186781286880438992,
    9009473060196280620,
    2263627713345316734,
    3526592129800781228,
    7523219853545479401,
    3058800941898127182,
    1399157147912745620,
    7883723670353513522,
    15117146499641793900,
    18346918855244446411,
    1590766620278564289,
    6053301615010229020,
    793644068825472836,
    1906263855834230107,
    17139808536024685759,
    17300911493796730655,
    10723043335260140578,
    7593890158603327859,
    15124794819128603229,
    8918729130143066900,
    10998675647846662693,
    10206241122002439250,
    2392082195596694671,
    8899955666042446678,
    7944130999709705968,
    3787427912347619625,
    10224645888662564585,
    4867187765207640153,
    2417997709392794256,
    7658678461027194710,
    16032450554766326011,
    16290557988565829577,
    16069655983829327388,
    11419730791404576181,
    5404665976373416514,
    900433828278418582,
    9231399693823785563,
    5691781522940073210,
    13234489555776364557,
    14782075238387908777,
    9719639345363895663,
    1771394918038206174,
    11463266312459897646,
    4790573886132000981,
    14503088008045508796,
    6722909333488566037,
    805900417815918063,
    1188960164209130596,
    6577344085028288930,
    7988348540604670759,
    13660680295963538257,
    3924036815788420198,
    3987882768231254938,
    327706274472449159,
    11933729133323550657,
    15763897001332530672,
    1769746194953069049,
    7998485784620467898,
    5100817946078180898,
    2642398636712097904,
    10789914427783803820,
    12231618414150512847,
    17514477747574785814,
    4814985509871957839,
    14093603812395081492,
    8050757574496390168,
    3702721463409557017,
    1353314868712225673,
    16944560256366068286,
    11592345320758970360,
    7272174451978062818,
    6769527483192356408,
    5576399294523104701,
    14619713731073892429,
    9404134855729577338,
    10900553447101454526,
    4629773106696906692,
    18222191163222374042,
    7506493849204317169,
    9682611539235112002,
    10377832752442710543,
    2252200314301331496,
    15338377887158183570,
    12663438497511797402,
    11132241301965698701,
    17941825011919945475,
    5432476731011479941,
    12351576117797411315,
    14969777449547971278,
    10901728672421735074,
    15892599552612872036,
    13964486002420297551,
    9947520740063177353,
    10806296518867270599,
    3578431406683037057,
    1572409893251093895,
    15296468258420246735,
    6558335559370697448,
    15519859130714680357,
    12949913715312889624,
    1695114355555584437,
    8061942141546561894,
    5055342521234715134,
    17664414001258977004,
    10927731899431804770,
    1470381308387444008,
    10301812921425067798,
    186084174112913883,
    4291050634899889153,
    5547729026145568353,
    16200009173420021101,
    7810591013387931795,
    6030486913027428638,
    4639448334475535995,
    7480884865911187836,
    17573126033001851690,
    14376776540452150864,
    12157516418623240682,
    10798125631262459703,
    4826887740496398721,
    10056301437945415257,
    16856192247218210827,
    11685252148249251277,
    9307952619290365201,
    14621814665229813975,
    10402051132861087502,
    16583530927427373126,
    17155190637725254956,
    3824697125646996739,
    3093710998787441741,
    9691370295844314256,
    16914311928070970888,
    13614605466211518276,
    1214928657083257928,
    5199869885768074276,
    4491968972397491052,
    1502422360194593218,
    17320012215383762901,
    6686916860983869192,
    16172086761834524246,
    14530619749870362318,
    14593814352206823882,
    7398046876593541830,
    1901734874259530726,
    7322254688864347724,
    6311668949176654989,
    18088203618496463952,
    7989327426647924389,
    4535700207522741591,
    10548471986859648394,
    3369197400947127508,
    2106601980347756488,
    9205260767470757446,
    3349820123476066053,
    9434279857063045080,
    14443201734367852352,
    15564995348248643430,
    2673072847228001123,
    17431717001195163839,
    13327608742334849742,
    7025961161812894405,
    8241529728026009030,
    6599498638080224606,
    244263358078188850,
    4213690819604551912,
    3177480157288966274,
    1513605425519512920,
    14398952898983890677,
    11963195402911225797,
    10894353428460939785,
    12549634386137927635,
    2780259256168152670,
    15939829109867734596,
    7188017607468257445,
    5215680598607108371,
    17306824222049495014,
    11954100410631682722,
    18235237403031481038,
    12118701494125869083,
    9170554992627194030,
    5770209491857981193,
    12170333396066045848,
    14251275448316714452,
    1013519358920066007,
    13753579427383752369,
    12649476645985183868,
    3722656923817842062,
    8678132875983933512,
    429073072216595696,
    4188351428250246525,
    1714553009096690250,
    13288800165078437322,
    14169512918317202666,
    13501148468170619200,
    5990759962182561411,
    3576127224713717517,
    7370903475870881050,
    1911144801550738490,
    9644881668550663570,
    566686840458171716,
    7304493484978330826,
    13048770686855201090,
    1839496014657722783,
    8238941641131320678,
    7903119997950112808,
    12348134640786658319,
    1806088404443432012,
    8003403512921605500,
    5104873623960925668,
    7079714078459847131,
    474841957321442022,
    15277058696952689166,
    13141805646308850780,
    15183845273509093217,
    9801880386316036280,
    7117516606032745116,
    18273988188309629748,
    5901284570906887291,
    9761373477673014422,
    8280528464168030480,
    18279622032977572663,
    13631678447821394001,
    2035443024384739668,
    12144459086824327051,
    1453450464667309781,
    5227835471713782303,
    3710743007809109823,
    3013810724231808618,
    1509433530912531673,
    8080686451943802852,
    16666052396831120127,
    17117667037522348526,
    4848755388084728939,
    5119373839262028082,
    14885465944935735587,
    9607496360869192933,
    4062286879983548063,
    18204283965595795367,
    2144233483208233375,
    13735134229658481735,
    16602822383653151787,
    5351844651685787431,
    10762143546523673739,
    1882165095459014316,
    5114716165391719366,
    17198672257876179766,
    896043349911895888,
    6181796070758292573,
    10865147662901449380,
    7938927314636865323,
    17223001551014224653,
    10075305426888589360,
    8735365893512554652,
    13758587397225181074,
    10772027906116753496,
    12761164265453833474,
    14832525054322228258,
    12839262129004591496,
    7214751272850604866,
    1619520317820127720,
    5726505907821203307,
    17046028772363402517,
    13132272529776462540,
    10976595527870381881,
    18074972579096360631,
    15384452526544015632,
    4636862343996509157,
    1178563341076889083,
    6037978056909362131,
    3387903976586502899,
    14703868495596837668,
    5732852478287196771,
    11155234143185705286,
    9089400290469973566,
    9647108909392352088,
    14854570386656368286,
    8251845063202719680,
    11867218288691250048,
    9544337577925286327,
    7257389242886371033,
    7574077662125484508,
    16536929031333244689,
    10593014641025570905,
    14266954997718107864,
    11316291090143647412,
    15478002939419218802,
    14392463811174573601,
    13265243500792043999,
    18041095369736823797,
    6229138008178258169,
    17461058964576084444,
    4902440639244992444,
    2368400302187748481,
    10741025555647892759,
    16053268566290450641,
    2539585701242635001,
    5181571631676154308,
    2242034790936785320,
    2453366771389065341,
    4840772943205613808,
    8637019544974631129,
    977192339719650872,
    11572265927476376507,
    4510121505936824486,
    4333556857170900198,
    7294541006981940278,
    17442480017214189650,
    12376317667019150253,
    11955980206416380418,
    2356019268990981266,
    6664543153222264242,
    8804354350910153728,
    1859315156655615285,
    15278848368035622237,
    13153570928715490515,
    10345641712311042313,
    1308960687696981383,
    8118262233060680713,
    4281204504222819971,
    13421074524311911077,
    2559373498062970904,
    11468065637396738374,
    3420731321474553666,
    10795998168151643696,
    12251844750001215421,
    444672497876223153,
    10321187628847902977,
    7539428483107320271,
    5680456381232542844,
    2928316900210827426,
    14410060727058008576,
    10105429413433659334,
    10197748332298298110,
    6448669244251759421,
    9584567060532413298,
    14445901192229237713,
    1489698200405376517,
    15367319185692478272,
    4181854186849990624,
    7132860720642434956,
    5081877978003124106,
    7465966527713457850,
    6524910844993597295,
    1188937546369017755,
    2983125839795536449,
    4234260281735594083,
    6779559033676473182,
    4357167147333299468,
    18063640474334590852,
    5141439682330959743,
    2561660236773063746,
    4688063437894710732,
    16188908477006165861,
    2384976851787299667,
    9785218654604579500,
    10314955191210990601,
    1830328666427072067,
    1756958090533856383,
    17490036741249598809,
    8971153192171860538,
    11143578447999090359,
    16187775283665106567,
    6793388709061515833,
    17549013126053338331,
    8075510245096877916,
    8064510758250266130,
    16190157438239218741,
    13284232796155628633,
    4759914722389224552,
    1944270851834625028,
    6740342540364264574,
    7702779409387918251,
    10168814261511456643,
    64817214122641140,
    6346605248333387416,
    13303305939396133467,
    10739266073410129271,
    4087900426547395373,
    12777769835326810840,
    12526075207130274525,
    13449227594401511903,
    15886518073112371445,
    1505646987048606738,
    18139523634998556336,
    14025702242420151146,
    9086912868788457385,
    11093119728991583143,
    10098695657762140826,
    15462364545305904451,
    17261347265632788633,
    4553717586444034411,
    154068875125520574,
    12245342759561618785,
    17367533197990343949,
    17220237243998325164,
    9185255686800499438,
    1994634140973498835,
    10938157150044948254,
    12312121400133082827,
    13809107404224848381,
    5178042549494400312,
    5710749633530460110,
    3628383863480588326,
    1010721375968168868,
    18413876122347695798,
    1891064491234755614,
    14602835092186470948,
    2599377627623967392,
    2504525886853600084,
    1954241996866206658,
    8333080527589377761,
    4957881423319680934,
    17901692725490128632,
    12141246586773895831,
    6653212227410192644,
    496638227687235293,
    7075895645140925920,
    10463976906979040705,
    4562377327534782103,
    5910216966040229412,
    4717290384199726482,
    772110261675554584,
    16456481391859108079,
    12655416287692540517,
    7566855636565089407,
    15393487494740512033,
    6549567526363868934,
    8710508363680235907,
    2052425570161302091,
    11382804718820562591,
    5278168403536217384,
    10627460090137109915,
    5712517300868201746,
    4649355994588525774,
    13841089004714559854,
    12649432651036887650,
    13300854205081062531,
    10502984344187589200,
    14499327646428332604,
    4814438574867929792,
    8294835617586965392,
    4096673033838767702,
    17858978689205400908,
    17428939309597084591,
    16134991301707297199,
    16641579461471999093,
    13051030109559786573,
    18363974884240640395,
    3222072171495520814,
    16329582011208389611,
    12671791683195577959,
    6016615226824493656,
    16037098475785530259,
    6310747360995820673,
    10090130245274528151,
    588383537785479010,
    3545352113743362744,
    17540388622759193473,
    5138374134325026388,
    2481072279891226161,
    2142260789921247164,
    2361413625342812777,
    11442787225202261871,
    12093632957448755893,
    14390886325282094818,
    10282737621726954967,
    16890001831376348906,
    6896314495723861884,
    1473734939174875834,
    12325322819235207775,
    14543599135945760032,
    17710094127417146369,
    10900228925305271695,
    918749813125133366,
    10316052223860588988,
    16432532659657940546,
    3661126860894407367,
    11448836208294255051,
    10478166036399549939,
    5826041420173337029,
    13611997227216554572,
    4678407294324582335,
    15986353045794583360,
    17587407505177969624,
    12596740490570321825,
    6889574531169504330,
    9938833668317161567,
    5749320642916201816,
    17724010422695269387,
    16244260064509533167,
    12806289817104474771,
    4538430240514022688,
    8763261671581954636,
    6877408923003236697,
    17037189659957113973,
    9846709324406413128,
    4429149111796123358,
    28760413099695883,
    2946503117506625328,
    6926716364309171933,
    6715413512244844296,
    154998743616551258,
    9360284086381175568,
    13483447920875650740,
    4367970199958389572,
    17052020275933090403,
    9831985082824847710,
    16576142462720580485,
    15597410039038909726,
    17419021064687283590,
    5557075886450785496,
    14276528681142131889,
    14433803770174967714,
    7233815276334865582,
    1778034615756664206,
    10652537038037541878,
    13771613469471796007,
    17422544624496190110,
    11018204156270653092,
    9146893711718085662,
    234661746075175709,
    3712107955030053573,
    5260301333259467584,
    530294597463370166,
    12355481011346094778,
    14109485750127465325,
    6203437065187674520,
    16669122220041365703,
    12993841774634910427,
    5431579900564022099,
    11068292080874012787,
    18252158338889753422,
    8763061358151378381,
    7284964543900204223,
    16336716701118853632,
    648530951811950279,
    6104144261764753688,
    13542305988882023354,
    14051039881042708938,
    4365771137481681616,
    2205530560284930285,
    4219942424815089924,
    7090304599594297704,
    6878263957054566589,
    3870555419596362623,
    6292185047982149610,
    7247972471353242298,
    502127054759656680,
    8108979628064820128,
    2722300754944320345,
    4688606012407349348,
    2560147851522457802,
    9407650715403017177,
    623110891020851705,
    13550011334396563095,
    7581376463097268847,
    13580042913865900362,
    6427378714593011564,
    9178562477103785694,
    13816562915443910428,
    16666599520956086128,
    9483859238041064525,
    14718425404583089243,
    9710765096635144001,
    7560484006306758404,
    3628952107405740253,
    6235912055919340512,
    3871934967021795079,
    13066952347297143669,
    9819660533230764704,
    4701634943568925568,
    3145591210662269001,
    818772257430979288,
    13518670905296754848,
    15854731681949524362,
    8082785757879169670,
    14265754909673808656,
    80292696698602230,
    17578144300176092163,
    349467770220184690,
    14026389256099680585,
    2226570209069831096,
    10053296236107222805,
    13102696872532988094,
    1871986469530338226,
    1478022872943405193,
    3084852324042118755,
    1429432853668557753,
    16152798977650820806,
    14466338826460590874,
    17057863613527948060,
    12991857591015582576,
    8361057750783144797,
    3366701489539515524,
    536218344239789084,
    1984412507662075535,
    6023132628968221045,
    11503123018618757843,
    17828511718613828389,
    13472132875401467024,
    1091937279325734150,
    14166195158365716091,
    18267068702144945057,
    3520330285284530175,
    6741647555580278604,
    12289217641430316816,
    9602994552139476991,
    16686893340992570860,
    456228288010405326,
    14313925463242622333,
    15480051590211319131,
    384036189152706099,
    11283326756325146248,
    17674029558332211564,
    9910647639191892044,
    17757763245233157606,
    14210427511874122075,
    744528163138917868,
    3430955257807877462,
    10545377667761985204,
    4500280996453189048,
    1333806683543098695,
    12075734668501704711,
    7654574537039970898,
    3238072624675925037,
    13419582415772139628,
    3916321657046615713,
    1423674509479447927,
    12638103452136758313,
    3657619653171989399,
    7526329858930080846,
    18399985658600932691,
    5098913116263201205,
    11435290667570205846,
    3612986870213889099,
    3416394169404740740,
    11378692307033683329,
    7499535120054177136,
    920277189248921165,
    1345469192904236736,
    9760970233409012789,
    12320412410712445060,
    17314785157112719197,
    13514486903110247067,
    2722436913488057655,
    12225715141858972666,
    14806920140379697351,
    3728599216037951786,
    13993618912106978360,
    9116283103788626768,
    4808770093843092179,
    17190431405428490719,
    14467660449385027822,
    3430211691266325545,
    5614743643123375276,
    13575649045873867198,
    7619954355346865298,
    9766281860369591248,
    15885055655024763032,
    17496945300294482034,
    7676495561712649425,
    4789293082998663435,
    698515131834313933,
    1970058104698878794,
    805882144868099220,
    9051939124764214119,
    6071720960053626356,
    12273860640156783012,
    10634642585581556426,
    2303496382940910599,
    10426873717033285152,
    2784667192814131052,
    1833383754670937825,
    8002464605265928617,
    12797466282718750259,
    8977722121797618424,
    9639023357479291704,
    8721656201292959489,
    16889946536121585505,
    7287175237176323398,
    15768584153991831101,
    10286020676841566297,
    13233418545176024451,
    15913408617517548792,
    17662592480463025248,
    12107649600342420653,
    6969446500184145977,
    319398119653909252,
    432835672465068796,
    14003707989603499201,
    6187299211682292515,
    1111596680561300730,
    8810695147119335260,
    17654668170188387707,
    9784197126099852012,
    4128550078889936387,
    3266129261629277459,
    17206073369098846957,
    1918179034790339856,
    11399769074577991633,
    2224016844411792788,
    12847911729648448841,
    17327729178561427976,
    14249206970881828519,
    16730961981556371990,
    17599833680611132820,
    14693969864900772889,
    12425225637491267813,
    1210690599545244091,
    149970321525001203,
    1325670789342804949,
    7213272814785301661,
    3722035751888795319,
    4872929185593635733,
    10220250105466108687,
    2397084326557115346,
    12476758403841287455,
    7958844841393820492,
    13455229975274589830,
    17660274617282213899,
    548001086683788593,
    9536977303612436347,
    12779110609233175887,
    15200660629142331007,
    11039344010467908853,
    2840768653216878099,
    16580779786842047045,
    11007030346516384714,
    15206316906105212428,
    10234369073377891488,
    300830797569234560,
    15017339135088948111,
    17246960855753038204,
    6248782086967082002,
    11537136190933917663,
    17104176146889780501,
    7006549071956585432,
    5543672007551302211,
    1227793340795285847,
    13273685255427234522,
    9255340710620045229,
    3790941029296752882,
    5785692345023933391,
    9341391415232028978,
    17435622721773542552,
    15687685377287799149,
    14567468418650492051,
    7658453488966963504,
    3090547575715622927,
    1439482710535378389,
    14053488910707691798,
    15875811575714952888,
    6670647959764476527,
    4054416920605409618,
    3464749129485317931,
    8922230754880402157,
    343559579418772251,
    4970003076406459302,
    2318851573791905947,
    16975176171826832774,
    3117265782293261897,
    5734218094367673716,
    2594989587973605734,
    4671790852706134619,
    15215411754405343839,
    11020135667715078041,
    9844328624314779583,
    3843212511524138989,
    6443037649222996576,
    3374457398949570861,
    6115623216327406464,
    13909069781586930037,
    9315542745418441638,
    5313349338549793885,
    4001236685519426321,
    4753607094811534425,
    16395736994367733426,
    14573843529076379178,
    14456990734080082425,
    16742010082853711515,
    14665724921496005144,
    4954809214975068223,
    17536443410942485077,
    2787727654359681524,
    9980961030079421176,
    350734780563400021,
    5699705602809127059,
    1189085776470201839,
    5346027456778560054,
    14228274210101411857,
    8116905247452909959,
    4999355811914494868,
    8723635201317788599,
    11950900281419942870,
    10967232062293019554,
    18064201740515541548,
    15413378537681233291,
    14729087758055875588,
    8600006568354974610,
    11788844859754381335,
    18117856789579911291,
    12640811653431941137,
    8171012186191207701,
    6886835894584519922,
    12578452342399456351,
    13051582431652584916,
    18367907384175056914,
    3475133032416697698,
    12340268006146061166,
    1458978524355803825,
    11112730163310167751,
    18183025040718927396,
    11863319102967989832,
    11732461761684194649,
    3392778836204898964,
    3691037844494424098,
    11275966508521934303,
    4284608369500086859,
    14349203890283032353,
    10152801899517045479,
    17178557721569649513,
    7177569057947832335,
    15034489495727382339,
    2218874810203594953,
    10584447030724641245,
    9331212029122237983,
    2536433660571147972,
    13849678463286808883,
    8336585498518413443,
    4379308809165965832,
    4848495598467279984,
    13741289634409587152,
    3309887739699649783,
    9350482144431247355,
    8394204833605780480,
    17878246006537257677,
    7908400358702769847,
    683292754008200731,
    1779355497319476391,
    8523520272455408379,
    5927162490360210524,
    4687741061380804123,
    2727206547450719526,
    15564054648002503137,
    16912258044815196453,
    11633653767044662499,
    1791248956140902429,
    7717402797830646360,
    17483169733565069264,
    8472768256896705914,
    3124636154325862723,
    5277903667653084945,
    8539270385756806521,
    15757274348090478197,
    752726031354622059,
    16980927788195543771,
    1884097441387159497,
    18059625282482470678,
    9016307822241071846,
    10890347298759870563,
    5994963074803702832,
    5106467188913114764,
    8141469981941529742,
    6957279024953134248,
    11252605027334758551,
    6895821589711271708,
    4326976919250246878,
    8632241556278958360,
    7041599446286937067,
    5630782768194372319,
    12158419922692905096,
    9262654352130250845,
    15094042629064260471,
    8029017566696816054,
    7631284941100692724,
    8761386396052276434,
    3448482560434531008,
    14006194376661286976,
    4633056790794406877,
    12011569219833820042,
    14008273879974482103,
    1143697905254822106,
    9492820521714316823,
    690820469986931767,
    17569352497049194658,
    15855566752221559489,
    5319796075947663412,
    8477152249640033175,
    11488929794357780632,
    15327864607855224158,
    13391285305935677232,
    3745987186060035344,
    17949245410277505931,
    12889272418526330445,
    2071750213608764203,
    8653277014260420630,
    106513616255805503,
    9318684189323850750,
    1291121590303209038,
    11834063912096049112,
    10305691180705277929,
    7402824900186286855,
    15235088566369925777,
    16856999518582639663,
    4037787594876589257,
    3242870129351646749,
    4427472237649473083,
    579808459723296095,
    15288243011908730552,
    14066764258868000857,
    9193350824041785607,
    15606571364951007283,
    2104381612157967886,
    16999661585242214993,
    12195449018894261153,
    16836732565513929163,
    2874602908944246188,
    11988222610083554151,
    16612148962351171943,
    3756784032065752544,
    5977808843384101511,
    9609277518674378972,
    4782188612449473807,
    15925291095661980839,
    16366970276191853655,
    9241523313979305950,
    8574047228145910816,
    6860328112827315384,
    13979465783069713729,
    820857494354279802,
    5609749420890454723,
    3412134028571898701,
    7249226943994868167,
    6691907850934644685,
    7136572073113529920,
    12562558983609852567,
    12810272473339261644,
    3605123465721505926,
    12748799336339447847,
    12616774389223649641,
    14682017611703456340,
    2087762388863658932,
    8662692321059993528,
    14460704183458400382,
    14354441304418548052,
    11746298374131749820,
    17992901620742202469,
    15892934904586585061,
    13283864458833993944,
    16719913396193393762,
    6101085469755583804,
    2264197393542211834,
    7269564223268771685,
    7213958967487636753,
    18106337390867764773,
    15239033321779180326,
    7497840257084012760,
    760595206246609238,
    786424878502462925,
    3223344028390147199,
    12185811110107906005,
    10574473809223581849,
    2660450188813456344,
    6099262607786646354,
    15620332686775786911,
    6626231975534824632,
    3753824257079513175,
    16895244313086266263,
    11703602742672974957,
    5325104294807518160,
    17673508575703100046,
    6442516698872985986,
    12170526407940348548,
    9308817137847396892,
    12822643603206625888,
    12945867399683265342,
    466408005050778280,
    12018280853122158023,
    17601067180765512124,
    14286206868507207480,
    2443669088548562443,
    16851749038495259901,
    12929250569829673764,
    15545058857431599129,
    8962428023903812603,
    15009495256331200759,
    17264598271674083706,
    15588728647671249051,
    11170981517355180125,
    14119525566528761905,
    8026287124130916588,
    7631968763132972760,
    5293675706992313256,
    8101746212649707122,
    18162262577698676927,
    5597742714401799884,
    2072046395337341517,
    12564848175201533284,
    7205975479934534216,
    8390684653179548270,
    9654666765408950140,
    9396152228981719865,
    8996637552609871415,
    1732956216557479178,
    13073293815116917983,
    17301740370035977674,
    6642987117100498630,
    12004020570088005309,
    7638609326271114410,
    16230712011322723523,
    13013168845878930757,
    15122412453528843168,
    5762026531461755579,
    12589602414834989882,
    6916248669376559313,
    16808610465008268881,
    5999082792468449298,
    18354094184516866705,
    5283419395802315805,
    3627517569021267641,
    1990136483888872993,
    12560421463189925815,
    18217267526223211240,
    13193259009864232750,
    17077903273417964413,
    13111398784642329361,
    1829784808767321036,
    2019517307899277387,
    10525424653166778957,
    1408573523824240921,
    2489540570321843438,
    15824738392443603161,
    12057180752663263737,
    4925060897331904558,
    10725570019750034831,
    4055871035372004690,
    3666011044470528252,
    67849281654171081,
    10355208379855800302,
    15349828541064196151,
    4403116542227812045,
    11336117210540596701,
    14217027146981622298,
    12054108403885684947,
    17968626694322114870,
    12420345624056158033,
    17356929933401762820,
    15648277269736873116,
    7317119083899424194,
    8081335832887906345,
    12506292974412288504,
    4563477917737083525,
    7473841257454631746,
    15689165522351320545,
    13649799853943328476,
    5434361540947299706,
    3681533701961209520,
    6323519151666115981,
    13233923016697493608,
    6770459718763494694,
    4117215235864780315,
    15220640680970480413,
    14933626053953975681,
    13875624245487891367,
    9947569506528396971,
    6627986778543957568,
    4615234679507918947,
    9482123056693127069,
    17301292466174759351,
    14139341856432033415,
    1154829222924437288,
    6182703400797601799,
    6964711110439898961,
    15265629162976697625,
    16125408320562778140,
    15673449235743259672,
    13642968179665723173,
    17071022854387272078,
    10322581329993456525,
    15942315768890590246,
    12632970913033962476,
    17648312167149854912,
    7712778152122673208,
    14621233378964096169,
    15921097360018227288,
    5805535079321585301,
    1969766310039421779,
    8733629144865213697,
    4624581281748805851,
    8564826462467934987,
    10825230973946173775,
    14903894409184182663,
    10817831235209679339,
    145430860535396547,
    12743112729990995136,
    4616270261922573019,
    6883792495092413683,
    8350929121760969488,
    484888022758120274,
    7780013626693201284,
    1672907588263664804,
    3755747587183023899,
    11657258828153179630,
    5803684239183302149,
    9747559085701309639,
    16119408252131570859,
    3453410611325343504,
    8914309852619916918,
    14471133715094594352,
    9077051896448193866,
    4390102850958901068,
    6715014323707734709,
    438670220542737204,
    3503871270172204649,
    1545725894091651606,
    5586267508071099002,
    14364689296186327640,
    13517226027439829880,
    12433095930744485273,
    2023729115077868279,
    6807658720137931597,
    2501428755991665261,
    6723683073299324871,
    11575605406111336606,
    3449990886839855801,
    11110699403746327343,
    14208399121472884595,
    7046876209988407804,
    9101298434990102230,
    897077289217324858,
    6201137098005863084,
    1164535289919825095,
    15230337450662177885,
    6855114792849218004,
    5500229285762670503,
    10586995201310680078,
    13022674289816809860,
    3682886799586298808,
    4362299180455534101,
    4759168273032298942,
    16262565428218247515,
    15612282654379039873,
    16680669576790121740,
    5427158878263397888,
    13674754319786632514,
    2268079205515346223,
    4549523315592529021,
    12995049133070367957,
    1594817385552810016,
    17790006411093327694,
    12551127699309430993,
    15195679084858997233,
    14266288759238328080,
    13355909849221974532,
    6923557873780492871,
    13661173946612602484,
    5967150782907497686,
    17395037309821057621,
    17232367820665285924,
    8137571974823812392,
    510837173657731207,
    16617724477032187520,
    2038771151692979988,
    3855856554641502092,
    9365690426920840335,
    4421379932045082065,
    17595415194066819905,
    6394760486193087388,
    16319142630859815111,
    14752392372876220951,
    2859339170532671955,
    8724983474640668089,
    8266619645867912516,
    10035952698507643184,
    17262321016351042061,
    11637770769548039849,
    11652687764345222025,
    14892544705857215027,
    11498294166010407462,
    4063281309354888677,
    9258142648984656967,
    8247278570803602400,
    1416516847421322777,
    15662318640545719131,
    10579973745259183671,
    5863513898876093384,
    12651744274913091893,
    16452844400042255295,
    691927369230918614,
    845781682571284360,
    2003440258113044398,
    16879580695459371708,
    17138945206485159673,
    5385885446397924995,
    11973993517564108500,
    14194037703066285255,
    11542042331921422330,
    4331307736011085742,
    1572336696117847862,
    1863549532406351795,
    8689365939288195158,
    1212903674223946984,
    13686968539707122415,
    9492607792323795973,
    683225035194554133,
    18190917850760021703,
    3810463719662893304,
    4185658882323696195,
    3459174217916815151,
    7688181180795166966,
    5108129511778137758,
    3960178219889514335,
    11158197762670068819,
    3388985023311844805,
    10706909554572796231,
    17760607990762840609,
    12578474523540910669,
    11333338795369369373,
    1506389536331237840,
    10084806996843967991,
    5785493863783695365,
    8000193074258879654,
    659239520031624641,
    8721034622658437866,
    5760041950875033230,
    1380338067376265902,
    7108552449918600842,
    12556608307175011293,
    1801208305525787036,
    6568010642680593726,
    1216290216119171320,
    7567835679182611853,
    2041014816403161415,
    12522469571767522632,
    12602977452898619173,
    10809221047029331498,
    9920321654074965632,
    10106669374949279233,
    3603335365109855457,
    560973126306508241,
    6245920609262324952,
    14871392942557588923,
    16001494739025108381,
    3723380757545704965,
    6163207793636464880,
    7097324762144199954,
    1990616688489121190,
    12905435963695379835,
    3744126383273365566,
    11650634600262495967,
    7924429783589757450,
    1686395854290289598,
    8458469831030536567,
    16879145794035859679,
    15486074732587406624,
    18058841638905468878,
    1488517763680420713,
    795967250094789835,
    3075556954255102299,
    1069012074885060957,
    16238599459086582036,
    3513373218429513170,
    12899083468285425560,
    5762870282809689555,
    7228139032853584610,
    575351311357364428,
    3900406869173488550,
    1953797567629829602,
    2650672156411028591,
    5567969883447222690,
    13077550374901265912,
    3306009734982282192,
    17524155863997392648,
    17213877820024059138,
    9958922105145038399,
    8081467842149475019,
    6632823840480644459,
    17544175776149385923,
    13661640808527230030,
    2637532598318954163,
    4483021372524586838,
    6548282652896068715,
    22712558402515890,
    14748462594155839839,
    12709523088439035491,
    17498952379970844558,
    14972664110542335579,
    6116313611456527776,
    11435693922080433284,
    14287197276171695336,
    7668172887846987748,
    8254068332889313558,
    15689782711364363686,
    648917686498054080,
    6389954090313414482,
    6415216532209447904,
    17666475469585539308,
    11268667972145953291,
    1737451504772589687,
    4804274702892441518,
    15402471873863223341,
    9707001088546210558,
    5003829414087581894,
    10320763708533572411,
    3762773788108230824,
    14033324995941737938,
    4744498015025660676,
    12078098296142234793,
    842884293512255397,
    1602106735932811803,
    15278137866461270860,
    125247623417359199,
    16856420591240247687,
    11686332623578839821,
    7326408671361862402,
    9175922286320934954,
    8407931065680586390,
    13788174704063675086,
    9963763408459145954,
    8610771111487075831,
    9350777623086159508,
    16133674570767286823,
    17063601664348487734,
    4777533937511001824,
    889400854217245322,
    14155079968130889011,
    14407628725464069812,
    14656878083601407410,
    12041736409209628828,
    14108123114892495777,
    2440625584263807352,
    6192873574642909778,
    9072048456099597920,
    4843195966832694851,
    1531138554036094394,
    11278077835067440599,
    3776423546260901381,
    14571435267130146155,
    4997915634475910763,
    18093439460424761499,
    14561795428983634624,
    7366027992024951244,
    8648671191405964724,
    7527545884369935762,
    6109363020297235059,
    9460396364724696184,
    345341131150150024,
    3023937532772123579,
    16242768339631551150,
    13775208081386775114,
    12485922829377652196,
    11959274289896295549,
    8075555856133245672,
    5647309849298859010,
    17344063875978991865,
    761165268549977342,
    15920655343581106766,
    5267300056410823340,
    8560614735840533281,
    4398339941438398183,
    13063291728773729615,
    10107239067069852473,
    8906041306691409344,
    10358080655820970173,
    7081062882837387610,
    10596832491002926279,
    17859388599351048186,
    8915778821202473271,
    5091995915179763938,
    9595588075226043491,
    1010735195420532417,
    8681200282319169929,
    11922599086840093050,
    5391955460291837938,
    7830608392523847183,
    8302330396892468164,
    5345219434578081310,
    14444265735124189007,
    12439680387563297906,
    5939373972926133390,
    638519088918479703,
    12427216816929518800,
    7948068199856822607,
    4723399381062083118,
    16441625587151700894,
    5188397428753322130,
    2692015304284615548,
    8610027278522689725,
    2156346366625851020,
    11742735735411934095,
    5329657801237137155,
    6534756487107059414,
    14001578253779883970,
    17617916856836865210,
    4765250482596808809,
    17160516553120116501,
    10443898019245308403,
    1806780275017442105,
    2309005570023524067,
    9591541879386611118,
    5206636865224649090,
    3314086436329614346,
    14472844286011678421,
    1883753492612490698,
    4356670515623535736,
    267294803001759297,
    18158048793485962301,
    16430614865412144499,
    14829420904820854251,
    523857561997380351,
    3157653582194625698,
    2156021723732582848,
    12172899461560414750,
    1416850393207385461,
    6372253232208111795,
    8387084760705501265,
    17100606893443022288,
    16217172788560962070,
    14022218640045761219,
    5140151285988165154,
    221005662716564026,
    7478195527220523271,
    6915336998492340087,
    14558529561668210181,
    17805855614513911760,
    12078671100425739543,
    6027536925186241107,
    10057672887609658873,
    14955235309543896882,
    9936217681111746107,
    17091795113181046022,
    7215462647997508660,
    4301702565903410195,
    9758807890664308037,
    5529319377973049131,
    3332547312418848421,
    13824692535914549162,
    10164104065226016097,
    5081567279457022579,
    2193455007114617375,
    6798672339425980579,
    6760972345750380647,
    17623329627701895667,
    9246884830721469832,
    971963247621700519,
    15197704893520108270,
    14765572578259738554,
    8091739595925442276,
    5283398601885035326,
    17612570921633348645,
    1503119517396904905,
    10950007494087765400,
    17687251463129752598,
    7535495218343410578,
    3071188018990243991,
    12064996654142079324,
    13113193876752964392,
    17243140348147707121,
    4134852132790887610,
    6752930299634681735,
    18318193200592392569,
    5276118693470569060,
    12437419981700098311,
    10935630095866054344,
    12599643365967176466,
    14638025465223178086,
    10349004362280597211,
    5296175883367988886,
    7361263592304351537,
    2595158192232615133,
    8053766271446976795,
    13553774261322894990,
    8068509194731125953,
    732176041883794000,
    11093523975972790207,
    14070513886669285478,
    5908042179887731208,
    15376034984882447660,
    3010329333805746451,
    17432620010950070473,
    13927060026311709599,
    9221821696838193988,
    3764198509258020482,
    10398041127726105792,
    5269442308521058331,
    687730942798496036,
    7880604359274947097,
    11493357457909714185,
    11267727270590399134,
    11909425083073668932,
    536149894987864031,
    11923650893825016718,
    18325696716347861048,
    15535200471895054496,
    3792949139913744361,
    14558639591792721178,
    14282466489936777862,
    17047660812020216132,
    16391837667792691200,
    9152796175677912174,
    17853009347210637837,
    5819314054435044153,
    1267102431226482958,
    12805736102072611848,
    13908825356951974183,
    2164482210453490895,
    4233770402223834729,
    3501327502501732908,
    4087602872543277643,
    175227101008388305,
    11146148617643647230,
    8686002566548623459,
    9435753031323396725,
    4490369403974220587,
    6522962003690821654,
    2399561909188806325,
    9412881778354076341,
    13340707307134688793,
    5389065903483096347,
    3486468759670833616,
    6186119335474486542,
    10462796965046830871,
    3283451967448553033,
    12030740144306836447,
    2662330066039817242,
    12047900135699373648,
    4121707917876063265,
    6606990870606964143,
    15596103432331493349,
    16590443335739392271,
    14965526963695647081,
    2973018118538927111,
    9614842838903869195,
    16712093148947334789,
    7155142677190039052,
    71791121046723451,
    18442413347628288496,
    12359317677220315975,
    16466123460597497559,
    13580183814691712699,
    17848295674778725245,
    14402905344546859524,
    8400296995983992643,
    2328406821067309503,
    3526148751667920128,
    5084685458407639624,
    9827233712395832087,
    11310290301243737331,
    9984990613577801597,
    50210677865770089,
    8267371387410281555,
    15223497437973183298,
    14387940733986328009,
    11960916025120260548,
    12922329958343479843,
    5285392281386936348,
    3737817423480115783,
    1559716799560527903,
    603521292395311283,
    11007579422380569115,
    9129884804328570559,
    10810727367996660559,
    6811048213126009696,
    5391238693040806934,
    9609927742341023186,
    12854398214030937658,
    18325117547843930737,
    11937343569584731904,
    9929448938453015906,
    2729460201410121326,
    18053226695274160863,
    17158314234122038587,
    15231582279362281552,
    12996108838640830420,
    4831476396576364240,
    17266064314796412429,
    11877971498342610715,
    4462978005056096314,
    6429282740913761031,
    18165174568663803672,
    14243925773191660998,
    8501374629752709354,
    11612841369388101503,
    9863150529984290791,
    14175471655384482229,
    5961490660764580486,
    6927137500747630632,
    10735091244982475749,
    1427455765397834063,
    15398411957141219992,
    10901522258586660542,
    2117519528452697246,
    8880791188067345737,
    5004499365806234531,
    7331675231594732310,
    16014629314589394039,
    1636382727642776257,
    5638634667359687341,
    627167393804147263,
    690679084534171196,
    3521482044651619309,
    3623324772070881664,
    1780206884184784457,
    8851404849159860251,
    8193782667462772737,
    2973436633523279981,
    7098983345671839147,
    7663014064906640939,
    9248823169138283694,
    6439696768353611862,
    11565729963690941763,
    667488913719635815,
    14161471489926360847,
    6715034261833221436,
    415867338159773008,
    15019284885175078347,
    13022703990657773302,
    18026285580643731285,
    12821000861931192979,
    16796374284329292543,
    11537780092099379293,
    7596098035200941697,
    2053809783673099375,
    7617498223692028765,
    3487660308083106740,
    11126144420931252583,
    13371831942246137223,
    14221593529132551335,
    15150338271766547014,
    3135587823812195022,
    1527779855523418715,
    1392774332306630635,
    8042115407241729739,
    1180747156669223777,
    11493620005161750311,
    13388248911886346903,
    17072513435175704875,
    12542449332949284763,
    15722010757068218891,
    9114422605851736882,
    5476643169135238458,
    18052019130239747402,
    3647730875272142968,
    13574320685161316895,
    11471147173475637999,
    627165069236566442,
    654852042295739786,
    3782917344605693132,
    3135775614006900098,
    9775067296416687758,
    17442747902286998742,
    14189528413757148177,
    2141304393737734084,
    3851652764333466008,
    6336654261146052630,
    15606292200889293506,
    8956523934490748209,
    258181569823635420,
    11985524195623681515,
    3491027432533263280,
    14125916878214123284,
    7770487571894458636,
    690151227588778330,
    4779598495543483548,
    14420521604280618332,
    13264651638907049316,
    2729176004508392983,
    11447378565357276548,
    771469688105466353,
    6708395071664957330,
    9956432683448806033,
    2115560817341094739,
    9212249277210153693,
    15556374063672110198,
    16658368995508907620,
    7523951406731447363,
    1292361405414955744,
    8317272663349449582,
    17214733284919150462,
    17821718373916555865,
    11438997114057794905,
    16771528860469760529,
    13293298926897586705,
    8064893126556749165,
    14001317477175240109,
    7787201735398682983,
    14480443665215255827,
    18178589927134227588,
    6990387764290282904,
    1195465442856276160,
    10492199679110704698,
    15465054815234786657,
    16929400978876685696,
    10973810941351877829,
    2340000913711383884,
    6343377336518954121,
    17592384715665121157,
    4319201227299907748,
    7029252526256949725,
    2038868201061985311,
    10571510804153806997,
    10932515428171495656,
    3899451632201721891,
    5853574958589508034,
    681450135994134189,
    10205174380826907742,
    5091008424225132662,
    10985497336596913306,
    436371362793617174,
    1139292684666813050,
    13742036983148357276,
    3146607621421632547,
    3603057075953443835,
    2266034735592410808,
    9011847976690155754,
    3462973736115272412,
    17651274307751340063,
    9777130243560006082,
    273076900736317444,
    4814200703335573006,
    11362022527345508758,
    14508428104205327833,
    1075844603605073280,
    11184642788927924085,
    13037777630512499990,
    28097853812200178,
    2806959002614896474,
    12418330391913859081,
    16589965445815034988,
    10936272226530483511,
    8691185993028816674,
    15331678436964694373,
    12687722076434143832,
    17961777712451161120,
    6868958014893174976,
    13839333228393007464,
    3058944442505787939,
    13438851327514640380,
    5268002910432443110,
    10098765232968871874,
    15085896936557422461,
    16170412164515224952,
    7501927763001499453,
    7875691956320875508,
    17442534608876833372,
    16347183139791381390,
    15681488261550478194,
    11039527781803225677,
    15506594858776505771,
    13092524247701103653,
    9792083774137671805,
    11790158154433341563,
    17287986292581798914,
    18042605491637256854,
    13283018609055324083,
    4293620728344727033,
    11775090130689033453,
    16766590760459649323,
    12137926361671142142,
    12104456011440208913,
    6090572177362583022,
    3481501791745744595,
    16761663579851288195,
    14764508392350836357,
    10670152426802034409,
    16128688213264462639,
    165402529234934074,
    17090800313776855508,
    3849816494903914601,
    14284108420976423359,
    11443283663968657875,
    1816001748357379491,
    8984759260398243396,
    1758249655105662329,
    13411265567358790636,
    5063083939178996104,
    8840950749669950592,
    6014758828534461464,
    14862129449467571043,
    178926219691553179,
    7398679978927264422,
    14871003649053261539,
    9303773699166155948,
    6756919952141370291,
    4845061519345932271,
    8147850778906187562,
    10713709122863618357,
    14852345169375227628,
    16059134022196530375,
    13800417083113489530,
    7823691992573732840,
    4810020639338895520,
    10686674885266108595,
    920518232405373677,
    7467647806024664747,
    6480415146588207169,
    15217635487597958678,
    2421737202177960950,
    18369031411706467182,
    15098575111913042805,
    14514579562690166358,
    6635143157073034393,
    10611917808310218844,
    4211792463289443036,
    4605412329672847742,
    17367637997637363846,
    6643634378196153721,
    14971993398559928006,
    3587803887643662086,
    12249914526977509990,
    4442696431120982114,
    7900941559443478140,
    9762527050378227405,
    883756791745244177,
    17890003599216292950,
    11579847890094908474,
    1933685385732094752,
    3891396046371278535,
    7587073351357340278,
    3728279503166217667,
    13588268401212757011,
    8733645372778026945,
    7812861959602884221,
    7657101705482032399,
    8805112143334712617,
    15466402337367719412,
    1664883450361584623,
    18080455413760789840,
    6464786796293793928,
    4666873124420659613,
    6764502309160426064,
    13433681849064845179,
    16093240733221381069,
    7070569635269231801,
    489060368574064516,
    2554540121543878833,
    10330989334401514429,
    15367897615969215914,
    14603900802637501067,
    10211698314972805370,
    4527015074720082807,
    2434685875848822245,
    15410604757695623028,
    8870255028694624491,
    16511797656484914811,
    1756642456728828028,
    18418622145923285522,
    5990852463526001223,
    1517116930944808714,
    5071422766127712052,
    14556530053155160569,
    12598229223202251460,
    13402381346841052811,
    8209323022047591587,
    12322870886078304640,
    14975992165954859709,
    8636925591346565736,
    17052908474268943514,
    13059899849725517943,
    12238721281621197908,
    14252567944105308509,
    13227296794066070115,
    702789759996361557,
    5598692288002420765,
    16594759396123370105,
    17241897608751247687,
    2402034988935094292,
    9520989818519526874,
    13634819983632103424,
    10936222640272656610,
    17515498145750952927,
    8260649261392558239,
    8708361167239236645,
    17076714465942491781,
    10454756077372920554,
    5151651692838182850,
    11850295677414401557,
    10867400778395165211,
    11470877901135796404,
    11713499538660840750,
    15722356424643138145,
    1771553434798059874,
    12382380766894684894,
    4447492293657723827,
    4673305931393412906,
    1903463983953910003,
    10151939487058321542,
    1478999251640258112,
    8539872301151351945,
    17599880808060624096,
    12238044925737881661,
    16987761358617140088,
    17011866725440932226,
    6727656059800949121,
    16014010440158136128,
    18294512910247333408,
    3268111619767338857,
    6634614371998141466,
    1679624327717391217,
    3539079986399546051,
    7062140866553666720,
    3768397623628001970,
    3439558290021741713,
    14565763133946582305,
    12930480612001047450,
    3607931759992129046,
    5183163769407221416,
    9148938122607676274,
    5000757681119904180,
    17027593261320014951,
    7133440847195425283,
    18440238805839252088,
    328210875633232003,
    12282796208565112201,
    8363826451462164543,
    15582111100033418035,
    5601020516787936633,
    5166873875857322748,
    15381160967011590466,
    8743840711634760270,
    15632179181226779415,
    16655706586529637018,
    8587532398856503635,
    17943390431068054392,
    2385026523462456542,
    17364499229796406891,
    6509630532658196781,
    5246504386035827417,
    3048043334118263639,
    12575525091979139233,
    14802883372101830006,
    7242675912955697485,
    11428212790588365678,
    5827411646691082579,
    18338110046578646928,
    12000932871209570300,
    9545460156638404305,
    7698397792233411693,
    14526912581113254740,
    12065636575346776920,
    1477208443096497483,
    5558951756141174342,
    707405320687768264,
    9634127261784560645,
    9593509130255357213,
    16640765547241326816,
    5719842567261319003,
    8460027281607536989,
    6839378305747603754,
    7204361387259693311,
    6817076675037169784,
    2290574782778059797,
    13869037195425879438,
    9837713286150597603,
    16521043364775382880,
    5310986776900949167,
    7586530155347304216,
    16050190798130508579,
    12160476116060914795,
    16698524409449375000,
    9430379581895136129,
    2049813917266218604,
    11361296552785640775,
    3547384231330784666,
    12673373005276881978,
    844079367248245357,
    15239303316541516475,
    8163601013476252192,
    7374363236026768577,
    13584306715985587216,
    7629127137736126615,
    17302388729099757173,
    10721002186738779464,
    8054699270236778628,
    3567558916045312666,
    5596163784025644498,
    1539242055414828478,
    15314939172240929991,
    517669233735336588,
    7760495821934030251,
    11637373533499755326,
    13511405886417133473,
    15678858355366527713,
    5329926441015605490,
    14037827282251837700,
    14734439107112602343,
    6942227261811644907,
    12527611605885744380,
    2181963277915175441,
    8875545386607975694,
    5973372989031181718,
    17052364620038119286,
    9399923907338834888,
    13317423266798234409,
    2981881765633147724,
    8233606989022684237,
    4658349745494195713,
    16735398847401100255,
    10080312251124693750,
    15813788436630984862,
    15436753088184828186,
    10893680495611648549,
    4763608084198464985,
    6974548106337236361,
    3102401715811151951,
    8356460844349562915,
    398312631939340139,
    13085196365631660068,
    15756769185178355614,
    3549619136220159148,
    9224855368936567675,
    82962106962320007,
    9676155448039420118,
    1352584366195202920,
    10890469680591978772,
    9600337382682786124,
    10397826830551282515,
    13561365149225120185,
    12369270763002958741,
    14951584032995700904,
    3926537276012587852,
    2764171764798819980,
    13877015605867058865,
    8567987822431156370,
    15765724759584893854,
    7778581931225670404,
    10794120274284772366,
    16220851095791725090,
    10595229303660427080,
    16170036914586448244,
    17455251021133170109,
    16510316931844981596,
    16516535361088206892,
    5011899017829973143,
    8504912668834254327,
    1436998511109759675,
    2715808234786166037,
    11928377228910062350,
    6055068373612760604,
    6209576808925225990,
    15414725812261584460,
    6446652226134745741,
    4206899897368167967,
    10876282186758266562,
    5694010443916256468,
    16233678843883014475,
    5872006002650650536,
    9211374624114641302,
    164686331808196287,
    10264818926630866407,
    6621066009893650350,
    5502468100320065937,
    5972314542766639198,
    13896885881143835650,
    5658509549615639322,
    11560525539480616733,
    6518326021784092373,
    14804976977894047547,
    8203201521354485349,
    73187742301085226,
    1479096317911852483,
    4131176960240916666,
    6617867290272338228,
    17207864279890699728,
    15552996764758491146,
    12328654135045337386,
    6676184595228376780,
    9197520833062800931,
    14053007900021120893,
    12092332631693376576,
    14232820541615686781,
    17661053412236187602,
    12806739418069601819,
    14862762464953273265,
    8096190327246451694,
    8085646574253928058,
    1477672573917608535,
    1772079971086284618,
    16896532136715180576,
    17908537856835911216,
    3059142790610732586,
    13121274837620481263,
    1454152010277661812,
    230739218699332245,
    2683635255123653425,
    11204309739486724561,
    6331265394175450644,
    7981760727028562894,
    16437113917865059233,
    5055944223272932154,
    5852355077134161371,
    13686821526478012036,
    2739467856191476850,
    8291504425050180911,
    6812654683965383568,
    3237290434851301690,
    12896729127611747818,
    17558166638096765329,
    5350475224960822800,
    571117110899736737,
    16698859771315627793,
    15618999075239169520,
    805354921982289205,
    17539322328263373709,
    6915708880896158544,
    7888445876266965258,
    16433183475136496135,
    8886206355228277679,
    10335776551696383988,
    2091061080326669605,
    1565501534808127254,
    12439520630572796898,
    1760640752923884185,
    17699016376909848720,
    16215466074596798324,
    8783369403492613459,
    10451861068266712664,
    2182103851655662719,
    10532262783585684146,
    17797334557151193487,
    329235272481271800,
    16750951215339230679,
    8586980985140604031,
    16088206730172458410,
    7335698588257224552,
    11479830867496823282,
    14908975729286787046,
    13301479940956231854,
    4053091074022899549,
    11288430206402192765,
    16419569791638805598,
    15764783392474003844,
    4986867942257866731,
    4494274341845964751,
    17333291263889602031,
    8474012889194648073,
    2677000640537930894,
    3704614818225670713,
    3527411413888262870,
    15488590144811266520,
    18238400058929774561,
    11956561172740583309,
    13923147933158001840,
    596357556529508294,
    18331784318509138114,
    342928005352972562,
    10244616458209196837,
    4756940480396478864,
    7768118632395618568,
    9125464446959940435,
    4649795024991231054,
    658157860272472688,
    688069680323861427,
    5171942719063687517,
    14673871453812033424,
    4724025466359662000,
    4849756615850725329,
    3065066121846837089,
    831915371709116765,
    7052113163400467990,
    3034893039024379036,
    10107187482198481814,
    2273478221601026136,
    16909015571786469641,
    8888193088425988562,
    10664697447923186544,
    5824273888333601995,
    14476991045293082777,
    4631348539253092956,
    8831333251136793732,
    15466401713075289992,
    300478443492851617,
    8278296009942490045,
    15754112862748590869,
    14928049979997086403,
    17699490455356754074,
    9215140500216603875,
    17546234847569797678,
    14122248778168660164,
    7208803706434535341,
    3444606817297660523,
    1879501856150571480,
    8269652747223547109,
    10149421109560226697,
    5951998545701689202,
    414609212701268755,
    5645687276071689520,
    3231897985350505662,
    7429315085993236020,
    8504950413578105747,
    8748183667191225429,
    9021257363549574459,
    3944343496794422738,
    9814885280857287983,
    9199904740864538518,
    12185543312821970504,
    8744899443780360070,
    3569886786859543336,
    5983394583679312550,
    12129109117480520105,
    17380940597393683614,
    14343491326990244966,
    15729196862714385078,
    10669855608345903168,
    2580978061257822740,
    5553418805449512583,
    3047763651377782829,
    252833181202747573,
    1087831882947603229,
    10765279951284929892,
    3982887244003077032,
    8554281734750268413,
    6000549847534972675,
    5685351638217190959,
    14083534199901176560,
    9485514424787692380,
    17200082416734493884,
    13550073678534810125,
    3695523206757377470,
    10887290515685492265,
    11875224822729152941,
    6936634339359748164,
    12877021467529949513,
    16315684922292404641,
    6101645262953620152,
    8837857861396338872,
    4490707537299923132,
    10993466536332338997,
    10498872444201323451,
    11616528492799105094,
    12810974276674259370,
    12526237187335950840,
    10432430813068675117,
    8934184217532998252,
    1172739694767251076,
    961251461937598350,
    10274909074147841476,
    1614619660321643728,
    6750662084925823254,
    5912564292732383167,
    3532671443598383247,
    16537490421255809876,
    3541267201997547575,
    6027676071684612537,
    3994784390039519876,
    5535647801361399091,
    16250894673768677178,
    11196617085885799045,
    15531669860296254990,
    10747525084020412527,
    16060261225631962537,
    2121502919271041893,
    4110205061653709667,
    16037837756650161494,
    14480975650215019527,
    10244349754061578936,
    48643758791517712,
    4136924661037132808,
    7628746609570841050,
    2457175391455495196,
    10976178179407963336,
    11797792884405031599,
    14576452968967149051,
    3427859752638930697,
    10635902496527262884,
    14685029473842751251,
    8338338877354471544,
    2494747654116234971,
    3367247456121951364,
    1854203630882124584,
    12019642667798024357,
    14632484129065841106,
    14483464636770957956,
    10187728809772796279,
    11544970531465647090,
    17251506021350945170,
    11000758102214508375,
    12299506920704167023,
    18302708857795425369,
    5283154362707822095,
    2111963853505001984,
    10247905698779439143,
    5490625871263987930,
    172003618233416891,
    14683840956785111108,
    11815255831827170378,
    14414676238503820403,
    4383667688443474222,
    15546440333792228634,
    2619931380200173568,
    17022702778664548823,
    13026983864274004411,
    6768281945753898196,
    1617000238175465822,
    11102151473329878931,
    6689302614728260322,
    9304461059646269706,
    2979749274634852336,
    5191311570829423344,
    14814019991555007083,
    4634827028758464361,
    11333786887316377114,
    16482919550481905617,
    13209449713403122498,
    16870789557609162740,
    5360590835377948466,
    4861704059767837312,
    11906305359223245834,
    10965545995453660682,
    7362725737658329631,
    13770083493183869571,
    2983687591532575987,
    16144142612868857141,
    8482636998389990465,
    16406577814079731342,
    374034617470737850,
    15462911855933256308,
    12844692916977078601,
    10244450390286306580,
    11970181586479500095,
    1305885821368130050,
    16515951813682983843,
    13673385427541435343,
    11663958832596674590,
    2906169575358942685,
    1101856160300448222,
    11117469276050533354,
    17015373230114398500,
    10574656041261730762,
    12542077834744237982,
    3300763690665743828,
    13056265505448647872,
    18371958788070906118,
    1028259815134910573,
    3989302624717553079,
    2026943073009918830,
    2706203159565056103,
    16089623394429174226,
    7680930294242283588,
    1031022116737119097,
    9991400726963221621,
    6773216225396765928,
    8116325914327485291,
    11033786940259479656,
    2387928100959500827,
    13675835068200257628,
    11869388034312863129,
    12422054161688926066,
    9239859085034094518,
    11575425139651817599,
    17766833351892542943,
    12566615246162687411,
    15103614168931056885,
    8275793443069308545,
    335110998300432841,
    6652767298720418213,
    16348838518046140795,
    13409277891708713239,
    13772084890041795051,
    5621079768688857238,
    8705486649219875305,
    12313349083417147595,
    5770307076981701855,
    16455047089879388915,
    7541298086831680126,
    11085804977965937843,
    12698783030112443693,
    10136391976940914069,
    9926955571312592969,
    9692241449917947783,
    7627901118881065815,
    2747298465588499879,
    4446186648502486202,
    4181898574006575762,
    16607722915548507463,
    9071562026277836279,
    9142214624577053831,
    11605468051981649782,
    8581362950449393226,
    11428300071901680580,
    15883366419182054428,
    5460510412785104123,
    404203956534633997,
    4694931570174326704,
    11997162588823306349,
    13617619213951491607,
    14909026668655779580,
    14532559779313673132,
    8176884226941736310,
    11844273267878800572,
    18246398047968178532,
    11026967610810312071,
    10881117640627580948,
    9270528284255903544,
    14879995210000460391,
    4359930025668901521,
    900109141860897795,
    11072705578753556960,
    10046693569323533433,
    7044160135312511089,
    17123702303405545148,
    8120730941314213155,
    2922888149530345307,
    907009267641370767,
    12753474547660094384,
    15167825808144200254,
    2243629983106608984,
    5176628992465895220,
    16512619484632042884,
    13601339903695618988,
    17081963009291672051,
    14522722521448673717,
    4320787099032816251,
    12667496526333928255,
    2702675370767502066,
    3864050099409348197,
    13861174663625861626,
    183301456868781936,
    8929667989079287318,
    16355253465778595696,
    12670562050762601184,
    17820910370712084414,
    810648551680760200,
    9735844154884577878,
    14356007631982517438,
    16234193758035129046,
    7865835690314937371,
    5902276341601998394,
    278078490066267359,
    2950765392741684116,
    110824298748894942,
    232638180331989763,
    17162013796311222385,
    7075949237844055147,
    3723868450345023148,
    11493562302953399712,
    8450288739195853899,
    12576286937892229829,
    10925230312741795615,
    5001600307025021589,
    15696929900713992075,
    33966871250420955,
    825011374934905363,
    409544539862695407,
    14849917992128041572,
    6318997946509970790,
    4377927832347147678,
    6666209342034609102,
    7683205530275213914,
    10676426080916097688,
    14762700369141078772,
    2184104759780711734,
    6646009786170111177,
    8456395324339692308,
    13947287004323929756,
    11186426013890046277,
    16873465407871018264,
    17214287953821163219,
    1653524125308186903,
    3453058242462487757,
    12549516579559837437,
    1046860362303639197,
    18274337845322649022,
    18288678957462244729,
    14226053789780927227,
    5483303747566036322,
    2026886942771641745,
    5389575774314429590,
    10909586205533078671,
    7804109990715586866,
    16894850916150915331,
    7227559755303199049,
    17350763643367168351,
    16185250941450633311,
    7589631732976499978,
    13113420856026042173,
    16957651795370578382,
    886100997372758583,
    8676947471466738538,
    8763625216007305620,
    5494095225063661988,
    2177910906566210525,
    10343873789072068733,
    10662808201344215874,
    14504259331429686427,
    3885775032437797028,
    9177003052145118516,
    1174074540079609430,
    4658533314641085114,
];

pub const OPENING_BOOK: &[(&str, &[(&str, u32)])] = &[
		(
		"rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 235627),
			("d2d4", 165568),
			("c2c4", 35435),
			("g1f3", 33718),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 59321),
			("e7e6", 25193),
			("c7c5", 81221),
			("c7c6", 18964),
			("g8f6", 6551),
			("d7d6", 12452),
			("d7d5", 13472),
			("g7g6", 7387),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 45538),
			("f2f4", 3464),
			("f1c4", 2850),
			("d2d4", 2536),
			("b1c3", 3791),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 38199),
			("g8f6", 3774),
			("d7d6", 2257),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 181),
			("f1b5", 358),
			("d2d4", 92),
			("b1c3", 53),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 67),
			("f8c5", 64),
			("f8e7", 33),
			("f7f5", 355),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d3", 34),
			("e1g1", 196),
			("f3g5", 962),
			("d2d4", 226),
			("b1c3", 129),
			("d1e2", 78),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8e7", 72),
			("f8c5", 159),
			("h7h6", 97),
			("d7d5", 73),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 17757),
			("b1c3", 1625),
			("d2d3", 1259),
			("g1f3", 2531),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 17638),
			("c7c5", 591),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 1740),
			("b1c3", 8006),
			("b1d2", 4419),
			("e4e5", 2938),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 1717),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 519),
			("f1d3", 500),
			("c2c4", 296),
			("b1c3", 354),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8d6", 126),
			("g8f6", 345),
			("b8c6", 94),
			("c7c6", 53),
			("c8g4", 25),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 55891),
			("b1c3", 8895),
			("c2c3", 5900),
			("d2d4", 3526),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 175),
			("d7d6", 267),
			("e7e6", 124),
			("g7g6", 41),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1b5", 49),
			("d2d4", 73),
			("b1c3", 2435),
			("c2c3", 479),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 385),
			("g8f6", 195),
			("g7g6", 674),
			("a7a6", 89),
			("d7d6", 220),
			("d8c7", 55),
			("c6d4", 80),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 214),
			("b5c6", 151),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8e7", 187),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/3P4/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 70307),
			("d7d5", 43619),
			("d7d6", 8267),
			("e7e6", 11181),
			("c7c5", 6925),
			("g7g6", 6994),
			("c7c6", 5488),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 42394),
			("g1f3", 14828),
			("b1c3", 2254),
			("c1g5", 5120),
			("e2e3", 2002),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 309),
			("g7g6", 393),
			("e7e6", 191),
			("d7d5", 265),
			("d7d6", 129),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 244),
			("c2c3", 25),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 122),
			("c5d4", 71),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 519),
			("e2e4", 187),
			("c1g5", 59),
			("g1f3", 804),
			("b1c3", 80),
			("c2c4", 381),
			("e2e3", 56),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/3P4/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6e5", 476),
		]
	),
	(
		"r1bqkbnr/pppppppp/8/3Pn3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 168),
			("b1c3", 39),
			("g1f3", 48),
			("f2f4", 183),
		]
	),
	(
		"r1bqkbnr/pppppppp/8/3Pn3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 68),
			("e5g6", 63),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/2P5/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 6918),
			("c7c5", 3083),
			("b8c6", 2211),
			("g8f6", 8316),
			("e7e6", 3884),
			("g7g6", 2252),
			("d7d6", 1309),
			("c7c6", 3875),
			("d7d5", 1162),
			("f7f5", 1001),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 2466),
			("b1c3", 3889),
			("g1f3", 214),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 1133),
			("f7f5", 204),
			("b8c6", 716),
			("d7d6", 137),
			("f8c5", 91),
			("d7d5", 87),
			("c7c6", 91),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 242),
			("b1c3", 48),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 63),
			("d7d5", 49),
			("b8c6", 50),
			("f8c5", 48),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 208),
			("g1f3", 25),
			("b1c3", 57),
			("e2e3", 27),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e5d4", 98),
			("e5e4", 84),
			("f8b4", 25),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8c5", 43),
			("a7a6", 64),
			("g8f6", 103),
			("f7f5", 941),
			("f8e7", 34),
			("g8e7", 50),
			("d7d6", 38),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("c2c3", 238),
			("e1g1", 296),
			("b1c3", 25),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8e7", 56),
			("g8f6", 103),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 14522),
			("f1b5", 12269),
			("c2c3", 1042),
			("b1c3", 841),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 111),
			("g8f6", 428),
			("e7e6", 110),
			("a7a6", 28),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 40),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 1177),
			("g8f6", 1937),
			("f7f5", 183),
			("d7d6", 222),
			("f8b4", 182),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 694),
			("g2g3", 686),
			("e2e3", 40),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 804),
			("f7f5", 26),
			("f8c5", 35),
			("d7d6", 28),
			("g7g6", 92),
			("f8b4", 65),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 884),
			("e2e3", 120),
			("e2e4", 29),
			("d2d4", 74),
			("d2d3", 35),
			("a2a3", 41),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 1277),
			("f8c5", 188),
			("f8b4", 302),
			("c6d4", 58),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 2285),
			("b1c3", 657),
			("d2d4", 2075),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 257),
			("d7d6", 378),
			("e7e5", 57),
			("e7e6", 242),
			("f7f5", 534),
			("d7d5", 143),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 141),
			("b1c3", 96),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/4P3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 104),
			("f6g4", 29),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1152),
			("g2g3", 1035),
			("g1f3", 777),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 62),
			("b8c6", 673),
			("g7g6", 154),
			("g8f6", 147),
			("b7b6", 72),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 40),
			("g2g3", 28),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("b7b6", 97),
		]
	),
	(
		"r1bqkbnr/1ppp1ppp/p1n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("b5a4", 16813),
			("b5c6", 2071),
		]
	),
	(
		"r1bqkbnr/1ppp1ppp/p1n5/4p3/B3P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 15703),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 23094),
			("e7e5", 1615),
			("c7c5", 4951),
			("g7g6", 11349),
			("d7d6", 1173),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 394),
			("b1c3", 443),
			("g2g3", 87),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b6", 117),
			("d7d5", 117),
			("c7c5", 51),
			("f8b4", 74),
			("f8e7", 101),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 2792),
			("a2a3", 1117),
			("b1c3", 1143),
			("e2e3", 193),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8a6", 1277),
			("c8b7", 1351),
			("f8b4", 132),
		]
	),
	(
		"r1bqkbnr/1ppp1ppp/p1B5/4p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7c6", 2035),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/2N5/PPPPPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 1580),
			("d7d5", 3160),
			("g8f6", 1330),
			("e7e5", 611),
			("d7d6", 700),
			("g7g6", 729),
			("c7c6", 534),
			("e7e6", 699),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 823),
			("d2d4", 174),
			("g1f3", 507),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 25501),
			("g1f3", 11020),
			("b1c3", 1283),
			("e2e3", 1537),
			("c1g5", 1594),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 10385),
			("e7e6", 8832),
			("g8f6", 899),
			("b8c6", 1065),
			("d5c4", 2814),
			("e7e5", 840),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 5676),
			("b1c3", 4778),
			("c4d5", 1659),
			("e2e3", 586),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 1565),
			("e7e6", 416),
			("c8g4", 107),
			("d5c4", 127),
			("c8f5", 93),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 1328),
			("e2e3", 502),
			("d1c2", 167),
			("g2g3", 66),
			("c4d5", 180),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 461),
			("d5c4", 135),
			("a7a6", 128),
			("c8f5", 122),
			("c8g4", 61),
			("g7g6", 111),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 841),
			("e2e3", 327),
			("d2d4", 456),
			("e2e4", 1205),
			("d2d3", 244),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5d4", 234),
			("g8f6", 305),
			("c7c6", 170),
			("b8c6", 29),
			("c7c5", 78),
			("e7e6", 44),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3p4/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e4", 214),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3pN3/5N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 45),
			("f7f5", 38),
			("c8f5", 25),
			("c7c5", 26),
			("b8c6", 36),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 46),
			("c1g5", 38),
			("b1c3", 409),
			("g2g3", 171),
			("c4d5", 233),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 25),
			("d5c4", 58),
			("c7c5", 97),
			("f8e7", 32),
			("c7c6", 75),
			("a7a6", 60),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 88),
			("d7d5", 53),
			("c7c5", 40),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e3", 80),
			("d1c2", 26),
			("c1d2", 289),
			("f2f3", 28),
			("g1f3", 49),
			("a2a3", 310),
			("c1g5", 289),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e8g8", 1326),
			("c7c5", 468),
			("b7b6", 243),
			("d7d5", 87),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 605),
			("e7e6", 558),
			("d7d5", 485),
			("c7c5", 148),
			("d7d6", 74),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 2161),
			("e2e3", 279),
			("c1g5", 634),
			("b1c3", 432),
			("g2g3", 830),
			("c1f4", 540),
			("c2c3", 278),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 1112),
			("c7c5", 64),
			("d7d5", 75),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 40),
			("b1c3", 498),
			("e2e3", 37),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("e8g8", 600),
			("c7c6", 55),
			("d7d5", 82),
			("d7d6", 163),
			("c7c5", 59),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1505),
			("g2g3", 183),
			("d2d4", 485),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 1429),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 26),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 3136),
			("b1c3", 6567),
			("e2e3", 274),
			("c4d5", 605),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 676),
			("c7c5", 118),
			("f8e7", 51),
			("f7f5", 75),
			("c7c6", 290),
			("d5c4", 66),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 34),
			("c7c6", 50),
			("f8e7", 74),
			("b8d7", 169),
			("c7c5", 29),
			("b8c6", 100),
			("d5c4", 176),
			("h7h6", 756),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 69),
			("d7d6", 27),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 80),
			("f1c4", 152),
			("c2c3", 122),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8c5", 1240),
			("g8f6", 639),
			("d8f6", 67),
			("d8h4", 114),
			("f8b4", 61),
			("c6d4", 77),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 253),
			("g2g3", 220),
			("e2e3", 60),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 228),
			("d7d6", 35),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 1026),
			("d7d5", 1020),
			("d7d6", 68),
			("e7e6", 82),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 1451),
			("g2g3", 2379),
			("b1c3", 3617),
			("g1f3", 701),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PPQ1PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("e8g8", 40),
			("c7c5", 433),
			("b7b6", 95),
			("d7d5", 354),
			("b8c6", 113),
			("c7c6", 66),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/5N2/PPPPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 9301),
			("g8f6", 9473),
			("c7c6", 1531),
			("c7c5", 3641),
			("e7e6", 1551),
			("d7d6", 2096),
			("b8c6", 1829),
			("g7g6", 2288),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 2853),
			("c2c4", 2658),
			("b2b3", 344),
			("g2g3", 2630),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 827),
			("g8f6", 4404),
			("c7c6", 2142),
			("e7e6", 1395),
			("b8c6", 652),
			("c7c5", 761),
			("c8g4", 444),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 328),
			("c1g5", 40),
			("e2e3", 217),
			("c1f4", 98),
			("g2g3", 96),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 230),
			("f5b1", 59),
			("c7c6", 100),
			("d5c4", 37),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4p3/3p1b2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 180),
			("d1b3", 28),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4p3/3p1b2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 38),
			("c7c6", 104),
			("b8c6", 59),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 485),
			("e2e4", 472),
			("d2d4", 169),
			("f2f3", 50),
			("g2g4", 103),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 45),
			("g7g6", 119),
			("b8c6", 87),
			("d7d5", 305),
			("e7e6", 52),
			("d7d6", 53),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 14035),
			("g8f6", 449),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 13380),
			("d1d4", 533),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 12788),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 541),
			("d7d6", 63),
			("g7g6", 67),
			("e7e6", 97),
			("a7a6", 35),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 261),
			("g1f3", 230),
			("f2f4", 25),
			("g1e2", 169),
			("f1b5", 301),
			("f1c4", 109),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 1125),
			("d7d6", 126),
			("g8f6", 53),
			("e7e6", 98),
			("e7e5", 91),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 1105),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 1086),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5f4", 1969),
			("b8c6", 408),
			("d7d6", 164),
			("d7d5", 601),
			("f8c5", 217),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/4Pp2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 1056),
			("f1c4", 501),
			("f1e2", 157),
			("d2d4", 195),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/4Pp2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g5", 286),
			("f8e7", 120),
			("g8f6", 99),
			("h7h6", 30),
			("d7d5", 292),
			("d7d6", 150),
			("g8e7", 37),
		]
	),
	(
		"rnbqkbnr/pppp1p1p/8/6p1/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("h2h4", 125),
			("f1c4", 89),
			("b1c3", 32),
		]
	),
	(
		"rnbqkbnr/pppp1p1p/8/6p1/4Pp1P/5N2/PPPP2P1/RNBQKB1R b KQkq - 0 0",
		&[
			("g5g4", 121),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/2PP4/5N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8e7", 86),
			("h7h6", 84),
			("b8d7", 48),
			("f8b4", 155),
			("c7c6", 105),
			("d5c4", 93),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 11611),
			("b1c3", 2476),
			("d2d3", 1317),
			("g1f3", 2284),
			("c2c4", 513),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 10719),
			("d7d6", 845),
			("g7g6", 497),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 3676),
			("e4e5", 3050),
			("e4d5", 2912),
			("b1d2", 719),
			("f2f3", 406),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 956),
			("e7e6", 74),
			("g7g6", 103),
			("g8f6", 83),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e4", 3637),
			("f2f3", 345),
			("f1c4", 244),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3PN3/8/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8d7", 1127),
			("c8f5", 1804),
			("g8f6", 1182),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 311),
			("e2e4", 181),
			("f2f3", 32),
			("d2d4", 51),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 223),
			("d7d6", 65),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 100),
			("e2e4", 120),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e5d4", 106),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 114),
			("d5e4", 1151),
			("c7c6", 158),
			("g8f6", 44),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 242),
			("g8f6", 409),
			("f8b4", 339),
			("b8c6", 47),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 69),
			("c3e4", 1325),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 592),
			("d7d5", 1658),
			("g8f6", 1628),
			("g7g6", 329),
			("d7d6", 455),
			("e7e6", 913),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 499),
			("g1f3", 59),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 142),
			("c5d4", 332),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4c5", 51),
			("e4d5", 82),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2Pp4/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 44),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/2N1P3/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 105),
			("e7e5", 101),
			("c7c6", 45),
			("c7c5", 27),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 96),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/2N1PN2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 227),
			("g1f3", 97),
			("b1c3", 110),
			("e2e3", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 70),
			("f6d5", 374),
			("d8d5", 129),
			("c7c6", 96),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5e6", 62),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4Pn2/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8e6", 64),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N5/PP1BPPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e8g8", 129),
			("d7d5", 53),
			("c7c5", 61),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 25),
			("d7d6", 44),
			("g7g6", 92),
			("e7e6", 42),
			("g8f6", 36),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1c4", 431),
			("f1b5", 34),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f8e7", 231),
			("d7d6", 184),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 46),
			("g2g3", 105),
			("c1g5", 143),
			("c1f4", 35),
			("e2e3", 70),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 635),
			("c7c6", 240),
			("c7c5", 68),
			("e7e6", 144),
			("c8f5", 107),
			("e7e5", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 47),
			("c1f4", 132),
			("c1g5", 388),
			("e2e4", 33),
			("e2e3", 44),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 39),
			("e7e6", 28),
			("c8f5", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c1f4", 168),
			("e2e3", 108),
			("c1g5", 82),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
		&[
			("f8g7", 137),
			("c7c6", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4c5", 115),
			("c2c4", 353),
			("c1f4", 45),
			("e2e3", 205),
			("c2c3", 197),
			("c1g5", 31),
			("g2g3", 107),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2Pp4/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 37),
			("b8c6", 63),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 1276),
			("e2e3", 70),
			("g1f3", 250),
			("e2e4", 71),
			("c2c4", 188),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4P3/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 735),
			("d7d6", 228),
			("f7f6", 200),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 653),
			("e2e4", 27),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4P3/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8e7", 409),
			("f8c5", 26),
			("g8e7", 111),
			("f7f6", 30),
			("d7d6", 48),
		]
	),
	(
		"r1b1kbnr/ppppqppp/2n5/4P3/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 134),
			("c2c3", 79),
			("c1f4", 66),
			("d1d5", 47),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 291),
			("g1f3", 54),
			("c1f4", 59),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3PN3/8/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 246),
			("b8d7", 1014),
			("f8e7", 225),
			("c8d7", 244),
			("d8d5", 76),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 306),
			("c7c5", 281),
			("c7c6", 339),
			("g8f6", 677),
			("c8f5", 126),
			("b8c6", 66),
			("e7e5", 48),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 73),
			("g1f3", 166),
			("f2f4", 75),
			("f1d3", 87),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 87),
			("g8f6", 171),
			("c7c6", 48),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 39),
			("g1f3", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 850),
			("f2f3", 237),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3Pp3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 530),
			("c8f5", 42),
			("e7e5", 166),
			("f7f5", 31),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 62),
			("d4e5", 1399),
			("b1c3", 85),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/3Pp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8c5", 48),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5d4", 186),
			("b8c6", 28),
			("e5e4", 55),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3p4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e3d4", 170),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 59),
			("d7d5", 96),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 25),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 4091),
			("e2e3", 154),
			("g1f3", 428),
			("b1c3", 159),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2P5/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 67),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b7b5", 2151),
			("e7e6", 1315),
			("g7g6", 162),
			("d7d6", 286),
			("e7e5", 429),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4b5", 1477),
			("d1c2", 114),
			("g1f3", 225),
			("b1d2", 150),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1PpP4/8/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("a7a6", 1429),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1362),
			("g2g3", 1310),
			("d2d4", 641),
			("g1f3", 381),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 964),
			("f8b4", 49),
			("b7b6", 46),
			("f7f5", 55),
			("g8f6", 158),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 212),
			("d2d4", 605),
			("e2e3", 67),
			("g1f3", 32),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 207),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 185),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 148),
			("d7d5", 274),
			("b4c3", 30),
			("e8g8", 93),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 1037),
			("d2d4", 6457),
			("c2c4", 581),
			("b1c3", 2296),
			("d2d3", 467),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 30),
			("g8f6", 339),
			("b8c6", 73),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 194),
			("e4d5", 373),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d5d4", 57),
			("b8c6", 170),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 2699),
			("c2c4", 3851),
			("d2d4", 1958),
			("b2b3", 270),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 1220),
			("d7d5", 599),
			("e7e6", 227),
			("c7c5", 153),
			("b7b6", 274),
			("b7b5", 82),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 943),
			("b2b3", 213),
			("c2c4", 40),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 951),
			("d7d5", 50),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 66),
			("d2d4", 89),
			("c2c4", 86),
			("d2d3", 25),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("e8g8", 36),
			("d7d6", 67),
			("d7d5", 55),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 156),
			("d7d5", 260),
			("b7b6", 178),
			("b7b5", 93),
			("f8e7", 32),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 153),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 66),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 256),
			("e8g8", 30),
			("d7d5", 167),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 144),
			("b8c6", 34),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/4P3/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 42),
			("b1c3", 70),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5d4", 59),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2Pp4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e3d4", 55),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 32),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 3712),
			("b1c3", 2110),
			("d2d3", 351),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 3503),
			("f6e4", 97),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/3nP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 1989),
			("g1f3", 230),
			("b1c3", 475),
			("c2c4", 656),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/3nP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 1679),
			("e7e6", 74),
			("d5b6", 149),
			("c7c5", 58),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p4/3nP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 906),
			("f1c4", 46),
			("e5d6", 123),
			("c2c4", 537),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p4/3nP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d6e5", 28),
			("c8g4", 38),
			("c7c6", 26),
			("c8f5", 28),
			("b8c6", 33),
			("g7g6", 139),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 8863),
			("b1c3", 1440),
			("g1f3", 1296),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 4865),
			("c7c6", 1704),
			("g7g6", 2008),
			("e7e5", 803),
			("b8c6", 326),
			("b8d7", 364),
			("e7e6", 300),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 4027),
			("f2f3", 293),
			("f1d3", 404),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 388),
			("c7c6", 98),
			("e7e5", 75),
			("b8d7", 103),
			("b8c6", 25),
			("c8g4", 27),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 530),
			("f2f4", 541),
			("c1e3", 526),
			("f1d3", 76),
			("c1g5", 225),
			("f1e2", 105),
			("f2f3", 221),
			("g1e2", 179),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 383),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 489),
			("b1c3", 734),
			("f3e5", 1979),
			("f1c4", 330),
			("d2d3", 188),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 147),
			("f6e4", 310),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 136),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4P3/3p4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6e4", 116),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/3nP3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 189),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p4/3nP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 31),
			("d2d4", 102),
			("e5d6", 30),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/3nP3/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c3", 252),
			("e7e6", 141),
			("d5b6", 50),
			("c7c6", 26),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/4P3/8/2n5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2c3", 165),
			("b2c3", 98),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/4P3/8/2P5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 44),
			("d7d6", 95),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 28),
			("e7e6", 295),
			("d7d5", 98),
			("c7c5", 208),
			("d7d6", 92),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 164),
			("c1g5", 71),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 123),
			("f8g7", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f7f5", 241),
			("c7c6", 144),
			("g8f6", 247),
			("c7c5", 84),
			("f8b4", 236),
			("f8e7", 68),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 57),
			("g1f3", 101),
			("c1f4", 54),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3P1p2/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 55),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 108),
			("g7g6", 270),
			("e7e6", 213),
			("b7b6", 193),
			("c7c6", 39),
			("d7d6", 116),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 114),
			("g2g3", 55),
			("d2d4", 47),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 40),
			("g7g6", 89),
			("d7d5", 33),
			("b8c6", 26),
			("b7b6", 39),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 130),
			("e2e4", 78),
			("e2e3", 31),
			("d2d4", 40),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 167),
			("e4d5", 64),
			("c1g5", 161),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d7", 1668),
			("f6e4", 66),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 73),
			("g2g3", 665),
			("g1f3", 1223),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 126),
			("d7d5", 45),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 100),
			("g1f3", 63),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 4195),
			("e8g8", 1191),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 27),
			("g8f6", 37),
			("f7f5", 30),
			("d7d6", 57),
			("f8c5", 93),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 830),
			("a1b1", 27),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 307),
			("g8f6", 606),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 27),
			("c1g5", 32),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 340),
			("f1c4", 29),
			("b1c3", 31),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8c5", 57),
			("d7d6", 74),
			("e5f4", 109),
			("f7f5", 57),
			("d7d5", 25),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f4e5", 38),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 221),
			("g2g3", 889),
			("b1c3", 898),
			("g1f3", 101),
			("e2e4", 107),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 2510),
			("g8f6", 93),
			("c7c5", 72),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 488),
			("b1c3", 1634),
			("g1f3", 325),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/5P2/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 2360),
			("e7e6", 324),
			("g8f6", 645),
			("e7e5", 1221),
			("c7c6", 444),
			("c7c5", 664),
			("g7g6", 567),
			("d7d6", 515),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 108),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 41),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8e7", 160),
			("d5c4", 86),
			("f8b4", 38),
			("c7c6", 176),
			("c7c5", 145),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 2008),
			("e2e3", 165),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 520),
			("g1f3", 75),
			("d2d4", 183),
			("g2g3", 468),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 120),
			("g7g6", 64),
			("e7e5", 142),
			("b8d7", 30),
			("g8f6", 116),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 90),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1f4", 79),
			("g1f3", 741),
			("e2e3", 549),
			("c4d5", 193),
			("e2e4", 478),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("d5c4", 26),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 2239),
			("b1c3", 335),
			("c2c4", 2804),
			("c1g5", 323),
			("e2e4", 1805),
			("e2e3", 340),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 201),
			("b8c6", 34),
			("e7e5", 64),
			("c7c6", 145),
			("c8g4", 108),
			("g7g6", 176),
			("b8d7", 53),
			("f7f5", 45),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 340),
			("g2g3", 126),
			("c1g5", 66),
			("c1f4", 65),
			("b1c3", 52),
			("e2e3", 86),
			("c2c3", 32),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8d7", 97),
			("c8g4", 94),
			("g7g6", 40),
			("c7c6", 59),
			("e7e5", 40),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 76),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 138),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("c2c3", 1176),
			("e1g1", 595),
			("d2d3", 545),
			("b2b4", 3636),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 1055),
			("d7d6", 74),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 39),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/6P1/PPPPPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 765),
			("g7g6", 1326),
			("d7d6", 619),
			("g8f6", 1149),
			("e7e5", 1038),
			("d7d5", 3629),
			("c7c5", 797),
			("e7e6", 433),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 643),
			("g1f3", 88),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 450),
			("d7d6", 71),
			("g7g6", 89),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 391),
			("b2b3", 138),
			("d2d4", 62),
			("g1f3", 418),
			("d2d3", 202),
			("e2e3", 172),
			("h2h3", 61),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d5c4", 65),
			("g8f6", 328),
			("e7e6", 48),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1a3", 42),
			("g1f3", 106),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/1P2P3/8/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5b4", 1053),
			("e7e6", 102),
			("e7e5", 71),
			("d7d6", 49),
			("b8c6", 49),
			("b7b6", 78),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/1p2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 265),
			("g1f3", 393),
			("a2a3", 377),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/1p1PP3/8/P1P2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 161),
			("e7e6", 42),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/1p1PP3/8/P1P2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 149),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3pP3/1p1P4/8/P1P2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 92),
			("c8f5", 37),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 4874),
			("e2e4", 2052),
			("g1f3", 2818),
			("g2g3", 296),
			("e2e3", 282),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 1988),
			("g8f6", 954),
			("f7f5", 818),
			("b7b6", 550),
			("f8b4", 559),
			("c7c5", 295),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 184),
			("e2e4", 250),
			("g1f3", 233),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 90),
			("g7g6", 64),
			("g8f6", 91),
			("e7e5", 48),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 125),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8c7", 787),
			("g8f6", 106),
			("b8d7", 219),
			("g7g6", 276),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1e3", 129),
			("f2f4", 181),
			("g1f3", 176),
			("g1e2", 198),
			("a2a4", 27),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 454),
			("g8f6", 1508),
			("c7c6", 231),
			("d7d6", 90),
			("f8c5", 427),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 9709),
			("b1c3", 1570),
			("d2d4", 744),
			("d2d3", 340),
			("e4e5", 528),
			("g1f3", 425),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3P4/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 5539),
			("g8f6", 2251),
			("c7c6", 1737),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 4403),
			("g1f3", 565),
			("d2d4", 373),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5a5", 1561),
			("d5d6", 929),
			("d5e5", 640),
			("d5d8", 1021),
			("d5e6", 204),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/q7/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 290),
			("f1e2", 78),
			("d2d4", 835),
			("f1c4", 210),
			("b2b4", 64),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/q7/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 179),
			("c7c6", 73),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 1206),
			("d2d4", 61),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 1152),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 248),
			("c2c4", 225),
			("d2d4", 318),
			("e2e3", 139),
			("d2d3", 83),
			("e2e4", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1503),
			("f2f4", 210),
			("c2c4", 213),
			("f1d3", 98),
			("g1f3", 239),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8d7", 184),
			("e7e5", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 432),
			("g1f3", 303),
			("g2g3", 356),
			("a2a4", 57),
			("f1c4", 87),
			("g1e2", 79),
			("f1b5", 50),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 115),
			("b8c6", 187),
			("e7e6", 61),
			("g8f6", 32),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/3p2p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 113),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/3p2p1/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 103),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d1e2", 145),
			("d2d3", 25),
			("e1g1", 52),
			("b5c6", 66),
			("b1c3", 83),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPPQPPP/RNB1K2R b KQkq - 0 0",
		&[
			("f8c5", 55),
			("d7d6", 49),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 81),
			("d7d6", 88),
			("c7c5", 62),
			("c7c6", 26),
			("d7d5", 37),
			("e7e5", 51),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e8g8", 289),
			("d7d6", 43),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/5N2/PPPNPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 58),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5N2/PPPNPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e3", 67),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 43),
			("f8d6", 47),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 263),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 152),
			("g8e7", 34),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 371),
			("c2c4", 126),
			("c2c3", 263),
			("b1c3", 123),
			("g2g3", 50),
			("a2a4", 39),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 367),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 57),
			("f3d4", 323),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 178),
			("c7c5", 137),
			("g8e7", 73),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/P7/8/1PPPPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 28),
			("e7e5", 185),
			("g8f6", 38),
			("d7d5", 110),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 55),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5d4", 1822),
			("d7d5", 104),
			("d7d6", 127),
			("g8f6", 139),
			("b8c6", 366),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3pP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 560),
			("g1f3", 535),
			("d1d4", 616),
			("f1c4", 81),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3pP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d4c3", 256),
			("d8e7", 37),
			("d7d5", 180),
			("b8c6", 35),
			("d4d3", 30),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/4P3/2p5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 206),
			("b1c3", 45),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2B1P3/2p5/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 38),
			("c3b2", 107),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 36),
			("g7g6", 40),
			("g8f6", 82),
			("c7c6", 57),
			("b8d7", 40),
		]
	),
	(
		"rnbqkbnr/p1pppppp/8/1p6/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 214),
			("g1f3", 123),
			("c2c4", 45),
			("e2e3", 43),
		]
	),
	(
		"rnbqkbnr/p1pppppp/8/1p6/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 148),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 2055),
			("e2e3", 615),
			("c1g5", 402),
			("c1f4", 663),
			("g2g3", 408),
			("b1c3", 36),
			("c2c3", 48),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 36),
			("d5c4", 467),
			("c7c6", 37),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 569),
			("d2d4", 1586),
			("b1c3", 167),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8e7", 223),
			("b8c6", 57),
			("g8f6", 55),
			("c7c6", 28),
			("c8e6", 129),
			("d8f6", 33),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d4", 97),
			("c2c3", 31),
			("e1g1", 51),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/3p4/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e5d4", 80),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 602),
			("e7e6", 240),
			("g8f6", 32),
			("d7d6", 127),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 577),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 568),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4p3/3nP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 59),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4p3/3nP3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5b6", 83),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p1p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 31),
			("g1f3", 64),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 49),
			("f2f3", 29),
			("g1f3", 88),
			("e4d5", 29),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 116),
			("e7e6", 31),
			("e7e5", 189),
			("d7d6", 89),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 27),
			("e4d5", 52),
			("d2d4", 83),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3pP3/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5d4", 181),
			("f6d7", 100),
			("f6e4", 292),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/4P3/3p4/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e2", 39),
			("e5f6", 137),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 54),
			("g8f6", 78),
			("f7f5", 203),
			("d7d6", 266),
			("f8c5", 118),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 105),
			("f1c4", 56),
			("g1f3", 88),
			("g2g3", 137),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d5", 439),
			("c7c6", 38),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 158),
			("c3d5", 371),
			("d2d4", 94),
			("f1c4", 533),
			("d1f3", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 51),
			("d5c3", 45),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1485),
			("c2c4", 211),
			("f2f4", 96),
			("g1f3", 175),
			("f1d3", 60),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 199),
			("c7c6", 73),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 366),
			("c1e3", 931),
			("f2f4", 472),
			("c1g5", 87),
			("f1e2", 54),
			("g1e2", 82),
			("f2f3", 59),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 171),
			("c7c6", 48),
			("b8c6", 30),
			("a7a6", 51),
			("c8g4", 57),
			("b8d7", 32),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 187),
			("d2d4", 34),
			("g1f3", 104),
			("f2f4", 171),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 169),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 161),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("b8c6", 115),
			("e7e6", 28),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 29),
			("a7a6", 294),
			("b8c6", 157),
			("c7c6", 176),
			("b8d7", 43),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 270),
			("c2c4", 1785),
			("c1g5", 283),
			("g2g3", 377),
			("c1f4", 447),
			("e2e3", 488),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 42),
			("g8f6", 136),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 38),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/4P3/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8g4", 53),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8c5", 115),
			("d7d6", 85),
			("e5f4", 196),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 101),
			("d5e4", 800),
			("e7e6", 76),
			("d5d4", 535),
			("g8f6", 80),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 973),
			("g1f3", 702),
			("d1f3", 69),
			("f2f4", 213),
			("e4d5", 296),
			("d2d3", 85),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 519),
			("c1g5", 80),
			("e2e4", 634),
			("g1f3", 326),
			("e2e3", 94),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 528),
			("e7e6", 30),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 386),
			("g1f3", 40),
			("d4d5", 63),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 54),
			("e7e6", 33),
			("g7g6", 93),
			("g8f6", 33),
		]
	),
	(
		"rn1qkbnr/pbp1pppp/1p1p4/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 39),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d5e4", 146),
			("g8f6", 96),
			("c7c5", 43),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3Pp3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d3e4", 146),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3PB3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 109),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8d7", 8621),
			("b8d7", 1849),
			("b8c6", 1797),
		]
	),
	(
		"rn1qkbnr/pp1bpppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("b5d7", 8454),
		]
	),
	(
		"rn1qkbnr/pp1Bpppp/3p4/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d8d7", 8058),
			("b8d7", 395),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 237),
			("e7e6", 32),
			("d5c4", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 543),
			("g1f3", 91),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 32),
			("b8d7", 61),
			("e7e5", 33),
			("c8f5", 100),
			("b8c6", 27),
			("c7c6", 53),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 609),
			("c1g5", 27),
			("g1f3", 85),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 821),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1g5", 60),
			("g1f3", 114),
			("c4d5", 62),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8e7", 25),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 228),
			("c7c5", 25),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 75),
			("c2c4", 65),
			("f1e2", 26),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e8g8", 61),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 144),
			("e5d6", 62),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4P3/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 43),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4P3/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 36),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 91),
			("g7g6", 37),
			("e7e5", 31),
			("g8f6", 192),
			("d7d6", 43),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 40),
			("g1f3", 26),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 53),
			("g1f3", 111),
			("e4d5", 32),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3P4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 52),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g6", 432),
			("f6e4", 1679),
			("e7e6", 1220),
			("c7c5", 912),
			("d7d5", 588),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 39),
			("g5f6", 329),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 1001),
			("c7c5", 179),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 796),
			("b1c3", 46),
			("d1e2", 76),
			("e4d5", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 388),
			("b8c6", 53),
			("c7c5", 213),
			("g7g6", 28),
			("d5e4", 71),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/6B1/3Pn3/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f4", 1088),
			("g5h4", 287),
			("h2h4", 225),
			("g5c1", 61),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3PnB2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 443),
			("c7c5", 488),
			("g7g5", 51),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3PnB2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 174),
			("f2f3", 146),
			("b1d2", 113),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3PnB2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 59),
			("c8f5", 49),
		]
	),
	(
		"rnbqkbnr/ppp3pp/3p1p2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 33),
			("b1c3", 30),
		]
	),
	(
		"rnbqk1nr/ppppbppp/8/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 87),
		]
	),
	(
		"rnbqk1nr/ppppbppp/8/8/2B1Pp2/5N2/PPPP2PP/RNBQK2R b KQkq - 0 0",
		&[
			("e7h4", 66),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 103),
			("d2d3", 44),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 38),
			("e5f4", 29),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/5P2/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 504),
			("e7e6", 98),
			("e7e5", 596),
			("g7g6", 103),
			("g8f6", 101),
			("c7c6", 130),
			("c7c5", 121),
			("d7d6", 120),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e1f2", 155),
			("g1h3", 46),
			("e2e4", 147),
			("d2d4", 91),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 0",
		&[
			("e7e5", 83),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/5P2/PPPPPKPP/RNBQ1BNR w kq - 0 0",
		&[
			("g2g3", 64),
			("f2g3", 46),
			("e2e3", 85),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6c5", 529),
			("c8f5", 2318),
			("e7e6", 120),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2ppP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4c5", 254),
			("c2c3", 115),
			("g1f3", 69),
			("c2c4", 42),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2PpP3/8/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 126),
			("e7e6", 118),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 299),
			("e7e6", 111),
			("c7c5", 178),
			("d7d5", 200),
			("d7d6", 31),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 286),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 212),
			("d7d5", 67),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 28),
			("e2e4", 77),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 336),
			("g1f3", 302),
			("b1c3", 212),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d6e5", 305),
			("b8c6", 36),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1d8", 299),
		]
	),
	(
		"rnbQkbnr/ppp2ppp/8/4p3/4P3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("e8d8", 299),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("e4e5", 93),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f6d7", 91),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 37),
			("f6e4", 256),
			("g7g6", 35),
			("e7e6", 30),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 36),
			("e7e6", 25),
			("g8f6", 65),
			("c7c5", 27),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f7f5", 373),
			("g8f6", 516),
			("d5c4", 556),
			("b8d7", 39),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8e7", 186),
			("g8f6", 1783),
			("c7c5", 1329),
			("d5e4", 597),
			("b8c6", 278),
			("a7a6", 112),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1d3", 64),
			("g1f3", 70),
			("e4e5", 28),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/3PP3/3B4/PPPN1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c5", 53),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/3P4/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 1395),
			("g8f6", 367),
			("d7d6", 307),
			("g7g6", 352),
			("c7c5", 251),
			("e7e6", 160),
			("e7e5", 326),
			("c7c6", 227),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 144),
			("c2c3", 80),
			("b1d2", 264),
			("e2e4", 215),
			("g2g3", 478),
			("e2e3", 52),
			("b1c3", 38),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/3P1N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 75),
			("c7c6", 45),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/3P1N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 38),
			("c1g5", 25),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2PP4/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b7b6", 30),
			("d7d5", 82),
			("c7c5", 38),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1d3", 322),
			("e4e5", 1439),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3PP3/3B4/PPPN1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("d5e4", 36),
			("c7c5", 223),
			("b7b6", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 3112),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 1591),
			("f1d3", 684),
			("g1f3", 586),
			("c2c3", 369),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 1329),
			("b8c6", 188),
			("e7e6", 173),
			("g7g6", 53),
			("d5c4", 123),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 178),
			("d7d5", 160),
			("d7d6", 35),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 51),
			("f2f4", 34),
			("g1f3", 33),
			("b1c3", 70),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 244),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c7c6", 134),
			("e7e6", 40),
			("c8f5", 53),
			("g7g6", 61),
			("c8g4", 34),
			("c7c5", 35),
			("b7b5", 38),
			("b8c6", 38),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 391),
			("c2c4", 55),
			("d2d4", 28),
			("d2d3", 58),
			("b2b3", 38),
			("h2h3", 41),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("c8g4", 131),
			("c8f5", 124),
			("e7e6", 43),
			("g7g6", 54),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 141),
			("b8d7", 57),
			("c8g4", 28),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 180),
			("c1g5", 81),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 142),
			("f8b4", 71),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e1f2", 39),
			("g1h3", 34),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 195),
			("c7c5", 58),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 131),
			("e4d5", 51),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/4P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 131),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/4P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1b3", 28),
			("e4d5", 90),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 928),
			("g8f6", 187),
			("f7f5", 68),
			("c7c5", 37),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 838),
			("c4d5", 30),
			("g1f3", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 38),
			("c7c6", 143),
			("d5c4", 131),
			("f7f5", 29),
			("c7c5", 42),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 59),
			("g1f3", 262),
			("e2e3", 59),
			("c4d5", 90),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8e7", 31),
			("d5c4", 249),
			("f8b4", 35),
			("b8d7", 32),
			("c7c6", 31),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 149),
			("g7g6", 172),
			("d7d6", 177),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 566),
			("d4d5", 2884),
			("e2e3", 897),
			("d4c5", 526),
			("c2c4", 623),
			("g1f3", 1014),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 370),
			("g8f6", 70),
			("e7e6", 84),
			("d7d5", 55),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c3d4", 351),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 199),
			("g7g6", 39),
			("g8f6", 60),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1059),
			("c1f4", 121),
			("g1f3", 468),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 55),
			("g8f6", 138),
			("e7e6", 107),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 186),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 6280),
			("c2c3", 281),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 1491),
			("a7a6", 2985),
			("b8c6", 1578),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 278),
			("g1f3", 73),
			("b1c3", 60),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e8f7", 216),
			("e7e6", 26),
			("g7g5", 97),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 254),
			("e7e6", 78),
			("g7g6", 38),
			("c8f5", 56),
			("c7c6", 54),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4d5", 769),
			("g1f3", 424),
			("c2c3", 87),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8d5", 370),
			("e6d5", 389),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 59),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c1g5", 95),
			("e2e3", 34),
			("c1f4", 32),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 5422),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 133),
			("c2c3", 72),
			("f1c4", 72),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 1130),
			("g8f6", 2211),
			("g7g6", 1086),
			("e7e6", 475),
			("d8b6", 172),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/4P3/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 362),
			("d7d5", 1386),
			("c7c5", 506),
			("e7e5", 927),
			("g8f6", 567),
			("g7g6", 448),
			("c7c6", 348),
			("d7d6", 290),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 83),
			("g1f3", 37),
			("b2b3", 48),
			("d2d3", 78),
			("g1e2", 48),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 651),
			("g2g3", 300),
			("b2b3", 49),
			("d2d4", 325),
			("e2e4", 70),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 525),
			("d7d6", 70),
			("g8f6", 41),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 483),
			("g2g3", 301),
			("d2d4", 659),
			("c4d5", 436),
			("b2b3", 153),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 407),
			("e7e6", 39),
			("c8g4", 27),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 42),
			("d2d4", 51),
			("b2b3", 55),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 83),
			("a7a6", 66),
			("e7e6", 147),
			("g7g6", 39),
		]
	),
	(
		"rnbq1bnr/pppppkpp/5p2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQ - 0 0",
		&[
			("f2f4", 29),
			("f1c4", 28),
			("f1d3", 39),
			("b1c3", 58),
			("g1f3", 33),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/4P3/8/2P5/P1PP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 30),
			("d7d6", 53),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/P4N2/1P2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8a6", 128),
			("c8b7", 608),
			("f8e7", 334),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 359),
			("g1e2", 194),
			("g1f3", 167),
			("c2c4", 82),
			("b2b3", 142),
			("d2d3", 249),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4c5", 29),
			("f2f4", 40),
			("g1f3", 118),
			("c2c3", 93),
			("c2c4", 47),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 2587),
			("e2e4", 1624),
			("e2e3", 220),
			("g1f3", 1542),
			("b1c3", 242),
			("c1g5", 191),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 593),
			("c7c5", 175),
			("e7e6", 36),
			("c7c6", 25),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 162),
			("f2f4", 30),
			("g1f3", 72),
			("f2f3", 26),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 80),
			("e7e5", 25),
			("b8c6", 27),
			("b8d7", 96),
			("a7a6", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 127),
			("c8g4", 100),
			("b8d7", 52),
			("g8f6", 99),
			("b8c6", 95),
			("f7f6", 66),
			("d8e7", 40),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 689),
			("d1d4", 206),
			("c2c3", 32),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 444),
			("c7c5", 74),
			("c8d7", 30),
			("b8c6", 70),
			("g7g6", 25),
			("f8e7", 28),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3Pn2B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 133),
			("g7g5", 55),
			("d7d5", 80),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2p5/3Pn2B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 45),
			("f2f3", 62),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4p3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e4", 1032),
			("f2f3", 149),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4N3/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 403),
			("b8d7", 158),
			("f7f5", 36),
			("c8f5", 175),
			("e7e5", 105),
			("b8c6", 26),
			("d8d5", 77),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4f6", 195),
			("e4g3", 130),
			("d1e2", 52),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5N2/8/8/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7f6", 163),
			("g7f6", 32),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 948),
			("c7c6", 255),
			("d7d5", 157),
			("e7e6", 447),
			("d7d6", 83),
			("c7c5", 197),
			("e7e5", 245),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 930),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 65),
			("d7d5", 119),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e3", 59),
			("b1c3", 116),
			("g1f3", 36),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/4P1P1/PP1P1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e8g8", 160),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 923),
			("b2b3", 166),
			("e2e4", 637),
			("c2c4", 1321),
			("d2d4", 291),
			("e2e3", 97),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 72),
			("g8f6", 56),
			("b8c6", 71),
			("b7b6", 90),
			("e7e6", 62),
			("g7g6", 36),
			("d7d6", 31),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 476),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 470),
			("g8f6", 44),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f6", 164),
			("e2e4", 497),
			("e2e3", 218),
			("g1f3", 124),
			("b1d2", 101),
			("g1h3", 56),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pB2/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7f6", 30),
			("d8f6", 132),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 902),
			("d2d4", 948),
			("b1c3", 860),
			("e2e4", 436),
			("g1f3", 597),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 688),
			("d7d6", 60),
			("g7g6", 78),
			("g8f6", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 477),
			("g1f3", 172),
			("c4d5", 53),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 191),
			("c4d5", 37),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 31),
			("g7g6", 111),
			("e7e5", 71),
			("b8d7", 26),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 62),
			("g1f3", 38),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 177),
			("d5d4", 381),
			("c7c6", 1035),
			("e7e6", 526),
			("d5c4", 364),
			("c7c5", 73),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c4d5", 48),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 45),
			("e7e6", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2B5/4PN2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 31),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/4P1P1/PP1P1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8e7", 32),
			("d5c4", 30),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2pP4/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 795),
			("e7e5", 639),
			("g8f6", 518),
			("e7e6", 530),
			("g7g6", 108),
			("b7b5", 163),
			("f7f5", 98),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 161),
			("b1c3", 200),
			("c2c4", 385),
			("g1f3", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2pP4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 62),
			("e7e5", 27),
			("g7g6", 31),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c4d5", 581),
			("b1c3", 35),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3Pp3/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f6d5", 601),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 514),
			("g2g3", 431),
			("e2e4", 79),
			("b2b3", 56),
			("d2d4", 370),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 256),
			("f7f5", 64),
			("g8f6", 68),
			("b7b6", 29),
			("c7c5", 39),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 393),
			("e2e3", 135),
			("b2b3", 107),
			("d2d4", 323),
			("c4d5", 61),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 36),
			("d5c4", 36),
			("c7c6", 77),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 258),
			("d2d4", 60),
			("b2b3", 49),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8e7", 35),
			("c7c5", 91),
			("c7c6", 25),
			("d5c4", 51),
			("f8d6", 25),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2p1p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e3", 30),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 570),
			("d2d4", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 76),
			("g8f6", 105),
			("c7c6", 208),
			("b8c6", 64),
			("g7g6", 60),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 29),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e1f2", 223),
			("g1h3", 76),
			("g2g4", 131),
			("e2e4", 45),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 0",
		&[
			("d7d5", 145),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 2416),
			("b1c3", 178),
			("e2e4", 665),
			("g1f3", 1353),
			("c1g5", 292),
			("c2c3", 164),
			("e2e3", 259),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 250),
			("f7f5", 54),
			("b7b6", 32),
			("g8f6", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 407),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 323),
			("c7c6", 32),
			("f7f5", 54),
			("c7c5", 76),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 370),
			("c2c4", 45),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8d6", 44),
			("f8e7", 193),
			("c7c5", 110),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 25),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 1089),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 281),
			("e7e6", 122),
			("e7e5", 97),
			("d7d6", 52),
			("g7g6", 426),
			("c7c5", 48),
			("c7c6", 35),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d3", 97),
			("g1f3", 157),
			("c2c4", 174),
			("d2d4", 36),
			("b2b3", 29),
			("e2e3", 111),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 36),
			("c1g5", 30),
			("c2c4", 26),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c7c5", 115),
			("b7b5", 28),
			("f8e7", 78),
			("b8d7", 33),
			("f8d6", 26),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 238),
			("b1c3", 106),
			("d4d5", 68),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 35),
			("e7e5", 27),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("a2a3", 35),
			("b1c3", 74),
			("g2g3", 29),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/P4N2/1P2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 57),
			("d7d5", 55),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 5787),
			("c7c6", 248),
			("d7d6", 260),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 933),
			("b1c3", 3364),
			("c1e3", 303),
			("f2f4", 198),
			("c2c3", 328),
			("c2c4", 386),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 141),
			("c7c6", 77),
			("d7d6", 434),
			("e7e6", 36),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4c5", 75),
			("c2c3", 76),
			("b1c3", 32),
			("d4d5", 41),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2P5/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8a5", 80),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 131),
			("g8f6", 169),
			("f7f5", 32),
			("f8c5", 29),
			("d7d5", 43),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 26),
			("b1d2", 39),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 823),
			("c7c6", 131),
			("d7d5", 300),
			("g7g6", 1224),
			("e7e5", 543),
			("c7c5", 381),
			("d7d6", 112),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 68),
			("g2g3", 163),
			("d2d4", 46),
			("g1f3", 124),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 31),
			("f8b4", 32),
			("d7d6", 71),
			("c7c5", 81),
			("d7d5", 203),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 226),
			("e2e3", 428),
			("b1c3", 819),
			("d1c2", 298),
			("c4d5", 104),
			("b1d2", 83),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f5", 61),
			("g8f6", 69),
			("d5c4", 77),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 65),
			("c7c5", 296),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 742),
			("e4d5", 766),
			("b1c3", 556),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3pP3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 653),
			("g8e7", 35),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 71),
			("b2b4", 262),
			("d2d4", 116),
			("f1e2", 84),
			("f1b5", 102),
		]
	),
	(
		"r1bqk1nr/ppppbppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d3", 98),
			("e1g1", 123),
			("d2d4", 154),
			("c2c3", 29),
			("b1c3", 41),
		]
	),
	(
		"r1bqk1nr/ppppbppp/2n5/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 91),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/3p4/4p3/3PP1b1/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4e5", 82),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/3p4/4P3/4P1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f3", 56),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 149),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g7g6", 106),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 153),
			("f2f3", 131),
			("e2e3", 59),
			("c4d5", 52),
			("c1f4", 185),
			("c1g5", 189),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/8/PPQ1PPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b5c4", 49),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 49),
			("c8g4", 168),
			("b8c6", 55),
			("e7e5", 131),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 53),
			("g2g3", 122),
			("b1d2", 65),
			("e2e4", 74),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3PPP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8c7", 146),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PPP2/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 117),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PPP2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8d7", 92),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 107),
			("g1f3", 80),
			("d4e5", 170),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/3Pp3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6e7", 601),
			("c6b8", 25),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 180),
			("b1c3", 35),
			("c1e3", 53),
			("f2f4", 88),
			("g1f3", 121),
			("f1d3", 39),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/2P1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7g6", 111),
			("g8f6", 149),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/3q4/8/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 510),
			("g1f3", 215),
			("f1e2", 39),
			("c3e4", 52),
			("f1c4", 49),
			("g2g3", 31),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/3q4/8/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 431),
			("c7c6", 43),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 35),
			("g1f3", 149),
			("f1e2", 30),
			("c1e3", 86),
			("f2f3", 68),
			("a2a4", 30),
			("c1g5", 46),
			("g1e2", 35),
			("f1d3", 47),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8a5", 163),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 80),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 382),
			("d4e5", 254),
			("g1e2", 59),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 43),
			("e5d4", 74),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 348),
			("g8f6", 260),
			("e7e6", 28),
			("g7g6", 36),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8c5", 361),
			("d7d6", 140),
			("f8d6", 29),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 805),
			("b2b3", 83),
			("g2g3", 477),
			("c2c4", 527),
			("e2e4", 73),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 98),
			("c1f4", 64),
			("g2g3", 106),
			("c2c4", 264),
			("e2e3", 35),
			("b1c3", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 143),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 50),
			("c2c3", 99),
			("f1c4", 26),
			("c1e3", 27),
			("f1d3", 53),
			("c2c4", 52),
			("h2h3", 100),
			("f1e2", 71),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 62),
			("c8g4", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 406),
			("b1c3", 90),
			("d1a4", 29),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 394),
			("b7b5", 45),
			("c8g4", 107),
			("a7a6", 29),
			("c7c5", 25),
			("c8e6", 40),
			("b8c6", 26),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 74),
			("b1c3", 61),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 536),
			("d7d6", 107),
			("b7b5", 202),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 315),
			("d2d4", 145),
			("g1f3", 219),
			("h2h4", 37),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 881),
			("c7c6", 63),
			("d7d5", 26),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 326),
			("g1f3", 114),
			("g2g3", 52),
			("f1c4", 108),
			("f2f4", 199),
			("h2h4", 37),
			("d2d3", 26),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 1816),
			("c7c6", 506),
			("c7c5", 595),
			("h7h6", 146),
			("e7e6", 110),
			("b7b6", 34),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 30),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 91),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 72),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/3Pp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6e7", 73),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e3", 93),
			("g1f3", 955),
			("c4d5", 1651),
			("c1f4", 152),
			("c1g5", 161),
			("f2f3", 158),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 54),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 876),
			("g2g3", 347),
			("b2b4", 86),
			("d2d4", 241),
			("e2e3", 44),
			("b2b3", 49),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 718),
			("d7d6", 42),
			("d7d5", 221),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 169),
			("e2e4", 366),
			("e2e3", 35),
			("g2g3", 126),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 93),
			("c8g4", 67),
			("c8f5", 72),
			("a7a6", 25),
			("g7g6", 153),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 997),
			("d5c4", 50),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 167),
			("a2a3", 32),
			("e4d5", 86),
			("g1e2", 27),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3pP3/1b1P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8e7", 592),
			("c7c5", 1748),
			("b7b6", 167),
			("d8d7", 74),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 70),
			("c7c6", 42),
			("c8f5", 32),
			("c7c5", 43),
			("c8g4", 34),
			("g7g6", 63),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 104),
			("c2c4", 25),
			("b1c3", 45),
			("f1e2", 136),
			("b1d2", 73),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c7c5", 28),
			("f8e7", 37),
			("f8d6", 54),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 183),
			("e2e4", 193),
			("g1f3", 129),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 111),
			("d7d6", 35),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1g5", 116),
			("e2e4", 276),
			("g1f3", 57),
			("c1f4", 71),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 40),
			("d8b6", 41),
			("h7h6", 37),
			("c8f5", 49),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 295),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 32),
			("g1f3", 179),
			("f2f4", 33),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/7P/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 391),
			("e7e6", 31),
			("e7e5", 286),
			("g8f6", 65),
			("d7d6", 44),
			("g7g6", 66),
			("c7c5", 71),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 25),
			("a2a3", 184),
			("g2g4", 27),
			("b2b4", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 39),
			("c1e3", 50),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 53),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4c5", 155),
			("g1f3", 141),
			("g1e2", 114),
			("c1e3", 191),
			("d4d5", 171),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2P5/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8a5", 80),
			("g7c3", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 332),
			("b8c6", 349),
			("g7g6", 26),
			("e7e6", 28),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 540),
			("e2e3", 34),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 178),
			("g7g6", 197),
			("e7e6", 28),
			("d7d6", 53),
			("c7c5", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 394),
			("b2b3", 69),
			("g2g3", 79),
			("d2d4", 48),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 626),
			("g8f6", 667),
			("e7e5", 867),
			("b8d7", 130),
			("g7g6", 480),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 796),
			("e2e4", 66),
			("g1f3", 189),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8c7", 60),
			("g8f6", 102),
			("g7g6", 172),
			("b8d7", 106),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 49),
			("g1f3", 45),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8d7", 108),
			("g8f6", 27),
			("g7g6", 27),
			("e7e5", 32),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c4", 42),
			("e7e6", 49),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 515),
			("g1f3", 55),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 99),
			("e5e4", 264),
			("e5f4", 138),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3Pp3/5P2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5c6", 25),
			("b1c3", 26),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 155),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2p5/3Pn2B/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g5", 36),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c3d4", 48),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 240),
			("d7d6", 52),
			("g8f6", 40),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 756),
			("e2e4", 420),
			("c1g5", 454),
			("g1f3", 745),
			("b1c3", 500),
			("g2g3", 799),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 696),
			("g7g6", 34),
			("e7e6", 66),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 429),
			("g2g3", 170),
			("g1f3", 77),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 35),
			("e7e6", 129),
			("d7d6", 81),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("h2h4", 69),
			("g2g3", 45),
			("g1f3", 61),
			("c1g5", 27),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP3P/2N5/PP2PPP1/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 30),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 40),
			("g2g3", 94),
			("b1d2", 72),
			("g1f3", 33),
			("c2c3", 29),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 107),
			("g7g6", 52),
			("e7e5", 39),
			("c7c6", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 41),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 69),
			("e7e5", 30),
			("e7e6", 42),
			("d7d6", 26),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 29),
			("d2d4", 37),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("h7h6", 33),
			("c7c6", 101),
			("b8d7", 37),
			("g8f6", 43),
			("g7g6", 42),
			("f7f6", 30),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 4998),
			("b1c3", 1038),
			("g1f3", 723),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 231),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8c7", 166),
			("b8d7", 36),
			("g7g6", 54),
			("g8f6", 28),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 127),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 98),
			("d2d3", 107),
			("g1e2", 96),
			("b2b3", 49),
			("g1f3", 66),
			("c2c4", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c6", 365),
			("c8f5", 183),
			("h7h6", 336),
			("c7c5", 187),
			("g8f6", 194),
			("b8c6", 52),
			("f7f6", 138),
			("b8d7", 58),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b1c3", 81),
			("c2c3", 118),
			("e2e3", 182),
			("g1f3", 62),
			("c2c4", 27),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 107),
			("d7d5", 190),
			("e7e6", 123),
			("g7g6", 180),
			("d7d6", 36),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 40),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 84),
			("c1f4", 83),
			("g2g3", 30),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b7b6", 41),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pPp3/8/8/PPP1PPPP/RNBQKBNR w KQkq  - 0 0",
		&[
			("b1c3", 167),
			("e2e4", 198),
			("c2c4", 233),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3QP3/5N2/PPP2PPP/RNB1KB1R b KQkq - 0 0",
		&[
			("g8f6", 135),
			("b8c6", 256),
			("a7a6", 110),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8g7", 549),
			("f6e4", 88),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("b1d2", 307),
			("g5f6", 49),
			("b1c3", 28),
			("e2e3", 111),
			("c2c3", 37),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/6B1/3P4/5N2/PPPNPPPP/R2QKB1R b KQkq - 0 0",
		&[
			("e8g8", 140),
			("d7d6", 65),
			("d7d5", 75),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 168),
			("e7e6", 174),
			("g7g6", 95),
			("c8f5", 78),
			("c7c5", 88),
			("c8g4", 49),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 181),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8f5", 65),
			("c8g4", 49),
			("g7g6", 34),
			("e7e6", 26),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 85),
			("f1d3", 148),
			("f1e2", 42),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 181),
			("e2e3", 257),
			("c1g5", 28),
			("g2g3", 145),
			("c1f4", 571),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8b7", 985),
			("f8b4", 183),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 515),
			("c7c5", 63),
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8g7", 70),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 64),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("g8f6", 49),
			("b8d7", 29),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 40),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4p3/3nP3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 30),
			("d2d4", 30),
			("c3d5", 55),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/7N/PPPPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 219),
			("e7e5", 85),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 298),
			("g8f6", 615),
			("g7g6", 380),
			("e7e6", 57),
			("c7c5", 294),
			("c7c6", 168),
			("c8f5", 54),
			("b8c6", 106),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/5Pb1/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 213),
			("f3e5", 64),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8d7", 65),
			("g4f3", 113),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6d5", 586),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8g4", 59),
			("e7e6", 67),
			("c8f5", 30),
			("a7a6", 187),
			("g7g6", 33),
		]
	),
	(
		"rnbqkbnr/ppppppp1/7p/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 36),
			("d2d4", 128),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1b5", 29),
			("d2d4", 128),
			("f1c4", 39),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("c8d7", 36),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/2PP4/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 29),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d5", 172),
			("b7b6", 45),
			("h7h6", 260),
			("c7c5", 32),
			("f8e7", 54),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 84),
			("g5f6", 35),
			("b1d2", 35),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8e7", 51),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 394),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 39),
			("b1c3", 174),
			("g1f3", 86),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/3P4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6b8", 26),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 356),
			("e7e6", 136),
			("d7d6", 45),
			("g7g6", 60),
			("e7e5", 41),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 233),
			("g1f3", 56),
			("d2d3", 39),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 153),
			("e7e6", 42),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d3", 59),
			("g1f3", 29),
			("g2g3", 83),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2NP4/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 55),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 593),
			("e7e5", 144),
			("b8c6", 42),
			("g7g6", 221),
			("b8d7", 41),
			("c7c6", 164),
			("c7c5", 50),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 434),
			("e4e5", 54),
			("d2d4", 25),
			("d2d3", 52),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 282),
			("c7c6", 53),
			("c8g4", 26),
			("e7e5", 53),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1c4", 55),
			("d2d4", 286),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f8g7", 54),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 62),
			("g7g6", 83),
			("d5e4", 118),
			("d5d4", 141),
			("g8f6", 60),
			("e7e6", 42),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("h2h3", 58),
			("e4d5", 42),
			("f1e2", 40),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/4P1b1/2N2N1P/PPPP1PP1/R1BQKB1R b KQkq - 0 0",
		&[
			("g4f3", 312),
			("g4h5", 98),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/1B6/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c6d4", 49),
			("e7e5", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 86),
			("c4d5", 781),
			("g2g3", 163),
			("d2d4", 38),
			("e2e3", 35),
			("g1f3", 44),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c4", 27),
			("d5d4", 39),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8d7", 146),
		]
	),
	(
		"r2qkbnr/pppbpppp/2np4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1e2", 46),
			("f2f4", 33),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 656),
			("e7e6", 210),
			("g7g6", 183),
			("d7d6", 152),
			("g8f6", 37),
			("d7d5", 76),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d5", 1651),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p3B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("d4d5", 424),
			("d4c5", 35),
			("g5f6", 369),
			("b1c3", 51),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2pP2B1/8/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 179),
			("f6e4", 117),
			("e7e6", 39),
			("d7d6", 44),
		]
	),
	(
		"rnb1kb1r/pp1ppppp/1q3n2/2pP2B1/8/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b1c3", 153),
		]
	),
	(
		"rnb1kb1r/pp1ppppp/1q3n2/2pP2B1/8/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("b6b2", 97),
			("h7h6", 30),
		]
	),
	(
		"rnbqkb1r/pppppppp/7n/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 38),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/3nP3/2B5/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d5b6", 48),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4P3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6g4", 834),
			("f6e4", 568),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P3n1/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 62),
			("g1f3", 335),
			("c1f4", 317),
			("e2e3", 58),
			("b1c3", 27),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P1P1n1/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g4e5", 55),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 230),
			("f2f4", 159),
			("d2d4", 801),
			("g2g3", 86),
			("e4d5", 203),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 100),
			("d5e4", 50),
			("d5d4", 42),
			("c7c5", 40),
			("b8c6", 27),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 59),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3N4/8/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8d5", 371),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3pP3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e2", 968),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3pP3/8/PPPPNPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8g4", 49),
			("d4d3", 47),
			("e7e5", 361),
			("c7c5", 369),
			("b8c6", 43),
			("f7f5", 32),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("a7a6", 100),
			("b8c6", 428),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 107),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 25),
			("d8c7", 63),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 220),
			("g2g3", 372),
			("e2e4", 45),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 75),
			("g1f3", 34),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8d7", 29),
			("e7e5", 75),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 72),
			("g1f3", 293),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f2f4", 43),
			("g1f3", 276),
			("f1d3", 118),
			("c2c4", 93),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("b8d7", 25),
			("c8f5", 33),
			("h7h6", 27),
			("c7c6", 63),
			("e7e6", 52),
			("c7c5", 46),
			("f6e4", 27),
		]
	),
	(
		"r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("g1f3", 50),
			("d1d3", 57),
			("e2e3", 29),
			("f2f3", 27),
		]
	),
	(
		"r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
		&[
			("e7e6", 48),
			("g7g6", 50),
			("h7h6", 32),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/6B1/3Pn2P/8/PPP1PPP1/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 60),
			("c7c5", 88),
			("h7h6", 32),
			("e4g5", 29),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p2B1/3Pn2P/8/PPP1PPP1/RN1QKBNR w KQkq - 0 0",
		&[
			("b1d2", 27),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 53),
			("g8f6", 50),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 151),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 941),
			("b8c6", 27),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 155),
			("f1c4", 108),
			("h2h3", 275),
			("d2d4", 30),
			("d2d3", 53),
			("f1e2", 221),
			("d1c2", 44),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2PB1N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 78),
			("g7g6", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 54),
			("c7c5", 42),
			("g8f6", 125),
			("c8g4", 59),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3pP3/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d7", 1368),
			("f6e4", 45),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8f5", 55),
			("d8b6", 57),
			("b8d7", 36),
			("h7h6", 58),
			("g8f6", 102),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 64),
			("f7f5", 125),
			("f8c5", 29),
			("d7d6", 103),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1b5", 32),
			("a2a3", 103),
			("f1e2", 55),
			("f1c4", 74),
			("d2d4", 31),
			("g2g3", 106),
			("d2d3", 60),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f8b4", 25),
			("c6d4", 298),
			("d7d6", 62),
			("f8c5", 78),
			("f8d6", 46),
			("a7a6", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 512),
			("d2d4", 866),
			("g1f3", 246),
			("c2c4", 405),
			("f1b5", 168),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1bB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 103),
			("c2c4", 26),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1bB1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c6", 66),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1bB1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f1d3", 44),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3P4/3n4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7d6", 54),
			("e7d6", 62),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 2422),
			("d7d6", 479),
			("g8f6", 135),
			("g7g6", 269),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3PP3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d6", 134),
			("c7c5", 82),
			("c7c6", 47),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/4B3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("d1d2", 52),
			("c2c3", 28),
			("b1c3", 28),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/q7/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c6", 26),
			("g8f6", 35),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8f5", 105),
			("e7e5", 31),
			("c7c5", 91),
			("e7e6", 80),
			("g8f6", 220),
			("c7c6", 138),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 39),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/7P/8/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 258),
			("e7e5", 233),
			("g8f6", 40),
			("c7c5", 39),
			("b8c6", 25),
			("c7c6", 28),
			("e7e6", 32),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/7P/8/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("h4h5", 157),
			("g2g4", 26),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/P6P/8/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("b2b4", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 88),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pP3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5d4", 62),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4P3/3pP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 48),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 278),
			("d4e5", 288),
			("d4d5", 120),
			("b1c3", 141),
			("e2e3", 36),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 29),
			("e5e4", 152),
			("b8c6", 55),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 643),
			("g7g6", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3Pp3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2e4", 619),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("b8d7", 55),
			("f8e7", 44),
			("c7c6", 56),
			("f8b4", 38),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("d7d6", 85),
			("f6e4", 1065),
			("f8e7", 77),
			("f8c5", 26),
		]
	),
	(
		"rnbqkb1r/pppppp1p/8/6p1/3Pn2B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f2f3", 29),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5e4", 128),
			("e5d4", 82),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3Pp3/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d2", 116),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3Pp3/8/PPPNPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 81),
			("e4e3", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3Pp3/8/PPPNPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 65),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3Pp3/4P3/PPPN1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 32),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 276),
			("f1c4", 407),
			("g1f3", 429),
			("g2g3", 34),
			("d2d4", 42),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/5P2/1P3N2/P1PPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 38),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/2P5/PP2PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 41),
			("c8f5", 26),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 153),
			("g1f3", 280),
			("e2e4", 240),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 162),
			("c7c5", 26),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 308),
			("d2d4", 1141),
			("e2e4", 97),
			("g2g3", 180),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 261),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 31),
			("b1c3", 48),
			("d2d3", 28),
			("g2g3", 39),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 34),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/1P6/P1PPPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 656),
			("e7e6", 196),
			("e7e5", 980),
			("d7d5", 938),
			("c7c5", 293),
			("d7d6", 254),
			("b7b6", 141),
			("g7g6", 195),
			("c7c6", 146),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 620),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g6", 246),
			("d7d5", 90),
			("e7e6", 183),
			("d7d6", 42),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b2f6", 42),
			("e2e3", 32),
			("e2e4", 35),
			("b1c3", 26),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 947),
			("b1c3", 302),
			("h2h4", 358),
			("g1e2", 99),
			("f1d3", 130),
			("g2g4", 239),
			("b1d2", 70),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 884),
			("h7h5", 30),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 34),
			("f1c4", 56),
			("c1e3", 62),
			("f2f4", 52),
			("h2h3", 32),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/q7/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 317),
			("g8f6", 438),
			("e7e5", 32),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1b5", 33),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/2P5/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 40),
			("d7d5", 217),
			("c7c5", 92),
			("e7e5", 162),
			("g8f6", 133),
			("e7e6", 43),
			("g7g6", 45),
			("c7c6", 68),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 47),
			("g7g6", 26),
			("e7e5", 45),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 34),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("f6e4", 83),
			("f8c5", 70),
			("f8e7", 36),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 29),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 139),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 31),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 41),
			("d7d5", 39),
			("d7d6", 26),
			("g7g6", 116),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 49),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 31),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 72),
			("d2d4", 51),
			("g1f3", 105),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 292),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 154),
			("f1c4", 33),
			("g1f3", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e5d4", 47),
			("b8c6", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 228),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 204),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 330),
			("g2g3", 320),
			("g1f3", 159),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 253),
			("f8b4", 62),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1g5", 64),
			("g2g3", 44),
			("g1f3", 36),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 32),
			("e2e4", 35),
			("d2d4", 41),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 93),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 31),
			("e2e3", 48),
			("c4d5", 32),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/1P6/8/P1PPPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 141),
			("e7e5", 412),
			("e7e6", 144),
			("a7a5", 45),
			("d7d5", 192),
			("c7c6", 98),
			("g7g6", 30),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 102),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 171),
			("d7d5", 227),
			("d7d6", 32),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 153),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 106),
			("e8g8", 53),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 232),
			("f1b5", 41),
			("b1c3", 62),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 159),
			("c8g4", 35),
			("g7g6", 38),
			("e7e5", 34),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2np1n2/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 116),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2np1n2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 46),
			("g7g6", 29),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 47),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 277),
			("b8c6", 160),
			("d7d6", 71),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4p3/1b2P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1c4", 90),
			("d2d3", 62),
			("f3e5", 73),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4p3/1bB1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("e8g8", 62),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 77),
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("f4e5", 923),
			("g1f3", 168),
			("e2e4", 105),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4P3/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 793),
			("b8c6", 92),
			("f7f6", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4P3/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e5d6", 577),
			("g1f3", 205),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3P4/8/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8d6", 505),
			("c8g4", 30),
			("b8c6", 32),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3b4/8/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 498),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3b4/8/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g5", 206),
			("c8g4", 87),
			("g8f6", 143),
			("b8c6", 25),
		]
	),
	(
		"r1b1kbnr/ppppqppp/2n5/4P3/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c6e5", 134),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8e7", 215),
			("d7d6", 1430),
			("f6e4", 297),
		]
	),
	(
		"rnb1kb1r/ppppqppp/5n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 98),
			("e5f3", 113),
		]
	),
	(
		"rnb1kb1r/ppppqppp/5n2/4N3/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 84),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 184),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 92),
			("g8f6", 35),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 52),
			("g1f3", 34),
			("g2g3", 33),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 130),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 120),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 466),
			("c8g4", 242),
			("c7c6", 57),
			("d8d5", 69),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 187),
			("c2c4", 245),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 59),
			("c8g4", 48),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 218),
			("g1f3", 112),
			("g1e2", 102),
			("b2b3", 120),
			("c2c4", 66),
			("d2d3", 147),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8d6", 152),
			("c7c5", 30),
			("g8f6", 186),
			("b8c6", 95),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 199),
			("d7d5", 79),
			("b7b6", 129),
			("f8e7", 50),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 157),
			("c2c3", 47),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8b6", 39),
			("b8c6", 33),
			("b7b6", 32),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 116),
			("e2e4", 37),
			("h2h4", 45),
			("g1f3", 39),
			("b1d2", 57),
		]
	),
	(
		"rnbqkbnr/pppp2pp/5p2/4P3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 45),
			("e5f6", 69),
			("g1f3", 73),
		]
	),
	(
		"rnbqkbnr/pppp2pp/5p2/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 34),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 351),
			("c4d5", 32),
			("g1f3", 279),
			("e2e3", 74),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c4", 282),
			("g8f6", 40),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 84),
			("e2e3", 50),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 79),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 92),
			("c2c4", 31),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 58),
			("d7d6", 49),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 216),
			("g2g3", 35),
			("c1g5", 234),
			("e2e4", 113),
			("e2e3", 96),
			("c1f4", 63),
			("c2c3", 43),
			("b1c3", 35),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 169),
			("c7c5", 68),
			("g8f6", 48),
			("e7e6", 30),
			("c7c6", 44),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 32),
			("g2g3", 84),
			("e2e4", 33),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 27),
			("e7e5", 36),
			("g8f6", 33),
			("c8g4", 89),
			("b8c6", 33),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3p4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f3d4", 110),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3N4/2N5/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 43),
			("f8c5", 28),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1e2", 80),
			("f1c4", 83),
			("g1f3", 721),
			("f2f4", 750),
			("g2g3", 664),
			("a2a4", 109),
			("d2d4", 64),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 31),
			("a7a6", 42),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 321),
			("c2c4", 1029),
			("c1g5", 378),
			("c2c3", 124),
			("g2g3", 440),
			("c1f4", 410),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f5", 48),
			("g8f6", 75),
			("c7c5", 43),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 38),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d3", 66),
			("g1f3", 241),
			("b1c3", 78),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 41),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 65),
			("g1f3", 257),
			("c2c3", 25),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 789),
			("d7d5", 39),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 786),
			("c2c4", 54),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d6", 96),
			("e8g8", 62),
			("d7d5", 103),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 251),
			("f2f4", 271),
			("c2c4", 104),
			("f1d3", 129),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8g4", 106),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3Pp3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("e4f3", 146),
			("g8f6", 100),
			("e4e3", 41),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3e5", 145),
			("e4d5", 180),
			("b1c3", 27),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pN3/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8d6", 95),
			("d5e4", 26),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3b4/3pN3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 84),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3b4/3pN3/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d5e4", 79),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2BPp3/2N5/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8f5", 30),
			("g8f6", 143),
			("b8d7", 38),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f5", 41),
			("g8f6", 57),
			("d7d5", 161),
			("c7c5", 302),
			("b7b6", 203),
			("f8e7", 75),
			("g8e7", 40),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 210),
			("c1g5", 131),
			("b1c3", 34),
			("g2g3", 189),
			("c1f4", 37),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2P2N1P/PP1P1PP1/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 43),
			("b8c6", 175),
			("b8d7", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4P3/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d6e5", 104),
			("c8g4", 40),
			("b8c6", 50),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/8/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 91),
			("f3e5", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/4P3/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 33),
			("f8c5", 32),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 27),
			("e4e5", 60),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5b6", 173),
			("d5f6", 52),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4N3/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e5f3", 1309),
			("e5f7", 82),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6e4", 1300),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5c6", 929),
			("g1f3", 91),
			("d2d4", 265),
			("b1c3", 319),
			("c2c4", 73),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2P5/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 853),
			("e7e5", 40),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 141),
			("g8f6", 57),
			("c7c5", 45),
			("h7h6", 72),
			("c7c6", 45),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 297),
			("g1f3", 38),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 142),
			("c1g5", 69),
			("f1c4", 37),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/3Pp3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8f5", 108),
			("e4e3", 41),
			("e4f3", 337),
		]
	),
	(
		"r1bqk1nr/ppppbppp/2n5/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 112),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6d5", 918),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 1681),
			("d7d6", 84),
			("a7a6", 1057),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 241),
			("f1b5", 174),
			("g2g3", 46),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 636),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 216),
			("b4b5", 47),
			("a2a3", 27),
			("c2c3", 30),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8b4", 100),
			("f7f6", 35),
			("d7d6", 52),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 187),
			("d2d4", 297),
			("c2c3", 46),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 53),
			("c8d7", 84),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 81),
			("b1c3", 105),
			("b1d2", 75),
			("f2f4", 36),
			("g2g3", 283),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 40),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 30),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 380),
			("e2e4", 30),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 33),
			("d7d5", 258),
			("g7g6", 74),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 75),
			("e2e3", 30),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8c7", 42),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 138),
			("d8d5", 37),
			("c8g4", 48),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 94),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 2797),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d3", 43),
			("e2e3", 292),
			("b2b3", 41),
			("g2g3", 30),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 55),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 37),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 51),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("d7d6", 88),
			("g8f6", 489),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 70),
			("f8g7", 521),
			("c7c6", 56),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 27),
			("c4d5", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 87),
			("g8f6", 186),
			("c8g4", 46),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 32),
			("e4e5", 272),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 103),
			("c7c5", 30),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 29),
			("g7g6", 41),
			("e7e6", 74),
			("g8f6", 49),
		]
	),
	(
		"r1bqkbnr/pppnpppp/8/8/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4g3", 32),
			("f1c4", 44),
			("g1f3", 45),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 136),
			("c7c6", 114),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 27),
			("g2g3", 86),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 42),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 371),
			("b8c6", 105),
			("a7a6", 155),
			("c7c6", 88),
			("e7e6", 35),
			("b8d7", 31),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 131),
			("g1f3", 45),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 115),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 36),
			("g1f3", 28),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6d5", 306),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 280),
			("b1c3", 364),
			("g2g3", 35),
			("b2b3", 32),
			("e2e3", 37),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 82),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 51),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 63),
			("c7c5", 61),
			("b7b6", 65),
			("f8e7", 36),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 103),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 726),
			("g2g3", 456),
			("b2b3", 88),
			("d2d4", 574),
			("e2e4", 279),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 90),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 30),
			("b1c3", 25),
			("g2g3", 28),
			("e2e3", 49),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 109),
			("d4d5", 28),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 175),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 244),
			("g1f3", 40),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 261),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d5", 258),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 58),
			("g2g3", 117),
			("e2e4", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5c3", 29),
			("g7g6", 35),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 2330),
			("g1f3", 265),
			("d1g4", 123),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8d7", 193),
			("d8b6", 408),
			("b8c6", 1769),
			("c5d4", 140),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 96),
			("g1f3", 28),
			("e2e4", 32),
			("b1d2", 50),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 49),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 168),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/5P2/4P3/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 52),
			("g8f6", 67),
			("c7c5", 27),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 87),
			("c7c6", 27),
			("g8f6", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/4P3/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 90),
			("e2g3", 57),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/3P4/4P3/PPP1NPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5e4", 28),
			("b8c6", 39),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 308),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 299),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8e7", 74),
			("f8b4", 35),
			("c7c6", 78),
			("d7d5", 91),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d6", 187),
			("g8f6", 236),
			("c7c5", 174),
			("c7c6", 74),
			("e7e5", 69),
			("f7f5", 32),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 79),
			("e2e3", 31),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/3nP3/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5b6", 647),
		]
	),
	(
		"rnbqkb1r/pppppppp/1n6/4P3/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 322),
			("c4c5", 260),
			("a2a4", 30),
		]
	),
	(
		"rnbqkb1r/pppppppp/1n6/4P3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 343),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 403),
			("b8c6", 50),
			("c8g4", 40),
			("g8f6", 260),
			("c7c5", 59),
			("e7e6", 44),
			("g7g6", 116),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 46),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 535),
			("g7g6", 216),
			("g8f6", 139),
			("e7e6", 55),
			("d7d5", 28),
			("d7d6", 43),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 516),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 76),
			("g7g6", 38),
			("d7d6", 44),
			("g8f6", 73),
			("e7e6", 28),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 62),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 68),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 255),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/6B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8g7", 72),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/6B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("d1d2", 25),
			("e2e4", 34),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 3173),
			("e7e6", 234),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3pP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 1985),
			("g1f3", 793),
			("d1d4", 355),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3pP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d4c3", 1108),
			("g8f6", 306),
			("b8c6", 60),
			("d7d5", 156),
			("d4d3", 235),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/4P3/2p5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 1040),
			("f1c4", 50),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/4P3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 578),
			("d7d6", 116),
			("a7a6", 58),
			("e7e6", 250),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/3P1N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b7b6", 44),
			("c7c5", 109),
			("f8e7", 74),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 859),
			("c2c4", 90),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 203),
			("g8f6", 197),
			("d7d5", 332),
			("f7f5", 55),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 25),
			("d2d3", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 934),
			("c2c4", 190),
			("h2h3", 209),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 71),
			("c8g4", 376),
			("g8f6", 608),
			("c8f5", 129),
			("b8d7", 39),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 51),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8g7", 50),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 156),
			("e2e4", 197),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 37),
			("b8c6", 27),
			("e7e6", 30),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P3n1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8c5", 216),
			("b8c6", 112),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 83),
			("g1f3", 38),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f5e4", 67),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/8/4p3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d3", 40),
			("c3e4", 44),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 40),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/8/1P2PN2/PBPP1PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 28),
			("f8e7", 34),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 104),
			("a7a6", 38),
			("e7e6", 30),
			("c7c6", 43),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 79),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 35),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 287),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/4P3/3p4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 294),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 27),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 296),
			("e4e5", 422),
			("b1c3", 282),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3P4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 298),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 190),
			("c1e3", 51),
			("c2c3", 25),
			("b1c3", 25),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 113),
			("e7e5", 69),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f5e4", 372),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/8/3Pp3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 352),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/8/3Pp3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 328),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 68),
			("c1g5", 223),
			("g2g4", 34),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/8/3Pp3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 44),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 83),
			("d2d4", 48),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e5", 60),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 36),
			("d2d3", 43),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/3PP3/8/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8e7", 130),
			("c7c5", 41),
			("h7h6", 264),
			("d7d5", 43),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/6B1/3PP3/8/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b1c3", 62),
			("b1d2", 29),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/6B1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("d7d5", 32),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p1N1/2B1P3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d5", 629),
			("f8c5", 318),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 376),
			("f1b5", 143),
			("f1c4", 464),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 41),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 758),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 688),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 53),
			("f8d6", 28),
			("f7f5", 178),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 852),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 832),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("e8g8", 390),
			("d7d6", 51),
		]
	),
	(
		"rnbqkbnr/ppp1p1pp/3p4/5p2/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 29),
			("e4f5", 30),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 36),
			("g8f6", 54),
			("e7e5", 27),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 349),
			("c2c3", 130),
			("g1f3", 244),
			("b1c3", 32),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2pP4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 269),
			("g8f6", 45),
			("d7d6", 59),
			("e6e5", 25),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pp4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 265),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pP4/8/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 233),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 159),
			("g1f3", 222),
			("b1c3", 273),
			("e2e4", 88),
			("d2d4", 71),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 152),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 124),
			("f2f3", 25),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b7f3", 38),
		]
	),
	(
		"rn1qkbnr/p1pppppp/1p6/8/2P5/5bP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("e2f3", 52),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4e5", 31),
			("e2e4", 28),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("e2e4", 121),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
		&[
			("f8e7", 54),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 32),
			("f7f5", 65),
			("d7d6", 47),
			("f8c5", 263),
			("f8b4", 31),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 51),
			("a2a3", 31),
			("f2f4", 38),
			("g1f3", 31),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8c5", 134),
			("f8b4", 123),
			("f8e7", 40),
			("c6a5", 62),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 41),
			("e4e5", 32),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 77),
			("g1f3", 54),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 39),
			("d7d6", 33),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3PPP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 114),
			("d7d5", 29),
			("c7c5", 62),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 66),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 36),
			("b8d7", 25),
			("c7c5", 30),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 146),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 65),
			("d2d4", 36),
			("b1c3", 26),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 43),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 104),
			("g7g6", 109),
			("b8d7", 45),
			("e7e5", 75),
			("c7c6", 93),
			("c8g4", 40),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 36),
			("d2d4", 43),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f7f5", 60),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 34),
			("d7d5", 38),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5pB1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 57),
			("g8f6", 61),
			("g7g6", 121),
			("h7h6", 142),
			("c7c6", 39),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 200),
			("g2g3", 178),
			("c2c4", 119),
			("b2b3", 28),
			("e2e4", 46),
			("d2d3", 97),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 548),
			("e7e6", 146),
			("g7g6", 34),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 250),
			("c2c4", 126),
			("c1g5", 51),
			("c1f4", 43),
			("e2e3", 31),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 64),
			("g7g6", 195),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 171),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8e7", 50),
			("d7d5", 106),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 66),
			("e7e5", 72),
			("g8f6", 107),
			("c7c6", 98),
			("b8d7", 32),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 43),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 275),
			("d7d6", 60),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4d5", 36),
			("f4e5", 220),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 81),
			("a7a6", 144),
			("g8f6", 606),
			("b8c6", 226),
			("e7e5", 58),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 119),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 116),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 435),
			("c7c6", 30),
			("g8f6", 37),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4p3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d3e4", 300),
			("b1c3", 109),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d1", 288),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/8/4P3/8/PPP2PPP/RNBqKBNR w KQkq - 0 0",
		&[
			("e1d1", 288),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/8/4P3/8/PPP2PPP/RNBK1BNR b kq - 0 0",
		&[
			("e7e5", 65),
			("g8f6", 57),
			("b8c6", 55),
			("b7b6", 42),
			("f7f5", 29),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1h3", 28),
			("e1f2", 43),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3PnB2/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
		&[
			("e4f6", 122),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 43),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/5P1N/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 55),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/5P1N/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("h3f2", 63),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 95),
			("g8f6", 71),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 49),
			("f1b5", 79),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 68),
			("e5d4", 39),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 52),
			("e7e6", 65),
			("d7d6", 30),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 78),
			("g2g3", 59),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 131),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 925),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 635),
			("d7d6", 219),
			("f7f6", 35),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 407),
			("g1f3", 54),
			("g2g3", 74),
			("c2c4", 60),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 226),
			("g7g6", 25),
			("d7d5", 81),
			("a7a6", 28),
			("d7d6", 32),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f1b5", 145),
			("c2c4", 32),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/1B2p3/8/1P2P3/PBPP1PPP/RN1QK1NR b KQkq - 0 0",
		&[
			("f8d6", 101),
			("d7d6", 35),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 45),
			("g7g6", 77),
			("b7b6", 81),
			("b8c6", 71),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 211),
			("g1f3", 33),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 169),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 48),
			("c4d5", 116),
			("b2b3", 31),
			("d2d4", 50),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("d5c4", 37),
			("c8g4", 34),
			("e7e6", 58),
			("g7g6", 37),
			("c8f5", 38),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("b5c6", 63),
			("b1c3", 53),
			("e4e5", 50),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2B2n2/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7c6", 48),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/5P2/5NP1/PPPPP2P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 39),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P3P/8/PPP2PP1/RNBQKBNR b KQkq - 0 0",
		&[
			("h7h5", 246),
			("h7h6", 72),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 300),
			("b5c6", 323),
			("c2c3", 52),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8g7", 297),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 233),
			("g8f6", 99),
			("d7d5", 526),
			("d8c7", 31),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c3d4", 239),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 152),
			("g8f6", 32),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 137),
			("d2d4", 53),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 116),
		]
	),
	(
		"rnbqkb1r/pppp2pp/5n2/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 54),
			("d2d4", 30),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 159),
			("g1f3", 127),
			("g2g3", 181),
			("a2a4", 79),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("b7b5", 123),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1pp5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 97),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 94),
			("d7d6", 109),
			("g8f6", 62),
		]
	),
	(
		"r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("c2c3", 65),
			("e1g1", 29),
			("b1c3", 32),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1pp5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8b7", 88),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 117),
			("g1f3", 123),
			("b1c3", 31),
			("g2g3", 64),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 63),
			("d7d5", 30),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 77),
			("c1g5", 34),
			("g2g3", 39),
			("e2e4", 84),
			("c1f4", 25),
			("d4d5", 101),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 64),
			("c8d7", 29),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f6", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 26),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5pB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f6", 51),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5B2/5p2/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7f6", 50),
		]
	),
	(
		"rnbqkb1r/pppp2pp/5p2/5p2/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 47),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 49),
			("e7e6", 29),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/2NP4/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1e3", 42),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2Pp4/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b2b4", 76),
			("e2e3", 148),
			("g2g3", 85),
			("d2d3", 69),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 26),
			("g8f6", 107),
			("e7e5", 74),
			("g7g6", 87),
			("c8g4", 26),
			("c7c6", 115),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b2b4", 30),
			("g2g3", 405),
			("b1c3", 380),
			("d2d4", 74),
			("g1f3", 47),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1h3", 32),
			("e1f2", 40),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 120),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 64),
			("g8f6", 26),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 102),
			("f1c4", 166),
			("g1f3", 108),
			("d2d4", 60),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 43),
			("e5f4", 40),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2Pp4/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 87),
			("b8c6", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/2Pp4/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e3d4", 41),
			("b2b4", 57),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPP1NPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 77),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 25),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 115),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4p3/5P2/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("f3e4", 115),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4P3/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 55),
			("f7f5", 35),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 263),
			("d7d5", 889),
			("f8b4", 278),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 163),
			("g1f3", 72),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/2P5/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 182),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 81),
			("d7d5", 28),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 57),
			("b1d2", 56),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/3P1NP1/PPP2P1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 127),
			("g8e7", 36),
			("d7d5", 63),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2PP4/5P2/PP2P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 86),
			("f8g7", 101),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/5P2/PP2P1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 85),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3P4/3P4/5P2/PP2P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 84),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 37),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 1305),
			("d2d3", 206),
			("e4e5", 96),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pP4/8/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 1251),
			("g8f6", 50),
		]
	),
	(
		"rnb1kbnr/pp2pppp/8/2pq4/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 1127),
			("g1f3", 110),
		]
	),
	(
		"rnb1kbnr/pp2pppp/8/2pq4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 460),
			("b8c6", 381),
			("c5d4", 131),
			("g7g6", 47),
			("e7e6", 66),
			("e7e5", 30),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/6b1/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 227),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8d7", 76),
			("g4f3", 46),
			("c7c6", 59),
		]
	),
	(
		"r2qkbnr/pppnpppp/8/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("c2c4", 27),
			("e1g1", 28),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 299),
			("b1c3", 179),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 153),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 110),
			("d7d6", 36),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 124),
			("c7c5", 80),
			("d7d5", 217),
			("b7b6", 43),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bP5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d1c2", 67),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bP5/2N2N2/PPQPPPPP/R1B1KB1R b KQkq - 0 0",
		&[
			("e8g8", 46),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 73),
			("b8c6", 375),
			("g8f6", 188),
			("d7d6", 31),
			("e7e6", 51),
			("b7b6", 53),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 46),
			("d2d4", 128),
			("g2g3", 45),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 79),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 37),
			("e2e3", 43),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 74),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 119),
			("g8f6", 321),
		]
	),
	(
		"rnbqkbnr/ppp1p1pp/8/3p1p2/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1g5", 49),
			("c1f4", 43),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 89),
			("b1c3", 127),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 56),
			("d7d6", 62),
			("f8c5", 112),
			("g7g6", 117),
			("f7f5", 143),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 50),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 62),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2PPp3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 47),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1b1PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 43),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c6", 124),
			("g8f6", 400),
			("c8f5", 87),
			("e7e6", 129),
			("c7c5", 118),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 102),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2p1p3/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 83),
			("f1c4", 33),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2p1p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 71),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 27),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/2B1P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8e7", 38),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("c8f5", 30),
			("e7e6", 69),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 121),
			("d2d4", 134),
			("b2b3", 62),
			("g1e2", 70),
			("d2d3", 51),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 70),
			("d7d5", 35),
			("e7e6", 36),
			("b8c6", 27),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 38),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 35),
			("e8g8", 143),
			("c7c5", 31),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 308),
			("f7f5", 174),
			("f8d6", 43),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 30),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3Pp3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2e4", 591),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c3", 76),
			("e2e4", 47),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/6B1/3P4/2P5/PP2PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8d7", 26),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e4e5", 247),
			("e4d5", 97),
			("d2d3", 49),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3pP3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6d7", 248),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4e5", 59),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2nP4/8/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8d6", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 38),
			("d2d4", 36),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 365),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 97),
			("b1c3", 79),
			("e2e3", 189),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 82),
		]
	),
	(
		"rnb1kb1r/ppp1pppp/5n2/3q4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 115),
		]
	),
	(
		"rnb1kb1r/ppp1pppp/5n2/3q4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5a5", 54),
			("d5d8", 28),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 293),
			("d2d3", 769),
			("g1f3", 229),
			("b1c3", 126),
			("d1f3", 40),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2BPP3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f6e4", 89),
			("e5d4", 167),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p3/2BPn3/8/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d4e5", 89),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2B1n3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d8h4", 60),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 102),
			("c5d4", 181),
			("g8f6", 123),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pP2B1/4n3/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5c1", 40),
			("g5f4", 61),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p2B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e7e6", 59),
			("d5c4", 42),
			("f6e4", 44),
		]
	),
	(
		"rnbqkbnr/1p2pppp/p2p4/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 140),
		]
	),
	(
		"rnbqkbnr/1p2pppp/p2p4/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 138),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f5d3", 122),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1bB1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("f2f3", 27),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3P4/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 205),
			("d8d5", 456),
			("g8f6", 98),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5c6", 162),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2P5/8/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 154),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/8/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 49),
			("b1c3", 75),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 2557),
			("g1f3", 909),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 1058),
			("b8c6", 91),
			("c7c5", 240),
			("g8f6", 655),
			("e7e5", 236),
			("e7e6", 83),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/1P4P1/P1PPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 85),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P4P1/P1PPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c1b2", 85),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P4P1/PBPPPPBP/RN1QK1NR b KQkq - 0 0",
		&[
			("c8f5", 36),
		]
	),
	(
		"rnbqkb1r/ppp1ppp1/5n1p/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("g5h4", 29),
			("g5f6", 50),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 26),
			("b1c3", 60),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 488),
			("g1f3", 38),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d6", 73),
			("e8g8", 305),
			("d7d5", 78),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 172),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 412),
			("f1b5", 188),
			("d2d4", 46),
			("b1c3", 64),
			("d2d3", 69),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/8/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 348),
			("g8f6", 33),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 93),
			("b5c4", 45),
			("c8b7", 27),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2P5/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 28),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 124),
			("b1c3", 33),
			("d4c5", 78),
			("g1f3", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pP4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 54),
			("g8f6", 40),
		]
	),
	(
		"rnb1kbnr/pp2pppp/8/2pq4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 617),
			("g1f3", 106),
			("e2e3", 75),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c5d4", 303),
			("e6d5", 313),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 44),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 91),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 53),
			("g8f6", 25),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 100),
			("e2e3", 45),
			("c4d5", 35),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 58),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 54),
			("g1f3", 537),
			("e2e3", 32),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 133),
			("d7d6", 74),
			("d7d5", 268),
			("b8c6", 212),
			("g7g6", 76),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 98),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 28),
			("d7d5", 47),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 726),
			("d7d6", 46),
			("g8f6", 33),
			("g7g6", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 228),
			("e2e3", 150),
			("d2d4", 304),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6d5", 221),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 209),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 51),
			("d2d4", 45),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 49),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 106),
			("c5d4", 620),
			("e7e6", 112),
			("g7g6", 41),
			("d7d5", 75),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 102),
			("c2c3", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 32),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 49),
			("f1d3", 53),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 40),
			("c7c6", 31),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 138),
			("e2e3", 45),
			("g1f3", 102),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3P4/2p5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6e5", 66),
			("c6a5", 77),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2pP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 1188),
			("e2e4", 714),
			("e2e3", 442),
			("b1c3", 439),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2pP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 520),
			("b8c6", 46),
			("a7a6", 88),
			("c7c6", 54),
			("c7c5", 135),
			("b7b5", 44),
			("e7e6", 140),
			("c8g4", 46),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 833),
			("e7e6", 435),
			("c7c5", 210),
			("g7g6", 534),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3p4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c3d4", 133),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 62),
			("d7d6", 28),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 117),
			("g1f3", 31),
			("e2e3", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 40),
			("g8f6", 29),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/1P3N2/P1P1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 35),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/7p/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 147),
			("g1f3", 65),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/7p/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("h5h4", 141),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/8/7p/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 30),
			("g3h4", 31),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8c5", 36),
			("c7c6", 72),
			("d7d5", 266),
			("f8b4", 168),
			("b8c6", 128),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c3", 36),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 59),
			("c4d5", 97),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d5c4", 29),
			("e7e6", 67),
			("c7c6", 91),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c6d5", 180),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/6P1/8/PPPPPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 49),
			("e7e5", 161),
			("d7d5", 630),
			("d7d6", 100),
			("h7h5", 39),
			("c7c6", 61),
			("c7c5", 55),
			("g7g6", 56),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c5", 33),
			("d7d6", 36),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 785),
			("c2c3", 181),
			("b1c3", 127),
			("f1c4", 81),
			("c2c4", 96),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 685),
			("f8g7", 100),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d1d4", 201),
			("f3d4", 477),
			("c2c3", 31),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/3QP3/5N2/PPP2PPP/RNB1KB1R b KQkq - 0 0",
		&[
			("g8f6", 196),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 26),
			("d7d6", 48),
			("g8f6", 58),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 96),
			("g1f3", 37),
			("g2g3", 26),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 42),
			("b1c3", 144),
			("d2d4", 165),
			("e2e3", 43),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 108),
			("e7e5", 53),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 98),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 124),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 693),
			("g7g6", 54),
			("e7e6", 30),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 665),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 459),
			("d7d5", 26),
			("e7e6", 124),
			("d7d6", 36),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c3", 64),
			("g1f3", 184),
			("g1h3", 103),
			("c2c4", 61),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/3P4/2P3P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 60),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 420),
			("d2d4", 59),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 198),
			("g8f6", 35),
			("e7e5", 90),
			("d7d5", 48),
			("d7d6", 27),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 134),
			("c2c4", 48),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8g7", 129),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 70),
			("g1f3", 297),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 27),
			("g1f3", 33),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 67),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 190),
			("e7e5", 32),
			("e7e6", 86),
			("d5c4", 60),
			("g8f6", 50),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/2PP2b1/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 80),
			("c4d5", 80),
			("e2e3", 29),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("d5e4", 79),
			("f8e7", 105),
			("f8b4", 29),
			("h7h6", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 1513),
			("g8f6", 89),
			("d8d5", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P1B2/8/PP2PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 67),
			("g8f6", 46),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/1B2p3/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8d7", 36),
			("e5d4", 150),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 176),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 103),
			("c7c5", 53),
			("f8e7", 29),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 164),
			("c7c5", 38),
			("f7f5", 34),
			("g8f6", 46),
			("g8e7", 26),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4d5", 696),
			("e2e3", 79),
			("b1c3", 165),
			("g2g3", 84),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g2g3", 68),
			("e2e3", 109),
			("d2d4", 36),
			("g1f3", 29),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 70),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 93),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 57),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3pp3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 223),
			("g1f3", 286),
			("f1d3", 25),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4c5", 41),
			("f1d3", 26),
			("c2c4", 78),
			("c2c3", 62),
			("f1e2", 67),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2Pp4/8/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 38),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 61),
			("g1f3", 215),
			("c2c4", 47),
			("f2f4", 66),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/5P2/4Pb2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d1f3", 112),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/5P2/4PQ2/PPPP2PP/RNB1KB1R b KQkq - 0 0",
		&[
			("f7f5", 73),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8e7", 75),
			("g8f6", 55),
		]
	),
	(
		"rnbqk1nr/ppppb1pp/4p3/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5e7", 44),
			("h2h4", 27),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 279),
			("e8g8", 164),
			("b7b6", 286),
			("d7d5", 93),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 93),
			("e7e6", 61),
			("d5c4", 38),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 83),
			("b2b3", 27),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 32),
			("g8f6", 91),
			("c7c5", 32),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 49),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 63),
			("c2c3", 359),
			("d4d5", 651),
			("g2g3", 150),
			("e2e3", 25),
			("c2c4", 174),
			("d4c5", 47),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p3B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c5d4", 34),
			("f6e4", 27),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("h7h6", 32),
			("d7d6", 57),
			("c7c5", 75),
			("g8f6", 39),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3p4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e3d4", 594),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 51),
			("d7d5", 295),
			("d7d6", 43),
			("e7e6", 26),
			("g8f6", 81),
			("b8c6", 44),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5B2/2p5/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7f6", 288),
			("e7f6", 81),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5p2/2p5/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 67),
			("d4d5", 154),
			("d4c5", 38),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 432),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 384),
			("d2d4", 41),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/3q4/8/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 195),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 100),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 100),
		]
	),
	(
		"rn1qkbnr/pb1ppppp/1p6/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 75),
		]
	),
	(
		"rn1qkbnr/pb1ppppp/1p6/2p5/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 41),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 47),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d4c3", 146),
			("g8f6", 60),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 94),
			("b1c3", 74),
			("g1f3", 73),
			("c2c4", 34),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c8b7", 60),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 353),
			("d1e2", 98),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 217),
			("c7c5", 67),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2p5/3PnB2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("d4d5", 92),
			("f2f3", 309),
			("f4b8", 46),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pP4/4nB2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 44),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 105),
			("c7c6", 111),
			("d5c4", 56),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 99),
			("b1c3", 52),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 34),
			("d2d4", 28),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 225),
			("d7d5", 35),
			("c7c6", 28),
			("c7c5", 42),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f2f4", 75),
			("f1d3", 59),
			("g1f3", 41),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PPP2/2P5/PP4PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 38),
			("b8c6", 29),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 74),
			("b1c3", 26),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/4P3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 86),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 221),
			("c7c6", 47),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 486),
			("e7e5", 770),
			("d7d6", 34),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 66),
			("d7d6", 38),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 30),
			("c2c4", 47),
			("f3e5", 37),
			("d2d3", 38),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/6b1/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("b8d7", 139),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/3QP3/5N2/PPP2PPP/RNB1KB1R b KQkq - 0 0",
		&[
			("b8c6", 90),
			("g8f6", 71),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 1449),
			("d2d3", 89),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p1P3/8/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 1447),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 481),
			("d2d4", 898),
			("f1c4", 38),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 183),
			("d7d6", 59),
			("e7e6", 235),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 195),
			("g2g3", 397),
			("e2e4", 459),
			("d2d3", 31),
			("g1f3", 118),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 67),
			("d2d4", 40),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 66),
			("a7a6", 25),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 177),
			("b1c3", 28),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 218),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8e7", 36),
			("d7d5", 169),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 55),
			("c7c5", 37),
			("d7d6", 53),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f6e4", 326),
			("b8c6", 203),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p3/2B1n3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("f3e5", 48),
			("e1g1", 28),
			("c4f7", 26),
			("b1c3", 174),
			("d1e2", 25),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 38),
			("c2c4", 32),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 36),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 341),
			("c7c6", 200),
			("f8c5", 117),
			("f8e7", 48),
			("d7d5", 32),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1e2", 384),
			("g1e2", 100),
			("d1e2", 156),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8g4", 318),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 32),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/1Q3N2/PP2PPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("d5c4", 31),
			("e7e6", 28),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 116),
			("g8f6", 286),
			("e7e6", 25),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 68),
			("f1b5", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 52),
			("f6e4", 222),
			("c7c6", 34),
			("c8f5", 30),
			("b8d7", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 180),
			("g8f6", 194),
			("e7e6", 32),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3pP3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8f5", 128),
			("c7c5", 214),
			("c7c6", 28),
			("d5d4", 55),
			("e7e6", 78),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3pPb2/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 43),
			("g1f3", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f6", 143),
			("b1d2", 27),
			("e2e3", 36),
			("b1c3", 46),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5B2/3p4/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7f6", 393),
			("g7f6", 103),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5p2/3p4/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 337),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5p2/3p4/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8d6", 137),
			("c8e6", 34),
			("c7c6", 54),
			("f6f5", 26),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 98),
		]
	),
	(
		"rnbqkbnr/ppppppp1/7p/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 36),
			("c2c4", 47),
			("e2e4", 72),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5Bp1/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7f6", 333),
		]
	),
	(
		"rnbqkb1r/pppp1p1p/5pp1/8/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("h2h4", 88),
			("e2e3", 103),
			("g2g3", 86),
			("c2c4", 33),
		]
	),
	(
		"rnbqkb1r/pppp1p1p/5pp1/8/3P3P/8/PPP1PPP1/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 61),
		]
	),
	(
		"rnbqk2r/ppppppbp/5Bp1/8/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("g7f6", 76),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 277),
			("d5d6", 34),
			("g8f6", 93),
			("d5d8", 56),
			("d5a5", 29),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5p2/3p4/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 77),
		]
	),
	(
		"rnb1kb1r/pppp1ppp/4pq2/8/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 93),
		]
	),
	(
		"rnb1kb1r/pppp1ppp/4pq2/8/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 43),
			("d7d5", 27),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/7N/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 65),
			("h3g1", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 25),
		]
	),
	(
		"rnbqkbnr/pppp1p1p/4p1p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 33),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 33),
			("c1f4", 31),
			("c2c4", 61),
			("g2g3", 64),
			("c1g5", 37),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("e2e3", 55),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 81),
			("c8f5", 64),
			("g8f6", 526),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 44),
			("g1f3", 81),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2pPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 235),
			("g8f6", 178),
			("b8c6", 80),
			("c7c5", 51),
			("b7b5", 60),
			("e7e6", 69),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 170),
			("d4d5", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2pPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 116),
			("f8b4", 46),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("c8f5", 47),
			("e7e6", 38),
			("c7c6", 28),
			("g7g6", 50),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("e2e3", 51),
			("f2f3", 28),
		]
	),
	(
		"rnb1kb1r/pp1ppppp/1q3n2/2p5/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b1a3", 67),
			("b1c3", 43),
		]
	),
	(
		"rnb1kb1r/pp1ppppp/1q3n2/2p5/3P1B2/N3P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("b6b2", 43),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pB2/3p4/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8f6", 61),
			("g7f6", 25),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 43),
			("e7e5", 27),
			("c7c6", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e4", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 154),
			("g7g6", 53),
			("e7e6", 38),
			("g8f6", 72),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 54),
			("e5e4", 40),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 45),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5d4", 25),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/2B1P3/3P4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 165),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d5", 115),
			("f8e7", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c3d4", 184),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 111),
			("g7g6", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("a7a6", 50),
			("c7c6", 31),
			("e7e6", 27),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 102),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 62),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2pP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 137),
			("e7e5", 162),
			("e7e6", 42),
			("c7c5", 28),
			("c8e6", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2pP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 52),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2BP4/4P3/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e6", 155),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2BpP3/8/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 142),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 187),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 222),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3pP3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8f5", 246),
			("f7f6", 134),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3pPb2/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 105),
			("g1f3", 73),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3pPb2/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 76),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 36),
			("c7c5", 65),
			("d7d5", 81),
			("f7f5", 79),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 63),
			("d5d4", 50),
			("g8f6", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4pP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e4", 72),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4NP2/8/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c4", 25),
			("c7c6", 49),
			("e7e6", 84),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 72),
			("d5e4", 346),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3pP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 369),
			("d7d5", 70),
			("c7c5", 29),
			("f8b4", 27),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 480),
			("f8c5", 169),
			("f8e7", 25),
			("d7d6", 46),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 326),
			("e4e5", 286),
			("d2d4", 52),
			("d2d3", 42),
			("d1e2", 36),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 44),
			("d7d5", 56),
			("d7d6", 41),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f2f3", 127),
			("e2e3", 161),
			("g1f3", 30),
			("b1c3", 32),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P1B2/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 42),
			("f6d5", 25),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3PP3/6P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e8g8", 112),
			("d7d6", 163),
		]
	),
	(
		"r1bqkbnr/pppn1ppp/3p4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 31),
			("f1c4", 55),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8g7", 143),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 52),
			("c4d5", 29),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 28),
			("c7c5", 79),
			("c8g4", 106),
			("e7e6", 36),
			("g7g6", 185),
			("c8f5", 25),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 47),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5d4", 158),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2Pp4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1d4", 79),
			("g1f3", 42),
			("e2e3", 34),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2PQ4/8/PP2PPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b8c6", 64),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/2PQ4/8/PP2PPPP/RNB1KBNR w KQkq - 0 0",
		&[
			("d4d1", 50),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 36),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 111),
			("g8f6", 45),
			("e7e5", 64),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 100),
			("d2d4", 42),
			("e2e3", 100),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 101),
		]
	),
	(
		"r1bqkbnr/pp1npppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d4", 178),
			("e1g1", 195),
			("b1c3", 1396),
		]
	),
	(
		"r1bqkbnr/pp1npppp/3p4/1Bp5/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 40),
			("c5d4", 123),
		]
	),
	(
		"r1b1kbnr/ppppqppp/2n5/4P3/8/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6e5", 80),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 110),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8c5", 119),
			("f8b4", 93),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 197),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("a7a6", 27),
			("d7d5", 40),
			("g8e7", 69),
			("g7g6", 28),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 67),
			("e2e4", 166),
			("c2c4", 104),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e8f7", 46),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/5p2/2p5/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 35),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/3p2p1/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 53),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 79),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 44),
			("b8c6", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8f5", 111),
			("c7c5", 99),
			("b8c6", 65),
			("e7e6", 74),
			("c8g4", 76),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 25),
			("g1f3", 69),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8g4", 33),
			("g8f6", 213),
			("c8f5", 130),
			("d8b6", 41),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 63),
			("c2c3", 38),
		]
	),
	(
		"rn2kbnr/ppp1pppp/8/3q4/6b1/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 28),
			("f1e2", 242),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 123),
			("e7e5", 52),
			("d5d8", 71),
			("d5a5", 25),
			("b8c6", 40),
		]
	),
	(
		"rnb1kb1r/ppp1pppp/5n2/3q4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 72),
			("b1c3", 33),
		]
	),
	(
		"rnb1kb1r/ppp1pppp/5n2/3q4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 65),
		]
	),
	(
		"rn2kbnr/ppp1pppp/8/3q4/6b1/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 112),
			("b8c6", 65),
			("g8f6", 38),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 63),
			("d2d4", 94),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 79),
			("e7e6", 40),
			("e7e5", 78),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e4d5", 64),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3P4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6d5", 69),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 263),
			("e4e5", 38),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/5p2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 77),
			("f8d6", 25),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 792),
			("e2e3", 66),
			("g1f3", 29),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c8g4", 175),
			("e7e6", 56),
			("c7c6", 60),
			("b8c6", 45),
			("c7c5", 201),
			("g8f6", 189),
			("c8f5", 47),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/6b1/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("h2h3", 39),
			("f2f3", 31),
			("g1f3", 28),
			("g2g3", 42),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 1405),
			("b1c3", 320),
			("g1f3", 192),
			("d2d3", 62),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 1857),
			("c8a6", 57),
			("e7e6", 60),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 605),
			("f1d3", 1071),
			("b1d2", 53),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 132),
			("g8f6", 39),
			("g7g6", 61),
			("d7d6", 30),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1d3", 199),
			("a2a3", 83),
			("g1f3", 138),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c5", 33),
			("f8b4", 100),
			("g8f6", 41),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P2Bn1/8/PP2PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g5", 84),
			("f8b4", 52),
			("b8c6", 163),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 154),
			("f6e4", 95),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 63),
			("g8f6", 190),
			("c8f5", 57),
			("e7e6", 56),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 30),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 40),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 45),
			("d7d6", 82),
			("g8f6", 62),
			("d7d5", 939),
			("e7e5", 36),
		]
	),
	(
		"r1bqkbnr/pppppp1p/2n3p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4d5", 27),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 49),
			("d2d3", 52),
			("e2e3", 47),
			("g2g3", 26),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/5P2/2N2N2/PPPPP1PP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 38),
		]
	),
	(
		"r1bqkbnr/pp1npppp/3p4/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 76),
			("a7a6", 109),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 127),
			("b1c3", 246),
			("a2a3", 71),
			("e2e4", 117),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 110),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 68),
			("g2g3", 31),
			("a2a3", 30),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 102),
			("c1f4", 35),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 37),
			("e7e6", 51),
			("c7c6", 45),
			("c7c5", 53),
		]
	),
	(
		"rnbqk1nr/p1ppppbp/1p4p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1e3", 43),
			("g1f3", 26),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2pP4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 35),
			("c2c4", 26),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3Pn3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 99),
			("b1c3", 108),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3Pn3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 57),
			("e4f6", 40),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3pP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 38),
			("c2c3", 29),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pn1p/6B1/3PP3/8/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f6", 262),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pB1p/8/3PP3/8/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8f6", 253),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8g7", 506),
			("d7d6", 25),
			("c7c5", 25),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 354),
			("h2h3", 38),
			("b1c3", 40),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e8g8", 65),
			("d7d6", 110),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("d7d5", 31),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/7p/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 161),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/7p/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("h5h4", 122),
			("a7a5", 48),
			("g7g5", 26),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/8/3PP2p/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("h2h3", 25),
			("b1c3", 37),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d4", 75),
			("e4f5", 54),
			("d2d3", 126),
			("c4g8", 80),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3pP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 282),
			("d7d6", 113),
			("a7a6", 32),
			("e7e6", 139),
			("e7e5", 117),
			("g7g6", 32),
			("g8f6", 53),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 26),
			("g8f6", 29),
			("d4c3", 75),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 50),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 130),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 134),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 37),
			("f8e7", 72),
			("f8d6", 205),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 632),
			("g1f3", 41),
			("e2e3", 34),
			("c4d5", 80),
			("b1c3", 64),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pP3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5d4", 617),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4P3/2Pp4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 519),
			("a2a3", 60),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4P3/2Pp4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 476),
			("c7c5", 26),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g6", 42),
			("e7e6", 42),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 397),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 47),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("b8d7", 81),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c6", 31),
			("b8c6", 37),
			("f8c5", 36),
			("f8e7", 51),
			("f8b4", 58),
			("f6e4", 92),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3e5", 26),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 28),
			("f1g2", 38),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 43),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 75),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 34),
			("c4d5", 28),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 26),
			("g1f3", 41),
		]
	),
	(
		"rnbqkb1r/pppp1p1p/5pp1/8/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f6f5", 27),
			("f8g7", 64),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p1P3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 282),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 238),
			("g1f3", 72),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 225),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c3d4", 223),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 172),
			("f8g7", 43),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c4d5", 396),
			("b1c3", 61),
			("e2e3", 93),
			("g2g3", 38),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 368),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 139),
			("b1c3", 135),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3P1b2/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f5b1", 118),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3P4/3P4/8/PP2PPPP/RbBQKBNR w KQkq - 0 0",
		&[
			("d1a4", 86),
			("a1b1", 32),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3P4/Q2P4/8/PP2PPPP/RbB1KBNR b KQkq - 0 0",
		&[
			("c7c6", 58),
			("d8d7", 26),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 113),
			("g1f3", 41),
			("g1e2", 75),
			("c2c4", 38),
			("b2b3", 34),
			("d2d3", 59),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 59),
			("b7b5", 64),
			("g7g6", 43),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 32),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 68),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 171),
			("f8b4", 59),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 33),
			("e2e4", 115),
			("a2a3", 48),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 227),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 56),
			("f7f5", 78),
			("g7g6", 31),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 304),
			("e2e4", 77),
			("b1c3", 131),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 331),
			("d7d6", 58),
			("g8f6", 66),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pp4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 323),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pP4/8/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 242),
			("g8f6", 34),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p1P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 79),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p1P3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f2f4", 43),
			("g1f3", 30),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 38),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 400),
			("g1f3", 32),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 184),
			("g8f6", 189),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 146),
			("c5d4", 109),
			("g7g6", 31),
			("d7d5", 29),
			("b7b6", 62),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 33),
			("d2d4", 27),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 59),
		]
	),
	(
		"rnbqkb1r/pppppppp/1n6/4P3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 26),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/5b2/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4g3", 153),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/5b2/8/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f5g6", 148),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 106),
			("e2e4", 66),
			("c2c4", 357),
			("b1c3", 35),
			("c2c3", 83),
			("c1f4", 58),
			("e2e3", 69),
			("c1g5", 70),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 86),
			("f4e5", 30),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 75),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 2683),
			("f6d5", 126),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 40),
			("d2d4", 36),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 288),
			("b1c3", 175),
			("c4d5", 29),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2P5/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 261),
			("d8a5", 56),
			("g8f6", 47),
			("e7e5", 96),
			("b8a6", 32),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2P5/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 36),
			("c1e3", 71),
			("e2e4", 25),
			("b1c3", 65),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/6B1/3Pp3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("c8f5", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3PnB2/8/PPPNPPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e4d2", 29),
			("c8f5", 42),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 629),
			("g1f3", 179),
			("f1c4", 108),
			("f1e2", 57),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 237),
			("g8f6", 285),
			("e7e6", 39),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("b8c6", 71),
			("a7a6", 60),
			("g8f6", 28),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 34),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 32),
		]
	),
	(
		"r1bqkbnr/1p1ppppp/p1n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("b5c6", 82),
		]
	),
	(
		"r1bqkbnr/1p1ppppp/p1B5/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7c6", 57),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 96),
			("c7c6", 28),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 39),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d8c7", 52),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 48),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8d7", 29),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p4/3nP3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5b6", 536),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 59),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P1NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8g4", 43),
			("c8f5", 31),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 94),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("g8f6", 73),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g2g3", 31),
			("e2e3", 35),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 54),
			("e7e5", 37),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1d2", 28),
			("b1c3", 27),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 50),
			("c1g5", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 247),
			("e7e6", 145),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 67),
			("d2d4", 109),
			("d5c6", 63),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/2P5/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6d5", 60),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 32),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/1B2p3/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 52),
			("c2c3", 29),
			("d2d4", 33),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8g7", 50),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("g7g6", 39),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/8/5bP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("g2f3", 44),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c8b7", 70),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 34),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 102),
			("f1c4", 48),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 39),
			("g7g6", 114),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 27),
			("d2d4", 659),
			("f3e5", 223),
			("f1b5", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 51),
			("g8f6", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p2B1/3Pn3/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5h4", 30),
			("g5f4", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3Pn2B/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 29),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 38),
			("e4f5", 51),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f5e4", 266),
			("g8f6", 50),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 235),
			("d7d6", 55),
			("g7g6", 27),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 50),
			("d2d3", 39),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 43),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8e7", 25),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8f5", 42),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 96),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N4P/PPP2PP1/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 55),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1d2", 43),
			("b1d2", 27),
			("b1c3", 51),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/5N2/PP1BPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8e7", 141),
			("c7c5", 75),
			("b4d2", 39),
			("a7a5", 38),
			("b4e7", 25),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 144),
			("g1f3", 320),
			("c1f4", 74),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 37),
			("e2e3", 56),
			("d2d3", 29),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e8g8", 167),
			("b7b6", 149),
			("c7c5", 39),
			("d7d5", 70),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 216),
			("g1f3", 38),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7e6", 155),
			("g7g6", 27),
			("c7c5", 214),
			("c8f5", 56),
			("b8c6", 47),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 38),
			("f1d3", 34),
			("b1d2", 84),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 351),
			("a7a6", 36),
			("c7c5", 36),
			("b7b6", 128),
			("f8e7", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 60),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 33),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 63),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 174),
			("g2g3", 57),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P1N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 152),
			("g8f6", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1B2/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7f6", 65),
			("g7f6", 39),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1p2/8/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 32),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 390),
			("g1f3", 56),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5d8", 283),
			("d5a5", 40),
			("d5d6", 40),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 92),
			("g1f3", 130),
			("g2g3", 46),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 44),
			("g8f6", 28),
		]
	),
	(
		"rnbqkbnr/ppp1ppp1/7p/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5h4", 309),
		]
	),
	(
		"rnbqkbnr/ppp1ppp1/7p/3p4/3P3B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 42),
			("g8f6", 30),
			("c7c6", 134),
			("g7g5", 36),
			("c8f5", 39),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("b1c3", 300),
			("d2d3", 321),
			("e4f5", 70),
			("d2d4", 82),
			("b5c6", 134),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 65),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3QP3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b8c6", 539),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3QP3/8/PPP2PPP/RNB1KBNR w KQkq - 0 0",
		&[
			("d4e3", 414),
			("d4d1", 55),
			("d4d3", 41),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/4P3/4Q3/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("d7d6", 53),
			("g8f6", 230),
			("f8b4", 32),
			("g7g6", 43),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c6", 85),
			("g8f6", 68),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 99),
			("d2d4", 148),
			("e2e3", 49),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 91),
			("e7e6", 25),
			("d7d5", 75),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5P2/8/3p4/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d4c3", 136),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/8/1B2p3/3nP3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("f3d4", 244),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 709),
			("e7e5", 75),
			("e7e6", 25),
			("d7d6", 77),
			("g7g6", 43),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 438),
			("c4d5", 246),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 227),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 223),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 216),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 168),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("b8c6", 219),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/4p3/8/1bPP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 146),
			("b1d2", 106),
			("c1d2", 307),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/4p3/8/1bPP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 93),
		]
	),
	(
		"rnbqk1nr/pp1p1ppp/4p3/2p5/1bPP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 26),
			("e2e3", 33),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 88),
			("f1e2", 67),
			("f1d3", 131),
			("c2c3", 42),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 79),
			("c5d4", 43),
			("b7b6", 38),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 132),
			("d2d4", 34),
			("b1c3", 34),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 102),
			("c7c5", 33),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 103),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 26),
			("e7e6", 45),
		]
	),
	(
		"rnbqkb1r/pppp1p1p/5pp1/8/3P4/6P1/PPP1PP1P/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 60),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 45),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 29),
			("d7d5", 56),
			("c7c5", 32),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 29),
			("g7g6", 34),
			("e7e6", 33),
			("g8f6", 163),
			("e7e5", 48),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 174),
			("e4e5", 34),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pP4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 173),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 119),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 25),
			("g2g3", 29),
			("b1c3", 33),
			("d2d4", 26),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 81),
		]
	),
	(
		"rnbqkb1r/ppp1p1pp/3p1n2/5p2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 30),
			("c1g5", 27),
		]
	),
	(
		"rnbqkb1r/ppp1p1pp/3p1n2/5p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 46),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2pP4/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 63),
			("g7g6", 25),
			("e7e6", 38),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 79),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1d2", 56),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d6", 130),
			("g8f6", 410),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 825),
			("e7e6", 44),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2pP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 132),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2BP4/4P3/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("e5d4", 127),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 63),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p2B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("f6e4", 97),
			("f8g7", 42),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2pP4/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 273),
			("b7b5", 141),
			("d7d6", 155),
			("g7g6", 94),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 122),
			("c2c4", 152),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e6d5", 76),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 107),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 64),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 215),
			("g1f3", 98),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 259),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/4p3/8/1bPP4/8/PP1NPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 48),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/1P6/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 29),
			("d7d5", 396),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 85),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 46),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2ppP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 42),
			("f2f4", 60),
			("c2c3", 51),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 458),
			("e2e4", 90),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 390),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 273),
			("e2e3", 36),
			("g2g3", 36),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 65),
			("d7d6", 87),
			("d7d5", 52),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 35),
			("e2e4", 147),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 46),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b8c6", 69),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 61),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/1P2PN2/PBPP1PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f7f6", 35),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 52),
			("d4d5", 191),
			("e2e3", 29),
			("b1c3", 39),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 89),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 121),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 48),
			("a7a6", 32),
			("b8c6", 51),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 384),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d5e4", 256),
			("g8f6", 94),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4p3/1P6/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b1c3", 253),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 272),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 162),
			("d7d5", 54),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 76),
			("g1f3", 32),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 191),
			("c7c5", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 29),
			("e4e5", 119),
			("e4d5", 32),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 74),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d6", 37),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/3P4/PPPNPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 60),
			("g8f6", 58),
			("c7c6", 40),
			("c7c5", 49),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 32),
			("g2g3", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4p3/1PN5/PBPP1PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f7f5", 27),
			("g8f6", 216),
		]
	),
	(
		"rnbqkb1r/pppppppp/1n6/4P3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 54),
			("g1f3", 71),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 87),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 113),
			("d4c3", 94),
			("d4d3", 28),
			("g8f6", 26),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 233),
			("d2d4", 27),
			("g2g3", 88),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 207),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 58),
			("c2c4", 25),
			("f1e2", 112),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 25),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d4", 205),
			("e1g1", 33),
			("h2h3", 25),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c5d4", 74),
			("g8f6", 61),
			("e7e6", 40),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/4p3/1p6/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1b5", 46),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 36),
			("c7c5", 28),
		]
	),
	(
		"rnbqkbnr/1ppppppp/8/p7/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 124),
			("g1f3", 62),
		]
	),
	(
		"rnbqkbnr/1ppppppp/8/p7/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("a5a4", 51),
			("a8a6", 32),
			("b7b5", 32),
			("h7h5", 47),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4P3/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 33),
			("c6e5", 69),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 529),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d5c4", 109),
			("c7c6", 36),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2p5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 95),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d8a5", 42),
			("c5d4", 188),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 295),
			("d7d6", 65),
			("d7d5", 28),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 359),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 36),
			("d2d4", 48),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/4pP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d3", 201),
			("b1c3", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/4pP2/3P4/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 25),
			("g8f6", 148),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 42),
			("c5d4", 548),
			("e7e6", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5f6", 72),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/P1N2N2/1PPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 44),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/3p4/8/3P2b1/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 35),
			("e2e4", 25),
			("g2g3", 39),
			("e2e3", 54),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2B3p1/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b7c6", 124),
			("d7c6", 199),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 370),
			("d4c5", 45),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 352),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 54),
			("b8c6", 42),
			("e7e6", 28),
		]
	),
	(
		"rnbqkbnr/1p2pppp/p2p4/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 33),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 26),
			("c7c5", 36),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 310),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 86),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 130),
			("g7g6", 143),
			("e7e6", 35),
			("e7e5", 77),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2B1Pp2/8/PPPP2PP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 127),
			("g8f6", 146),
			("d7d6", 39),
			("d8h4", 78),
			("g8e7", 25),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/2B1Pp2/8/PPPP2PP/RNBQK1NR w KQkq - 0 0",
		&[
			("e4d5", 27),
			("c4d5", 100),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 126),
			("d5c4", 85),
			("e7e6", 110),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c4d5", 78),
			("d1d4", 45),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3P4/3p4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 59),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 96),
			("d7d6", 44),
			("e7e6", 80),
		]
	),
	(
		"rnbqkbnr/1p2pppp/p7/2pp4/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 84),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 118),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 42),
			("g7g6", 30),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 112),
			("f8b4", 31),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 55),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f7f5", 74),
			("g8f6", 179),
			("d5c4", 39),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 55),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 124),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8b7", 116),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 54),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d5", 76),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 370),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 70),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 69),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 26),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 262),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3Pp3/8/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d5", 261),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/6b1/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 44),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/8/1P3b2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2f3", 37),
		]
	),
	(
		"rnb1kbnr/pppp1ppp/8/4p3/4PP1q/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 63),
		]
	),
	(
		"rnb1kbnr/pppp1ppp/8/4p3/4PP1q/6P1/PPPP3P/RNBQKBNR b KQkq - 0 0",
		&[
			("h4e7", 56),
		]
	),
	(
		"rnb1kbnr/ppppqppp/8/4p3/4PP2/6P1/PPPP3P/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 28),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 50),
			("g1f3", 60),
			("d2d3", 27),
			("d2d4", 51),
			("e2e3", 39),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 81),
			("g7g6", 31),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 61),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 33),
			("d7d6", 60),
			("e7e6", 64),
			("b8c6", 127),
			("g8f6", 33),
		]
	),
	(
		"rnbqkbnr/1ppppppp/8/8/p2PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 28),
		]
	),
	(
		"rn1qkbnr/pbpppppp/8/1p6/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f2f3", 46),
			("f1d3", 58),
			("f1b5", 28),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4p3/1bP5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 123),
			("c3d5", 25),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4p3/1bP5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("e8g8", 94),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 51),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P2P1/8/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f6g4", 44),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 48),
			("c1g5", 63),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d5d8", 27),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 86),
			("g2g3", 54),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 143),
			("c7c6", 40),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e5d4", 216),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("e7e5", 101),
			("c7c6", 29),
			("c7c5", 37),
			("b8c6", 39),
			("g8f6", 49),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 58),
			("d2d4", 139),
			("g1f3", 47),
			("g2g3", 29),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 51),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 80),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 28),
			("c7c5", 29),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 111),
			("g1f3", 34),
			("e2e4", 60),
			("b1d2", 46),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 62),
			("b8c6", 29),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 28),
			("g1f3", 32),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 150),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8e7", 179),
			("g8f6", 99),
			("f7f6", 86),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5e7", 116),
			("h2h4", 53),
		]
	),
	(
		"rnbqk1nr/ppp1Bppp/4p3/3p4/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8e7", 49),
			("g8e7", 68),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 67),
			("g8f6", 133),
			("c8g4", 140),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 65),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p1b2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 28),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1h3", 53),
			("e1f2", 52),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/5P1N/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 41),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8g7", 297),
			("c7c6", 186),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 70),
			("f8g7", 69),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 68),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 25),
			("f8g7", 37),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4P3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d6e5", 275),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1d8", 253),
		]
	),
	(
		"rnbQkbnr/ppp2ppp/8/4p3/2P5/8/PP2PPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("e8d8", 253),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 124),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/2N1P3/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 101),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 142),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d3", 89),
			("e4d5", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 71),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4p3/2PP4/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d3e4", 71),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d1", 63),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 77),
			("g7g6", 68),
			("d7d5", 25),
			("c5d4", 89),
			("b7b6", 26),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3p4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e3d4", 108),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 48),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f7f5", 70),
			("g7g6", 159),
			("g8f6", 80),
			("e7e6", 558),
			("d7d6", 132),
			("d7d5", 31),
		]
	),
	(
		"rn1qkbnr/pbppp1pp/1p6/5p2/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("e4f5", 25),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 42),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 132),
			("g1f3", 34),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 66),
			("g8f6", 41),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d3", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 45),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f3e5", 58),
			("f1c4", 60),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1N3/4P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5f2", 37),
			("c6e5", 38),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 237),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 94),
			("g1f3", 48),
			("d2d4", 62),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 44),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 616),
			("d7d5", 31),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 354),
			("f1c4", 64),
			("b1c3", 121),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 95),
			("c7c5", 62),
			("c7c6", 26),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/4P3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6g8", 26),
		]
	),
	(
		"rnbqkbnr/ppppppp1/8/7p/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 35),
			("e2e4", 92),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 56),
			("d1e2", 37),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 55),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 25),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 192),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 75),
			("e7e6", 32),
			("d7d5", 45),
			("b8c6", 61),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 105),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 176),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 144),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 153),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 71),
			("c2c4", 25),
			("d2d4", 34),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("b8c6", 69),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 31),
			("c1f4", 76),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 166),
		]
	),
	(
		"rnb1kbnr/pp2pppp/1qp5/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("b2b3", 34),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e6d5", 114),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2P5/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 120),
			("c7c6", 77),
			("g8f6", 96),
			("g7g6", 106),
			("b8d7", 30),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 60),
			("d4e5", 32),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/5P2/3P1N2/PPP1P1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 35),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/4p3/8/1bPP4/8/PP1BPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8e7", 129),
			("a7a5", 73),
			("b4d2", 90),
		]
	),
	(
		"rnb1k1nr/ppppqppp/4p3/8/1bPP4/8/PP1BPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 50),
			("a2a3", 27),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 151),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 806),
			("e7e6", 58),
			("g8f6", 101),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3p4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 70),
			("f3d4", 693),
			("d1d4", 148),
			("e2e3", 43),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3p4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d4c3", 36),
		]
	),
	(
		"r1bqkbnr/ppppp1pp/2n5/5p2/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4f5", 443),
			("b1c3", 27),
			("e4e5", 64),
		]
	),
	(
		"r1bqkbnr/ppppp1pp/2n5/5P2/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 446),
		]
	),
	(
		"r1bqkbnr/ppp1p1pp/2n5/3p1P2/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 332),
			("f1b5", 69),
		]
	),
	(
		"r1bqkbnr/ppp1p1pp/2n5/3p1P2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 330),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 47),
			("d7d6", 174),
			("g7g6", 93),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 273),
			("e4d5", 937),
			("b1c3", 786),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3pP3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 65),
			("c8g4", 135),
			("c6c5", 28),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 43),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3N4/8/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 291),
			("d7d5", 74),
			("e7e6", 68),
			("e7e5", 67),
			("d7d6", 43),
			("g8f6", 122),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3N4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 41),
			("d4c6", 49),
			("d4b3", 35),
			("g2g3", 47),
			("d4f3", 65),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 27),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d6", 66),
			("d7d5", 47),
			("c7c5", 38),
			("g8f6", 83),
			("c7c6", 33),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 28),
			("e2e4", 112),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 51),
			("b8c6", 35),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d5c3", 26),
			("g7g6", 70),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 82),
			("e7e5", 42),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d1a4", 38),
			("g1f3", 71),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 581),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 516),
			("g1f3", 74),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 56),
			("c7c6", 88),
			("c7c5", 43),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d3", 137),
			("b1c3", 211),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 51),
			("b1c3", 27),
			("g1f3", 40),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d6", 36),
			("g8f6", 47),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 142),
			("g8e7", 38),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/2N2P2/PPP1P1PP/R2QKBNR b KQkq - 0 0",
		&[
			("e7e6", 41),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/2N2Q2/PPPP1PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("d5e4", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 110),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 109),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 39),
			("b8c6", 33),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 134),
			("d2d4", 1611),
			("b5c6", 39),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/1Bp5/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("c8d7", 111),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 43),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e5e4", 49),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 605),
			("g1f3", 148),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e6", 107),
			("b8c6", 286),
			("g7g6", 72),
			("g8f6", 47),
			("d7d5", 64),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e3", 44),
			("g1f3", 29),
			("c2c4", 26),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 92),
			("d7d5", 100),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 413),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 239),
			("f7f5", 28),
			("g8f6", 35),
			("g8e7", 46),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d3", 37),
			("g1f3", 131),
			("e2e3", 32),
			("c2c4", 76),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3P4/3P2b1/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1e2", 92),
			("g1f3", 48),
			("f1b5", 28),
			("f2f3", 66),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3P4/3P2b1/8/PPP1BPPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g4e2", 89),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PPQ1PPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("g8f6", 139),
			("d5c4", 37),
			("f7f5", 76),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 51),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d5", 28),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 29),
			("g8f6", 40),
		]
	),
	(
		"r1b1kbnr/ppppqppp/2n5/4P3/5B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7b4", 63),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 41),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/3p4/8/2PP2b1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f3", 67),
			("b8d7", 29),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/3p4/8/2PP4/5b2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2f3", 42),
			("g2f3", 26),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 110),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2P5/PP1N1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c5d4", 55),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 25),
			("f8g7", 838),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 102),
			("g1f3", 42),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 97),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 67),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 362),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 358),
		]
	),
	(
		"rnbqkbnr/ppp1p1pp/3p4/5p2/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 37),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3p4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 49),
			("e2e3", 71),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 159),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 78),
			("e2e4", 28),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 31),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 44),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 79),
			("c7c6", 42),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("d1d2", 29),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/1B2pp2/4P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 72),
			("f5e4", 246),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 286),
			("e7e6", 43),
			("b8c6", 164),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/6P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 57),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 202),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 170),
			("g1f3", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 197),
			("c7c6", 80),
			("b8c6", 26),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8d6", 41),
			("g8f6", 73),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 66),
			("c7c5", 31),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 28),
			("b2b3", 26),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8e7", 26),
			("c7c5", 33),
			("c8d7", 25),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 64),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 36),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P4/6P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 185),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 130),
			("e2e4", 40),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 364),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 259),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 258),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 202),
			("c2c4", 33),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("e7e6", 90),
			("c7c5", 75),
			("g7g6", 33),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/1Bp5/4P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c6d4", 155),
			("g7g6", 61),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/8/1Bp5/3nP3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("b5c4", 96),
			("g1f3", 42),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/8/2p5/2BnP3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("e7e6", 67),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 131),
			("g8f6", 25),
			("e5e4", 41),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 225),
			("e2e3", 42),
			("d4c5", 32),
			("g1f3", 81),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7c3", 98),
			("d7d6", 90),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d6", 33),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 221),
			("e6d5", 152),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 184),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 55),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3Pn3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e4", 95),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3PN3/8/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 95),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 292),
			("e7e6", 458),
			("d7d6", 34),
			("g8f6", 48),
			("a7a6", 46),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 74),
			("b1c3", 35),
			("c2c3", 44),
			("d1h5", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 78),
			("d5e4", 91),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 140),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 129),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 74),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 62),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/3pP3/8/PPPPNPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2g3", 181),
			("g1f3", 87),
			("f2f4", 55),
			("d2d3", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/3pP3/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8e6", 67),
			("c7c5", 31),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e4e5", 65),
			("d2d4", 50),
			("f1c4", 26),
			("f1b5", 60),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p1P3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d6e5", 68),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 63),
			("f1d3", 33),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p1P3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 63),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5N2/PPPPQPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("b8c6", 123),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5N2/PPPPQPPP/RNB1KB1R w KQkq - 0 0",
		&[
			("g2g3", 102),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5NP1/PPPPQP1P/RNB1KB1R b KQkq - 0 0",
		&[
			("g7g6", 38),
		]
	),
	(
		"r1bqkbnr/pppppppp/n7/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1a6", 57),
			("d2d4", 108),
			("g1f3", 41),
		]
	),
	(
		"r1bqkbnr/pppppppp/B7/8/4P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b7a6", 52),
		]
	),
	(
		"r1bqkbnr/p1pppppp/p7/8/4P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 26),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("c6d4", 48),
			("g7g6", 49),
			("d8c7", 55),
			("e7e6", 55),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 37),
			("f7f6", 43),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/1B2pP2/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e5e4", 67),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/6B1/3Pp3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("b8c6", 133),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/3p1n2/8/2PP2b1/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 50),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 64),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 53),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 43),
			("b8c6", 41),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 63),
			("f1c4", 65),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 50),
			("c2c4", 60),
			("e2e3", 96),
			("c2c3", 67),
			("c1g5", 43),
			("d4d5", 32),
			("d4c5", 35),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 228),
			("g1f3", 28),
			("e2e3", 35),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 419),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/1P2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b4c5", 87),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2P5/4P3/8/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8c5", 76),
		]
	),
	(
		"rnbqk1nr/pp1p1ppp/4p3/2b5/4P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 72),
		]
	),
	(
		"rnbqk1nr/pp1p1ppp/4p3/2b5/3PP3/8/P1P2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5e7", 34),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 47),
			("d2d4", 57),
			("f3e5", 238),
			("f1c4", 40),
			("e4f5", 89),
			("d2d3", 69),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 46),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2p5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 35),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2p5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 64),
			("b8d7", 27),
			("b7b5", 25),
			("f7f6", 28),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 108),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6e4", 110),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/4n3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 27),
			("d1d5", 55),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 30),
			("b8d7", 32),
			("c8f5", 34),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f5e4", 45),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 809),
			("g2g3", 127),
			("f1e2", 81),
			("a2a4", 58),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 791),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3pp3/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 37),
			("b8d7", 42),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 67),
			("b1c3", 28),
			("g2g3", 30),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 56),
			("d7d5", 196),
			("e7e6", 89),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 48),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2p5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 53),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("a2a3", 31),
			("d2d4", 48),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/1p1PP3/5N2/P1P2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 44),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PPP3/5P2/PP4PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 65),
			("e8g8", 30),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 337),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 40),
			("e7e6", 40),
			("e7e5", 35),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 154),
			("e2e3", 44),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 291),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("b2b3", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/2B5/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8e6", 68),
			("d5b6", 166),
			("c7c6", 153),
			("d5c3", 76),
			("e7e6", 70),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq  - 0 0",
		&[
			("e5d6", 41),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pP4/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7d6", 63),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2pp4/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 29),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3P4/3P2b1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f3", 112),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 77),
			("b1c3", 409),
			("c2c4", 54),
			("f1d3", 31),
			("f2f4", 40),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 30),
			("f8g7", 49),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/4q3/8/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1e2", 177),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/4q3/8/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
		&[
			("e6g6", 114),
			("c7c6", 27),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 250),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 47),
			("e4e5", 148),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 91),
			("d4c5", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp2B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 37),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4p3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e4", 144),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4N3/5N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8e7", 25),
			("g8f6", 42),
			("b8d7", 43),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p2B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 47),
			("h7h6", 35),
			("g8f6", 26),
			("c8f5", 44),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4p3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e4d3", 82),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3pP3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e2", 212),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 201),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("h7h6", 51),
			("f8e7", 37),
			("d7d5", 36),
			("c7c5", 72),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pn1p/6B1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5h4", 34),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3Pn3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e4f6", 31),
			("e4c3", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 57),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 43),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 70),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 85),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 267),
			("g1f3", 93),
			("g2g3", 324),
			("e2e4", 85),
			("d2d3", 26),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 146),
			("d2d4", 30),
			("c2c4", 41),
			("b2b3", 50),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("f8g7", 138),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 107),
			("e4e5", 94),
			("b1c3", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pP4/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 64),
			("d8d5", 43),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pn1p/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5h4", 188),
			("g5f6", 85),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pn1p/8/3P3B/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d6", 30),
			("c7c5", 69),
			("b7b6", 41),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 271),
			("h2h3", 50),
			("e2e3", 105),
			("g4g5", 63),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/6P1/8/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c8g4", 74),
			("c7c6", 144),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/6b1/8/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 72),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 290),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 210),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1b5", 134),
			("d2d4", 37),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/1Bpp4/5P2/4PN2/PPPP2PP/RNBQK2R b KQkq - 0 0",
		&[
			("f7f6", 26),
			("c8d7", 30),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 87),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 78),
			("b8c6", 70),
			("e7e6", 32),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3p4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c3d4", 123),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 57),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5N2/PPQ1PPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("e7e6", 55),
			("g7g6", 53),
			("d5c4", 34),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5b4", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/2PPp3/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d2", 45),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/8/1B2p3/3NP3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e5d4", 259),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("d7d6", 64),
			("g8f6", 146),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d4c3", 34),
			("g8f6", 71),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("a7a6", 154),
			("d7d5", 291),
			("b8c6", 243),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 144),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("b7b5", 64),
			("b8c6", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 60),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3Pp3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f2f3", 56),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3Pp3/2P2P2/PP4PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e4f3", 33),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 62),
			("f8c5", 59),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3Q4/5N2/PPP1PPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("b8c6", 91),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 214),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 86),
			("d7d6", 56),
			("d7d5", 30),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("d7d6", 33),
			("g8f6", 50),
			("g8e7", 159),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 179),
			("a7a6", 159),
			("b8c6", 247),
			("g7g6", 28),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e3", 54),
			("g1f3", 72),
			("d2d3", 29),
			("c2c4", 82),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2pP4/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 40),
			("g8f6", 93),
			("e7e5", 33),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2pP4/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 82),
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pp4/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3d5", 96),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 112),
			("d1d4", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e3", 28),
			("d2d3", 70),
			("c2c4", 49),
			("h2h3", 27),
			("g1f3", 91),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pN4/8/8/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 40),
			("g8f6", 44),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 111),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 35),
			("d2d4", 29),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("e7e5", 26),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 25),
			("e7e5", 111),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1g5", 263),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5pB1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e7e6", 94),
			("d7d5", 115),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5pB1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("e2e4", 73),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5pB1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f5e4", 57),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 46),
			("d2d3", 52),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e5e4", 34),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 27),
			("d7d6", 25),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 195),
			("f1e2", 74),
			("c2c4", 47),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 179),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/3p4/4p3/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 49),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/3Pp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f7f5", 76),
		]
	),
	(
		"rnbqkbnr/ppp3pp/3p4/3Ppp2/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 29),
			("b1c3", 42),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c3", 132),
			("g1f3", 25),
			("b1c3", 31),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/2P1P3/PP3PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 135),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 107),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 73),
			("c2c3", 27),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3PPP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 89),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b6", 39),
			("g8f6", 52),
			("b8c6", 79),
			("d7d5", 33),
			("a7a6", 45),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 131),
			("c7c6", 84),
			("e7e5", 120),
			("c7c5", 43),
			("b8c6", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 76),
			("c2c3", 53),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 41),
			("e7e5", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 32),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 58),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2p5/2B1P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("g8f6", 40),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3P4/3p4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 45),
			("g8f6", 65),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 146),
			("c2c3", 83),
			("e2e4", 36),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/8/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 85),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/1B2pp2/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f5e4", 59),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 149),
			("b1c3", 41),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 39),
			("e7e6", 49),
			("d7d6", 27),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 25),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2Pp4/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 30),
			("c7c5", 41),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 28),
			("c5d4", 104),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/3PP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5f6", 95),
			("d5b6", 92),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 138),
			("d7d6", 75),
			("g7g6", 45),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/2Pp4/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 32),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2p5/3PnB2/5P2/PPP1P1PP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8a5", 162),
			("e4f6", 141),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 113),
			("c1f4", 28),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 27),
			("g8f6", 68),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6d5", 277),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 458),
			("g1f3", 63),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 286),
			("g8f6", 42),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 238),
			("f8g7", 229),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2pP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 41),
			("e2e4", 30),
			("b1c3", 25),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 51),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 74),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c1f4", 28),
			("c1g5", 27),
			("e2e3", 28),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 31),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("h2h3", 64),
			("e4e5", 114),
			("g1f3", 110),
			("e4d5", 108),
			("f2f3", 39),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/2N4P/PPP2PP1/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 54),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 28),
			("c4d5", 40),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/2P5/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 103),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 90),
			("b8c6", 223),
			("e7e6", 56),
			("d7d6", 50),
			("d7d5", 64),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 64),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 68),
			("f1c4", 31),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/3PPp2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g5", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/3pP3/8/PPPPNPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2g3", 225),
			("g1f3", 31),
			("d2d3", 75),
			("f2f4", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/3pP3/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 199),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 126),
			("g2g3", 198),
			("d2d4", 106),
			("e2e4", 44),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 130),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 25),
			("d2d4", 50),
			("b1c3", 40),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 86),
			("e2e4", 100),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p3P/8/8/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 78),
			("h7h6", 43),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp2P/8/8/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d3", 50),
			("h5h6", 41),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3P4/2n5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("b2c3", 52),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/3P4/2P5/P1P2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 27),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 118),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3p4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 99),
			("f1g2", 29),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3N4/6P1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 47),
			("e7e5", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 39),
			("d4c5", 70),
			("b1c3", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 37),
			("e7e5", 168),
			("g7g6", 128),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 30),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 127),
			("d2d4", 609),
			("f1c4", 28),
			("b1c3", 57),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b5", 68),
		]
	),
	(
		"rnbqkbnr/pppp2pp/5P2/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 66),
		]
	),
	(
		"rnbqkb1r/pppp2pp/5n2/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 33),
		]
	),
	(
		"rnbqkbnr/1ppppppp/8/p7/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 34),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/1B6/8/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 150),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/2PP4/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 50),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/P7/1P2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 47),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 62),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1d2", 31),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 81),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/1P6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b4b5", 49),
			("a2a3", 27),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/1P6/8/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 31),
			("a7a6", 26),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("d1d2", 30),
			("e2e3", 31),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 128),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/1P6/8/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 55),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 27),
			("g1e2", 25),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 133),
			("c1g5", 31),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/8/3P1NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 54),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/8/3P1NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d5e4", 390),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4p3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3g5", 400),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/6N1/4p3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 238),
			("c8f5", 75),
			("e7e5", 46),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/6N1/4p3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 115),
			("f1c4", 101),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/6N1/4p3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 36),
			("c8f5", 152),
			("e7e5", 27),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/5NP1/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 192),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 190),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 47),
			("g7g6", 25),
			("d7d6", 28),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 137),
			("g1f3", 25),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2pP4/2P1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 128),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 33),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 64),
			("b1c3", 48),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 55),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c6", 85),
			("g8f6", 35),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/P1N5/1PP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 30),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4d5", 73),
			("e2e3", 112),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pP4/2p5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/2p1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 27),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 41),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 33),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 51),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P1P2/8/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 78),
			("e7e6", 28),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 41),
			("g2g3", 40),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 63),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/3P2P1/PPPNPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8g4", 36),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P2P1/8/PPP2P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f5e4", 71),
			("f5g6", 89),
			("f5d7", 65),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/P1N5/1P2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f7f5", 38),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 39),
			("e7e6", 108),
			("d7d6", 50),
			("g8f6", 46),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f8g7", 451),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 104),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 117),
			("c2c4", 54),
			("e2e3", 47),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 71),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pPp3/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 148),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 137),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8e7", 26),
			("f7f5", 63),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 67),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 99),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2p1p3/1P2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 29),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 53),
			("f8g7", 108),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 181),
			("g1f3", 104),
			("d2d4", 39),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f7f5", 74),
			("b8c6", 36),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/8/PPPPQPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("c7c5", 174),
			("f8e7", 60),
			("d7d5", 195),
			("g8f6", 36),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 45),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 93),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 386),
			("c2c3", 122),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b8c6", 305),
			("d8b6", 37),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 261),
			("e8g8", 93),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 138),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 143),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 49),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b8c6", 31),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4Np2/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 90),
			("d8f6", 81),
			("b8c6", 34),
		]
	),
	(
		"rnbqkb1r/pppp2pp/5n2/4Np2/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4f5", 31),
			("d2d4", 33),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 69),
			("b1c3", 70),
			("e2e3", 31),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 52),
			("e8g8", 30),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d3", 26),
			("d2d4", 803),
			("g1f3", 50),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3P4/1b1P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 386),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P1n3/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a3", 215),
			("g1f3", 223),
			("d1c2", 39),
			("b1d2", 53),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P1n3/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b7b6", 124),
			("d7d6", 61),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 72),
			("g2g3", 56),
			("d2d4", 107),
			("e2e3", 39),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/1P6/8/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 40),
			("c2c4", 34),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 234),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 43),
			("e7e5", 85),
			("c7c6", 51),
			("c7c5", 26),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 205),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/2PPp3/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3g5", 103),
			("f3d2", 71),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/6N1/2PPp3/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f5", 88),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 61),
			("c7c5", 25),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 26),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/2NP4/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 217),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/8/2NP4/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d3d4", 216),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 214),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 213),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 222),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 95),
			("e7e6", 66),
			("d7d6", 58),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 50),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 132),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c8b7", 129),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 47),
			("g1f3", 33),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 81),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2pp4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d4", 52),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c1g5", 28),
			("g1f3", 27),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("e4d5", 69),
			("f1g2", 88),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/8/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 69),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p2B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 44),
			("f6e4", 27),
			("e7e6", 28),
			("c7c6", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 154),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/4P3/2pP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 145),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4P3/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d6e5", 254),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 30),
			("f1e2", 30),
			("c2c4", 36),
		]
	),
	(
		"rnbqk1nr/pppp1pbp/6p1/4p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 62),
		]
	),
	(
		"rnbqk1nr/pppp1pbp/6p1/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 30),
			("g8e7", 29),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p3/3Pn3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 225),
			("d4e5", 72),
			("f3e5", 28),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p3/3Pn3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d5", 195),
			("b8c6", 26),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2BpP3/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 42),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/4q3/8/2N5/PPPPQPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("e5e2", 153),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("b1d2", 42),
			("g5f6", 44),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2B2/3p4/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7f6", 42),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 49),
			("e7e6", 46),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 33),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2p3B1/3Pn2P/8/PPP1PPP1/RN1QKBNR w KQkq - 0 0",
		&[
			("d4d5", 31),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("e7e5", 154),
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 29),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/6PN/PPPPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 29),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 64),
			("b1c3", 85),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8f5", 25),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1c4", 27),
			("g1f3", 29),
			("e4g3", 27),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 37),
			("c1g5", 28),
			("g2g3", 56),
			("e2e4", 26),
			("c1f4", 31),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 35),
			("e7e6", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/4P3/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 55),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b7b5", 89),
			("b8c6", 25),
			("d7d6", 64),
			("e7e6", 72),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1pp5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 64),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1pp5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 49),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 31),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 262),
			("e4d5", 252),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 119),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d8c7", 55),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 48),
			("e2e3", 161),
			("a2a4", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b7b5", 42),
			("g8f6", 28),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1h3", 37),
			("e1f2", 47),
		]
	),
	(
		"rnbqkb1r/p2ppppp/1p3n2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 113),
		]
	),
	(
		"rnbqkb1r/p2ppppp/1p3n2/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 113),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3PnB2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 76),
			("c7c6", 25),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 139),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("b7b5", 60),
			("b8c6", 37),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 179),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 173),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 202),
			("c2c4", 69),
			("g1f3", 46),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 83),
			("e7e6", 26),
		]
	),
	(
		"r1bqkbnr/pppn1ppp/3p4/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/5p2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 100),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 50),
			("e2e3", 67),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/P7/1P2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 191),
			("g7g6", 62),
			("e7e6", 98),
			("d8b6", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3Pp3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("f3e4", 177),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3PP3/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 134),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 48),
		]
	),
	(
		"r1bqkbnr/pppppppp/n7/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 72),
			("g1f3", 27),
		]
	),
	(
		"r1bqkbnr/pppppppp/n7/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("h7h5", 30),
			("a6b4", 35),
			("b7b5", 30),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/3P2b1/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 77),
			("c2c4", 27),
			("f3e5", 36),
			("g2g3", 41),
			("c1g5", 27),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f3", 44),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/3p4/8/3PP1b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 37),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6pB/8/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 30),
			("f8h6", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 87),
			("b1c3", 482),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 90),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 34),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 86),
			("b1c3", 88),
			("d1e2", 101),
			("d1f3", 25),
			("d2d3", 36),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("a7a6", 42),
			("b8c6", 95),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 68),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3pp3/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 33),
			("e6d5", 25),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/2P5/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 341),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 193),
			("d2d4", 53),
			("d2d3", 26),
			("g1f3", 33),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 131),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 139),
		]
	),
	(
		"rnbqkbnr/ppp2pp1/4p2p/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 29),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/P1N5/1PP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b4c3", 184),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4c5", 40),
			("e2e3", 25),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 101),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d3", 65),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5e4", 48),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 53),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P3P/8/PPP1PPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 52),
			("h7h5", 33),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P3P/8/PPP1PPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("h4h5", 51),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d1g4", 145),
			("d2d3", 62),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P1Q1/2N5/PPPP1PPP/R1B1K1NR b KQkq - 0 0",
		&[
			("g7g6", 97),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 85),
			("e6d5", 156),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 74),
			("e2e3", 61),
			("b2b3", 28),
			("g1f3", 73),
			("e2e4", 140),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 107),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 195),
		]
	),
	(
		"rnb1kbnr/pp2pppp/8/2pq4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 35),
			("b8c6", 43),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b7b5", 342),
			("h7h6", 254),
			("g7g5", 31),
			("g7g6", 36),
			("b7b6", 37),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 70),
			("c1d2", 199),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/2PP4/5N2/PP2PPPP/RbBQKB1R w KQkq - 0 0",
		&[
			("a1b1", 49),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 67),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 59),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 36),
			("b8c6", 88),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/6P1/5P2/PPPPP2P/RNBQKBNR b KQkq - 0 0",
		&[
			("d8h4", 130),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/3Pn2B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f2f3", 43),
			("e2e3", 34),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 75),
		]
	),
	(
		"r1bqk1nr/ppppbppp/2n5/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e5d4", 109),
			("d7d6", 48),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 160),
			("g1f3", 45),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 126),
		]
	),
	(
		"rnbqkb1r/ppp1nppp/4p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1d3", 53),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 69),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 106),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/1PB1P3/5N2/P1PP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c5b4", 3341),
			("c5b6", 297),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 41),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P1n3/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b6", 94),
			("b8c6", 29),
			("f8b4", 53),
			("d7d6", 42),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("a2a4", 38),
		]
	),
	(
		"rnbqkbnr/ppppp2p/6p1/5pB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 48),
			("b1c3", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("c4d5", 79),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d1a4", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/Q1p5/6P1/PP1PPPBP/RNB1K1NR b KQkq - 0 0",
		&[
			("c7c6", 28),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 48),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/3P4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 95),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d6", 60),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8b7", 50),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 41),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 49),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 26),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/1P2P3/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 52),
			("b8c6", 37),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/1P2P3/PB1P1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 36),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8c5", 121),
			("d7d5", 127),
			("b8c6", 52),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 125),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("e8g8", 35),
			("d7d6", 50),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 61),
			("e2e4", 99),
			("e2e3", 62),
			("a2a4", 58),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 38),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/5P2/4P3/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 36),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 29),
			("g8f6", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1c4", 66),
			("f2f4", 26),
			("d2d4", 68),
			("g1f3", 51),
			("g2g3", 44),
			("f1e2", 32),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c6", 27),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 71),
			("c7c5", 27),
			("g7g6", 30),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 37),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 200),
			("c2c3", 51),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 177),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("b5c6", 35),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4d5", 43),
			("b1c3", 74),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2pP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 25),
			("b7b5", 39),
		]
	),
	(
		"rnbqkbnr/ppppp1p1/7p/5pB1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5h4", 121),
		]
	),
	(
		"rnbqkbnr/ppppp1p1/7p/5p2/3P3B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g5", 101),
		]
	),
	(
		"rnbqkbnr/ppppp3/7p/5pp1/3P3B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("h4g3", 58),
		]
	),
	(
		"rnbqkbnr/ppppp3/7p/5pp1/3P4/6B1/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 42),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1e3", 97),
			("g1f3", 106),
			("f2f4", 115),
			("f1d3", 26),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 133),
			("g8f6", 36),
			("d5e4", 59),
			("e7e6", 46),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4e5", 115),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pP3/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5d4", 115),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 42),
			("d2d4", 74),
			("g2g3", 69),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 153),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 124),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 46),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c4d5", 130),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 27),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 51),
			("g2g3", 84),
			("e2e4", 35),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pP4/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6d5", 131),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/2N1P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8b4", 28),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/P7/1PPPPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 194),
			("d7d5", 171),
			("g8f6", 47),
			("b8c6", 63),
			("g7g6", 68),
			("c7c5", 33),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b2b4", 36),
			("h2h3", 32),
			("h2h4", 26),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/1P6/P7/2PPPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 36),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 27),
			("g1f3", 82),
			("f2f4", 69),
			("g1e2", 57),
			("c1e3", 45),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 26),
			("d7d6", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 51),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f7f5", 41),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 134),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 71),
			("b1c3", 29),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PPP3/3B4/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8b4", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 310),
			("e4d5", 67),
			("b1c3", 37),
			("d1e2", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 69),
			("e7e5", 111),
			("g8f6", 42),
			("g7g6", 60),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4p3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d3e4", 71),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 103),
			("e2e4", 164),
		]
	),
	(
		"rnbqkbnr/ppp3pp/3p4/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 59),
		]
	),
	(
		"rnbqkbnr/ppp3pp/3p4/4pp2/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 109),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 101),
			("d4c5", 55),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/3PP3/2PB4/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 33),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 49),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("e7e5", 26),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 88),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 37),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 56),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pP4/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 82),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 36),
			("b8d7", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 109),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/8/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 25),
			("g2g3", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2pP4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 89),
			("e7e5", 57),
			("g8f6", 159),
			("b7b5", 48),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/3p2p1/2pP4/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 80),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8c5", 63),
			("f8b4", 60),
		]
	),
	(
		"rnbqkbnr/1p2pppp/p2p4/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 47),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 44),
			("g7g6", 25),
		]
	),
	(
		"r1bqkbnr/pppn1ppp/3p4/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 41),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P1P2/8/PPP1P1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e3", 47),
			("g1f3", 25),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/2Pp4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1d4", 237),
			("g1f3", 190),
			("e2e3", 116),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/2PQ4/8/PP2PPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b8c6", 201),
			("g8f6", 29),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/2PQ4/8/PP2PPPP/RNB1KBNR w KQkq - 0 0",
		&[
			("d4d1", 146),
			("d4d2", 26),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 59),
			("g7g6", 36),
			("e7e5", 26),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 38),
		]
	),
	(
		"rnbqkbnr/2pppppp/p7/1p6/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a4", 63),
			("c2c4", 32),
			("g1f3", 67),
			("f1d3", 110),
			("b1c3", 33),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P3n1/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g4e5", 50),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/3p2p1/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 47),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/6P1/PP1BPP1P/RN1QKBNR b KQkq - 0 0",
		&[
			("d8e7", 65),
			("a7a5", 31),
			("b4e7", 37),
			("b4d2", 41),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f8g7", 127),
			("c6d4", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 80),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4P3/2Pp4/P7/1P2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 36),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 52),
			("f1c4", 69),
			("e4f5", 50),
			("g1f3", 26),
		]
	),
	(
		"rnbqkbnr/1pppppp1/8/p6p/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 39),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c4d5", 55),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pP4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f6d5", 59),
		]
	),
	(
		"rnbqkbnr/pppppp1p/8/6p1/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 26),
			("f4g5", 60),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 526),
			("c8f5", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 79),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8g7", 76),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1b5", 173),
			("d2d4", 244),
			("f1c4", 64),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a3a4", 26),
			("h2h3", 32),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/1p2P3/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 188),
			("b4a3", 71),
			("e7e6", 61),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/1p2P3/P7/2PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 121),
			("e4d5", 61),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3pP3/1p6/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 71),
			("c8f5", 25),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 335),
			("f8g7", 75),
			("c7c5", 37),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 256),
			("g2g3", 81),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 69),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4d5", 96),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 71),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 86),
			("f7f5", 51),
			("c7c6", 37),
			("g8f6", 173),
			("f8e7", 33),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 88),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c5d4", 37),
			("b8c6", 42),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 46),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2B1p3/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b7c6", 138),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1f4", 30),
			("g1f3", 52),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 49),
			("g1f3", 72),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3Pp3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 98),
			("e4f3", 46),
			("e7e5", 55),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/3Pp3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 30),
			("f3e4", 44),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 57),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e5", 45),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 60),
			("f8c5", 62),
			("g8f6", 39),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 53),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8c5", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 79),
			("c7c6", 28),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("d2d4", 28),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 49),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 36),
			("g1f3", 38),
			("f1e2", 29),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 32),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 55),
			("f1g2", 90),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/2PP2P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 35),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 191),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 26),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/5P2/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e1f2", 55),
			("e2e4", 31),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 88),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 78),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3Pp3/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 38),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3pP3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8f5", 42),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/6P1/8/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("h2h3", 55),
			("c2c4", 53),
			("g4g5", 56),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/6P1/7P/PPPPPPB1/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 30),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2ppP3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 114),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2ppP3/5P2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 70),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2ppP3/5P2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 43),
		]
	),
	(
		"rnbqkbnr/pp2ppp1/2p4p/3p4/3P3B/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 56),
		]
	),
	(
		"rnbqkbnr/pp2ppp1/2p4p/3p4/3P3B/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 53),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 46),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g7g6", 45),
			("e7e6", 73),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 32),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2BPP3/2N5/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 36),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 68),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("a7a6", 31),
			("e7e6", 136),
			("b8c6", 149),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 30),
			("d5e6", 102),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 55),
			("g7g6", 192),
			("e7e5", 44),
			("e7e6", 35),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 91),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g7g6", 28),
			("e7e6", 27),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 189),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 161),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3p4/2b1p3/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 39),
			("b1c3", 61),
			("c2c3", 40),
		]
	),
	(
		"rnbqkbnr/1ppppppp/8/p7/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 35),
			("e2e4", 84),
			("g1f3", 35),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/1PP5/5N2/P2PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 73),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/1PP5/5N2/P2PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 73),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/1PP5/5N2/PB1PPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e8g8", 67),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b8d7", 38),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/4P3/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 70),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 69),
		]
	),
	(
		"rn1qkbnr/pb1ppppp/1p6/2p5/3PP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 41),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/6B1/3PP3/2N5/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8g7", 156),
			("c7c6", 26),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 31),
			("e4e5", 29),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 52),
			("d7d5", 49),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 53),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 55),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/3Pp3/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6e7", 57),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 41),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 90),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 35),
			("b8c6", 33),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6e5", 434),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/8/4n3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 126),
			("f2f4", 238),
			("b1c3", 47),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/8/4n3/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5f3", 97),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e5d4", 84),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f1b5", 80),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/1B1pp3/8/1P2P3/PBPP1PPP/RN1QK1NR b KQkq - 0 0",
		&[
			("f8d6", 59),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 55),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/4p3/2N2P2/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("e4f3", 49),
			("g8f6", 69),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 43),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4p3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e4", 786),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4N3/5N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 389),
			("b8d7", 139),
			("c8f5", 109),
			("c8g4", 118),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/2PPp3/8/PP1NPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f5", 55),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/8/4n3/4PP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5g6", 145),
			("e5c6", 92),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("h2h4", 38),
			("b2b4", 28),
			("a4a5", 34),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 32),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 87),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g7g6", 43),
			("b8d7", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 43),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/4B3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c3", 42),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("c5d4", 144),
			("b8c6", 28),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/1P3NP1/P1PPPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 205),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/1P3NP1/P1PPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 205),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/1P3NP1/PBPPPP1P/RN1QKB1R b KQkq - 0 0",
		&[
			("e8g8", 147),
			("d7d6", 33),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e3", 34),
			("b1c3", 118),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1b5", 59),
			("b1c3", 212),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3pP3/2P5/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 68),
			("d1d4", 80),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5pB1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8b4", 31),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 53),
			("g7g6", 63),
			("e7e6", 47),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/2Pp4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e3d4", 50),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 38),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 189),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 160),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d8c7", 59),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 79),
			("g1f3", 30),
			("d2d4", 34),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 75),
		]
	),
	(
		"rnbqkb1r/pppp2pp/5n2/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 70),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 392),
			("g1f3", 67),
			("e2e4", 43),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7e6", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 37),
			("d5c4", 30),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/4P3/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2BPP3/2N5/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 56),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 97),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 114),
			("g2g3", 61),
			("g1f3", 40),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 42),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f1b5", 50),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/P3P3/2N5/1PPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 38),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2pP4/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 158),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8e6", 61),
			("g8f6", 27),
			("c7c6", 38),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/3pb3/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("c4b3", 26),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 242),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8e7", 36),
			("g8f6", 79),
			("g7g6", 81),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP1B2/5N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8b7", 42),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 85),
			("g1f3", 31),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 195),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/3P4/8/PPPNPPPP/R2QKBNR b KQkq - 0 0",
		&[
			("h7h6", 39),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 68),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 75),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f1d3", 27),
		]
	),
	(
		"rnbqkb1r/pppppp1p/8/6p1/3PnB2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f4c1", 33),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2B5/4pp2/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7c6", 120),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 55),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/1P2P3/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5b4", 114),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/1p2P3/5N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("a2a3", 85),
			("d2d4", 123),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/1p2P3/P4N2/2PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 25),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("b7b6", 42),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 35),
			("b1c3", 30),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n1pn2/8/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8b4", 47),
			("d7d5", 35),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 260),
			("f1d3", 54),
			("c2c4", 64),
			("f2f4", 92),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("e8g8", 87),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/5N1P/PPP2PP1/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 38),
			("b8c6", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 59),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 66),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8e6", 89),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 43),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8e7", 42),
			("c7c6", 31),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 203),
			("c2c4", 116),
			("g1f3", 103),
		]
	),
	(
		"rnbqkbnr/2pppppp/p7/1p6/P2PP3/8/1PP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 49),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 65),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2p5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1a3", 78),
			("e2e3", 157),
			("d1a4", 97),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("d7d6", 30),
			("d7d5", 124),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 179),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 116),
			("d2d3", 31),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 88),
			("g7g6", 32),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 92),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 79),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 366),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 43),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8g7", 126),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g2g3", 40),
			("e2e3", 40),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 35),
			("d2d4", 32),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2NP1N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8b4", 37),
			("d7d5", 50),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 133),
			("f2f4", 28),
			("g1f3", 44),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 67),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8d7", 34),
			("g7g6", 37),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c8f5", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 75),
			("b8c6", 83),
			("g8f6", 40),
			("e7e6", 35),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3p4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e3d4", 111),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 160),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4pP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e4", 160),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/4NP2/8/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 94),
			("b8d7", 34),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 59),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/4P1P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1e2", 103),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/8/4P1P1/PPPPNPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e8g8", 70),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("a7a6", 26),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e5d4", 28),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3Q4/2N5/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 36),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5c6", 42),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2P2n2/8/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 50),
		]
	),
	(
		"rnbqkb1r/ppp1p1pp/5n2/3p1pB1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("g5f6", 80),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("d5e4", 29),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 149),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/6B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 88),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/3P4/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c5d4", 142),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3p4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 268),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3Q4/2N5/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 225),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 25),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 281),
			("e7e6", 46),
			("g7g6", 49),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 546),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/2PN4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 206),
			("b8c6", 137),
			("e7e6", 84),
			("g7g6", 54),
			("b7b6", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 30),
			("e2e4", 38),
			("d2d4", 37),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 34),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 99),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/8/PPPPQPPP/RNB1KBNR w KQkq - 0 0",
		&[
			("g1f3", 59),
			("g2g3", 46),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3p4/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 69),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/8/3P2P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1d2", 27),
			("g1f3", 29),
			("b1c3", 30),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/8/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 27),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2PP4/5N2/PP1NPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f7f5", 47),
			("g8f6", 32),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3Q4/2N5/PPP1PPPP/R1B1KBNR w KQkq - 0 0",
		&[
			("d4d3", 69),
			("d4h4", 80),
			("d4a4", 52),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/8/2NQ4/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("g7g6", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5e6", 68),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4P3/8/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8e6", 73),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4b3/8/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 36),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3ppn2/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 199),
			("d2d3", 127),
			("e2e3", 84),
			("g1f3", 62),
			("b2b3", 27),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 55),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/8/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d4c3", 36),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 88),
			("g1e2", 82),
			("b2b3", 38),
			("d2d3", 71),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 58),
			("d7d5", 235),
			("g7g6", 48),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 84),
			("c2c4", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8c7", 36),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 56),
			("c1g5", 26),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 32),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/7P/8/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("h4h5", 92),
			("a2a4", 38),
			("g2g4", 28),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p2P/8/8/PPPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 55),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3e5", 145),
			("g2g3", 27),
			("c2c4", 53),
			("d2d4", 32),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4N3/8/8/PPPPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 71),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p3B1/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("c5d4", 78),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/6B1/3p4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("d1d4", 76),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/6B1/3Q4/2N5/PPP1PPPP/R3KBNR b KQkq - 0 0",
		&[
			("b8c6", 75),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 50),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 53),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 60),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 99),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1bB1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c6", 34),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/4p3/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 36),
			("d2d4", 53),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8e7", 35),
			("c7c5", 26),
		]
	),
	(
		"rnbqkbnr/1p2pppp/p7/2pP4/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 80),
		]
	),
	(
		"rnbqkbnr/1pp1pppp/p7/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 62),
		]
	),
	(
		"rnbqkbnr/1pp1pppp/p7/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b5", 31),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b8c6", 71),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5Q2/PPPP1PPP/RNB1KBNR w KQkq - 0 0",
		&[
			("f1c4", 49),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 117),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d6", 45),
			("e7e5", 33),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6d5", 38),
			("e6d5", 199),
		]
	),
	(
		"rnbqk1nr/ppppppb1/6pp/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c1e3", 85),
			("f2f4", 30),
			("g1e2", 27),
		]
	),
	(
		"rnbqk1nr/ppppppb1/6pp/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 57),
			("e7e6", 28),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/1p1PP3/5N2/P1P2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 64),
			("g8f6", 30),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/1P6/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 29),
			("c5c4", 31),
			("c5b4", 169),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 103),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 102),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2pPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b7b5", 55),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/4P3/1P6/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e4e5", 85),
		]
	),
	(
		"rnbqkbnr/p1pppppp/8/1p6/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 38),
			("c4b5", 137),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 52),
			("d8a5", 44),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 47),
		]
	),
	(
		"rnbqkb1r/p2ppppp/1p3n2/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("g2g3", 25),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d5c6", 76),
			("b1c3", 41),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2P2n2/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 111),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 34),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/6B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e8g8", 65),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8c7", 33),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 132),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p1P3/8/2P2N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 133),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 26),
			("c2c4", 46),
			("b2b3", 33),
			("f1d3", 28),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 312),
			("d7d5", 77),
			("e7e6", 434),
			("g8f6", 27),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 221),
			("d4e5", 44),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/3Pp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6e7", 210),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 101),
			("b1c3", 73),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("f6e4", 25),
			("f8e7", 100),
			("f8c5", 222),
			("f8b4", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d5", 92),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 83),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3n4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 34),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("e2e3", 37),
			("b1c3", 69),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2P5/4P1P1/PP1P1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e8g8", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b5", 66),
			("g8f6", 30),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/3P4/5P2/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 25),
			("d7d5", 100),
			("g7g6", 31),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 35),
			("f1g2", 55),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6d5", 410),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 363),
			("g2g3", 28),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("b8c6", 29),
			("d7d5", 42),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/1QN5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("c7c5", 66),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/8/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 38),
			("c1e3", 29),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 56),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 89),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1d2", 54),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8c7", 183),
			("g7g6", 59),
			("b8d7", 93),
			("g8f6", 46),
			("c8g4", 47),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 449),
			("e2e4", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 111),
			("c8g4", 56),
			("b8c6", 54),
			("g8f6", 54),
			("c7c5", 34),
			("c7c6", 95),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 27),
			("e2e4", 60),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 60),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 68),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 135),
			("c1f4", 47),
			("b1d2", 32),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/6B1/3P4/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d6", 27),
			("e8g8", 81),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8c7", 107),
			("g7g6", 36),
			("b8d7", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 31),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 69),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 39),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 45),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p2B1/3P4/5N2/PPPNPPPP/R2QKB1R b KQkq - 0 0",
		&[
			("f8e7", 95),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 86),
			("b8c6", 65),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/P1N5/1P2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b4c3", 316),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 98),
			("d8b6", 49),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c3", 54),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/3PP3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 35),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 151),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 110),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 37),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2pp1n2/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 109),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/2N1B3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8g7", 54),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/P3P3/2N5/1PPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 35),
			("d7d5", 35),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/4p3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f3g5", 289),
		]
	),
	(
		"rnbqkbnr/p1pppppp/8/1P6/8/8/PP1PPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("a7a6", 65),
			("c8b7", 29),
			("e7e5", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 90),
			("g7g6", 47),
			("g8f6", 86),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 33),
			("f1e2", 33),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/P1P5/8/1P2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b5c4", 26),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 58),
			("g8f6", 37),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 77),
			("g8f6", 62),
			("b8c6", 64),
			("c7c6", 37),
			("c7c5", 41),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1e2", 48),
			("b1d2", 28),
			("g1f3", 25),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 29),
		]
	),
	(
		"rnbqkbnr/1pp2ppp/p3p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c4d5", 43),
			("b1c3", 39),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P2P1/8/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f5g4", 58),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/8/3NP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 220),
			("e7e5", 63),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 26),
			("b8c6", 36),
			("d7d5", 37),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/4p3/3N4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4f3", 42),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/3P1N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 31),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/6B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 49),
			("b1d2", 76),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 48),
			("d5d4", 58),
			("g8f6", 34),
			("d5c4", 56),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c4d5", 67),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3Pp3/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c6d5", 67),
		]
	),
	(
		"r1bqkbnr/pppppppp/8/3Pn3/5P2/8/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5g6", 173),
		]
	),
	(
		"r1bqkbnr/pppppppp/6n1/3P4/5P2/8/PPP1P1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 103),
			("f4f5", 32),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3P4/4p3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6e5", 59),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5Bp1/8/8/1P6/P1PPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7f6", 133),
		]
	),
	(
		"rnbqkb1r/pppp1p1p/5pp1/8/8/1P6/P1PPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c4", 102),
		]
	),
	(
		"rnbqkb1r/pppp1p1p/5pp1/8/2P5/1P6/P2PPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 78),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/4P3/4n3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 33),
			("d2d3", 32),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("e4d5", 113),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3Pp3/8/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d5", 112),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2B5/4PN2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 32),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/6B1/3P4/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 41),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/2p2n2/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 50),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 26),
			("d1e2", 33),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pPp3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 176),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 70),
			("c2c4", 32),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4P3/8/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 78),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5e4", 49),
			("e5f4", 67),
			("d7d6", 41),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 78),
			("b8c6", 44),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 87),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 114),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8c5", 97),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f5", 42),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 198),
		]
	),
	(
		"r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N1P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("c7c6", 34),
			("e7e6", 33),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/4P3/p7/2PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1a3", 28),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/3p4/8/5Pb1/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 27),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 53),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 58),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 31),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp2B1/3P4/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("g5f6", 64),
		]
	),
	(
		"rnbqkbnr/ppp3pp/3p4/4pp2/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 66),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 78),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("b8c6", 42),
			("g8f6", 33),
		]
	),
	(
		"rnbqkbnr/ppp1p1pp/5p2/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5h4", 81),
			("g5f4", 41),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 139),
			("b8c6", 28),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 86),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 87),
			("e7e6", 30),
		]
	),
	(
		"rnb1kbnr/pp1ppppp/8/q1P5/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 39),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 74),
			("c7c5", 84),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/8/6N1/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c8g4", 53),
			("g7g6", 32),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 40),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 57),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 116),
			("c7c6", 64),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c8f5", 28),
			("c7c6", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N1P/PPPP1PP1/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 54),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/5N1P/PPPP1PP1/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 37),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 50),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3P4/8/8/8/PPP1PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8d6", 55),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3b4/8/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 32),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2Np4/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1d3", 82),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 41),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/6B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8g5", 43),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 40),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 78),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 59),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d6", 31),
			("c7c5", 40),
			("g8f6", 25),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/7Q/2N5/PPP1PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 36),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 171),
			("c4d5", 25),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3pP3/2P5/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5d4", 99),
			("f6e4", 31),
			("f6d7", 39),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 57),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 72),
			("g8f6", 35),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/P1pP4/5N2/1P2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 26),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p3B1/3P4/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c5d4", 47),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 54),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/6N1/2B1p3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 98),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5d8", 27),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 44),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/1B1P4/8/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c8d7", 136),
		]
	),
	(
		"rn1qkb1r/pppbpppp/5n2/1B1P4/8/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("b5e2", 51),
			("b5c4", 74),
		]
	),
	(
		"rn1qkb1r/pppbpppp/5n2/3P4/8/8/PPPPBPPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f6d5", 42),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 36),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/2PP2b1/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f3", 76),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 66),
			("e4e5", 64),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 64),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 33),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e8f7", 70),
		]
	),
	(
		"rnbq1bnr/pppppkpp/5p2/8/2PP4/8/PP2PPPP/RNBQKBNR w KQ - 0 0",
		&[
			("b1c3", 46),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("g8f6", 25),
			("a7a6", 38),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 28),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d1e2", 49),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8e7", 28),
			("d7d5", 51),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 36),
		]
	),
	(
		"rn1qkbnr/pp2pppp/3p4/2p5/2P1P1b1/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 42),
			("f1e2", 119),
		]
	),
	(
		"rnbqkbnr/pppp2pp/5p2/4P3/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 41),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 139),
			("d7d5", 148),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 70),
			("d2d4", 281),
			("f2f4", 74),
			("g2g3", 27),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 36),
			("c8g4", 31),
			("d8c7", 59),
			("g8f6", 30),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/2Pp4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 64),
			("b8c6", 99),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/4p3/2Pp4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 40),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/4P3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/pp2ppp1/2p4p/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5h4", 43),
		]
	),
	(
		"rnbqkbnr/pp2ppp1/2p4p/3p4/3P3B/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8b6", 43),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 30),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4P3/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g5", 26),
			("d7d6", 41),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d7", 49),
			("f6e4", 36),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2B5/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c6", 30),
			("g8f6", 60),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/2PP2b1/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 31),
			("f3e5", 50),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 69),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 39),
		]
	),
	(
		"rnbqkb1r/ppp1p1pp/5B2/3p1p2/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e7f6", 87),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/3P1NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 44),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 27),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2pPp3/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 202),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 171),
			("e2e4", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8e7", 69),
			("f7f5", 96),
			("g7g6", 27),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/1P6/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 97),
			("g1f3", 28),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/P7/8/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a1a3", 32),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/P6P/8/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 59),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pP2/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 28),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/1P3N2/P1PPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 115),
		]
	),
	(
		"rnbqkb1r/p1pppppp/5n2/1p6/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 76),
		]
	),
	(
		"rnbqkb1r/p1pppppp/5n2/1p6/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 72),
		]
	),
	(
		"rn1qkb1r/pbpppppp/5n2/1p6/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 53),
		]
	),
	(
		"rn1qkb1r/pbpppppp/5n2/1p6/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("e7e6", 38),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pP3/6b1/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 31),
			("d2d4", 77),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 84),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2pP4/4P3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 75),
			("d7d6", 26),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/8/2pP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 30),
			("c8g4", 62),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 30),
			("d5e4", 87),
			("g8f6", 25),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/8/PP1NPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b5c4", 44),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 63),
			("f1g2", 28),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/2PP4/6P1/PP2PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e5d4", 25),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/4pn2/1p6/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 80),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/4pn2/1p6/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 73),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 76),
			("d7d5", 43),
			("e7e6", 47),
			("b8c6", 308),
			("g7g6", 58),
			("d7d6", 29),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 54),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 48),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 26),
			("g1f3", 41),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 61),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 88),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 76),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5d4", 78),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p4/2P3b1/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 38),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 71),
			("c5d4", 49),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 102),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4e5", 28),
			("d4d5", 65),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 73),
		]
	),
	(
		"rnb1kbnr/ppp2ppp/8/3qp3/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 33),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2P5/8/4B3/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 27),
			("b8a6", 28),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/6B1/3P4/5N2/PPPNPPPP/R2QKB1R b KQkq - 0 0",
		&[
			("d7d5", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/1PPp4/4PN2/P2P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d4e3", 27),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p4/3P2b1/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 43),
			("c2c4", 28),
		]
	),
	(
		"rnbqkb1r/pppppppp/7n/8/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 97),
		]
	),
	(
		"rnbqkb1r/pppppppp/7n/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g5", 25),
			("b7b5", 42),
			("h6g8", 26),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/3P4/6PN/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 102),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2P5/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 49),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 124),
			("d2d4", 181),
			("f1c4", 26),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 41),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c3", 87),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/2PB4/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 30),
			("d6d5", 29),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2PP1N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 27),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 80),
			("g8f6", 149),
			("f8d6", 98),
			("f7f5", 27),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 65),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b8c6", 34),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 45),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/5bN1/4p3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 30),
			("g2g4", 28),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 25),
		]
	),
	(
		"r1bqkbnr/pp1npppp/2pp4/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 78),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/5n2/2pPp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq  - 0 0",
		&[
			("b1c3", 410),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/5n2/2pPp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 395),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 58),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2P2N2/PP1PBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 62),
			("b8c6", 57),
			("b8d7", 32),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1e3", 28),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/8/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 27),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 175),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 79),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 90),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/8/1bPP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 34),
			("e8g8", 27),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 196),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("b8c6", 146),
			("e7e6", 30),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 67),
			("d7d6", 131),
			("e7e5", 29),
			("c7c5", 65),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 46),
			("e7e6", 25),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2P5/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 36),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 136),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/6P1/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 143),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 54),
			("b8c6", 72),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 93),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 34),
			("f1d3", 56),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N2NP1/PPPP1P1P/R1BQKB1R b KQkq - 0 0",
		&[
			("b7b5", 80),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 96),
			("b8c6", 88),
			("e7e6", 46),
			("d7d6", 35),
			("g7g6", 27),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4P3/6P1/PPPP1P1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 42),
			("e4d5", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP1BPPP/R1BQK1NR b KQkq - 0 0",
		&[
			("f8g7", 92),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3p4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 74),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4P3/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 123),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 39),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pP4/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 35),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P1P3/2NP4/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8e7", 30),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e5d4", 498),
			("f8b4", 64),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/3pb3/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("c4e6", 95),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/3pB3/4p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f7e6", 95),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 60),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2ppP3/8/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 118),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2ppP3/8/2P5/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 85),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2ppP3/3P4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8f5", 31),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pP3/4P3/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6e4", 218),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/4P3/2PP4/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1c2", 30),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 171),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 29),
			("e4e5", 116),
			("e4d5", 32),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/1P6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 31),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6e4", 137),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/4n3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 48),
			("b1c3", 72),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2pP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 78),
			("c7c5", 54),
			("e7e5", 182),
			("a7a6", 49),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 32),
			("g1f3", 29),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 127),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 126),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2pP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b7b5", 46),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/1Bp1P3/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f6d5", 40),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3P4/1p6/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 45),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/4P3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 57),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/4P3/3P4/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d6", 34),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("a2a4", 65),
			("c2c4", 29),
			("c1g5", 28),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/P7/5N2/1PP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b5b4", 36),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 195),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 153),
			("e8g8", 36),
		]
	),
	(
		"rnbqkbnr/1pppppp1/p6p/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 66),
			("c2c4", 40),
			("f2f4", 39),
			("f1d3", 52),
			("g1f3", 48),
		]
	),
	(
		"rnbqkbnr/1pppppp1/p6p/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 52),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3pN3/3P2b1/8/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f5", 120),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/4BP2/PPP3PP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 36),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/4p3/1b6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b2e5", 97),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/4B3/1b6/8/P1PPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 93),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4B3/1b6/8/P1PPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 84),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4e5", 78),
			("c4d5", 32),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pP3/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8b4", 72),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/2N1P3/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 41),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1bPP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 35),
			("d2d4", 95),
			("g2g3", 37),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d8c7", 36),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/4P3/2PP4/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 51),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("b7b5", 165),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1pp5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 156),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1pp5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8b7", 151),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8b7", 48),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 109),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e5d4", 94),
			("e5e4", 25),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3p4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c3d4", 85),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3P4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 55),
			("g8f6", 28),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p3B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 58),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p3B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8b6", 27),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2P5/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8a5", 34),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 60),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 41),
			("c2c4", 41),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 28),
		]
	),
	(
		"rnbqk2r/pppp1ppp/4pn2/6B1/1bPP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("h7h6", 149),
			("c7c5", 114),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/1P6/8/PBPPPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c4", 26),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3Q4/5N2/PPP1PPPP/RNB1KB1R w KQkq - 0 0",
		&[
			("d4d1", 34),
			("d4h4", 38),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 58),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 31),
		]
	),
	(
		"rn1qkbnr/pbp1pppp/1p1p4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 40),
			("f1d3", 30),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/4p3/1bP5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3d5", 88),
			("d1c2", 35),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/3Np3/1bP5/8/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b4c5", 31),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/4p3/3P4/3P4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 169),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/2PP2b1/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 48),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 70),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p3B1/3P4/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("h7h6", 32),
			("d8b6", 37),
			("f8e7", 35),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4pp2/3p2B1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5h4", 45),
			("g5f4", 29),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/6B1/2PP4/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("d7d6", 26),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 161),
			("d7d5", 47),
			("a7a6", 25),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 155),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/5NP1/PPPP1PBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 82),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 34),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 100),
			("d7d6", 35),
		]
	),
	(
		"rnbqk1nr/1ppp1ppp/4p3/p7/1bPP4/8/PP1BPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 27),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4p3/1b2P3/2NP1N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 50),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/4PN2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 37),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/4N3/8/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4c3", 39),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/3QP3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b8c6", 285),
			("g8f6", 61),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3QP3/8/PPP2PPP/RNB1KBNR w KQkq - 0 0",
		&[
			("d4a4", 45),
			("d4d1", 78),
			("d4e3", 35),
			("d4d3", 93),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 222),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e4d5", 67),
			("d2d4", 40),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e6d5", 66),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 77),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("e4d5", 73),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/2B5/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c6d5", 71),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/2B5/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("c4b3", 53),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/8/1B6/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 28),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 165),
			("b1c3", 29),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d5", 123),
			("d7d6", 42),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("c4b3", 117),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 49),
			("d5d4", 34),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 33),
			("f6d5", 47),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/5N2/PPPNPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 36),
			("d7d5", 26),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 25),
			("e2e4", 101),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2P5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 51),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/4PN2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 53),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e8g8", 54),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("c2c4", 25),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 53),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("g8f6", 913),
			("e7e5", 68),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 90),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5d4", 70),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/3pP3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e2", 72),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 77),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 62),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/2P5/8/PP2PPPP/RNBQKBNR w KQkq  - 0 0",
		&[
			("b1c3", 40),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2pPp3/2P1P3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f7f5", 28),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 33),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c5d4", 40),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 39),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/5P2/8/PPPPP1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 98),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 28),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3PP3/3B4/PPP1QPPP/RNB1K1NR b KQkq - 0 0",
		&[
			("c7c5", 38),
			("g8f6", 31),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3bp3/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 45),
			("f4g3", 29),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/4Pp2/8/PPPPB1PP/RNBQK1NR b KQkq - 0 0",
		&[
			("d8h4", 92),
		]
	),
	(
		"rnb1kbnr/pppp1ppp/8/8/4Pp1q/8/PPPPB1PP/RNBQK1NR w KQkq - 0 0",
		&[
			("e1f1", 92),
		]
	),
	(
		"rnb1kbnr/pppp1ppp/8/8/4Pp1q/8/PPPPB1PP/RNBQ1KNR b kq - 0 0",
		&[
			("f7f5", 69),
		]
	),
	(
		"rnbqkbnr/p1pp1ppp/1p2p3/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 52),
		]
	),
	(
		"rn1qkbnr/pbpp1ppp/1p2p3/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f7f5", 34),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/1Bp5/3PP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("a7a6", 176),
			("c5d4", 1441),
		]
	),
	(
		"rn1qkb1r/pppbpppp/5n2/3P4/2B5/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7g4", 29),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4N3/1b2P3/2N5/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e8g8", 62),
		]
	),
	(
		"rnbqkbnr/1pp2ppp/p3p3/3p4/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f7f5", 45),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 47),
			("d2d4", 40),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 76),
			("d4e5", 52),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8d7", 38),
			("e5e4", 34),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3pPP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e2", 65),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3pPP2/8/PPPPN1PP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c5", 47),
		]
	),
	(
		"rnbqkbnr/ppp1p1pp/5p2/3p4/3P3B/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8h6", 29),
			("c7c5", 25),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 45),
			("b1c3", 107),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 144),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 119),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 50),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 134),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 151),
		]
	),
	(
		"rnbqkbnr/p1pppp1p/1p4p1/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 31),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 34),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/3p1n2/8/3PP1b1/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 39),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 98),
			("c2c3", 31),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8g4", 76),
			("d8b6", 44),
			("c8f5", 54),
			("e7e6", 42),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/3p1n2/8/3PP1b1/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g4d7", 25),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P1P2/8/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 33),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 38),
			("g1f3", 28),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1c4", 36),
			("g1f3", 37),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 114),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c4", 37),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4p3/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 98),
			("c2c4", 29),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4p3/3p1b2/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f5d3", 60),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 484),
			("g8f6", 26),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 282),
			("c4d5", 191),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 93),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 263),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 50),
			("d7d5", 31),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c5", 46),
			("e7e6", 68),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 47),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/2B1P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 30),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 29),
			("b1c3", 28),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 51),
			("c7c6", 35),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4p3/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 47),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 58),
			("g1f3", 34),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e8g8", 48),
		]
	),
	(
		"rnbqkbnr/ppppppp1/7p/8/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g5", 50),
			("a7a6", 34),
			("b7b5", 28),
		]
	),
	(
		"rnbqkbnr/pppppp2/7p/6p1/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 29),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f7f6", 76),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/3P4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6e5", 65),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4f5", 65),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5P2/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 32),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 72),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 33),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 119),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("d8d7", 55),
			("g8f6", 29),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 82),
			("d2d4", 123),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 118),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 133),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 65),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 26),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/4PP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7g6", 26),
			("e5f4", 53),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 79),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 45),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 29),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c5d4", 141),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/2Pp4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 141),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/2PQ4/2N5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("g7g6", 27),
			("b8c6", 105),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4PP2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1b5", 45),
			("e4d5", 40),
			("g1f3", 163),
			("e4e5", 47),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 34),
			("c5d4", 28),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 75),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3p4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 75),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3N4/6P1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 52),
			("e7e5", 27),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p4/3P1Pb1/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 42),
			("f1e2", 26),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 27),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/8/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 54),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/8/1P4P1/PBPPPP1P/RN1QKBNR b KQkq - 0 0",
		&[
			("d7d5", 37),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3P4/8/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 320),
			("d8d5", 100),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("c8f5", 43),
		]
	),
	(
		"rnbqkb1r/pppnpppp/8/3pP3/8/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 25),
			("d2d4", 41),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b8c6", 33),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4Pn2/8/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8e6", 101),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/4P3/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 80),
			("b2b3", 35),
			("g1f3", 43),
			("d2d3", 27),
			("g1e2", 61),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pP2/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 43),
		]
	),
	(
		"rnbqkb1r/p2ppppp/5n2/1ppP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b5b4", 34),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 76),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3pP3/5P2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 92),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/5P2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 84),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/5P2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 75),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a3", 137),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4p3/3NP3/8/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6d5", 70),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 47),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/1P6/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/1P6/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 41),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 30),
			("e2e4", 30),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/1P3NP1/P2PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8e7", 46),
		]
	),
	(
		"rnbqk2r/ppppbppp/5n2/4p3/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 37),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/q7/2B5/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 132),
			("c7c6", 57),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 59),
			("e2e4", 36),
			("e2e3", 184),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/2N3P1/PP2PP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 32),
			("e8g8", 58),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 34),
			("g8f6", 25),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2B1Pp2/8/PPPP2PP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 131),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2B1Pp2/2N5/PPPP2PP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c6", 75),
			("f8b4", 27),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3pP3/4n3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 80),
			("c3e2", 51),
			("c3e4", 72),
			("f2f4", 31),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/2NP4/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 27),
			("d5d4", 41),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/4P3/2Pp4/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1d3", 156),
			("g1f3", 26),
			("c3c4", 26),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/4P3/2PB4/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 66),
			("d7d6", 40),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 48),
			("d2d4", 26),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 34),
			("e2e4", 69),
			("g1f3", 89),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 188),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 115),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4e5", 72),
			("g1f3", 27),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 44),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/3P4/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 88),
			("b1d2", 60),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 64),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 73),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/4P3/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 32),
			("e2g3", 61),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 97),
		]
	),
	(
		"r1bqkbnr/pppnpppp/3p4/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 26),
		]
	),
	(
		"rnbqk1nr/ppppppb1/6pp/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 69),
		]
	),
	(
		"rnbqk1nr/ppppppb1/6pp/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 53),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/4p3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 31),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 58),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/4P1P1/PPPPNPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8g4", 32),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 25),
			("b8c6", 28),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 45),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/3P1N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 39),
			("b8c6", 29),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 79),
		]
	),
	(
		"rnbqk1nr/pppp1pbp/6p1/4p3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 36),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 117),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 89),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 69),
			("d8b6", 30),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3P4/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c6d5", 103),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/5P2/1P2PN2/P1PP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 29),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 263),
			("b1c3", 118),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/2B1P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 229),
			("d7d6", 41),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2Pp4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d5d4", 48),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3pP3/3P4/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d7", 68),
			("f6e4", 35),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 65),
			("c7c6", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3pN3/8/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 88),
			("g8f6", 72),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 41),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 51),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P5/2N2NP1/PP1PPP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("e8g8", 110),
			("d7d6", 31),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/6PP/PPPPPPB1/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 33),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3ppn2/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8e7", 37),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 38),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 130),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 49),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 36),
			("d7d5", 60),
			("g7g6", 74),
			("e7e6", 52),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3pP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f7f6", 72),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 50),
		]
	),
	(
		"r1bqkb1r/pppnpppp/3p1n2/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 56),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p1b2/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 115),
		]
	),
	(
		"rn1qkbnr/ppp2ppp/4p3/3p1b2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 46),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 45),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/3p4/3PN3/8/8/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d6e5", 79),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 79),
			("d5d4", 28),
			("d5e4", 45),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5p2/2pP4/8/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 73),
			("d7d6", 28),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2N2NP1/PPPP1P1P/R1BQKB1R b KQkq - 0 0",
		&[
			("f8c5", 60),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2N1P3/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 30),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/3PP3/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d5e4", 137),
			("g8f6", 43),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3Pp3/4B3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("b1d2", 78),
			("f2f3", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3Pp3/4B3/PPPN1PPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 62),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/P7/1PPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 27),
			("e7e6", 81),
			("g7g6", 32),
			("b8c6", 53),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2BpP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f6e4", 45),
			("b8c6", 56),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 29),
		]
	),
	(
		"r1bqkbnr/ppp1p1pp/2n2p2/3pP3/3P4/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1b5", 30),
			("f2f4", 51),
			("g1f3", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p2B1/3PN3/8/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("d5e4", 66),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 142),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/8/5NP1/PPPPPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 150),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("d8c7", 128),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/2PP4/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b7b5", 39),
			("h7h6", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c4d5", 56),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pP4/8/5N2/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 38),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/2P1P3/2N3P1/PP1P1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 83),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/2N1P3/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1h5", 88),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp2Q/8/2N1P3/PPPP1PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 48),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/2N1P3/PPP2PPP/R2QKBNR b KQkq - 0 0",
		&[
			("e7e6", 60),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 47),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/3QP3/4n3/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("e4c5", 42),
		]
	),
	(
		"rnbqkb1r/pppppppp/1n6/2P1P3/8/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b6d5", 259),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("c4d5", 54),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pP4/8/2N3P1/PP1PPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f6d5", 54),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/2NP1N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 32),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b7b5", 36),
		]
	),
	(
		"rnbqkbnr/2pppppp/p7/1p6/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 26),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/3P4/2P5/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 181),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 193),
			("c1f4", 36),
			("b1d2", 28),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3pNb2/3P4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 62),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3pNb2/2PP4/8/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f5b1", 43),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 29),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("b7b6", 41),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/8/PPPPQPPP/RNB1KBNR w KQkq - 0 0",
		&[
			("e4d5", 181),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/8/8/PPPPQPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("d8d5", 143),
			("g8f6", 34),
		]
	),
	(
		"rnb1kbnr/ppp2ppp/4p3/3q4/8/8/PPPPQPPP/RNB1KBNR w KQkq - 0 0",
		&[
			("b1c3", 111),
			("g1f3", 29),
		]
	),
	(
		"rnb1kbnr/ppp2ppp/4p3/3q4/8/2N5/PPPPQPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("d5d8", 81),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3B4/4Pp2/8/PPPP2PP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 76),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 55),
			("b1c3", 41),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 72),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g7g6", 25),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 82),
			("c1g5", 34),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 37),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 78),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 37),
			("d7d5", 36),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/8/PPPPBPPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 42),
		]
	),
	(
		"rnbqkbnr/ppp3pp/4p3/3p1p2/2PP1B2/2N5/PP2PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4PP2/3P4/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 25),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 72),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/4P3/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 42),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 61),
		]
	),
	(
		"rnbqkbnr/ppppp2p/6p1/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 52),
		]
	),
	(
		"rnbqkbnr/ppppp2p/6p1/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 40),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 79),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4d5", 42),
			("g1f3", 27),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 86),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 75),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 84),
			("d5c4", 66),
			("c8f5", 72),
			("g7g6", 28),
			("c8g4", 39),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 105),
		]
	),
	(
		"rnbqkb1r/ppppp2p/5np1/5p2/2PP4/6P1/PP2PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 80),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/4P3/1B3N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 98),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5e4", 145),
		]
	),
	(
		"rnbqkb1r/pppppppp/1n6/4P3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 44),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/8/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8b7", 49),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("h7h6", 25),
			("b7b5", 50),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/8/6P1/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 181),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/8/6P1/PPPPPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 60),
			("d2d3", 67),
		]
	),
	(
		"rnbqk1nr/ppppbppp/4p3/8/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c3", 38),
			("g1e2", 25),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2p5/4PN2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 35),
			("g8f6", 46),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/4p3/8/2PP4/8/PP1bPPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("d1d2", 74),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 59),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/4P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 246),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/4P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 232),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/1B1p4/3P4/8/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 29),
		]
	),
	(
		"rnbqk1nr/pppp1pbp/4p1p1/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 27),
			("g1f3", 64),
			("c1e3", 28),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/2P3b1/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c4d5", 26),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/4p3/1bP5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3d5", 54),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 91),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/P1N2N2/1P1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("a7a5", 27),
			("d7d5", 52),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/P1pP4/2N5/1P2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 34),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/4p3/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1c4", 41),
			("c2c3", 66),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p2Q/4P3/8/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("b8c6", 45),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3pP3/3Pn3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e4c3", 68),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 47),
		]
	),
	(
		"rnbqkb1r/pppppp1p/6p1/6B1/3Pn3/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5f4", 55),
		]
	),
	(
		"r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("a7a6", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 38),
			("e2e3", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/4p3/2PP4/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d3e4", 91),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/4P3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d1", 88),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8f5", 34),
			("c8g4", 40),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 43),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 215),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 50),
			("d2d4", 159),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 44),
		]
	),
	(
		"rnbqkbnr/pppppp1p/8/6p1/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 116),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4P3/5p2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6h5", 50),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e3", 26),
			("d2d4", 186),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 91),
			("b8c6", 26),
			("g8f6", 31),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3N4/8/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4f3", 27),
			("g2g3", 25),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 78),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 29),
			("g8f6", 78),
			("c7c6", 39),
			("c7c5", 32),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 76),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/1P2P3/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8d6", 36),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2P1PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 41),
			("b8c6", 41),
		]
	),
	(
		"r1bqkbnr/pp1npppp/2pp4/8/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f4", 84),
			("g1f3", 42),
			("c1e3", 33),
		]
	),
	(
		"r1bqkbnr/pp1npppp/2pp4/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8c7", 60),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 26),
			("e7e6", 28),
			("c7c6", 80),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/5P2/1P2PN2/P1PP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8e7", 29),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/2B1P3/5Q2/PPPP1PPP/RNB1K1NR b KQkq - 0 0",
		&[
			("g8f6", 50),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p1P3/2P5/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f6g8", 59),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 71),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g7g6", 30),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/4Pp2/2N5/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 170),
			("d2d4", 30),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/4Pp2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g5", 96),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/P7/1PPPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b2b4", 26),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p2B1/3P4/2N2N2/PPP1PPPP/R2QKB1R b KQkq - 0 0",
		&[
			("f8g7", 50),
		]
	),
	(
		"rnbqk1nr/pppp1pbp/4p1p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8e7", 65),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 56),
			("e2e3", 26),
			("c1g5", 41),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 33),
			("c7c5", 62),
			("b7b6", 27),
		]
	),
	(
		"rnbqkbnr/ppp1p1pp/3p4/5p2/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2ppP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 42),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/8/3Pp3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 73),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2pP4/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b7b5", 25),
			("d7d6", 86),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2pP4/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 32),
			("g7g6", 59),
		]
	),
	(
		"rnb1kb1r/ppp1pppp/5n2/3q4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 37),
			("b1c3", 27),
			("d2d4", 26),
		]
	),
	(
		"rnbqkbnr/pppp1p1p/8/6p1/2B1Pp2/5N2/PPPP2PP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 46),
		]
	),
	(
		"rnbqkb1r/p1pppppp/1p3n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 62),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 69),
		]
	),
	(
		"rn1qkb1r/pbpppppp/1p3n2/8/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 30),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/2PP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 26),
			("g1f3", 38),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("c6d4", 40),
			("g8e7", 119),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/3P4/6P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 87),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 75),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/2b1p3/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("e8g8", 43),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/4n3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e4f6", 40),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/2p2np1/8/2PP4/6P1/PP2PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 44),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 97),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 48),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e5d4", 63),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("c2c3", 32),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2np4/3P4/8/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6e5", 96),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/3p4/3Pn3/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3e5", 69),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/1P3N2/P1PPPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/8/1P3N2/PBPPPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("g8f6", 60),
		]
	),
	(
		"rnb1kbnr/pppp2pp/5q2/4Np2/4P3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 63),
		]
	),
	(
		"rnb1kbnr/pppp2pp/5q2/4Np2/3PP3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 63),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 78),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c5d4", 29),
			("b8c6", 52),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/4n3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 54),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pP2/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5e4", 27),
			("g8f6", 44),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/4P2P/8/PPPP1PP1/RNBQKBNR b KQkq - 0 0",
		&[
			("h7h5", 37),
			("f8g7", 43),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR w KQkq - 0 0",
		&[
			("e2e3", 40),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3p1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e3d4", 71),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3P1B2/8/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("d8b6", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 114),
			("c4d5", 70),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/5NP1/PPPPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e5e4", 26),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/8/8/2N5/PP1PPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 69),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4e5", 39),
			("f1b5", 232),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/3p1n2/5b2/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f2f3", 54),
		]
	),
	(
		"rnbqkbnr/pppppp1p/8/6P1/8/8/PPPPP1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("h7h6", 43),
		]
	),
	(
		"rnbqkbnr/p2ppppp/8/1ppP4/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 110),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/2N1P3/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 66),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2P5/2N1P3/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 49),
		]
	),
	(
		"r1bqkbnr/pppppppp/6n1/3P4/4PP2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 47),
			("e7e5", 54),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/Q1p5/5N2/PP1PPPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("c7c6", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/Q1p5/5N2/PP1PPPPP/RNB1KB1R w KQkq - 0 0",
		&[
			("a4c4", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2Q5/5N2/PP1PPPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/3p1n2/5b2/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 49),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/8/2N5/PPPPBPPP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 26),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/8/8/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e5", 53),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/5P2/1P2P3/PBPP2PP/RN1QKBNR b KQkq - 0 0",
		&[
			("c7c5", 28),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/6P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 86),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/2Pp4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d4e3", 64),
			("b8c6", 35),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/2P5/4p3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1e3", 63),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/8/2P5/4B3/PP3PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 30),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/3BP3/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c3", 58),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2PBP3/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("b8c6", 49),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/P7/1PPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b2b4", 70),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/1P2P3/P7/2PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5b4", 64),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/1p2P3/P7/2PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a3b4", 107),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/8/1P2P3/8/2PP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8b4", 97),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/3Pp3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 28),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/4P1P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1e2", 79),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/4Pp2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 103),
			("f1c4", 50),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/3PPp2/5N2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g5", 81),
		]
	),
	(
		"r1bqkbnr/ppp1p1pp/2n2p2/3pP3/3P1P2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8h6", 39),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/2P1P3/2N3P1/PP1P1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 78),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3pp3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 30),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NPP/PPPPPPB1/RNBQK2R b KQkq - 0 0",
		&[
			("c8f5", 65),
		]
	),
	(
		"rnbqk1nr/ppppbppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 31),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 44),
		]
	),
	(
		"rnb1kbnr/ppp2ppp/8/3qp3/8/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 54),
		]
	),
	(
		"rnb1kbnr/ppp2ppp/8/3qp3/8/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 32),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pB1p/8/3P4/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8f6", 83),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 54),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 59),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 43),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/4P3/6P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d6", 66),
			("f8g7", 66),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/4P3/6P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 63),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/q7/1P6/2N5/P1PP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("a5b4", 48),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP2P/2N5/PPP2PP1/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 27),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/6P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1e2", 37),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 59),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P4/2P5/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 83),
			("c1g5", 39),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n2n2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 95),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4e5", 91),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 26),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 38),
			("g8f6", 41),
			("a7a6", 30),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/4p3/3p4/1b1PP1Q1/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("g8f6", 32),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3PP3/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8c7", 42),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/3P2P1/PPP1PP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 29),
			("d7d5", 33),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 32),
			("f8g7", 73),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 63),
		]
	),
	(
		"rnbqkbnr/1pp2ppp/p3p3/3p4/3PP3/8/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 65),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/3BP3/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("f2f4", 61),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/4P3/2N2N2/PPPPBPPP/R1BQK2R b KQkq - 0 0",
		&[
			("b8c6", 26),
			("b7b5", 35),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 79),
			("e2e3", 35),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3pp3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8e7", 114),
			("g8f6", 37),
			("b8d7", 27),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/3pp3/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 39),
			("f1d3", 57),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/5P2/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 94),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 51),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 48),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e8f7", 50),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2B2n2/4p3/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7c6", 63),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/3p1n2/4P3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d6e5", 52),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 54),
			("d2d3", 31),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("d7d5", 29),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 40),
			("c7c5", 25),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/2N2P2/PP2P1PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 72),
			("c7c5", 57),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/8/8/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 68),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/8/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 56),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/4p3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d3e4", 64),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/4PP2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 27),
			("d8c7", 33),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4P3/8/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 48),
			("e5d6", 34),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4P3/4P3/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d6e5", 49),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 53),
		]
	),
	(
		"rnbqkbnr/1p1ppppp/p7/2p5/P3P3/2N5/1PPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 28),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/3P1B2/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("g8f6", 45),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/1P6/P1PPPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 69),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/1P6/PBPPPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7e5", 43),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/8/2N5/PPPPPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 41),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2p1p3/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f1c4", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e5d4", 129),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/2Pp4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 143),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/2PQ4/2N5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 121),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/4p3/8/2PP4/8/PP1QPPPP/RN2KBNR b KQkq - 0 0",
		&[
			("g8f6", 67),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/3PP3/2N4P/PPP2PP1/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 27),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/6P1/PPPP1P1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 87),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/6P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 50),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/3P2Q1/8/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("c5d4", 49),
			("b8c6", 47),
		]
	),
	(
		"rnbqk2r/ppppbppp/4pn2/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 141),
			("e8g8", 48),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2P1P3/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 131),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/2P1P3/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 43),
			("d2d4", 51),
			("g1f3", 34),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 67),
			("f8g7", 34),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 176),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p4/6b1/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 25),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("d5e4", 28),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/4pn2/2p5/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b7b6", 40),
			("d7d5", 47),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2Pp4/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d3", 32),
		]
	),
	(
		"r1bqkbnr/pppp1p1p/2n3p1/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e5d4", 42),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/4PN2/PPPPB1PP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 44),
			("g8h6", 28),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 85),
			("g2g3", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/4P3/3P1N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8d6", 73),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 34),
			("g2g3", 42),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 33),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 83),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 43),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/5Pb1/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 25),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/8/5p2/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 49),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a3", 29),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/2PPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 49),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c6", 39),
			("g8f6", 61),
			("g7g6", 38),
			("e7e5", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/3P4/5P2/PPP1P1PP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 32),
			("g8f6", 43),
		]
	),
	(
		"r1bqkbnr/pppn1ppp/3p4/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c7c6", 32),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/4P3/7P/PPPP1PP1/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 105),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 35),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/6p1/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 48),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 31),
			("f1e2", 29),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/2B1P3/8/PPPPQPPP/RNB1K1NR b KQkq - 0 0",
		&[
			("a7a6", 33),
			("b8c6", 41),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 202),
			("g1f3", 51),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 161),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2PP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 39),
			("g1f3", 28),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 116),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pP4/4P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 59),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2BP4/4P3/PP3PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 48),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/4P3/6P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d2d4", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 101),
			("d2d4", 63),
			("b2b3", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/2p5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 80),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 35),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 49),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/1P2P3/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6b4", 34),
			("c5b4", 53),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/3p4/2P5/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8d5", 60),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pP3/3P2b1/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 61),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2N3P1/PPPP1P1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 85),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n5/2p1p3/4P3/2N3P1/PPPP1PBP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 67),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p4/3P2b1/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 42),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3pN3/5Pb1/8/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g4f5", 48),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/5P2/4PN2/PPPPB1PP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 97),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 56),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/2P5/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8b7", 70),
		]
	),
	(
		"r1bqkb1r/pppnpppp/5n2/3p2B1/3P4/2N2P2/PPP1P1PP/R2QKBNR b KQkq - 0 0",
		&[
			("c7c5", 27),
		]
	),
	(
		"rn1qkbnr/pbp1pppp/1p1p4/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8d7", 31),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5pB1/3P4/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("g5f6", 25),
		]
	),
	(
		"rnbqkbnr/ppp3pp/3p4/4pp2/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 45),
		]
	),
	(
		"rnbqkbnr/1pp2ppp/p3p3/3p4/3PP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("f1d3", 33),
			("g1f3", 34),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 54),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 28),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e4d5", 41),
		]
	),
	(
		"r1b1kbnr/ppqppppp/2n5/1Bp5/4P3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/2p5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("d1a4", 34),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7g6", 102),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/3BPN2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f5d3", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2pP4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 89),
			("e2e3", 71),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3Pp3/2p5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 71),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 64),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pP4/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e6d5", 61),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 57),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/1P3N2/P1PP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 73),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/4P3/1P3N2/PBPP1PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d6", 30),
		]
	),
	(
		"r1bqkbnr/pp1npppp/2pp4/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 76),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/8/3P2P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("g7g6", 33),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("e7e6", 44),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 36),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("e8g8", 42),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("g8f6", 38),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/8/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 48),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 65),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/4PN2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/8/2B1Pp2/5N2/PPPP2PP/RNBQK2R b KQkq - 0 0",
		&[
			("h7h6", 25),
		]
	),
	(
		"rnbqk2r/pppp1ppp/5n2/4p3/1bB1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("c3d5", 26),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2p5/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 55),
			("f8g7", 28),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/5N2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 37),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 54),
			("c5d4", 54),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/3P4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 45),
		]
	),
	(
		"rnbqkbnr/ppppp1pp/5p2/8/4P3/2N5/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e8f7", 36),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/4P3/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 35),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/2B1P3/8/PPPP1PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("d7d5", 52),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("e4d5", 48),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 53),
			("e2e4", 222),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c5d4", 50),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/3p4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("f3d4", 50),
		]
	),
	(
		"rnbqkbnr/1p1p1ppp/p3p3/2p5/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 69),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/1P4P1/PBPPPP1P/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 55),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/1P4P1/PBPPPP1P/RN1QKBNR w KQkq - 0 0",
		&[
			("f1g2", 53),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/8/1P4P1/PBPPPPBP/RN1QK1NR b KQkq - 0 0",
		&[
			("e7e5", 55),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/4P3/2P5/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 46),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2P5/2N3P1/PP1PPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("f1g2", 82),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3p4/5NP1/PPP1PP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3N4/6P1/PPP1PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 27),
		]
	),
	(
		"rn1qkbnr/pbp1pppp/1p1p4/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 46),
			("d1e2", 28),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/4p3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d1e2", 84),
			("f3d4", 30),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/4p3/5N2/PPPPQPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("g8f6", 70),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3P4/2PP4/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 127),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3PP3/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 34),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 46),
			("f8c5", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/3Pp3/4BP2/PPP3PP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 26),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e5d4", 80),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2Pp4/2N5/PP2PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 82),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/2PQ4/2N5/PP2PPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 71),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8g4", 535),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/3P1Bb1/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 511),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4N3/4n3/8/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 47),
			("d1e2", 176),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4N3/3Pn3/8/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 35),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/3PP3/2N5/PPP2PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e5d4", 62),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/3pP3/2N5/PPP2PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d1d4", 62),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/8/3QP3/2N5/PPP2PPP/R1B1KBNR b KQkq - 0 0",
		&[
			("b8c6", 57),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/8/3p4/2PP4/5b2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2f3", 27),
			("e2f3", 48),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/3P4/5NP1/PPP1PPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c5d4", 62),
		]
	),
	(
		"rnbqk1nr/pppp1ppp/8/2b1p3/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3e5", 38),
			("f1c4", 31),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/4P3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d6e5", 50),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/4p3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3e5", 49),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2P5/2N1PN2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("b8c6", 32),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 51),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4N3/8/8/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e5c6", 39),
		]
	),
	(
		"rnb1k1nr/ppppqppp/4p3/8/1bPP4/5N2/PP1BPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("g8f6", 36),
		]
	),
	(
		"rnb1kbnr/ppp1pppp/8/3q4/8/5Q2/PPPP1PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("d5f3", 26),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1f4", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3pP3/1p6/5N2/P1PP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 27),
			("b8c6", 99),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/1P3N2/P2PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1b2", 72),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/1P3N2/PB1PPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8f5", 28),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/2P1p3/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 26),
			("c1e3", 30),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2P5/2N2N2/PP1PPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c4d5", 173),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3P4/8/2N2N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6d5", 173),
		]
	),
	(
		"rnbqkb1r/pppp2pp/5n2/4pp2/2B1P3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d2d3", 35),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/2p5/4P3/2N5/PPPPNPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 32),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 37),
			("b8c6", 120),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pn1p/8/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 39),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2nP4/8/8/5N2/PPPPP1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8d6", 42),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3pP3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e2", 165),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/8/3pP3/5N2/PPPPNPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c6c5", 153),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/2PP4/5NP1/PP2PP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8e7", 33),
			("d7d5", 49),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/2P5/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("g2g3", 60),
			("d2d4", 30),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 61),
		]
	),
	(
		"rn1qkbnr/pb1ppppp/1p6/2p5/2P5/5NP1/PP1PPP1P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 59),
		]
	),
	(
		"rn1qkbnr/pb1ppppp/1p6/2p5/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 31),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3QP3/2P5/PP3PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("g8f6", 27),
			("d5e4", 46),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c4d5", 93),
			("b1c3", 29),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3P4/8/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f6d5", 93),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/5NP1/PPPP1P1P/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 27),
			("g8f6", 46),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/2B1P3/5N2/PPPPQPPP/RNB1K2R b KQkq - 0 0",
		&[
			("f8e7", 54),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/6P1/2N5/PPPPPP1P/R1BQKBNR b KQkq - 0 0",
		&[
			("f6g4", 63),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/6n1/2N5/PPPPPP1P/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 63),
		]
	),
	(
		"rnbqkb1r/pppppppp/8/8/4P1n1/2N5/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d6", 27),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P1P2/4PN2/PPP3PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqkbnr/p1pppppp/8/1p6/4P3/8/PPPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1b5", 86),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n5/3nP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 62),
			("c2c4", 37),
		]
	),
	(
		"r1bqkb1r/pppppppp/2n5/3nP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 29),
			("e7e6", 36),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("e7e6", 67),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2pP4/8/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 31),
			("b1c3", 62),
		]
	),
	(
		"rnbqkb1r/p2ppppp/1p3n2/2p5/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 28),
			("c1f4", 26),
		]
	),
	(
		"rnbqkb1r/pp1ppp1p/5np1/2pP4/8/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 46),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P4/6P1/PPP1PPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d8c7", 38),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4N3/4n3/8/PPPPQPPP/RNB1KB1R b KQkq - 0 0",
		&[
			("d8e7", 149),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 74),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/5P2/4PN2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 26),
			("d2d4", 41),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/6p1/3p4/5P2/5NP1/PPPPP2P/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/5NP1/PPPPP2P/RNBQKB1R w KQkq - 0 0",
		&[
			("f1g2", 79),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/5P2/5NP1/PPPPP1BP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 42),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 62),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f2f4", 37),
		]
	),
	(
		"r1bqkbnr/pp1npppp/2pp4/8/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d8c7", 29),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1d2", 25),
			("e4d5", 29),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/5N2/PPP1PPPP/RN1QKB1R w KQkq - 0 0",
		&[
			("e2e3", 41),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 88),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p1P3/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 283),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c3", 48),
			("b1c3", 117),
			("c2c4", 39),
			("d2d4", 53),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 32),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3P4/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6d5", 54),
		]
	),
	(
		"rnbqkbnr/2pppppp/p7/1p6/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 60),
		]
	),
	(
		"rn1qkbnr/1bpppppp/p7/1p6/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 42),
		]
	),
	(
		"rn1qkbnr/1bpppppp/p7/1p6/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 68),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pP2/2B5/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("d7d5", 49),
		]
	),
	(
		"r1bqkb1r/ppppnppp/2n5/1B2p3/4P3/5N2/PPPP1PPP/RNBQ1RK1 b kq - 0 0",
		&[
			("d7d6", 43),
			("g7g6", 56),
			("a7a6", 41),
		]
	),
	(
		"rn1qkbnr/pp2pppp/3p4/2p5/2P1P1b1/5N2/PP1PBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e5", 51),
			("g8f6", 61),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e4e5", 46),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3pP3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f6e4", 33),
			("f6d7", 26),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/4p3/2pP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e5d4", 65),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2p5/3pP3/3P4/PPP1NPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 69),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3pp3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 53),
			("g1f3", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3P4/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e6d5", 53),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/2P5/8/PP1P1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 34),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/3PP3/2P5/PP1N1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e6e5", 42),
		]
	),
	(
		"rnbqkbnr/p1p1pppp/8/1p6/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a4", 40),
		]
	),
	(
		"r1bqkbnr/pppppp1p/2n3p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 44),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1f3", 43),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8c6", 44),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2P1P3/2N5/PP1P1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 47),
		]
	),
	(
		"rnbqkbnr/pppp2pp/4p3/5p2/2P5/2N5/PP1PPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 30),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/2P5/1P2PN2/P2P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 26),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N2P2/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("c7c6", 95),
			("f8g7", 127),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/4P3/8/8/PPPP1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 60),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3pP3/8/8/PPPP1PPP/RNBQKBNR w KQkq  - 0 0",
		&[
			("e5d6", 37),
		]
	),
	(
		"rnbqkbnr/p1pppppp/8/1p6/3P4/5N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8b7", 109),
		]
	),
	(
		"rn1qkbnr/pbpppppp/8/1p6/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e3", 53),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/4PP2/8/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 38),
			("e7e5", 40),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2p1p3/2P1P3/5N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 118),
		]
	),
	(
		"rnbqkbnr/pp3ppp/3p4/2p1p3/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8g4", 104),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/5P2/PPP1P1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 36),
			("b1c3", 35),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 42),
		]
	),
	(
		"rnbqk1nr/ppppbppp/4p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 34),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/2PP2b1/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e5", 60),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3P1b2/3P4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c6d5", 42),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/4P3/8/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 103),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/8/PPPPNPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 82),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3pP3/8/8/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 37),
		]
	),
	(
		"rnbqkbnr/1pppppp1/p6p/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 42),
		]
	),
	(
		"rnbqkbnr/pppppp1p/8/6p1/3P4/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1g5", 58),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/4P3/5P2/PPPP2PP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8c5", 33),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p1p3/3p4/3PP3/5P2/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 60),
			("c1e3", 30),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/4p3/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 33),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/4PP2/3P4/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 27),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/3P1N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 87),
		]
	),
	(
		"r1bqkbnr/pppppp1p/2n3p1/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 37),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/8/P6P/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 124),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/P6P/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 209),
		]
	),
	(
		"rnbqkbnr/pp1pp1pp/8/2pP1p2/8/8/PPP1PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 30),
			("c2c4", 25),
		]
	),
	(
		"r1bqkbnr/pppppppp/2n5/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 40),
			("g8f6", 49),
		]
	),
	(
		"rnbqk1nr/ppp1bppp/3pp3/8/3PP3/3B1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b8d7", 31),
		]
	),
	(
		"rn1qkbnr/pbpppppp/8/1p6/3P4/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("a7a6", 51),
		]
	),
	(
		"rn1qkbnr/1bpppppp/p7/1p6/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 28),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2np4/1B2p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("c8d7", 70),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 239),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 92),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P1B2/4P3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("b8c6", 72),
		]
	),
	(
		"rnbqkbnr/pppppp1p/8/6p1/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("a7a5", 29),
		]
	),
	(
		"rnbqkb1r/pppp2pp/4pn2/5p2/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1h3", 34),
			("c2c4", 48),
			("g1f3", 35),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/7P/PPPP1PP1/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 98),
		]
	),
	(
		"r1bqkbnr/pppn1ppp/3p4/4p3/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 26),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d4c5", 27),
		]
	),
	(
		"rnb1kbnr/pppp1ppp/8/8/2B1Pp1q/8/PPPP2PP/RNBQK1NR w KQkq - 0 0",
		&[
			("e1f1", 76),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2P1p3/5N2/PP1PPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f3d4", 50),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2pp4/4p3/2BPP3/5N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 41),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/8/2pP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4d5", 42),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/6B1/2PP4/8/PP2PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("h7h6", 53),
			("f8b4", 27),
		]
	),
	(
		"rnbqkb1r/pppp1pp1/4pn1p/6B1/2PP4/8/PP2PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g5h4", 48),
		]
	),
	(
		"rn1qkbnr/ppp1pppp/3p4/8/3P2b1/4PN2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8d7", 33),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2ppP3/5P2/2N5/PPPP2PP/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 38),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/3P1Bb1/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("e7e6", 541),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/2PPP3/5N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 27),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2P5/8/2N5/PPP1PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8c5", 53),
		]
	),
	(
		"rnbqk1nr/pp1p1ppp/4p3/2b5/8/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("c3e4", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2B1P3/8/PPPP1PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d1e2", 40),
		]
	),
	(
		"rn1qkbnr/pbpppppp/8/1p6/3PP3/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("a7a6", 48),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/2p5/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 46),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 25),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/2PP4/5P2/PP2P1PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e2e4", 97),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/8/5NP1/PPPPPPBP/RNBQK2R w KQkq - 0 0",
		&[
			("e1g1", 111),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/8/5NP1/PPPPPPBP/RNBQ1RK1 b kq - 0 0",
		&[
			("b8d7", 70),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 46),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 50),
			("d2d4", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 101),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pP4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 89),
		]
	),
	(
		"rnb1kbnr/pp2pppp/8/2pq4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 84),
		]
	),
	(
		"rnb1kbnr/pp2pppp/8/2pq4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5d8", 33),
			("d5e6", 42),
		]
	),
	(
		"r1bqkbnr/ppppp1pp/2n5/4Pp2/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 48),
		]
	),
	(
		"rn1qkbnr/pp1bpppp/3p4/1Bp5/P3P3/5N2/1PPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 32),
			("b8c6", 25),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2pp2p1/8/3PPP2/2N5/PPP3PP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 59),
			("d8b6", 35),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/4P3/6P1/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 91),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/6P1/PPPP1P1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 70),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/6P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("d5e4", 45),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/2pp4/4P3/3P2P1/PPPN1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("b8c6", 38),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/3P4/2P1P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 29),
			("c5d4", 68),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/3P4/4PN2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1e2", 30),
			("c2c4", 46),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("e2e4", 28),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 70),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 42),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c2c4", 39),
			("c1g5", 39),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/2PP4/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 33),
			("b7b6", 29),
		]
	),
	(
		"r1bqkbnr/pp1npppp/2pp4/8/2PPP3/2N5/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d8c7", 50),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/3P4/PPPNPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 45),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 30),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3pPb2/3P4/8/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7e6", 64),
		]
	),
	(
		"rnbqkbnr/p2ppppp/1p6/2p5/1P2P3/8/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b4c5", 47),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/2PP4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 153),
			("d4d5", 256),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/2PP4/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 148),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/3P4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c6e5", 258),
		]
	),
	(
		"rnbqkb1r/ppp2Npp/3p1n2/8/4P3/8/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e8f7", 81),
		]
	),
	(
		"rnbqkb1r/pp2pppp/3p1n2/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("c8d7", 53),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 91),
			("d7d6", 36),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/3P4/5N2/PPP1PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d4e5", 36),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/1Bp5/4P3/2P2N2/PP1P1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 33),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/3P1N2/PPP1PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 33),
			("g7g6", 33),
		]
	),
	(
		"rnbqkb1r/ppppp1pp/5n2/5p2/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 40),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 30),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 50),
			("c7c5", 50),
			("a7a6", 31),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2pp4/8/3P1B2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d8c7", 34),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/3P4/4B3/PPP2PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("e7e5", 44),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/8/PPPPPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 31),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P4/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("c7c5", 52),
			("f8d6", 25),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/8/1Bp5/3nP3/5N2/PPPP1PPP/RNBQK2R w KQkq - 0 0",
		&[
			("f3d4", 70),
		]
	),
	(
		"r1bqkbnr/pp1p1ppp/2n1p3/2p5/2P5/2N3P1/PP1PPPBP/R1BQK1NR b KQkq - 0 0",
		&[
			("g8f6", 44),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/3p2p1/2pP4/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 76),
		]
	),
	(
		"rnbqkb1r/ppp1ppp1/5B1p/3p4/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e7f6", 48),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/8/1P2P3/PBPP1PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("f2f4", 46),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p1p3/8/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d5", 27),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/4P3/5N2/PPPPBPPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 45),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3PP3/2N5/PPP1NPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 150),
		]
	),
	(
		"rnbq1bnr/pppppkpp/5p2/8/3PP3/2N5/PPP2PPP/R1BQKBNR b KQ - 0 0",
		&[
			("e7e6", 35),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P1B2/4PN2/PPP2PPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8g7", 45),
		]
	),
	(
		"r1bqkbnr/pppnpppp/8/3p2B1/3P4/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 30),
		]
	),
	(
		"rn1qkbnr/pbpppppp/8/1p6/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("a7a6", 36),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/2P5/5NP1/PP1PPP1P/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 27),
		]
	),
	(
		"rnbqkbnr/pp2pppp/3p4/1Bp5/4P3/2N5/PPPP1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8d7", 47),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p3/2B1n3/2N5/PPPP1PPP/R1BQK1NR w KQkq - 0 0",
		&[
			("d1h5", 64),
			("c4f7", 26),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p2Q/2B1n3/2N5/PPPP1PPP/R1B1K1NR b KQkq - 0 0",
		&[
			("e4d6", 63),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 73),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 56),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/4p3/2P5/P7/1P1PPPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 27),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/3P4/4P3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c8b7", 96),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/3P4/4P3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 62),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 34),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/4P3/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d3d4", 759),
		]
	),
	(
		"rnbqkb1r/pppppppp/5n2/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g6", 25),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/8/5N2/PPPPPPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 27),
			("g2g3", 26),
			("d2d4", 25),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e3", 25),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2P2n2/8/2P5/8/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 63),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/3p4/3p4/2P1P3/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("e3d4", 53),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 37),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3p4/3P2b1/4PN2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("g8f6", 30),
			("e7e6", 32),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2pPP3/8/PP3PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("f1c4", 58),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2pP4/4P3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 43),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/8/P6P/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 123),
			("c7c5", 36),
			("g8f6", 32),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/3P4/2N5/PPP1PPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d4d5", 28),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n5/3pp3/4P3/2P2N2/PP1P1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 26),
			("d1a4", 54),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/3P4/2P5/8/PP2PPPP/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e6", 47),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 40),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3P4/8/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 32),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/2PP4/4PN2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 47),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/1P2P3/P1PP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 54),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/2PP4/4P3/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c5d4", 37),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n5/4p3/2P5/1P6/PB1PPPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("g8f6", 46),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/2P5/6P1/PP1PPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 60),
			("g1f3", 26),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/2P5/6P1/PP1PPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 54),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3p4/4n3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("c3e4", 48),
		]
	),
	(
		"r1bqkbnr/ppppnppp/8/3Pp3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("e7g6", 42),
			("g8f6", 30),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/2P5/4P3/PP1P1PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 41),
		]
	),
	(
		"rnbqkbnr/ppp1pp1p/3p2p1/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 53),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c3", 36),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/2B1P3/3P1N2/PPP2PPP/RNBQK2R b KQkq - 0 0",
		&[
			("f5e4", 111),
		]
	),
	(
		"rnbqkbnr/1pp2ppp/p3p3/3p4/3PP3/5N2/PPPN1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c7c5", 43),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/1Bb1p3/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("d7d6", 27),
		]
	),
	(
		"rnbqkbnr/pppp2pp/8/4pp2/4P3/3P1N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 37),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 33),
		]
	),
	(
		"rnbq1bnr/pppppkpp/5p2/8/4P3/5N2/PPPP1PPP/RNBQKB1R w KQ - 0 0",
		&[
			("d2d4", 31),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4d5", 121),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3P4/8/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d8d5", 121),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/8/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("b1c3", 100),
		]
	),
	(
		"r1b1kbnr/ppp1pppp/2n5/3q4/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5a5", 58),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/5p2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 64),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3P1p2/5N2/PPP1P1PP/RNBQKB1R b KQkq - 0 0",
		&[
			("g7g5", 37),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/8/4p3/3pP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d4c3", 45),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2Bp4/2p5/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("b7c6", 73),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("g7g6", 89),
			("d7d5", 57),
			("e7e6", 35),
			("d7d6", 52),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 87),
		]
	),
	(
		"r1bqkbnr/pp1ppp1p/2n3p1/2p5/4P3/3P2P1/PPP2PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8g7", 84),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("f4e5", 47),
		]
	),
	(
		"r2qkbnr/ppp1pppp/2n5/3pPb2/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e7e6", 46),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/2P5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("e7e6", 31),
			("c7c6", 27),
		]
	),
	(
		"r1bqkbnr/pp1npppp/3p4/1Bp5/4P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("a7a6", 54),
			("g8f6", 1339),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 51),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/4P3/3P2P1/PPP2P1P/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 39),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/6P1/7P/PPPPPP2/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1e2", 55),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/2b1p3/2B1P3/2NP4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("d7d6", 41),
			("g8f6", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/8/3pP3/4N3/8/PPPP1PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d5e4", 76),
		]
	),
	(
		"rnbqkb1r/pppppp1p/5np1/8/3P1P2/4P3/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 51),
			("b8c6", 45),
		]
	),
	(
		"rnbqk2r/ppppppbp/5np1/8/3P1P2/4P3/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 43),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/3PP3/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1e2", 47),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/5P2/5N2/PPPPP1PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e2e4", 52),
		]
	),
	(
		"rnbqkbnr/pp1ppp1p/2p3p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R b KQkq - 0 0",
		&[
			("f8g7", 50),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/2p3p1/8/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 26),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/3P1B2/8/PPP1PPPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f8g7", 71),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P1B2/8/PPP1PPPP/RN1QKBNR w KQkq - 0 0",
		&[
			("e2e3", 34),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2n5/2pp4/3P1B2/4P3/PPP2PPP/RN1QKBNR w KQkq - 0 0",
		&[
			("g1f3", 32),
			("c2c3", 36),
		]
	),
	(
		"rnbqk1nr/pp1pppbp/6p1/2p5/2B1P3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("b8c6", 40),
		]
	),
	(
		"rnbqkbnr/pp3ppp/2p5/3pp3/8/4P1P1/PPPP1PBP/RNBQK1NR w KQkq - 0 0",
		&[
			("g1e2", 50),
		]
	),
	(
		"r1bqkbnr/pppp2pp/2n5/4pp2/4PP2/5N2/PPPP2PP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4f5", 26),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/8/4P1P1/PPPP1PBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 32),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3p1b2/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e6", 51),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/4P3/3P1N2/PPP1BPPP/RNBQK2R b KQkq - 0 0",
		&[
			("f8c5", 34),
			("d7d5", 37),
		]
	),
	(
		"rnbqkbnr/ppppp2p/5p2/6p1/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d1h5", 97),
		]
	),
	(
		"rnb1kbnr/pp2pppp/2p5/3q4/3P4/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b1c3", 43),
		]
	),
	(
		"r1bqkbBr/pppp2pp/2n5/4pp2/4P3/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("h8g8", 78),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/4PP2/PPPP1KPP/RNBQ1BNR b kq - 0 0",
		&[
			("g8f6", 28),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n1pn2/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 35),
		]
	),
	(
		"rnbqkbnr/pp2pp1p/2p3p1/3p4/4P3/3P4/PPPN1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g1f3", 37),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/2P1P3/5N2/PP1P1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("d7d6", 40),
		]
	),
	(
		"r1bqkbnr/ppp1p1pp/2n5/1B1p1P2/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8f5", 47),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5B2/2pp4/3P4/2N5/PPP1PPPP/R2QKBNR b KQkq - 0 0",
		&[
			("e7f6", 45),
		]
	),
	(
		"r1bqkb1r/pp1ppppp/2n2n2/2p5/2P1P3/2N2N2/PP1P1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e7e5", 35),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/4P3/2N3P1/PPPP1P1P/R1BQKBNR b KQkq - 0 0",
		&[
			("g7g6", 27),
		]
	),
	(
		"rnbqkbnr/2pppppp/p7/1p6/4P3/5N2/PPPP1PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("d2d4", 47),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/8/2Pp4/3P1N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c5", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/3P2P1/PPP1PP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 27),
			("f1g2", 32),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3P4/8/6P1/PP1PPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("c6d5", 44),
		]
	),
	(
		"rnb1kb1r/ppppqppp/5n2/8/4P3/5N2/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6e4", 43),
			("e7e4", 67),
		]
	),
	(
		"rnbqkbnr/p2ppppp/8/1ppP4/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("a7a6", 89),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1ppP4/4P3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a4", 53),
		]
	),
	(
		"rnbqkbnr/3ppppp/p7/1ppP4/P3P3/8/1PP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("b5b4", 50),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/3p2p1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 90),
		]
	),
	(
		"rnbqkb1r/p1pp1ppp/1p2pn2/6B1/2PP4/5N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c8b7", 27),
			("f8b4", 40),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 55),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/3p1np1/8/3P4/2P2N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1g5", 31),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/8/1Bp5/3NP3/8/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c5d4", 70),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/3p4/2b1p3/4PP2/2N2N2/PPPP2PP/R1BQKB1R b KQkq - 0 0",
		&[
			("g8f6", 49),
		]
	),
	(
		"rnbqkbnr/pppppppp/8/8/8/N7/PPPPPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("d7d5", 46),
			("e7e5", 101),
		]
	),
	(
		"r1bqkb1r/pppp1ppp/2n2n2/4p3/3PP3/2P2N2/PP3PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6e4", 36),
			("e5d4", 25),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/3p4/4P3/3P4/PPP1QPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("g8f6", 29),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3pP3/8/3P4/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6d7", 74),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/4p3/8/8/3PP3/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("d7d5", 38),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P3NP1/P1PPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("c8g4", 25),
			("c8f5", 40),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/2p5/3PP3/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("f6e4", 49),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/3P4/2N2N2/PPP1PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d6", 72),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/4p3/2N2P2/PPPP2PP/R1BQKBNR w KQkq - 0 0",
		&[
			("f3e4", 68),
		]
	),
	(
		"rnbqkbnr/2pppppp/p7/1p6/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c8b7", 102),
		]
	),
	(
		"rn1qkbnr/1bpppppp/p7/1p6/3PP3/3B4/PPP2PPP/RNBQK1NR w KQkq - 0 0",
		&[
			("d1e2", 28),
			("g1f3", 31),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/5n2/8/3pP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("e4e5", 34),
		]
	),
	(
		"rnbqkb1r/pppnpppp/8/3pP3/8/3P4/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d3d4", 34),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/2PNp3/8/PP1PPPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 31),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/5P2/PPPPPKPP/RNBQ1BNR b kq - 0 0",
		&[
			("d7d5", 47),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/5P2/PPPPPKPP/RNBQ1BNR w kq - 0 0",
		&[
			("g2g3", 39),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/3P4/PPPNPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8g7", 54),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/3P4/PPPNPPPP/R1BQKBNR w KQkq - 0 0",
		&[
			("g2g3", 27),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/2n1p3/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 38),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/2p5/4P3/P7/1PPP1PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("b2b4", 49),
		]
	),
	(
		"rnbqkbnr/pppp1ppp/8/8/3PPp2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8h4", 57),
			("d7d5", 92),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1P2/3BP3/PPP3PP/RNBQK1NR b KQkq - 0 0",
		&[
			("f8d6", 54),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/6P1/8/PPPPPP1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 25),
		]
	),
	(
		"rnbqkbnr/pp3ppp/4p3/1BppP3/8/5N2/PPPP1PPP/RNBQK2R b KQkq - 0 0",
		&[
			("c8d7", 82),
		]
	),
	(
		"rnb1kbnr/ppq1pppp/2pp4/8/2P5/6P1/PP1PPPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("b1c3", 39),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/5n2/4p3/4PP2/8/PPPP2PP/RNBQKBNR w KQkq - 0 0",
		&[
			("f4e5", 40),
		]
	),
	(
		"rnbqkbnr/1ppppppp/p7/8/2P5/8/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 41),
			("b1c3", 40),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4P3/2P1n3/8/PP1NPPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("f8b4", 46),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/8/4p3/2B1n3/2N2N2/PPPP1PPP/R1BQK2R b KQkq - 0 0",
		&[
			("b8c6", 54),
			("e4c3", 108),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/4p3/8/2p5/5NP1/PP1PPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("b8d7", 26),
			("g8f6", 25),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/8/4N3/8/PPPPQPPP/R1B1KBNR b KQkq - 0 0",
		&[
			("f6e4", 37),
		]
	),
	(
		"rnbqk1nr/ppp1ppbp/6p1/3p4/3P4/6P1/PPP1PPBP/RNBQK1NR w KQkq - 0 0",
		&[
			("c2c4", 37),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/8/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5b4", 73),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/2P2N2/PP2PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("f8d6", 39),
		]
	),
	(
		"r1bqkbnr/ppp1pppp/2n5/3p4/3P4/2P2N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c8g4", 35),
		]
	),
	(
		"rnbqkbnr/p1pppppp/1p6/8/8/2N2N2/PPPPPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("c8b7", 113),
		]
	),
	(
		"rn1qkbnr/pbpppppp/1p6/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 106),
		]
	),
	(
		"rn1qkb1r/ppp1pppp/5n2/3P4/3P2b1/5P2/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("g4f5", 65),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/5n2/3p4/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5d4", 48),
			("d5e4", 281),
			("f6e4", 42),
		]
	),
	(
		"rnbqkb1r/ppp1pppp/3p1n2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 42),
			("e2e4", 48),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/4p3/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("b8c6", 48),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d7d5", 36),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/2p5/8/8/2P5/PP1PPPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 51),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/3p4/4p3/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 47),
			("e2e4", 56),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/4p3/3Pn3/2P5/8/PP2PPPP/RNBQKBNR w KQkq - 0 0",
		&[
			("g1f3", 97),
			("b1c3", 144),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/4p3/3Pn3/2P5/5N2/PP2PPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("e5f3", 97),
		]
	),
	(
		"rnbqkb1r/pppp1ppp/4pn2/8/8/2N2N2/PPPPPPPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e2e4", 51),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("d2d4", 44),
		]
	),
	(
		"rn1qkbnr/pbpppp1p/1p4p1/8/3PP3/2N2N2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("f8g7", 53),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/3P4/3BP3/PPPN1PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c8g4", 25),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/2N1PN2/PPP2PPP/R1BQKB1R b KQkq - 0 0",
		&[
			("g7g6", 208),
		]
	),
	(
		"rnbqkbnr/1pppppp1/p6p/8/2PPP3/8/PP3PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c6", 46),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3p4/3P1B2/4P3/PPPN1PPP/R2QKBNR b KQkq - 0 0",
		&[
			("f8d6", 54),
		]
	),
	(
		"rnbqk1nr/ppp2ppp/8/3pp3/Pb5P/8/2PPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("c1b2", 44),
		]
	),
	(
		"rnbqkbnr/pp1p2pp/2p1p3/5p2/2PP4/5N2/PP2PPPP/RNBQKB1R w KQkq - 0 0",
		&[
			("c1d2", 113),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a3", 52),
		]
	),
	(
		"rnbqkb1r/pppppp1p/6p1/8/3PnB2/5N2/PPP1PPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("c7c5", 36),
		]
	),
	(
		"rnbqkbnr/pp2pppp/8/2pp4/8/P6P/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 43),
		]
	),
	(
		"rnbqkbnr/pp3ppp/8/2ppp3/8/P1P4P/1P1PPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("g2g3", 31),
		]
	),
	(
		"rnbqkbnr/pppppp1p/6p1/8/8/P6P/1PPPPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("f8g7", 60),
		]
	),
	(
		"rnbqk1nr/ppppppbp/6p1/8/8/P6P/1PPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("c2c3", 55),
		]
	),
	(
		"rnbqkbnr/1pp2ppp/p3p3/3p4/2PP4/2N2N2/PP2PPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("d5c4", 35),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/3PP3/PPP1NPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 27),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/P1P4P/1P1PPPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("c7c5", 41),
			("f7f5", 29),
			("g8f6", 66),
			("f8d6", 33),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/5NPP/PPPPPP2/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 82),
			("c8f5", 59),
			("g7g6", 28),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/5NPP/PPPPPP2/RNBQKB1R w KQkq - 0 0",
		&[
			("b2b3", 77),
		]
	),
	(
		"rnbqkbnr/1p1pppp1/p1p4p/8/3PP3/5N2/PPP2PPP/RNBQKB1R w KQkq - 0 0",
		&[
			("f1d3", 25),
		]
	),
	(
		"r1bqkbnr/pp1ppppp/2n5/8/4P3/3Q4/PPP2PPP/RNB1KBNR b KQkq - 0 0",
		&[
			("g8f6", 64),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/3P4/P1P4P/1P2PPP1/RNBQKBNR b KQkq - 0 0",
		&[
			("e5e4", 31),
			("b8c6", 29),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/5n2/3pp3/8/P1P4P/1P1PPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("d2d4", 64),
		]
	),
	(
		"rnbqkbnr/1pppppp1/p6p/8/3PP3/3B4/PPP2PPP/RNBQK1NR b KQkq - 0 0",
		&[
			("c7c6", 44),
		]
	),
	(
		"r1bqk1nr/pppp1ppp/2n5/3Np3/1bP5/5N2/PP1PPPPP/R1BQKB1R b KQkq - 0 0",
		&[
			("e5e4", 45),
		]
	),
	(
		"rnbqkbnr/pp1ppppp/8/2p5/8/7P/PPPPPPP1/RNBQKBNR w KQkq - 0 0",
		&[
			("a2a3", 35),
		]
	),
	(
		"rnbqkbnr/pp1p2pp/2p1p3/5p2/2PP4/5N2/PP1BPPPP/RN1QKB1R b KQkq - 0 0",
		&[
			("d7d5", 105),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p4/6P1/4P3/PPPP1P1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 47),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/8/3p2P1/8/8/PPPPPP1P/RNBQKBNR b KQkq - 0 0",
		&[
			("e7e5", 42),
		]
	),
	(
		"rnbqkb1r/pp2pppp/5n2/2pp4/4P3/2N2N2/PPPP1PPP/R1BQKB1R w KQkq - 0 0",
		&[
			("e4d5", 48),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p2P1/8/8/PPPPPPBP/RNBQK1NR b KQkq - 0 0",
		&[
			("e7e5", 25),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/4P3/3P4/PPP1QPPP/RNB1KBNR b KQkq - 0 0",
		&[
			("d5d4", 29),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3pp3/8/4P1N1/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("b8c6", 32),
		]
	),
	(
		"rnbqkb1r/ppp2ppp/4pn2/3pP3/8/1P6/PBPP1PPP/RN1QKBNR b KQkq - 0 0",
		&[
			("f6d7", 85),
		]
	),
	(
		"rnbqkb1r/ppp1nppp/4p3/3p4/3PP3/2NB4/PPP2PPP/R1BQK1NR b KQkq - 0 0",
		&[
			("c7c6", 49),
		]
	),
	(
		"rnb1kbnr/pppp1ppp/8/8/3PPp1q/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e1e2", 55),
		]
	),
	(
		"r1bqkbnr/pp2pppp/2np4/2p5/4P3/3P2P1/PPP2P1P/RNBQKBNR w KQkq - 0 0",
		&[
			("f1g2", 36),
		]
	),
	(
		"r1bqkbnr/ppp2ppp/2n1p3/3p4/2PP4/2N1P3/PP3PPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 55),
		]
	),
	(
		"rnbqkb1r/ppppnppp/4p3/8/4P3/2N5/PPPP1PPP/R1BQKBNR w KQkq - 0 0",
		&[
			("d2d4", 43),
		]
	),
	(
		"rnbqkb1r/ppp1pp1p/5np1/3p4/8/1P3NP1/P1PPPPBP/RNBQK2R b KQkq - 0 0",
		&[
			("f8g7", 48),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3P4/3P1p2/8/PPP3PP/RNBQKBNR b KQkq - 0 0",
		&[
			("d8h4", 75),
		]
	),
	(
		"r1bqkbnr/pppp1ppp/4p3/3Pn3/2P5/2N5/PP2PPPP/R1BQKBNR b KQkq - 0 0",
		&[
			("g8f6", 144),
		]
	),
	(
		"rnbqkb1r/pp1ppppp/8/2pnP3/3P4/5N2/PPP2PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c5d4", 51),
		]
	),
	(
		"rnbqkbnr/ppp1pppp/3p4/8/8/4P3/PPPPNPPP/RNBQKB1R b KQkq - 0 0",
		&[
			("c7c6", 32),
		]
	),
	(
		"rnbqkbnr/pp2pppp/2p5/3p4/8/4P1N1/PPPP1PPP/RNBQKB1R b KQkq - 0 0",
		&[
			("g8f6", 42),
		]
	),
	(
		"rnbqkbnr/ppp2ppp/8/3p4/3PPp2/8/PPP3PP/RNBQKBNR w KQkq - 0 0",
		&[
			("e4d5", 88),
		]
	),
	(
		"rnbqkbnr/pp1p1ppp/2p5/4p3/3PP3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("g8f6", 759),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4p3/3PP3/8/PPP2PPP/RNBQKBNR w KQkq - 0 0",
		&[
			("d4e5", 759),
		]
	),
	(
		"rnbqkb1r/pp1p1ppp/2p2n2/4P3/4P3/8/PPP2PPP/RNBQKBNR b KQkq - 0 0",
		&[
			("f6e4", 746),
		]
	),
	(
		"r1bqkb1r/ppp1pppp/2n2n2/3p4/3P4/2N2NP1/PPP1PP1P/R1BQKB1R b KQkq - 0 0",
		&[
			("h7h6", 83),
		]
	),
	(
		"rnbqkb1r/pp2pppp/2p2n2/3p4/8/1P3NPP/P1PPPP2/RNBQKB1R b KQkq - 0 0",
		&[
			("c8f5", 36),
		]
	),
	(
		"rn1qkbnr/pp2pppp/2p5/3p1b2/8/5NPP/PPPPPP2/RNBQKB1R w KQkq - 0 0",
		&[
			("b2b3", 58),
		]
	),
];
