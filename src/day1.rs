fn calories() -> Vec<u32> {
    let file = crate::read_input_file(1);

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
