use regex::Regex;

fn stacks() -> [Vec<char>; 9] {
    [
        vec!['D', 'L', 'J', 'R', 'V', 'G', 'F'],
        vec!['T', 'P', 'M', 'B', 'V', 'H', 'J', 'S'],
        vec!['V', 'H', 'M', 'F', 'D', 'G', 'P', 'C'],
        vec!['M', 'D', 'P', 'N', 'G', 'Q'],
        vec!['J', 'L', 'H', 'N', 'F'],
        vec!['N', 'F', 'V', 'Q', 'D', 'G', 'T', 'Z'],
        vec!['F', 'D', 'B', 'L'],
        vec!['M', 'J', 'B', 'S', 'V', 'D', 'N'],
        vec!['G', 'L', 'D'],
    ]
}

pub fn question1() -> String {
    let input = crate::read_input_file(5);
    let mut stacks = stacks();
    let re = Regex::new("move (\\d+) from (\\d) to (\\d)")
        .expect("Invalid Regex Pattern");

    for line in input.lines() {
        let captures = re.captures(line)
            .expect("Didn't get any captures");
        let number: u8 = captures[1].parse()
            .expect("Failed to parse number");
        let from: usize = captures[2].parse()
            .expect("Failed to parse number");
        let to: usize = captures[3].parse()
            .expect("Failed to parse number");

        for _ in 0..number {
            let item = stacks[from - 1].remove(stacks[from - 1].len() - 1);
            stacks[to - 1].push(item);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}

pub fn question2() -> String {
    let input = crate::read_input_file(5);
    let mut stacks = stacks();
    let re = Regex::new("move (\\d+) from (\\d) to (\\d)")
        .expect("Invalid Regex Pattern");

    for line in input.lines() {
        let captures = re.captures(line)
            .expect("Didn't get any captures");
        let number: usize = captures[1].parse()
            .expect("Failed to parse number");
        let from: usize = captures[2].parse()
            .expect("Failed to parse number");
        let to: usize = captures[3].parse()
            .expect("Failed to parse number");

        let from_len = stacks[from - 1].len();

        for _ in 0..number {
            let item = stacks[from - 1].remove(from_len - number);
            stacks[to - 1].push(item);
        }
    }

    stacks
        .iter()
        .map(|stack| stack.last().unwrap())
        .collect()
}