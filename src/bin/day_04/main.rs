use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt").trim_end();

    part_1(input);
    part_2(input);
}

fn parse_passport(passport: &str) -> HashMap<&str, &str> {
    passport
        .split_whitespace()
        .map(|field| field.split(":"))
        .map(|mut field| (field.next().unwrap(), field.next().unwrap()))
        .collect::<HashMap<&str, &str>>()
}

fn part_1(input: &str) {
    let result = input
        .split("\n\n")
        .map(parse_passport)
        .filter(|passport| {
            passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
        })
        .count();

    println!("{}", result);
}

fn part_2(input: &str) {
    let result = input
        .split("\n\n")
        .map(parse_passport)
        .filter(|passport| {
            passport.len() == 8 || (passport.len() == 7 && !passport.contains_key("cid"))
        })
        .filter(|passport| {
            let byr = (1920..=2002).contains(&passport.get("byr").unwrap().parse().unwrap());
            let iyr = (2010..=2020).contains(&passport.get("iyr").unwrap().parse().unwrap());
            let eyr = (2020..=2030).contains(&passport.get("eyr").unwrap().parse().unwrap());

            let hgt = passport.get("hgt").unwrap();
            let hgt = match hgt.split_at(hgt.len() - 2) {
                (x, "cm") => (150..=193).contains(&x.parse().unwrap()),
                (x, "in") => (59..=76).contains(&x.parse().unwrap()),
                _ => false,
            };

            let hcl = passport.get("hcl").unwrap();
            let hcl = match hcl.split_at(1) {
                ("#", h) => i32::from_str_radix(h, 16).is_ok(),
                _ => false,
            };

            let ecl = match *passport.get("ecl").unwrap() {
                "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
                _ => false,
            };

            let pid = passport.get("pid").unwrap();
            let pid = pid.len() == 9 && pid.chars().all(|c| c.is_digit(10));

            byr && iyr && eyr && hgt && hcl && ecl && pid
        })
        .count();

    println!("{}", result);
}
