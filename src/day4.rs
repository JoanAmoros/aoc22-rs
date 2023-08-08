pub fn question1() -> u32 {
    let file = crate::read_input_file(4);

    let mut acc = 0u32;

    for line in file.lines() {
        let assignments = line.split(",")
            .map(|assignment| assignment.split("-").collect())
            .collect::<Vec<Vec<&str>>>();
        let a11 = assignments[0][0].parse::<u32>()
            .expect("Failed to parse number");
        let a12 = assignments[0][1].parse::<u32>()
            .expect("Failed to parse number");
        let a21 = assignments[1][0].parse::<u32>()
            .expect("Failed to parse number");
        let a22 = assignments[1][1].parse::<u32>()
            .expect("Failed to parse number");

        if (a11 >= a21 && a12 <= a22) || (a21 >= a11 && a22 <= a12) {
            acc += 1;
        }
    }

    acc
}

pub fn question2() -> u32 {
    let file = crate::read_input_file(4);

    let mut acc = 0u32;

    for line in file.lines() {
        let assignments = line.split(",")
            .map(|assignment| assignment.split("-").collect())
            .collect::<Vec<Vec<&str>>>();
        let a11 = assignments[0][0].parse::<u32>()
            .expect("Failed to parse number");
        let a12 = assignments[0][1].parse::<u32>()
            .expect("Failed to parse number");
        let a21 = assignments[1][0].parse::<u32>()
            .expect("Failed to parse number");
        let a22 = assignments[1][1].parse::<u32>()
            .expect("Failed to parse number");

        if
            (a11 >= a21 && a11 <= a22)
            || (a12 >= a22 && a12 <= a21)
            || (a21 >= a11 && a21 <= a12)
            || (a22 >= a12 && a22 <= a11)
        {
            acc += 1;
        }
    }

    acc
}