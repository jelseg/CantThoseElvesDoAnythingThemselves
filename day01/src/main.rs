mod read_instructions;
mod part1;
mod part2;

fn main() {
    
    let instructions = read_instructions::read_file(String::from("input/input.txt")).unwrap();

    //part1::part1(&instructions);
    part2::part2(&instructions);

}


