mod part1;
mod part2;

use std::fs;

fn main() {
    // get file input
    let file_path = "./input.txt";
    let file_contents = fs::read_to_string(file_path).expect("File should be readable!");

    // part 1
    println!("Output of part 1: {}", part1::solution(&file_contents));

    // part 2
    println!("Output of part 2: {}", part2::solution(&file_contents));
}