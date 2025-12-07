use std::fs::File;
use std::io::{BufRead, BufReader};


#[derive(Clone)]
#[derive(PartialEq)]
enum ManifoldCell {
    TACHYON,
    EMPTY,
    SPLITTER,
    NONE,
}

impl std::fmt::Display for ManifoldCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifoldCell::TACHYON => write!(f,"|"),
            ManifoldCell::EMPTY => write!(f,"."),
            ManifoldCell::SPLITTER => write!(f,"^"),
            ManifoldCell::NONE => write!(f,"_")
        }
    }
    
}


struct ManifoldRow {
    row : Vec<ManifoldCell>
}

impl ManifoldRow {

    fn new(s:&str) -> Self {
        let mut row:Vec<ManifoldCell> = vec![ManifoldCell::NONE;s.len()];

        for (i,c) in s.char_indices() {
            match c {
                '.' => row[i] = ManifoldCell::EMPTY,
                '^' => row[i] = ManifoldCell::SPLITTER,
                'S' => row[i] = ManifoldCell::TACHYON,
                _ => {},
            }
        }

        ManifoldRow { row }
    }

    fn get(&self,i:usize) -> ManifoldCell {
        if i >= self.row.len() {
            return ManifoldCell::NONE
        }

        self.row[i].clone()
    }

    fn beam_down(&mut self, prev_row : &ManifoldRow) -> u32 {

        let l = self.row.len();

        if l != prev_row.row.len() {
            panic!("something went wrong: there are two rows with different lengths");
        }

        let mut splits : u32 = 0;

        for i in 0..l {
            if prev_row.get(i) == ManifoldCell::TACHYON {

                match self.get(i) {
                    ManifoldCell::EMPTY => self.row[i] = ManifoldCell::TACHYON,
                    ManifoldCell::SPLITTER => {


                        //asumes no two splitters are next to each other (seems to be correct for the current input)
                        if let Some(j) = i.checked_sub(1) {

                            self.row[j] = ManifoldCell::TACHYON;
                        }

                        if i+1 < l {

                            self.row[i+1] = ManifoldCell::TACHYON;
                        }

                        splits += 1;
                    
                    }

                    _ => ()
                }

            }
        }

        splits
    }

    fn _count_beams(&self) -> usize {
        self.row.iter().filter(|c| **c == ManifoldCell::TACHYON).count()
    }
}

impl std::fmt::Display for ManifoldRow {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f,"{}",self.row.iter().map(|e| e.to_string()).collect::<Vec<_>>().join(""))
    }
}


pub fn part1(filepath:&str,verbose:bool) -> u32 {
    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {eprintln!("could not open file {filepath}"); return 0},
    };

    let mut reader = BufReader::new(file);

    let mut first_line = String::new();
    reader.read_line(&mut first_line).unwrap();
    let mut prev_row = ManifoldRow::new(&first_line.trim());

    let mut tot_splits = 0;

    for line in reader.lines() {
        let line = line.unwrap();

        let mut current_row = ManifoldRow::new(&line);

        let r = current_row.beam_down(&prev_row);

        tot_splits += r;

        if verbose {
            println!("{},{}",current_row,r);
        }

        prev_row = current_row;
    }

    tot_splits
}