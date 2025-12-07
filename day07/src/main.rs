mod part1;
mod part2;

fn main() {

    //get filename from arguments
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

    
    if part == "1" {
        let result = part1::part1(&filepath, verbose);
        println!("part1: {}", result);
    }

    

    if part == "2" {
        let result = part2::part2(&filepath, verbose);
        println!("part2: {}", result);
    }
    


}
