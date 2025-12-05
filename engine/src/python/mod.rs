use std::time::Duration;
use pyo3::prelude::*;

use crate::bots::alphaengine::AlphaEngine;
use crate::bots::dumbengine::DumbEngine;
use crate::bots::randomengine::RandomEngine;
use crate::bots::simpleengine::SimpleEngine;
use crate::bots::sortedengine::SortedEngine;

use crate::fenlib::fen::*;
use crate::fenlib::tests;
use crate::bots::run_matchup;
use crate::bots::matchup::Engine;
use crate::utils::*;
use crate::parsing;

#[pyfunction]
pub fn rust_access() {
}

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
        self.fen.get_legal_moves_for_tile(tile)
    }

    /// Returns true if the current player is in check.
    ///
    /// This may mutate internal state during the computation.
    pub fn in_check(&mut self) -> bool {
        self.fen.player_in_check(true)
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
        self.fen.get_legal_moves_lan()
    }

    /// Checks if the game has ended, and returns the result.
    pub fn game_ended(&mut self) -> String {
        self.fen.game_outcome_str()
    }

    /// Returns true if it's white's turn to move, false if it's black's.
    pub fn white_to_move(&self) -> bool {
        self.fen.white_to_move()
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct DumbEnginePy {
    engine: DumbEngine
}

#[pymethods]
impl DumbEnginePy {
    #[staticmethod]
    pub fn new_game(fen_str: &str) -> Self {
        Self { engine: DumbEngine::new_game(fen_str) }
    }

    pub fn select_move(&mut self, time_per_move_milli: u64) -> String {
        let time_per_move = Duration::from_millis(time_per_move_milli);
        let move1: Move = self.engine.select_move(time_per_move);

        parsing::move_to_lan(&move1)
    }

    pub fn apply_move(&mut self, move_lan: &str) {
        let move1: Move = parsing::lan_to_move(move_lan);
        self.engine.apply_move(move1);
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct RandomEnginePy {
    engine: RandomEngine
}

#[pymethods]
impl RandomEnginePy {
    #[staticmethod]
    pub fn new_game(fen_str: &str) -> Self {
        Self { engine: RandomEngine::new_game(fen_str) }
    }

    pub fn select_move(&mut self, time_per_move_milli: u64) -> String {
        let time_per_move = Duration::from_millis(time_per_move_milli);
        let move1: Move = self.engine.select_move(time_per_move);

        parsing::move_to_lan(&move1)
    }

    pub fn apply_move(&mut self, move_lan: &str) {
        let move1: Move = parsing::lan_to_move(move_lan);
        self.engine.apply_move(move1);
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SimpleEnginePy {
    engine: SimpleEngine
}

#[pymethods]
impl SimpleEnginePy {
    #[staticmethod]
    pub fn new_game(fen_str: &str) -> Self {
        Self { engine: SimpleEngine::new_game(fen_str) }
    }

    pub fn select_move(&mut self, time_per_move_milli: u64) -> String {
        let time_per_move = Duration::from_millis(time_per_move_milli);
        let move1: Move = self.engine.select_move(time_per_move);

        parsing::move_to_lan(&move1)
    }

    pub fn apply_move(&mut self, move_lan: &str) {
        let move1: Move = parsing::lan_to_move(move_lan);
        self.engine.apply_move(move1);
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct AlphaEnginePy {
    engine: AlphaEngine
}

#[pymethods]
impl AlphaEnginePy {
    #[staticmethod]
    pub fn new_game(fen_str: &str) -> Self {
        Self { engine: AlphaEngine::new_game(fen_str) }
    }

    pub fn select_move(&mut self, time_per_move_milli: u64) -> String {
        let time_per_move = Duration::from_millis(time_per_move_milli);
        let move1: Move = self.engine.select_move(time_per_move);

        parsing::move_to_lan(&move1)
    }

    pub fn apply_move(&mut self, move_lan: &str) {
        let move1: Move = parsing::lan_to_move(move_lan);
        self.engine.apply_move(move1);
    }
}

#[pyclass]
#[derive(Debug, Clone)]
pub struct SortedEnginePy {
    engine: SortedEngine
}

#[pymethods]
impl SortedEnginePy {
    #[staticmethod]
    pub fn new_game(fen_str: &str) -> Self {
        Self { engine: SortedEngine::new_game(fen_str) }
    }

    pub fn select_move(&mut self, time_per_move_milli: u64) -> String {
        let time_per_move = Duration::from_millis(time_per_move_milli);
        let move1: Move = self.engine.select_move(time_per_move);

        parsing::move_to_lan(&move1)
    }

    pub fn apply_move(&mut self, move_lan: &str) {
        let move1: Move = parsing::lan_to_move(move_lan);
        self.engine.apply_move(move1);
    }
}

#[pyfunction]
pub fn perft_check(max_depth: usize, fen_str: &str, per_move: bool) {
    let count: usize = tests::perft(max_depth, fen_str, per_move);
    println!("Checked a total of {:?} moves", count);
}

#[pyfunction]
pub fn move_gen_perft_py() {
    tests::move_gen_perft();
}

#[pyfunction]
pub fn moves_per_second_perft_py() {
    tests::moves_per_second_perft();
}

#[pyfunction]
pub fn run_matchup_py(print_matches: bool, time_per_move_milli: u64, number_of_games: usize) {
    let time_per_move: Duration = Duration::from_millis(time_per_move_milli);
    run_matchup(print_matches, time_per_move, number_of_games);
}

/// The Python module entry point for the `engine` package.
#[pymodule]
fn engine(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(rust_access, m)?)?;

    m.add_class::<FenPy>()?;
    m.add_class::<DumbEnginePy>()?;
    m.add_class::<RandomEnginePy>()?;
    m.add_class::<SimpleEnginePy>()?;
    m.add_class::<AlphaEnginePy>()?;
    m.add_class::<SortedEnginePy>()?;
    m.add_function(wrap_pyfunction!(move_gen_perft_py, m)?)?;
    m.add_function(wrap_pyfunction!(perft_check, m)?)?;
    m.add_function(wrap_pyfunction!(moves_per_second_perft_py, m)?)?;
    m.add_function(wrap_pyfunction!(run_matchup_py, m)?)?;
    Ok(())
}
