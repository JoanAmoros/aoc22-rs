use std::fs;
use std::path::PathBuf;

mod day5;
pub fn read_input_file(day: u8) -> String {
    let mut path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));

    path.push(format!("inputs/adventofcode.com_2022_day_{}_input.txt", day));

    fs::read_to_string(path)
        .expect("Failed to read input file")
}

fn main() {
    let answer = day5::question2();
    println!("The answer is: {}", answer);
}