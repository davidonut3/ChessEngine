use pyo3::prelude::*;
use fenlib::fen::*;
use fenlib::tests::*;
use botv1::BotV1;

use std::time::Instant;
use std::time::Duration;

/// A Python-exposed wrapper for the `Fen` struct from fenlib, representing a chess position.
/// Code by David van den Beukel, documentation by ChatGPT.
#[pyclass]
#[derive(Debug, Clone)]
pub struct FenPy {
    fen: Fen,
}

#[pymethods]
impl FenPy {
    /// Creates a new empty (default) FEN position.
    #[new]
    pub fn new() -> Self {
        let fen: Fen = Fen::new();
        Self { fen }
    }

    /// Constructs a `FenPy` object from a FEN string.
    ///
    /// # Arguments
    /// * `fen_str` - A valid FEN string representing the chess position.
    #[staticmethod]
    pub fn from_str(fen_str: &str) -> Self {
        let fen: Fen = Fen::from_str(fen_str);
        Self { fen }
    }

    /// Returns the current FEN string representation of the position.
    pub fn to_string(&self) -> String {
        self.fen.to_string()
    }

    /// Returns a visual 8x8 representation of the board as strings.
    ///
    /// Each element is a piece symbol or empty string.
    pub fn to_visual(&self) -> [[String; 8]; 8] {
        self.fen.to_visual()
    }

    /// Checks if a given move in LAN (long algebraic notation) is legal.
    ///
    /// # Arguments
    /// * `lan` - A move string in the format "e2e4", "e7e8q", etc.
    ///
    /// This may temporarily mutate internal state for checking legality.
    pub fn is_legal_move_lan(&mut self, lan: &str) -> bool {
        self.fen.is_legal_move_lan(lan)
    }

    /// Returns a list of legal moves for the piece on the given tile.
    ///
    /// # Arguments
    /// * `tile` - A square in algebraic notation, e.g., "e2".
    pub fn get_possible_moves_tile(&mut self, tile: &str) -> Vec<String> {
        self.fen.get_possible_moves_tile(tile)
    }

    /// Returns true if the current player is in check.
    ///
    /// This may mutate internal state during the computation.
    pub fn in_check(&mut self) -> bool {
        self.fen.in_check()
    }

    /// Makes a move (in LAN format) and updates the FEN state accordingly.
    ///
    /// # Arguments
    /// * `lan` - The move string to apply.
    pub fn lan_to_fen(&mut self, lan: &str) {
        self.fen.lan_to_fen(lan);
    }

    /// Returns a list of all legal moves for the current player.
    pub fn get_all_possible_moves_lan(&mut self) -> Vec<String> {
        self.fen.get_all_possible_moves_lan()
    }

    /// Checks if the game has ended, and returns the result.
    ///
    /// Return values:
    /// * "1-0" if white wins
    /// * "0-1" if black wins
    /// * "½-½" if draw
    /// * "not ended" if the game is still ongoing
    pub fn game_ended(&mut self) -> String {
        self.fen.game_ended()
    }

    /// Returns true if it's white's turn to move, false if it's black's.
    pub fn white_to_move(&self) -> bool {
        self.fen.white_to_move
    }
}


#[pyclass]
#[derive(Debug, Clone)]
pub struct BotV1Py {
    botv1: BotV1,
}

#[pymethods]
impl BotV1Py {
    #[new]
    pub fn new() -> Self {
        let botv1: BotV1 = BotV1::new();
        Self { botv1 }
    }

    #[staticmethod]
    pub fn from_fen(fen_str: &str) -> Self {
        let botv1: BotV1 = BotV1::from_fen(fen_str);
        Self { botv1 }
    }

    pub fn get_move(&mut self) -> String {
        self.botv1.get_move()
    }

    pub fn receive_move(&mut self, lan: &str) {
        self.botv1.receive_move(lan);
    }
}

fn analyze_durations(durations: &[Duration; 100]) -> (Duration, Duration, Duration) {
    let mut total_nanos: u128 = 0;
    let mut min = durations[0];
    let mut max = durations[0];

    for &d in durations {
        total_nanos += d.as_nanos();
        if d < min {
            min = d;
        }
        if d > max {
            max = d;
        }
    }

    let avg = Duration::from_nanos((total_nanos / durations.len() as u128) as u64);
    (min, max, avg)
}

#[pyfunction]
pub fn perft_check(max_depth: usize, fen_str: &str, per_move: bool) {
    perft(max_depth, fen_str, per_move)
}

