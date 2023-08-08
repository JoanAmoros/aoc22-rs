enum Shape {
    Rock,
    Paper,
    Scissors
}

impl Shape {
    const fn points(&self) -> u32 {
        match self {
            Shape::Rock => 1,
            Shape::Paper => 2,
            Shape::Scissors => 3,
        }
    }

    const fn same(&self, other: &Shape) -> bool {
        return (matches!(self, Shape::Rock) && matches!(other, Shape::Rock))
            || (matches!(self, Shape::Paper) && matches!(other, Shape::Paper))
            || (matches!(self, Shape::Scissors) && matches!(other, Shape::Scissors));
    }

    const fn winning_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Paper,
            Shape::Paper => Shape::Scissors,
            Shape::Scissors => Shape::Rock
        }
    }

    const fn losing_shape(&self) -> Shape {
        match self {
            Shape::Rock => Shape::Scissors,
            Shape::Paper => Shape::Rock,
            Shape::Scissors => Shape::Paper
        }
    }

    const fn wins(&self, other: &Shape) -> bool {
        return (matches!(self, Shape::Rock) && matches!(other, Shape::Paper))
            || (matches!(self, Shape::Paper) && matches!(other, Shape::Scissors))
            || (matches!(self, Shape::Scissors) && matches!(other, Shape::Rock));
    }
}

impl TryFrom<&str> for Shape {
    type Error = ();
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Shape::Rock),
            "B" | "Y" => Ok(Shape::Paper),
            "C" | "Z" => Ok(Shape::Scissors),
            _ => Err(())
        }
    }
}

pub fn question1() -> u32 {
    let file = crate::read_input_file(2);

    let mut points = 0u32;

    for line in file.lines() {
        let (m1, m2) = line.rsplit_once(" ")
            .expect("Line didn't contain 2 moves");
        let opponent_shape = Shape::try_from(  m1)
            .expect("Unknown Shape");
        let my_shape = Shape::try_from(m2)
            .expect("Unknown Shape");

        points += my_shape.points();

        if opponent_shape.wins(&my_shape) {
            points += 6;
        } else if opponent_shape.same(&my_shape) {
            points += 3;
        }
    }

    points
}

pub fn question2() -> u32 {
    let file = crate::read_input_file(2);

    let mut points = 0u32;

    for line in file.lines() {
        let (m1, m2) = line.rsplit_once(" ")
            .expect("Line didn't contain 2 moves");
        let opponent_shape = Shape::try_from(  m1)
            .expect("Unknown Shape");
        let my_shape = match m2 {
            "X" => opponent_shape.losing_shape(),
            "Y" => {
                points += 3;
                opponent_shape
            },
            "Z" => {
                points += 6;
                opponent_shape.winning_shape()
            },
            _ => panic!("Unknown Shape")
        };

        points += my_shape.points();
    }

    points
}