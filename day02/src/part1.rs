use core::{num, panic};
use std::collections::{BTreeSet};
use std::fs::File;
use std::io::Read;



pub struct IDRange {
    start: String,
    end: String
}

impl IDRange {
    pub fn to_string(&self) -> String{
        format!("{}-{}",self.start,self.end)
    }

    fn sum_doubles_in_range(&self) ->u64{
        let l = self.start.len();

        //if length of a number is uneven it can't be a repeat of two times the same number
        //(asumes start and end have same length)
        if l%2 == 1 {
            return 0;
        }

        // cut start and end in the middle and parse as numbers
        let mid = l/2;

        let start_left:u64 = self.start[..mid].parse().expect("start_left not a number");
        let start_right:u64 = self.start[mid..].parse().expect("start_left not a number");
        let end_left:u64 = self.end[..mid].parse().expect("start_left not a number");
        let end_right:u64 = self.end[mid..].parse().expect("start_left not a number");

        
        let factor = 10u64.pow(mid as u32)+1;

        let mut result: u64 = 0;

        if start_left == end_left {
            if start_left >= start_right && start_left <= end_right {
                return factor*start_left
            }
            return 0
        }

        if start_left >= start_right {
            result += factor*start_left;
        }

        for i in (start_left + 1)..end_left {
            result += factor * i;
        }

        if end_left <= end_right {
            result += factor*end_left;
        }

        return result;

    }


    fn sum_repeat_ns(&self, n:usize, numbers: &mut BTreeSet<u64>) {

        let l = self.start.len();

        if l%n != 0 {
            return
        }

        let splitsize = l/n;

        let mut start_val: Vec<u64> = vec![0;n];
        let mut end_val: Vec<u64> = vec![0;n];

        //parse string parts to numbers
        for i in 0..n {
            start_val[i] = self.start[i*splitsize..(i+1)*splitsize].parse().expect("could not parse to uint");
            end_val[i] = self.end[i*splitsize..(i+1)*splitsize].parse().expect("could not parse to uint");
        }

        let pow10 = 10u64.pow(splitsize as u32);

        let factor = (pow10.pow(n as u32) - 1)/(pow10 -1);

        //println!("factor: {}", factor);

        let start_left = start_val[0];
        let end_left = end_val[0];

        //println!("start: {} {}, end {} {}",start_val[0],start_val[1],end_val[0],end_val[1]);

        if start_left == end_left {

            //println!("isequal");
            
            let repeated_larger_start = compare_repeated(start_left, start_val);

            //println!("{repeated_larger_start}");

            if repeated_larger_start > 0 {
                return
            }

            let repeated_larger_end = compare_repeated(start_left, end_val);

            //println!("{repeated_larger_end}");

            if repeated_larger_end < 0 {
                return
            }

            //println!("has multi");

            numbers.insert(factor * start_left);
            return;
        }

        if compare_repeated(start_left, start_val) <= 0 {
            numbers.insert(factor * start_left);
        }

        if compare_repeated(end_left, end_val) >= 0 {
            numbers.insert( factor*end_left);
        }

        for i in start_left+1..end_left {
            numbers.insert(  factor * i);
        }
    }

    fn sum_all_repeats(&self) -> u64 {

        let mut numbers: BTreeSet<u64> = BTreeSet::new();

        for i in 2..=self.start.len() {
            self.sum_repeat_ns(i,&mut numbers);
        }

        let mut result:u64 = 0;
        for number in numbers {
            result += number;
        }

        result
    }
}


//expects s to "start-end"
fn read_one_range(s:&str) -> Vec<IDRange> {
    let mut result: Vec<IDRange> = Vec::new();

    let mut split = s.split('-');

    let start = match split.next() {
        Some(s) => {s},
        None => {panic!("idrange not ok: start missing")},
    }.to_string();
    let end = match split.next() {
        Some(s) => {s},
        None => {panic!("idrange not ok: end missing")},
    }.to_string();

    let mut i = start.len();
    let l = end.len();

    if i == l {
        result.push(IDRange {start,end});
    }
    else {
        result.push(IDRange { start, end:"9".repeat(i) });

        i+=1;
        while i < l {
            result.push(IDRange {start:"1".to_string() + &"0".repeat(i-1), end: "9".repeat(i)});
            i += 1;
        }

        result.push(IDRange { start:"1".to_string() + &"0".repeat(i-1), end });
    }

    



    result
}

pub fn read_all_ranges(filepath:&str) -> Vec<IDRange> {

    let mut result : Vec<IDRange> = Vec::new();

    let mut file = match File::open(filepath) {
        Ok(f) => {f},
        Err(e) => {eprintln!("error opening file {}: {}",filepath,e); return result },
    };

    let mut contents = String::new();
    match file.read_to_string(&mut contents) {
        Ok(_) => {},
        Err(e) => {eprintln!("error reading file {}: {}",filepath,e); return result},
    };

    for part in contents.split(',') {
        let mut this_result = read_one_range(part);
        result.append(&mut this_result);
    }

    result
}

pub fn part1(filepath:&str) -> u64{

    let ranges = read_all_ranges(filepath);

    let mut result = 0;

    for idr in ranges {
        let this_result = idr.sum_doubles_in_range();

        println!("{} : {}", idr.to_string(), this_result);
        result += this_result;
    }

    result
}

pub fn part2(filepath:&str) -> u64{

    let ranges = read_all_ranges(filepath);

    let mut result = 0;

    for idr in ranges {
        let this_result = idr.sum_all_repeats();

        println!("{} : {}", idr.to_string(), this_result);
        result += this_result;
    }

    result
}

//-1 -> is smaller then repeated, 0 is equal, 1 -> larger
fn compare_repeated(repeated_val:u64,vals_parts:Vec<u64>) -> i8 {
    for valpart in vals_parts {
        if valpart > repeated_val {
            return 1
        }
        else if valpart < repeated_val {
            return -1
        }
    }

    0
}