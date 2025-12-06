use crate::roll_layout::part2;


mod roll_layout;

fn main() {
    
    let arg: Vec<String> = std::env::args().collect();

    //note arg[0] is program name
    if arg.len() < 3 {
        panic!("You need to pass the input filepath and part as arguments. -- filepath part(1/2) verbose(y/n)")
    }

    let filepath = &arg[1];

    let part = &arg[2];

    let mut verbose = false;

    if arg.len() > 3 && arg[3] == "y" {
        verbose = true;
    }

    let mut layout = roll_layout::read_file(&filepath);

    if part == "1" {
        let result = layout.part1();

        println!("part1: {}",result);
    }

    if part == "2" {
        let result = roll_layout::part2(&mut layout, verbose);

        println!("part2: {}",result);
    }




}

