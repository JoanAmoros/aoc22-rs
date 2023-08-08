fn calculate_priority(ch: char) -> u32 {
    let ascii = ch as u32;
    return if ascii >= 97 {
        ascii - 96
    } else {
        return ascii - 38
    }
}

pub fn question1() -> u32 {
    let file = crate::read_input_file(3);
    let mut acc = 0u32;

    for line in file.lines() {
        let (half1, half2) = line.split_at(line.len() / 2);
        let ch = half1.chars().find(|ch| {
            half2.chars().any(|c| &c == ch)
        }).expect("No duplicate found");

        acc += calculate_priority(ch);
    }

    acc
}

pub fn question2() -> u32 {
    let file = crate::read_input_file(3);
    let mut acc = 0u32;

    for sacks in file.lines().collect::<Vec<&str>>().chunks(3) {
        let ch = sacks[0].chars().find(|ch| {
            sacks[1].chars().any(|c| &c == ch)
                && sacks[2].chars().any(|c| &c == ch)
        }).expect("No duplicate found");

        acc += calculate_priority(ch);
    }

    acc
}