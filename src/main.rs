use std::ops::Add;

extern crate bit_vec;
extern crate bit_set;
extern crate time;

#[macro_use]
extern crate indoc;

#[macro_use]
extern crate structopt;

//TODO: colors and key events instead of typing commands
//extern crate termion;
//use termion::{color, style};

use time::{Duration, PreciseTime};

mod config;
use config::*;

mod sudoku;
use sudoku::*;

mod solve;
use solve::*;

fn main() {
    let options = Options::init();
    if options.debug {println!("{:?}", options);}
    match options.cmd {
        Command::Generate{size, difficulty: _} => {
            let sudoku = Sudoku::new(size);
            println!("{}", sudoku.to_string());
        },
        Command::Play{size, difficulty: _} => {
            let sudoku = Sudoku::new(size);
            println!("{}", sudoku.to_string());
        },
        Command::Solve{ref sudokus} => {
            let total_duration = Duration::nanoseconds(0);
            let mut durations = Vec::with_capacity(sudokus.len());
            for sudoku in sudokus {
                println!("Solving this one:\n\r{}", sudoku.to_string());
                let solve = Solve::new(sudoku.clone());
                let start = PreciseTime::now();
                let solveds = Solve::solve(solve);
                let end = PreciseTime::now();
                let duration = start.to(end);
                println!("{} seconds taken to solve the sudoku. {} solutions found", duration, solveds.len());
                total_duration.add(duration);
                durations.push(duration);
                for (i, solved) in solveds.iter().enumerate() {
                    println!("Solution {}:\n\r{}", i, Solve::to_sudoku(&solved).to_string());
                }
            }
            println!("{} sudokus solved in {} seconds, average of {}", sudokus.len(), total_duration, total_duration / sudokus.len() as i32)
        },
    }
}

#[cfg(test)]
mod tests {
    use super::sudoku::*;
    #[test]
    fn it_works() {
        let sudoku_1 = indoc!("
            260 050 038
            400 007 006
            000 010 000

            090 000 000
            301 020 607
            000 000 040

            000 060 000
            700 800 009
            950 070 013
        ");
        let sudoku_2 = indoc!("
            +---+---+---+
            |000|803|000|
            |009|000|300|
            |081|645|270|
            +---+---+---+
            |402|000|608|
            |003|080|400|
            |708|000|105|
            +---+---+---+
            |064|539|710|
            |007|000|900|
            |000|104|000|
            +---+---+---+
        ");

        let sudoku_1_parsed = Sudoku::from(SudokuMaxNumber::Nr9, sudoku_1).unwrap();
        println!("{}", sudoku_1_parsed.to_string());

        let sudoku_2_parsed = Sudoku::from(SudokuMaxNumber::Nr9, sudoku_2).unwrap();
        println!("{}", sudoku_2_parsed.to_string());
    }
}