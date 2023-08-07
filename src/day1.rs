use std::fs;
use std::path::PathBuf;

pub fn question1() -> u32 {
    let mut path = PathBuf::from(std::env!("CARGO_MANIFEST_DIR"));

    path.push("inputs/adventofcode.com_2022_day_1_input.txt");

    let file = fs::read_to_string(path)
        .expect("Failed to read input file");

    let mut calories = Vec::<u32>::new();
    let mut acc = 0u32;

    for line in file.split("\n") {
        if line == "" {
            calories.push(acc);
            acc = 0;
            continue;
        }

        acc += line.parse::<u32>()
            .expect("Failed to parse line into u32");
    }

    calories.into_iter().max().expect("Vec is empty, no max value")
}

#[cfg(test)]
mod tests {
    use super::question1;

    #[test]
    fn test_answer_works_for_provided_input() {
        let answer = question1();
        println!("Answer is {:?}", answer);
        assert_ne!(answer, 0);
    }
}