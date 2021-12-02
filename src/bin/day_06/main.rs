use std::{collections::HashSet, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn part_1(input: &str) -> impl Debug {
    input
        .split("\n\n")
        .map(|group| {
            group
                .chars()
                .filter(|c| !c.is_whitespace())
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>()
}

fn part_2(input: &str) -> impl Debug {
    input
        .split("\n\n")
        .map(|group| {
            group
                .lines()
                .map(|line| line.chars().collect::<HashSet<_>>())
                .reduce(|a, b| a.intersection(&b).map(|x| *x).collect())
                .unwrap()
                .len()
        })
        .sum::<usize>()
}
