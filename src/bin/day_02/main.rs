fn main() {
    let input = include_str!("./input.txt").trim_end();

    part_1(input);
    part_2(input);
}

struct Policy<'a> {
    min: usize,
    max: usize,
    char: char,
    password: &'a str,
}

fn parse_policy(policy: &str) -> Policy {
    let (min, line) = policy.split_at(policy.find("-").unwrap());
    let min = min.parse::<usize>().unwrap();

    let (max, line) = line.split_at(line.find(" ").unwrap());
    let max = max[1..].parse::<usize>().unwrap();

    let (char, line) = line.split_at(line.find(": ").unwrap());
    let char = char.chars().nth(1).unwrap();

    let password = &line[2..];

    Policy {
        min,
        max,
        char,
        password,
    }
}

fn part_1(input: &str) {
    let valid = input
        .lines()
        .map(parse_policy)
        .filter(|policy| {
            let count = policy
                .password
                .chars()
                .filter(|x| *x == policy.char)
                .count();

            count >= policy.min && count <= policy.max
        })
        .count();

    println!("{}", valid);
}

fn part_2(input: &str) {
    let valid = input
        .lines()
        .map(parse_policy)
        .filter(|policy| {
            let a = policy.password.chars().nth(policy.min - 1).unwrap() == policy.char;
            let b = policy.password.chars().nth(policy.max - 1).unwrap() == policy.char;

            a ^ b
        })
        .count();

    println!("{}", valid);
}
