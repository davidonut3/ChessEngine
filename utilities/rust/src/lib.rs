use pyo3::prelude::*;
use fenlib::Fen;
use botv1::BotV1;

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


/// The Python module entry point for the `rust_utils` package.
#[pymodule]
fn rust_utils(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<FenPy>()?;
    m.add_class::<BotV1Py>()?;
    Ok(())
}
