use std;
use std::str::FromStr;

use bit_set::BitSet;

use config::Options;

#[derive(StructOpt, Debug, Copy, Clone)]
pub enum SudokuMaxNumber {
    Nr4,
    Nr9,
    Nr16,
}

impl FromStr for SudokuMaxNumber {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "4x4" => Ok(SudokuMaxNumber::Nr4),
            "9x9" => Ok(SudokuMaxNumber::Nr9),
            "16x16" => Ok(SudokuMaxNumber::Nr16),
            _ => Err("Try one of these: '4x4', '9x9' or '16x16'"),
        }
    }
}

impl SudokuMaxNumber {
    pub fn side_parts(&self) -> usize {
        match self {
            &SudokuMaxNumber::Nr4 => 2,
            &SudokuMaxNumber::Nr9 => 3,
            &SudokuMaxNumber::Nr16 => 4,
        }
    }

    pub fn side_length(&self) -> usize {
        self.side_parts().pow(2)
    }

    pub fn lines_per_part (&self) -> usize {
        self.side_length() / self.side_parts()
    }

    pub fn nr_fields(&self) -> usize {
        self.side_parts().pow(4)
    }
}

#[derive(Clone, Debug)]
pub struct Sudoku {
    pub smn: SudokuMaxNumber,
    memory: Vec<usize>,
}

impl FromStr for Sudoku {
    type Err = &'static str;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Some(sudoku) = Sudoku::from(SudokuMaxNumber::Nr4, s) {
            Ok(sudoku)
        } else if let Some(sudoku) = Sudoku::from(SudokuMaxNumber::Nr9, s) {
            Ok(sudoku)
        } else if let Some(sudoku) = Sudoku::from(SudokuMaxNumber::Nr16, s) {
            Ok(sudoku)
        } else {
            Err(indoc!("
            Try something like this one:
            260 050 038
            400 007 006
            000 010 000

            090 000 000
            301 020 607
            000 000 040

            000 060 000
            700 800 009
            950 070 013
        "))
        }
    }
}

impl Sudoku {
    pub fn new(smn: SudokuMaxNumber) -> Sudoku {
        let mem = std::iter::repeat(0).take(smn.nr_fields() as usize).collect();
        Sudoku{
            smn: smn,
            memory: mem,
        }
    }

    pub fn from(smn: SudokuMaxNumber, sudoku_str: &str) -> Option<Sudoku> {
        let mut discarded = 0;
        let mut incorrect_range = Vec::new();
        let s: Vec<_> = sudoku_str
            .chars()
            .map(|c| c.to_digit(16))
            .filter(|n| if n.is_some() {true} else {discarded += 1; false})
            .map(|n| n.unwrap() as usize)
            .filter(|n| if *n <= smn.side_length() {true} else {incorrect_range.push(*n); false})
            .collect();
        if Options::current().debug {
            println!("characters discarded: {}", discarded);
            println!("numbers in incorrect_range: {:?}", incorrect_range);
        }
        if s.len() != smn.nr_fields() as usize {
            None
        } else {
            Some(Sudoku {
                smn: smn,
                memory: s,
            })
        }
    }

    pub fn get(&self, x: usize, y: usize) -> usize {
        self.memory[x + y * self.smn.side_length()]
    }
    pub fn set(&mut self, x: usize, y: usize, value: usize) {
        self.memory[x + y * self.smn.side_length()] = value;
    }

    fn header(&self) -> &'static str {
        match self.smn {
            SudokuMaxNumber::Nr4 => "+--+--+",
            SudokuMaxNumber::Nr9 => "+---+---+---+",
            SudokuMaxNumber::Nr16 => "+-----------+-----------+-----------+-----------+",
        }
    }

    fn line(&self, line_nr: usize) -> String {

        let line = &self.memory[line_nr * self.smn.side_length()..];
        match self.smn {
            SudokuMaxNumber::Nr4 => {
                format!("|{}{}|{}{}|", line[0], line[1], line[2], line[3])
            },
            SudokuMaxNumber::Nr9 => {
                format!("|{}{}{}|{}{}{}|{}{}{}|", line[0], line[1], line[2], line[3], line[4], line[5], line[6], line[7], line[8])
            },
            SudokuMaxNumber::Nr16 => {
                format!("|{:>2.} {:>2.} {:>2.} {:>2.}|{:>2.} {:>2.} {:>2.} {:>2.}|{:>2.} {:>2.} {:>2.} {:>2.}|{:>2.} {:>2.} {:>2.} {:>2.}|",
                    line[0], line[1], line[2], line[3], line[4], line[5], line[6], line[7], line[8], line[9], line[10], line[11], line[12], line[13], line[14], line[15])
            },
        }
    }

    pub fn to_string(&self) -> String {
        let sp = self.smn.side_parts();
        let lpp = self.smn.lines_per_part();
        let mut output = String::new();
        for sp_i in 0..sp {
            output += self.header();
            output += "\n";
            for line_nr in 0..lpp {
                output += self.line(sp_i * lpp + line_nr).as_str();
                output += "\n";
            }
        }
        output += self.header();
        output
    }

    pub fn horizontals(&self) -> Vec<BitSet> {
        let sl = self.smn.side_length();
        let mut bitsets = Vec::with_capacity(sl);
        for y in 0..sl {
            let mut bitset = BitSet::with_capacity(sl + 1);
            for x in 0..sl {
                bitset.insert(self.get(x,y));
            }
            bitset.remove(0);
            bitsets.push(bitset);
        }
        bitsets
    }
    pub fn verticals(&self) -> Vec<BitSet> {
        let sl = self.smn.side_length();
        let mut bitsets = Vec::with_capacity(sl);
        for x in 0..sl {
            let mut bitset = BitSet::with_capacity(sl + 1);
            for y in 0..sl {
                bitset.insert(self.get(x,y));
            }
            bitset.remove(0);
            bitsets.push(bitset);
        }
        bitsets
    }
    pub fn squares(&self) -> Vec<BitSet> {
        let sp = self.smn.side_parts();
        let sl = self.smn.side_length();
        let mut bitsets = Vec::with_capacity(sl);
        for yp in 0..sp {
            for xp in 0..sp {
                let mut bitset = BitSet::with_capacity(sl + 1);
                for y in 0..sp {
                    for x in 0..sp {
                        bitset.insert(self.get(xp*sp+x,yp*sp+y));
                    }
                }
                bitset.remove(0);
                bitsets.push(bitset);
            }
        }
        bitsets
    }
}