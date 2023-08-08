use std::fs;
use std::path::PathBuf;

fn calories() -> Vec<u32> {
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

    calories
}

pub fn question1() -> u32 {
    let calories = calories();
    calories.into_iter().max().expect("Vec is empty, no max value")
}

pub fn question2() -> u32 {
    let mut calories = calories();
    calories.sort();

    calories.into_iter().rev().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{question1, question2};

    #[test]
    fn test_answer_1_works_for_provided_input() {
        let answer = question1();
        println!("Answer is {:?}", answer);
        assert_ne!(answer, 0);
    }

    #[test]
    fn test_answer_2_works_for_provided_input() {
        let answer = question2();
        println!("Answer is {:?}", answer);
        assert_ne!(answer, 0);
    }
}
