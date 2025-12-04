use std::fs::File;
use std::io::{BufRead, BufReader};


fn main() {

    //get filename from arguments
    let arg: Vec<String> = std::env::args().collect();

    //note arg[0] is program name
    if arg.len() < 3 {
        panic!("You need to pass the input filepath and part as arguments. -- filepath part(1/2) verbose(y/n)")
    }

    let filepath = &arg[1];

    let part = &arg[2];


    let banks = read_file(&filepath);

    let mut verbose = false;

    if (arg.len() > 3 && arg[3] == "y") {
        verbose = true;
    }

    if part == "1" {
        let result = part1(&banks, verbose);
        println!("part1: {}", result);
    }

    if part == "2" {
        let result = part2(&banks, verbose);
        println!("part2: {}", result);
    }


    println!("Hello, world!");
}

struct Bank {
    joltages: Vec<u8>
}

impl Bank {
    fn new(s:&String) -> Self {
        let mut joltages: Vec<u8> = vec![0;s.len()];

        for (i,c) in s.char_indices() {
            joltages[i] = match c.to_digit(10) {
                Some(a) => {a as u8},
                None => {eprintln!("error"); 0},
            }
        };

        return Bank { joltages }
    }

    fn to_string(&self) -> String{
        format!("{:?}",self.joltages)
    }

    fn part1(&self) -> u8 {

        let mut max1 : u8 = 0;
        let mut max2 : u8 = 0;

        let mut i1 : usize = 0;
        let mut i2 : usize = 0;

        let l = self.joltages.len();

        //don't use last number in loop for max number
        for i in 0..(l-1){
            let a = self.joltages[i];
            if a > max1 {
                i1 = i;
                max1 = a;
            }
        }

        for i in (i1+1)..l {
            let a = self.joltages[i];
            if a > max2 {
                i2 = i;
                max2 = a;
            }
        }

        max1*10+max2

    }

    fn part2(&self,istart:usize,n_still_to_do:usize) -> u64 {

        if n_still_to_do <= 0 {
            return 0
        }
        
        let mut max : u64 = 0;
        let mut imax = 0;

        let l = self.joltages.len() - n_still_to_do + 1;

        for i in istart..l {
            let a = self.joltages[i] as u64;
            if a > max {
                max = a;
                imax = i;
            }
        }


        let r = self.part2(imax+1,n_still_to_do-1);

        return max * 10u64.pow((n_still_to_do as u32)-1) + r


    }
}


fn read_file( filepath: &str) -> Vec<Bank> {
    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {return Vec::new()},
    };

    let reader = BufReader::new(file);

    let mut result: Vec<Bank> = Vec::new();

    for line in reader.lines(){
        let line = match line {
            Ok(l) => l,
            Err(e) => {eprintln!("error reading line {}",e); return Vec::new()}
        };

        result.push(Bank::new(&line));
    }

    return result

}

fn part1(banks: &Vec<Bank>, verbose:bool) -> u32 {
    let mut result = 0;

    for bank in banks {
        let r = bank.part1() as u32;

        if verbose {
            println!("{} -> {}", bank.to_string(),r);
        }
        result += r;
    }

    result
}

fn part2 (banks: &Vec<Bank>, verbose:bool) -> u64 {
    let mut result = 0;

    for bank in banks {
        let r = bank.part2(0,12);

        if verbose {
            println!("{} -> {}", bank.to_string(),r);
        }
        result += r;
    }

    result
}
