use std::sync::{Arc, RwLock};
use structopt::StructOpt;

use sudoku::{Sudoku, SudokuMaxNumber};

thread_local! {
    static CURRENT_CONFIG: RwLock<Arc<Options>> = RwLock::new(Default::default());
}

#[derive(StructOpt, Debug)]
#[structopt(name = "sudoku", about = "generate, play and solve sudokus")]
pub struct Options {
    #[structopt(subcommand)]
    pub cmd: Command,
    /// Activate debug mode
    #[structopt(short = "d", long = "debug")]
    pub debug: bool,
}
#[derive(StructOpt, Debug)]
pub enum Command {
    #[structopt(name = "generate", about = "generate a random sudoku")]
    Generate {
        /// Size of the playing field (i.e. 4x4, 9x9 or 16x16)
        #[structopt(short = "s", long = "size", default_value = "9x9")]
        size: SudokuMaxNumber,
        /// Difficulty of the sudoku
        #[structopt(short = "d", long = "difficulty", default_value = "0")]
        difficulty: u8,
    },
    #[structopt(name = "play", about = "play a random sudoku")]
    Play {
        /// Size of the playing field (i.e. 4x4, 9x9 or 16x16)
        #[structopt(short = "s", long = "size", default_value = "9x9")]
        size: SudokuMaxNumber,
        /// Difficulty of the sudoku
        #[structopt(short = "d", long = "difficulty", default_value = "0")]
        difficulty: u8,
    },
    #[structopt(name = "solve", about = "solve a provided sudoku")]
    Solve {
        /// Sudokus to solve
        #[structopt(raw(required = "true"))]
        sudokus: Vec<Sudoku>,
    },
}

impl Default for Options {
    fn default() -> Options {
        Options {
            cmd: Command::Solve{sudokus: Vec::new()},
            debug: false,
        }
    }
}

impl Options {
    pub fn init() -> Arc<Self> {
        Options::from_args().make_current();
        Options::current()
    }
    pub fn current() -> Arc<Self> {
        CURRENT_CONFIG.with(|c| c.read().unwrap().clone())
    }
    pub fn make_current(self) {
        CURRENT_CONFIG.with(|c| *c.write().unwrap() = Arc::new(self))
    }
}