#[pyfunction]
pub fn benching() {
    let mut random_fens: Vec<Fen> = Vec::new();

    random_fens.push(Fen::from_str("r1bqk2r/ppp1bppp/4pn2/6B1/2N5/2NQ4/PPP2PPP/2KR3R b kq - 0 11"));
    random_fens.push(Fen::from_str("2k2b1r/ppp1pppp/5nb1/q7/3r2P1/7P/PPPBNPB1/R2Q1RK1 b - - 1 12"));
    random_fens.push(Fen::from_str("r1b1kb1r/pp3ppp/1qn1p3/3pPn2/3P4/2N3P1/PP2NPBP/R1BQK2R b KQkq - 2 9"));
    random_fens.push(Fen::from_str("r2qkb1r/1bpn1pp1/p2ppn1p/1p6/3PP2B/2P2N2/PPQN1PPP/R3KB1R w KQkq - 0 9"));
    random_fens.push(Fen::from_str("r2qk2r/1bpnbpp1/p2ppn1p/1p6/3PP2B/2PB1N2/PPQN1PPP/2KR3R b kq - 3 10"));
    random_fens.push(Fen::from_str("r2qk2r/ppp1bppp/2npbn2/8/4P3/2NB1N1P/PP2QPP1/R1B1K2R w KQkq - 1 9"));
    random_fens.push(Fen::from_str("r2q1rk1/ppp1bppp/2npbn2/8/4P3/2NB1N1P/PP2QPP1/R1B2RK1 w - - 3 10"));
    random_fens.push(Fen::from_str("r2q1rk1/ppp1bppp/2npbn2/8/4P3/2NB1N1P/PP2QPP1/R1BR2K1 b - - 4 10"));
    random_fens.push(Fen::from_str("rn1q1rk1/1p2bppp/p2pbn2/4p1B1/4P3/1NN5/PPPQBPPP/R3K2R w KQ - 6 10"));
    random_fens.push(Fen::from_str("r2q1rk1/1p1nbppp/p2pbn2/4p1B1/4P3/1NN5/PPPQBPPP/2KR3R w - - 8 11"));
    random_fens.push(Fen::from_str("rn2kb1r/3ppp1p/b5p1/q1pP4/5P2/2N2N2/PP1nP1PP/R2QKB1R w KQkq - 0 10"));
    random_fens.push(Fen::from_str("r1bq1rk1/pppn1pbp/5np1/4p3/4P3/3B1N2/PPPN1PPP/R1BQ1RK1 w - - 0 9"));
    random_fens.push(Fen::from_str("r1bq1rk1/ppp2pbp/5np1/2n1p3/4P3/1P1B1N2/P1PN1PPP/R1BQ1RK1 w - - 1 10"));
    random_fens.push(Fen::from_str("r1bq1rk1/ppp2pbp/5np1/2n1p3/4P3/1P1B1N2/PBPN1PPP/R2Q1RK1 b - - 2 10"));
    random_fens.push(Fen::from_str("r1bq1rk1/ppp2pbp/5np1/4p3/4P3/1P1P1N2/PB1N1PPP/R2Q1RK1 b - - 0 11",));
    random_fens.push(Fen::from_str("r1b2rk1/ppp2pbp/5np1/4p3/4P3/1P1q1N2/PB1N1PPP/R2Q1RK1 w - - 0 12",));
    random_fens.push(Fen::from_str("r1b2rk1/ppp2pbp/5np1/4N3/4P3/1P1q4/PB1N1PPP/R2Q1RK1 b - - 0 12",));
    random_fens.push(Fen::from_str("r1b2rk1/ppp2pbp/5np1/1q2N3/4P3/1P6/PB1N1PPP/R2Q1RK1 w - - 1 13",));
    random_fens.push(Fen::from_str("r3k1nr/pp1nb1pp/1qp1p3/3pp3/3P4/2PBB2P/PP2QPP1/RN2K2R w KQkq - 0 11",));
    random_fens.push(Fen::from_str("rnbq1rk1/pp4bp/3p1np1/2pPpp2/2P5/2N1P1P1/PP2NPBP/R1BQ1RK1 w - e6 0 10",));
    random_fens.push(Fen::from_str("rnbq1rk1/pp4bp/3p1np1/2pPpp2/2P5/2N1PPP1/PP2N1BP/R1BQ1RK1 b - - 0 10",));
    random_fens.push(Fen::from_str("r1bq1rk1/pp1n2bp/3p1np1/2pPpp2/2P1P3/2N2PP1/PP2N1BP/R1BQ1RK1 b - - 0 11",));
    random_fens.push(Fen::from_str("r1bq1rk1/pp4bp/1n1p1np1/2pPpp2/2P1P3/2N2PP1/PP2N1BP/R1BQ1RK1 w - - 1 12",));
    random_fens.push(Fen::from_str("r2q1rk1/pbpnnpbp/1p1pp1p1/4P3/3P4/P1NBBN2/1PP2PPP/R2QR1K1 b - - 0 10",));
    random_fens.push(Fen::from_str("rnb2rk1/pppp1ppp/3b4/4N2Q/2BPn3/2P5/PP3qPP/RNBK3R w - - 3 9",));
    random_fens.push(Fen::from_str("r1bq1rk1/pp1n2pp/2pbpn2/3p1p2/2PP4/P1N1PN1P/1P2BPP1/R1BQ1RK1 b - - 0 9",));
    random_fens.push(Fen::from_str("r1bq1rk1/1p1n2pp/2pbpn2/p2p1p2/2PP4/PPN1PN1P/4BPP1/R1BQ1RK1 b - - 0 10",));
    random_fens.push(Fen::from_str("r1bq1rk1/1p1n2pp/2pbp3/p2p1p2/2PPn3/PPN1PN1P/4BPP1/R1BQ1RK1 w - - 1 11",));
    random_fens.push(Fen::from_str("r1bq1rk1/1p1n2pp/2pbp3/p2p1p2/2PPn3/PPN1PN1P/1B2BPP1/R2Q1RK1 b - - 2 11",));
    random_fens.push(Fen::from_str("r1bq1rk1/pppp1ppp/8/8/1bPP4/2P5/P1Q1BPPP/R1B1K2R b KQ - 0 10"));
    random_fens.push(Fen::from_str("r1bq1rk1/ppp1bppp/3p4/8/2PP4/2P2B2/P1Q2PPP/R1B2RK1 b - - 1 12"));
    random_fens.push(Fen::from_str("r1bqk2r/pp1n1ppp/4p3/3p4/1bPNn3/1P2P3/P2NBPPP/R1BQK2R w KQkq - 3 9"));
    random_fens.push(Fen::from_str("r1bq1rk1/p3ppbp/1pn2np1/3p4/N7/1P1PP3/PBP2PPP/R2QKBNR w KQ - 0 9"));
    random_fens.push(Fen::from_str("r1bqk2r/pp1n1ppp/2pb1n2/8/3Pp3/2P1P3/PPBNN1PP/R1BQK2R w KQkq - 0 10"));
    random_fens.push(Fen::from_str("r1bqk2r/pp1n1ppp/2pb4/8/3PB3/2P1P3/PP2N1PP/R1BQK2R b KQkq - 0 11"));
    random_fens.push(Fen::from_str("r3kbnr/1bpnqp2/pp1pp3/7p/3PP1p1/2NBB2P/PPP2PPN/R2Q1RK1 w kq - 0 12"));
    random_fens.push(Fen::from_str("rnbq1rk1/pp2nppp/2p5/3p4/3PPP2/5N2/PP1N2PP/R2QKB1R w KQ - 0 9"));
    random_fens.push(Fen::from_str("rn1q1rk1/pp2nppp/2p5/3pP3/3P1Pb1/5N2/PP1NB1PP/R2QK2R b KQ - 2 10"));
    random_fens.push(Fen::from_str("rn1q1rk1/pp3ppp/2p5/3pPn2/3P1Pb1/5N2/PP1NB1PP/R2QK2R w KQ - 3 11"));
    random_fens.push(Fen::from_str("rn1q1rk1/pp3ppp/2p5/3pP3/Q2P1Pb1/4nN2/PP1NBKPP/R6R b - - 6 12"));
    random_fens.push(Fen::from_str("rn1q1rk1/p4ppp/2p5/1p1pP3/Q2P1Pb1/4nN2/PP1NBKPP/R6R w - - 0 13"));
    random_fens.push(Fen::from_str("r2q1rk1/ppp2ppp/2nb1n2/4p3/4P1b1/2N2NP1/PPP2PBP/R1BQ1RK1 w - - 3 9"));
    random_fens.push(Fen::from_str("r2q1rk1/ppp2ppp/2nb1n2/4p1B1/4P1b1/2N2NP1/PPP2PBP/R2Q1RK1 b - - 4 9"));
    random_fens.push(Fen::from_str("r1bqk2r/p1p2pp1/2pb1n1p/4p1B1/3pP3/3P1N1P/PPP1NPP1/R2QK2R w KQkq - 0 10"));
    random_fens.push(Fen::from_str("r1b1k2r/p1p1qpp1/2pb3p/4p3/3pP3/3P1N1P/PPPQNPP1/R3K2R w KQkq - 2 12"));
    random_fens.push(Fen::from_str("1rb1k2r/p1p1qpp1/2pb3p/4p3/3pP3/P2P1N1P/1PPQNPP1/R3K2R w KQk - 1 13"));
    random_fens.push(Fen::from_str("r2qk2r/pppbnppp/3p1n2/1B2p3/P3P3/2BP1N2/1PP2PPP/R2QK2R b KQkq - 0 9"));
    random_fens.push(Fen::from_str("r3k2r/pppqnppp/1P1p1n2/4p3/4P3/2BP1N2/1PP2PPP/R2QK2R b KQkq - 0 11"));
    random_fens.push(Fen::from_str("r3k2r/pp1qnppp/1p1p1n2/4p3/4P3/2BP1N2/1PP2PPP/R2QK2R w KQkq - 0 12"));
    random_fens.push(Fen::from_str("rnbqk2r/p4pbp/2pp1np1/1p6/3Pp3/1BP1P2P/PP2NPP1/RNBQ1RK1 b kq - 1 9"));
    random_fens.push(Fen::from_str("rnbqk2r/5pbp/2pp1np1/pp6/3Pp3/2P1P2P/PPB1NPP1/RNBQ1RK1 b kq - 1 10"));
    random_fens.push(Fen::from_str("rnbqk2r/5pbp/2pp1np1/1p6/p2Pp3/2P1P2P/PPB1NPP1/RNBQ1RK1 w kq - 0 11"));
    random_fens.push(Fen::from_str("rnbqk2r/5pbp/2pp1np1/1p6/p2Pp3/2P1P1NP/PPB2PP1/RNBQ1RK1 b kq - 1 11"));
    random_fens.push(Fen::from_str("r1bqk2r/1p3ppp/p1n1pn2/2b5/2B5/P3PN2/1P3PPP/RNBQ1RK1 b kq - 0 9"));
    random_fens.push(Fen::from_str("r1bq1rk1/1p2bppp/p1n1pn2/8/1PB5/P3PN2/2Q2PPP/RNB2RK1 b - - 2 11"));
    random_fens.push(Fen::from_str("r1bq1rk1/4bppp/p1n1pn2/1p6/1PB5/P3PN2/2Q2PPP/RNB2RK1 w - - 0 12"));
    random_fens.push(Fen::from_str("r2q1rk1/1b2bppp/p1n1pn2/1p6/1P6/P2BPN2/2Q2PPP/RNB2RK1 w - - 2 13"));
    random_fens.push(Fen::from_str("rn1qkb1r/1p3pp1/p3pn1p/3p1b2/3P4/2N2NP1/PP2PPBP/R1BQ1RK1 w kq - 0 9"));
    random_fens.push(Fen::from_str("r2qkb1r/1p3pp1/p1n1pn1p/3pNb2/Q2P4/2N3P1/PP2PPBP/R1B2RK1 b kq - 3 10"));
    random_fens.push(Fen::from_str("r1bq1rk1/pp2bpp1/2p2n1p/3pnN2/4P3/2N2P2/PPP1BBPP/R2Q1RK1 b - - 1 12"));
    random_fens.push(Fen::from_str("r1bqrnk1/pp3pbp/2p2np1/3ppP2/3PP3/2N3PP/PPP1N1B1/R1BQ1RK1 w - - 0 12"));
    random_fens.push(Fen::from_str("r1bqrnk1/pp3pbp/2p3p1/3npP2/3P4/2N3PP/PPP1N1B1/R1BQ1RK1 w - - 0 13"));
    random_fens.push(Fen::from_str("r1bqk1nr/pp5p/2n2pp1/2b3B1/3p4/3Q1N2/PPP1NPPP/R3KB1R w KQkq - 0 10"));
    random_fens.push(Fen::from_str("r1k4r/pp2ppbp/2p1bnp1/4B3/N3P3/8/PPP2PPP/2KR1B1R b - - 9 11"));
    random_fens.push(Fen::from_str("r1kr4/pp2ppbp/2p1bnp1/4B3/N3P3/8/PPP1BPPP/2KR3R b - - 11 12"));
    random_fens.push(Fen::from_str("rn1q1rk1/ppbb1ppp/4p1n1/2p1P3/P1Np4/1P1P1NP1/1BP2PBP/R2QK2R b KQ - 4 11"));
    random_fens.push(Fen::from_str("rn1q1rk1/ppbb1pp1/4p1np/2p1P3/P1Np4/1P1P1NP1/1BP2PBP/R2QK2R w KQ - 0 12"));
    random_fens.push(Fen::from_str("rn1q1rk1/ppb2pp1/2b1p1np/2p1P3/P1Np4/1P1P1NP1/1BP2PBP/R2Q1RK1 w - - 2 13"));
    random_fens.push(Fen::from_str("r1bqk1nr/1pp2ppp/3p4/p2Pp3/1b6/3PPN2/PP1B1PPP/R2QKB1R w KQkq - 0 9"));
    random_fens.push(Fen::from_str("r1bqk1nr/1pp2ppp/3p4/p2Pp3/1b6/P2PPN2/1P1B1PPP/R2QKB1R b KQkq - 0 9"));
    random_fens.push(Fen::from_str("r1bqk2r/1pp2ppp/3p1n2/p2Pp3/8/P2PPN2/1P1Q1PPP/R3KB1R w KQkq - 1 11"));
    random_fens.push(Fen::from_str("r1bqk2r/1pp2ppp/3p1n2/p2Pp3/4P3/P2P1N2/1P1Q1PPP/R3KB1R b KQkq - 0 11"));
    random_fens.push(Fen::from_str("r1bq1rk1/1pp2ppp/3p1n2/p2Pp3/4P3/P2P1N2/1P1Q1PPP/R3KB1R w KQ - 1 12"));
    random_fens.push(Fen::from_str("r1b1k2r/ppp1bppp/2np1n2/8/8/3P1N2/PPP1BPPP/RNB1K2R w KQkq - 2 9"));
    random_fens.push(Fen::from_str("r1b1k2r/ppp1bppp/2np1n2/8/8/3P1N1P/PPP1BPP1/RNB1K2R b KQkq - 0 9"));
    random_fens.push(Fen::from_str("r3k2r/pppbbppp/2np1n2/8/8/3P1N1P/PPP1BPP1/RNB1K2R w KQkq - 1 10"));
    random_fens.push(Fen::from_str("r3k2r/pppbbppp/2np1n2/8/8/P2P1N1P/1PP1BPP1/RNB1K2R b KQkq - 0 10"));
    random_fens.push(Fen::from_str("2kr3r/pppbbppp/2np1n2/8/8/P2P1N1P/1PP1BPP1/RNB1K2R w KQ - 1 11"));
    random_fens.push(Fen::from_str("2kr3r/pppbbppp/2np1n2/6B1/8/P2P1N1P/1PP1BPP1/RN2K2R b KQ - 2 11"));
    random_fens.push(Fen::from_str("r2qk2r/pp1n1pp1/2pbpn1p/3p1b2/8/1P1P1NPP/PBPNPPB1/R2QK2R w KQkq - 3 9"));
    random_fens.push(Fen::from_str("r2qk2r/pp1n1pp1/2pbpn1p/3p1b2/8/1P1P1NPP/PBPNPPB1/R2Q1RK1 b kq - 4 9"));
    random_fens.push(Fen::from_str("r2q1rk1/pp1n1pp1/2pbpn1p/3p1b2/8/1P1P1NPP/PBPNPPB1/R2Q1RK1 w - - 5 10"));
    random_fens.push(Fen::from_str("r2q1rk1/pp1n1pp1/2pbpn1p/3p1b2/7N/1P1P2PP/PBPNPPB1/R2Q1RK1 b - - 6 10"));
    random_fens.push(Fen::from_str("r2q1rk1/pp1nbppb/2p1pn1p/8/4P2N/1P4PP/PBPN1PB1/R2Q1RK1 w - - 1 13"));
    random_fens.push(Fen::from_str("rn2kbnr/1bp1qp2/pp2p2p/4P1p1/8/3BBN2/PPP2PPP/RN1Q1RK1 b kq - 2 9"));
    random_fens.push(Fen::from_str("rn2k1nr/1bp1qpb1/pp2p2p/4P1p1/8/3BBN2/PPP2PPP/RN1Q1RK1 w kq - 3 10"));
    random_fens.push(Fen::from_str("r3k1nr/1bp1qpb1/ppn1p2p/4P1p1/8/3BBN2/PPPN1PPP/R2Q1RK1 w kq - 5 11"));
    random_fens.push(Fen::from_str("r3k1nr/1bp1qpb1/ppn1p2p/4P1p1/8/2PBBN2/PP1N1PPP/R2Q1RK1 b kq - 0 11"));
    random_fens.push(Fen::from_str("r1bqr1k1/2pp1ppp/p1nb1n2/1p2p3/4P3/PBN2N2/1PPP1PPP/R1BQ1RK1 w - - 3 9"));
    random_fens.push(Fen::from_str("r1bqkb1r/1p3ppp/p1nppn2/8/2B1P3/2N2N2/PP3PPP/R1BQR1K1 w kq - 2 9"));
    random_fens.push(Fen::from_str("r1bqkb1r/1p3ppp/p1nppn2/3N4/2B1P3/5N2/PP3PPP/R1BQR1K1 b kq - 3 9"));
    random_fens.push(Fen::from_str("rnb1kb1r/4qp1p/1pp1pnp1/3pN3/3P4/3BP3/PP3PPP/RNBQK2R w KQkq - 4 9"));
    random_fens.push(Fen::from_str("r4rk1/pp1nq1pp/2p2n2/2bp4/5Bb1/1B2PN2/PPPN1PPP/R2QR1K1 b - - 7 11"));
    random_fens.push(Fen::from_str("r1b2rk1/ppp2pbp/2n3p1/3qp3/8/3PBNP1/PP2PPBP/R2Q1RK1 b - - 1 10"));
    random_fens.push(Fen::from_str("rn2kb1r/pp2pppp/8/2p1N3/5Pq1/8/PPP5/RNBQK2B b Qkq - 1 11"));
    random_fens.push(Fen::from_str("r3k1nr/1p1qppbp/p1n3p1/3p4/Q2P4/2P1P2P/P3NPP1/RNB1K2R w KQkq - 0 12"));
    random_fens.push(Fen::from_str("r2q1rk1/pb1nbppp/1pp1p3/8/2PPB3/1P3N2/PB3PPP/R2Q1RK1 b - - 0 12"));
    random_fens.push(Fen::from_str("r2q1rk1/pppb1ppp/2nb1n2/1B2p3/8/5NN1/PPPP1PPP/R1BQR1K1 w - - 6 9"));
    random_fens.push(Fen::from_str("r2q1rk1/ppp2ppp/2b2n2/4R3/8/6N1/PPPP1PPP/R1BQ2K1 b - - 0 11"));
    random_fens.push(Fen::from_str("r2qkbnr/pp3pp1/2pp3p/4n2b/2B1P3/2P2N1P/PP3PP1/RNBQ1RK1 w kq - 0 9"));
    random_fens.push(Fen::from_str("r2qkb1r/pp3pp1/2p2nbp/4p3/4P3/2P2P1P/PP1NB1P1/R1BQ1RK1 b kq - 0 12"));

    /*
    let fen: &Fen = &random_fens[i];
    let queen: &u64;
    if fen.white_to_move {
        queen = &fen.boards[4];
    } else {
        queen = &fen.boards[10]
    }
    fen.get_possible_moves(queen);

    random_fens[i].get_all_possible_moves();

    */

    let mut durations: [Duration; 100] = [Duration::from_nanos(0); 100];
    for i in 0..100 {
        let time: Instant = Instant::now();
        random_fens[i].in_check();
        durations[i] = time.elapsed();
    }

    let info: (Duration, Duration, Duration) = analyze_durations(&durations);
    let min: Duration = info.0;
    let max: Duration = info.1;
    let average: Duration = info.2;

    println!("Min duration {:?}", min);
    println!("Max duration {:?}", max);
    println!("Average duration {:?}", average);
}


/// The Python module entry point for the `rust_utils` package.
#[pymodule]
fn rust_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FenPy>()?;
    m.add_class::<BotV1Py>()?;
    m.add_function(wrap_pyfunction!(benching, m)?)?;
    m.add_function(wrap_pyfunction!(perft_check, m)?)?;
    Ok(())
}
