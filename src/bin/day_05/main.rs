fn main() {
    let input = include_str!("./input.txt").trim_end();

    part_1(input);
    part_2(input);
}

enum Instruction {
    Lower,
    Higher,
}

fn binary_space_partitioning(ins: &[Instruction], mut min: u8, mut max: u8) -> u8 {
    for i in ins {
        match i {
            Instruction::Lower => max = min + (max - min) / 2,
            Instruction::Higher => min = min + (max - min) / 2 + 1,
        }
    }

    min.max(max)
}

fn calculate_seat_ids(input: &str) -> Vec<i32> {
    input
        .lines()
        .map(|line| line.split_at(7))
        .map(|(row, column)| {
            (
                row.chars()
                    .map(|c| match c {
                        'F' => Instruction::Lower,
                        'B' => Instruction::Higher,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
                column
                    .chars()
                    .map(|c| match c {
                        'L' => Instruction::Lower,
                        'R' => Instruction::Higher,
                        _ => unreachable!(),
                    })
                    .collect::<Vec<_>>(),
            )
        })
        .map(|(row, column)| {
            (
                binary_space_partitioning(&row, 0, 127),
                binary_space_partitioning(&column, 0, 7),
            )
        })
        .map(|(row, column)| row as i32 * 8 + column as i32)
        .collect()
}

fn part_1(input: &str) {
    let seat_ids = calculate_seat_ids(input);

    println!("{}", seat_ids.iter().max().unwrap());
}

fn part_2(input: &str) {
    let seat_ids = calculate_seat_ids(input);

    for i in 0..seat_ids.len() {
        for j in i..seat_ids.len() {
            let x = seat_ids[i];
            let y = seat_ids[j];
            let id = i32::max(x, y) - 1;

            if (x - y).abs() == 2 && seat_ids.iter().filter(|s_id| **s_id == id).count() == 0 {
                println!("{}", id);
                return;
            }
        }
    }
}
