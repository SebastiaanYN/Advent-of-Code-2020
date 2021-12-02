use nom::{
    branch::alt,
    bytes::complete::{tag, take},
    character::{complete::alpha1, streaming::char},
    combinator::{map_res, opt, recognize},
    multi::separated_list1,
    sequence::tuple,
    IResult, Parser,
};
use std::{collections::HashMap, fmt::Debug};

fn main() {
    let input = include_str!("./input.txt").trim_end();

    println!("{:?}", part_1(input));
    println!("{:?}", part_2(input));
}

fn bag_name(s: &str) -> IResult<&str, &str> {
    recognize(tuple((alpha1, char(' '), alpha1)))(s)
}

fn bag_count(s: &str) -> IResult<&str, (&str, i32)> {
    let (s, count) = map_res(take(1usize), |s: &str| s.parse::<i32>())(s)?;
    let (s, _) = char(' ')(s)?;
    let (s, name) = bag_name(s)?;
    let (s, _) = tuple((tag(" bag"), opt(char('s'))))(s)?;

    Ok((s, (name, count)))
}

fn parser(s: &str) -> IResult<&str, (&str, Vec<(&str, i32)>)> {
    let (s, bag_name) = bag_name(s)?;
    let (s, _) = tag(" bags contain ")(s)?;
    let (s, bags) = alt((
        tag("no other bags").map(|_| Vec::new()),
        separated_list1(tag(", "), bag_count),
    ))(s)?;
    let (s, _) = char('.')(s)?;

    Ok((s, (bag_name, bags)))
}

fn contains_shiny_gold(map: &HashMap<&str, Vec<&str>>, bag: &str) -> bool {
    map.get(bag)
        .unwrap()
        .iter()
        .any(|&bag| bag == "shiny gold" || contains_shiny_gold(map, bag))
}

fn count_bags(map: &HashMap<&str, Vec<(&str, i32)>>, bag: &str) -> i32 {
    map.get(bag)
        .unwrap()
        .iter()
        .map(|&(bag, count)| count + count * count_bags(map, bag))
        .sum()
}

fn part_1(input: &str) -> impl Debug {
    let map = input.lines().map(|line| parser(line).unwrap().1).fold(
        HashMap::new(),
        |mut acc, (bag_name, bags)| {
            acc.insert(
                bag_name,
                bags.iter().map(|&(name, _)| name).collect::<Vec<_>>(),
            );

            acc
        },
    );

    map.keys()
        .filter(|bag| contains_shiny_gold(&map, bag))
        .count()
}

fn part_2(input: &str) -> impl Debug {
    let map = input.lines().map(|line| parser(line).unwrap().1).fold(
        HashMap::new(),
        |mut acc, (bag_name, bags)| {
            acc.insert(bag_name, bags);

            acc
        },
    );

    count_bags(&map, "shiny gold")
}
