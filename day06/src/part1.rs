
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
}

impl MathProblem {
    
    fn new() -> Self {
        MathProblem { numbers: Vec::new(), operator: Operator::NONE }
    }

    fn add(&mut self, s:&str) {

        match s {
            "+" => self.operator = Operator::PLUS,
            "*" => self.operator = Operator::MULT,
            a => self.numbers.push( a.parse().unwrap_or_else( |_| {eprintln!("could not parse {} as a number", a); 0} ))
        }
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
}

pub fn part1(filepath:&str, verbose:bool) -> u64 {

    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {eprintln!("could not open file {filepath}"); return 0},
    };

    let reader = BufReader::new(file);

    let mut problems: Vec<MathProblem> = Vec::new();

    for line in reader.lines(){
        let l = match line {
            Ok(s) => {s},
            Err(e) => {panic!("could not read one of the lines: {}",e)},
        };

        let mut i = 0;

        for s in l.split(" "){
            if s.trim().is_empty() {
                continue;
            }

            if i == problems.len() {
                problems.push(MathProblem::new());
            }

            problems[i].add(s);

            i += 1;
        }

    }

    let mut result = 0;

    for problem in &problems{
        let r = problem.calculate();
        if verbose {
            println!("{} = {}", problem.to_string(), r);
            result += r;
        }
    }


    result
}