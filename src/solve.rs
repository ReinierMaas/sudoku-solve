use std::sync::Arc;

use bit_vec::BitVec;
use bit_set::BitSet;

use sudoku::{Sudoku, SudokuMaxNumber};

#[derive(Debug)]
pub enum Solve {
    Sudoku(Sudoku),
    Next {prev: Arc<Solve>, step: Step},
}

#[derive(Debug)]
pub struct Step {
    v: usize,
    x: usize,
    y: usize,
    horizontals: Vec<BitSet>,
    verticals: Vec<BitSet>,
    squares: Vec<BitSet>,
}

impl Solve {
    /// Take ownership so the sudoku will stay the same during solving
    pub fn new(sudoku: Sudoku) -> Arc<Self> {
        Arc::new(Solve::Sudoku(sudoku))
    }

    pub fn to_sudoku(solve: &Arc<Self>) -> Sudoku {
        let smn = Self::smn(&solve);
        let mut sudoku = Sudoku::new(smn);
        let side_length = smn.side_length();
        for y in 0..side_length {
            for x in 0..side_length {
                sudoku.set(x, y, Self::get(solve, x, y));
            }
        }
        sudoku
    }

    pub fn solve(solve: Arc<Self>) -> Vec<Arc<Self>> {
        let smn = Self::smn(&solve);
        Self::solve_rec(solve, smn.side_parts(), smn.side_length())
    }

    fn solve_rec(solve: Arc<Self>, side_parts: usize, side_length: usize) -> Vec<Arc<Self>> {
        if Self::done(&solve, side_length) {
            vec![solve]
        } else {
            Self::generate_next(solve, side_parts, side_length).iter()
                .flat_map(|s| Self::solve_rec(s.clone(), side_parts, side_length)).collect()
        }
    }

    /// Return the current value on the location, traversing all taken steps
    fn get(solve: &Arc<Self>, x: usize, y: usize) -> usize {
        match **solve {
            Solve::Sudoku(ref sudoku) => sudoku.get(x,y),
            Solve::Next{ref prev, step: Step{v: vs, x: xs, y: ys, ..}} => {
                if x == xs && y == ys {
                    vs
                } else {
                    Self::get(&prev, x, y)
                }
            },
        }
    }

    fn smn(solve: &Arc<Self>) -> SudokuMaxNumber {
        match **solve {
            Solve::Sudoku(ref sudoku) => sudoku.smn,
            Solve::Next{ref prev, ..} => Solve::smn(&prev),
        }
    }

    fn generate_next(solve: Arc<Self>, side_parts: usize, side_length: usize) -> Vec<Arc<Self>> {
        let horizontals;
        let verticals;
        let squares;
        match *solve {
            Solve::Sudoku(ref sudoku) => {
                horizontals = sudoku.horizontals();
                verticals = sudoku.verticals();
                squares = sudoku.squares();
            },
            Solve::Next{prev: _, step: Step {v: _, x: _, y: _, horizontals: ref prev_horizontals, verticals: ref prev_verticals, squares: ref prev_squares}} => {
                horizontals = prev_horizontals.clone();
                verticals = prev_verticals.clone();
                squares = prev_squares.clone();
            },
        }
        Self::least_options(solve, side_parts, side_length, horizontals, verticals, squares)
    }

    fn least_options(solve: Arc<Self>, side_parts: usize, side_length: usize, horizontals: Vec<BitSet>, verticals: Vec<BitSet>, squares: Vec<BitSet>) -> Vec<Arc<Self>> {
        let mut minimum = None; // Some(x,y,bitset)
        for x in 0..side_length {
            for y in 0..side_length {
                if Self::get(&solve, x, y) != 0 {continue}
                let horizontal_set = &horizontals[y];
                let vertical_set = &verticals[x];
                let square_set = &squares[Solve::square_index(side_parts, x, y)];
                let mut local_set = horizontal_set.clone();
                local_set.union_with(&vertical_set);
                local_set.union_with(&square_set);
                let mut all_set = BitSet::from_bit_vec(BitVec::from_elem(side_length + 1, true));
                all_set.remove(0);
                local_set.symmetric_difference_with(&all_set);
                minimum = match minimum {
                    None => Some((x,y,local_set)),
                    Some((xm, ym, bitset)) => {
                        if bitset.len() > local_set.len() {
                            Some((x,y,local_set))
                        } else {
                            Some((xm, ym, bitset))
                        }
                    },
                }
            }
        }
        // Minimal set of optional paths found
        match minimum {
            None => vec![], // No options returned
            Some((x,y,bitset)) => {
                let mut nexts = Vec::with_capacity(bitset.len());
                for val in bitset.iter() {
                    let mut hs = horizontals.clone();
                    hs[y].insert(val);
                    let mut vs = verticals.clone();
                    vs[x].insert(val);
                    let mut sqs = squares.clone();
                    sqs[Solve::square_index(side_parts, x, y)].insert(val);
                    nexts.push(Arc::new(Solve::Next {
                        prev: solve.clone(),
                        step: Step {
                            v: val,
                            x: x,
                            y: y,
                            horizontals: hs,
                            verticals: vs,
                            squares: sqs,
                        },
                    }));
                }
                nexts
            }, // Optional paths returned
        }
    }
    fn square_index(side_parts: usize, x: usize, y: usize) -> usize {
        let xp = x / side_parts;
        let yp = y / side_parts;
        yp * side_parts + xp
    }

    fn done(solve: &Arc<Self>, side_length: usize) -> bool {
        for y in 0..side_length {
            for x in 0..side_length {
                if Self::get(&solve, x, y) == 0 {
                    return false
                }
            }
        }
        // There are no empty squares so solving is done
        true
    }
}