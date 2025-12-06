use std::fs::File;
use std::io::{BufRead, BufReader};


pub struct RollLayout {
    layout: Vec<Vec<bool>>
}


impl RollLayout {
    pub fn get(&self,i:i32,j:i32) -> bool {
        
        if i < 0 || j < 0 {
            return false
        }

        let i = i as usize;
        let j = j as usize;

        if i >= self.layout.len() {
            return false
        }

        let row = &self.layout[i];

        if j >= row.len() {
            return false
        }

        return row[j as usize]
    }

    pub fn new() -> Self {
        Self {layout : Vec::new()}
    }

    pub fn add_row(&mut self,s:&str) {
        
        let mut new_row: Vec<bool> = vec![false;s.len()];

        for (i,c) in s.char_indices() {
            if c == '@' {
                new_row[i] = true;
            }
        }

        self.layout.push(new_row);
    }


    pub fn part1(&self) -> u32 {
        let mut result: u32 = 0;

        for (i,row) in self.layout.iter().enumerate() {
            for (j,val) in row.iter().enumerate() {
                if *val {

                    let mut buren = 0;

                    let i = i as i32;
                    let j = j as i32;

                    for di in -1i32..=1 {
                        for dj in -1i32..=1 {
                            if self.get( i + di, j + dj) {
                                buren += 1;
                            }
                        }
                    }

                    // 4 buren + zichzelf
                    if buren <= 4 {
                        result += 1;
                    }
                }
            }
        }


        result
    }

    fn remove_posible_rolls(&mut self) -> u32{
        let mut result: u32 = 0;

        // note to self: can't user self.layout.iter_mut().enumerate() here cause changing the values requires &mut while self.get requires &self and I don't want to rewrite this
        for i in 0..self.layout.len() {
            for j in 0..self.layout[i].len() {
                if self.layout[i][j] {

                    let mut buren = 0;

                    let ii = i as i32;
                    let ji = j as i32;

                    for di in -1i32..=1 {
                        for dj in -1i32..=1 {
                            if self.get( ii + di, ji + dj) {
                                buren += 1;
                            }
                        }
                    }

                    // 4 buren + zichzelf
                    if buren <= 4 {
                        result += 1;
                        self.layout[i][j] = false;
                    }
                }
            }
        }


        result
    }


}


pub fn read_file(filepath:&str) -> RollLayout{
    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {return RollLayout::new() },
    };

    let reader = BufReader::new(file);

    let mut result = RollLayout::new();

    for line in reader.lines(){
        let line = match line {
            Ok(l) => l,
            Err(e) => {eprintln!("error reading line {}",e); return RollLayout::new()}
        };

        result.add_row(&line);

    }

    result
}

pub fn part2(rl:&mut RollLayout, verbose:bool) -> u32{

    let mut result = 0;
    let mut r = 1;

    while r != 0 {
        r = rl.remove_posible_rolls();
        result += r;
        if verbose {
            println!("removed {} rolls", r);
        }
    }

    return result
}