fn pos_of_first_n_unique_chars(n: usize) -> usize {
    let file = crate::read_input_file(6);

    let chars = file
        .chars()
        .collect::<Vec<char>>();

    for (idx, slice) in chars.windows(n).enumerate() {
        let mut repeats= false;
        for i in 1..slice.len() {
            if slice[i..].contains(&slice[i - 1]) {
                repeats = true;
            }
        }

        if !repeats {
            return idx + n;
        }
    }

    return 0;
}

pub fn question1() -> usize {
    pos_of_first_n_unique_chars(4)
}

pub fn question2() -> usize {
    pos_of_first_n_unique_chars(14)
}