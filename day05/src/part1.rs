use std::fs::File;
use std::io::{BufRead, BufReader};

struct FreshRange{
    start:u64,
    end:u64,
    wascombined:bool
}

impl FreshRange{
    fn new(s:&str) -> Self {
        let mut split = s.split("-");

        let start:u64 = match split.next() {
            Some(a) => {match a.parse() {
                Ok(b) => {b},
                Err(_) => {panic!("{} could not be parsed as a range. start {} could not be parsed",s,a)},
            }},
            None => {panic!("{} could not be parsed as a range. no start",s)},
        };
        let end:u64 = match split.next() {
            Some(a) => {match a.parse() {
                Ok(b) => {b},
                Err(_) => {panic!("{} could not be parsed as a range. start {} could not be parsed",s,a)},
            }},
            None => {panic!("{} could not be parsed as a range. no start",s)},
        };

        return FreshRange { start, end, wascombined:false }

    }

    fn includes(&self,i:u64)->bool{
        return i>=self.start && i <=self.end
    }

    fn combine(&mut self, other:&mut FreshRange) -> bool {

        if other.wascombined {
            return false
        }

        let mut has_changed = false;

        if other.start >= self.start && other.start <= self.end {
            has_changed = true;
            other.wascombined = true;
            if other.end > self.end {
                self.end = other.end;
            }
        }

        if other.end <= self.end && other.end >= self.start {
            has_changed = true;
            other.wascombined = true;
            if other.start < self.start {
                self.start = other.start;
            }
        }

        if other.start <= self.start && other.end >= self.end {
            has_changed = true;
            other.wascombined = true;
            self.start = other.start;
            self.end = other.end;
        }

        has_changed
    }
}

struct FreshRanges {
    ranges:Vec<FreshRange>
}

impl FreshRanges {
    fn new() -> Self{
        FreshRanges { ranges: Vec::new() }
    }

    fn add(&mut self, s:&str){
        self.ranges.push(FreshRange::new(s));
    }
    
    fn is_fresh(&self,i:u64) -> bool{
        for r in &self.ranges{
            if r.includes(i) {
                return true
            }
        }

        false
    }

    fn len(&self) -> usize {
        self.ranges.len()
    }

    fn combine_ranges(&mut self){

        let l = self.ranges.len();

        let mut something_changed = true;

        while something_changed {
            something_changed = false;
            for i in 0..l {

                let (firstpart,otherranges) = self.ranges.split_at_mut(i+1);

                let this_range = &mut firstpart[i];

                if !this_range.wascombined {
                    
                    for other in otherranges {
                        if this_range.combine(other) {
                            something_changed = true;
                        }
                    }
                    
                }

            }
        }
    }

    fn count_ranges(&self) -> u64 {

        let mut result = 0;

        for r in &self.ranges{
            if !r.wascombined{
                result+= r.end - r.start + 1;
            }
        }


        result
    }


}


pub fn part1(filepath:&str, verbose:bool)->u32{

    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {eprintln!("could not open file {filepath}"); return 0},
    };

    let mut encounterd_empty = false;
    let mut count_fresh:u32 = 0;

    let reader = BufReader::new(file);

    let mut ranges = FreshRanges::new();

    for line in reader.lines(){
        let l = match line {
            Ok(s) => {s},
            Err(e) => {panic!("could not read one of the lines: {}",e)},
        };

        if l.trim().is_empty() {
            encounterd_empty = true;
            if verbose {
                println!("read {} ranges",ranges.len())
            }
        }
        else if encounterd_empty {

            let i:u64 = match l.parse() {
                Ok(a) => {a},
                Err(_) => {panic!("could not parse {} as a number",l)},
            };

            if ranges.is_fresh(i) {
                count_fresh += 1;
            }
            else if verbose {
                println!("{} is spoiled",i);
            }
        }
        else {
            if verbose {
                println!("{l}");
            }

            ranges.add(&l);
        }
    }

    
    count_fresh
}

pub fn part2(filepath:&str, verbose:bool) -> u64 {

    let file = match File::open(filepath) {
        Ok(file) => {file}
        _ => {eprintln!("could not open file {filepath}"); return 0},
    };

    let reader = BufReader::new(file);

    let mut ranges = FreshRanges::new();

    for line in reader.lines(){
        let l = match line {
            Ok(s) => {s},
            Err(e) => {panic!("could not read one of the lines: {}",e)},
        };

        if l.trim().is_empty() {
            if verbose {
                println!("read {} ranges",ranges.len());
                
            }
            break ;
        }

        ranges.add(&l);

    }

    ranges.combine_ranges();

    if verbose {
        println!("combined ranges:");
        for r in &ranges.ranges{

            println!("{}-{}, {}",r.start,r.end, r.wascombined);
        }
        println!("");
    }

    ranges.count_ranges()



}