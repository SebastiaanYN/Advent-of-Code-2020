fn main() {
    let input = include_str!("./input.txt").trim_end();

    part_1(input);
    part_2(input);
}

fn part_1(input: &str) {
    let lines = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for i in 0..lines.len() {
        for j in (i + 1)..lines.len() {
            if lines[i] + lines[j] == 2020 {
                println!("{}", lines[i] * lines[j]);
                return;
            }
        }
    }
}

fn part_2(input: &str) {
    let lines = input
        .lines()
        .map(|line| line.parse::<i32>().unwrap())
        .collect::<Vec<_>>();

    for x in 0..lines.len() {
        for y in (x + 1)..lines.len() {
            for z in (y + 1)..lines.len() {
                if lines[x] + lines[y] + lines[z] == 2020 {
                    println!("{}", lines[x] * lines[y] * lines[z]);
                    return;
                }
            }
        }
    }
}
