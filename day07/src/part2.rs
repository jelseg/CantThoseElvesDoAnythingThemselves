use std::fs::File;
use std::io::{BufRead, BufReader};
use std::collections::BTreeMap;

#[derive(Clone)]
#[derive(PartialEq)]
enum ManifoldCell {
    EMPTY,
    SPLITTER,
    START,
    NONE,
}

impl std::fmt::Display for ManifoldCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ManifoldCell::EMPTY => write!(f,"."),
            ManifoldCell::SPLITTER => write!(f,"^"),
            ManifoldCell::START => write!(f,"S"),
            ManifoldCell::NONE => write!(f,"_")
        }
    } 
}

struct Manifold {
    cells:Vec<Vec<ManifoldCell>>
}

impl Manifold {
    fn new() -> Self {
        Manifold { cells: Vec::new() }
    }

    fn add_row(&mut self, s: &str) {

        let mut this_row : Vec<ManifoldCell> = vec![ManifoldCell::NONE;s.len()];

        for (i,c) in s.char_indices() {
            match c {
                '.' => this_row[i] = ManifoldCell::EMPTY,
                '^' => this_row[i] = ManifoldCell::SPLITTER,
                'S' => this_row[i] = ManifoldCell::START,
                _ => ()
            }
        }

        self.cells.push(this_row);
    }

    fn move_down(&self,i:usize,j:usize, verbose:bool, already_found : &mut BTreeMap<(usize, usize), u64>) -> u64 {
        
        let mut i_now = i;

        while i_now < self.cells.len() {

            match self.cells[i_now][j] {
                ManifoldCell::SPLITTER => {
                    if verbose {println!("splitting at ({},{})",i_now,j);}

                    if already_found.contains_key(&(i_now,j)) {
                        return *already_found.get(&(i_now,j)).unwrap();
                    }

                    let left = self.move_down(i_now, j-1,verbose, already_found);
                    let right = self.move_down(i_now, j+1,verbose,already_found);

                    let r =  left + right;

                    already_found.insert((i_now,j), r);

                    return r
                },
                _ => ()
            }

            i_now += 1;
        }

        1

    }

    fn get_start(&self) -> (usize,usize) {
        let i:usize = 0;
        for (j,v) in self.cells[i].iter().enumerate() {
            if *v == ManifoldCell::START {
                return (i,j)
            }
        }

        (0,0)
    }
}




pub fn part2(filepath:&str,verbose:bool) -> u64{
    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {eprintln!("could not open file {filepath}"); return 0},
    };

    let reader = BufReader::new(file);

    let mut manifold = Manifold::new();

    for line in reader.lines() {
        let line = line.unwrap();

        manifold.add_row(&line);
    }

    let (i,j) = manifold.get_start();

    let mut already_found : BTreeMap<(usize, usize), u64> = BTreeMap::new();

    manifold.move_down(i, j,verbose, &mut already_found)
}