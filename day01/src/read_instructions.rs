use std::fs::File;
use std::io::BufReader;

use std::error::Error;

// trait -> BufReader can use .lines
use std::io::BufRead;


pub enum Direction {
    L,
    R
}


pub struct Instruction {
    direction: Direction,
    amount: u32
}

impl Instruction {
    pub fn to_string(&self) -> String {
        match self.direction{
            Direction::L => return format!("-{:03}",self.amount),
            Direction::R => return format!(" {:03}",self.amount)
        }
    }

    pub fn amount(&self) -> u32 {
        self.amount
    }

    pub fn direction(&self) -> &Direction {
        &self.direction
    }
}

pub fn read_file(filepath: String) -> Result<Vec<Instruction>,Box<dyn Error>> {
    // Open the file (? -> throw error when problem)
    let file = File::open(filepath)?;

    // Wrap it in a buffered reader
    let reader = BufReader::new(file);

    let mut result : Vec<Instruction> = Vec::new();

    let mut i = 0;

    // Iterate over lines
    for line in reader.lines() {
        let line = line?; // Handle Result<String>
        //println!("{}", line); // Do whatever you want with each line

        let direction = match line.chars().next() {
            Some('L') => Direction::L,
            Some('R') => Direction::R,
            Some(c) => return Err(format!("Not a valid direction at line {}, {}",i,c).into()),
            _ => return Err(format!("Has None at line {}",i).into())
        };

        let num:u32 = match line.chars().skip(1).collect::<String>().parse(){
            Ok(n) => n,
            _ => return Err(format!("Could not ectract number from line {}",i).into())
        };

        i+=1;

        result.push(Instruction {direction, amount: num});
    }

    Ok(result)


}