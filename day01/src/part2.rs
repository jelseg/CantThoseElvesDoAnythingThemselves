// using absolute path
use crate::read_instructions::{Instruction,Direction};

pub struct Dial2 {
    current:u32,
    n_zeros:u32,
    n_numbers:u32
}

impl Dial2 {
    pub fn do_instruction(&mut self,inst:&Instruction) {

        let num = inst.amount()%self.n_numbers;

        let mut nSpins:u32 = inst.amount()/self.n_numbers;

        match inst.direction() {
            Direction::L => {
                if self.current <= num && self.current!=0 {
                    nSpins += 1;
                }
                self.current = (self.current + self.n_numbers - num)%self.n_numbers
            },
            Direction::R => {
                if self.current + num >= self.n_numbers {
                    nSpins += 1;
                }
                self.current = (self.current + num)%self.n_numbers
            },
        }


        self.n_zeros += nSpins;

        
    }


    pub fn to_string(&self) -> String{
        format!("current:{:02}, n_zeros:{}",self.current,self.n_zeros)
    }

    pub fn new() -> Self{
        Self {
            current: 50,
            n_zeros: 0,
            n_numbers: 100
        }
    }
}


pub fn part2(instructions:&Vec<Instruction>) {

    let mut dial = Dial2::new();

    for inst in instructions {
        //println!("{}",inst.to_string());

        dial.do_instruction(&inst);
        println!("{}", dial.to_string());
    }

    println!("{}", dial.to_string());
}