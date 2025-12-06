use std::fs::File;
use std::io::{BufRead, BufReader};

enum Operator {
    PLUS,
    MULT,
    NONE
}

struct MathProblem {
    numbers: Vec<u32>,
    operator: Operator,
    col_start: usize
}

impl MathProblem {
    fn new(col_start:usize, operator_char:char) -> Self {

        let operator = match operator_char {
            '+' => Operator::PLUS,
            '*' => Operator::MULT,
            a => {eprintln!("could not parse {} as operator", a); Operator::NONE}
        };

        MathProblem { numbers: Vec::new(), operator, col_start }
    }

    fn to_string(&self) -> String {
        self.numbers.iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(match self.operator {
                Operator::PLUS => "+",
                Operator::MULT => "x",
                _ => { " "},
            })
    }

    fn calculate(&self) -> u64 {
        match self.operator {
            Operator::PLUS => self.numbers.iter().map(|&n| n as u64).sum(),
            Operator::MULT => self.numbers.iter().map(|&n| n as u64).product(),
            _ => {eprintln!("mathproblem has no operator"); 0}
        }
    }

    fn set_n_numbers(&mut self, n: usize) {
        self.numbers = vec![0;n];
    }

    fn add_digit(&mut self, digit_char : char, pos_in_line: usize) {

        let digit : u32 = match digit_char.to_digit(10) {
            Some(a) => {a},
            None => {eprintln!("could not parse {digit_char} to digit"); return},
        };

        let i_number = pos_in_line - self.col_start;

        if i_number > self.numbers.len() {
            eprintln!("eeeeeerrrrrooooooooooooooor");
            return;
        }

        self.numbers[i_number] = self.numbers[i_number]*10+digit;

    }
}


pub fn part2(filepath:&str, verbose:bool) -> u64 {

    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {eprintln!("could not open file {filepath}"); return 0},
    };

    let reader = BufReader::new(file);

    let lines: Vec<String> = reader.lines().filter_map(Result::ok).collect();

    let mut problems: Vec<MathProblem> = Vec::new();

    let l_lines = lines.len();

    let mut prev_i:usize = 0;
    for (i,c) in lines[l_lines-1].char_indices() {
        if c == ' ' {
            continue;
        }

        match problems.last_mut() {
            Some(prob) => {prob.set_n_numbers(i-prev_i-1);},
            None => {},
        }

        problems.push(MathProblem::new(i,c));
        prev_i = i;

    }

    match problems.last_mut() {
            Some(prob) => {prob.set_n_numbers(lines[l_lines-1].len()-prev_i);},
            None => {},
        }

    
    for i_l in 0.. l_lines-1{

        let mut current_prob : usize = 0;
        let mut prev_space = true;

        for (i,c) in lines[i_l].char_indices(){
            if c == ' ' {
                prev_space = true;
                continue;
            }

            if prev_space {
                current_prob += 1;
            }

            prev_space = false;

            problems[current_prob-1].add_digit(c, i);


        }
    }

    problems.iter().map(|p| {
        let r = p.calculate();
        if verbose {
            println!("{} = {}", p.to_string(), r);
        }
        r
    }).sum()

}

