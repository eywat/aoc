use std::collections::{HashMap, HashSet};
use util::split_once;

const INPUT: &str = include_str!("../../../input/day04.txt");

fn filter_valid<'a>(
    input: &'a str,
    required_keys: &'a HashSet<&&str>,
) -> impl Iterator<Item = HashMap<&'a str, &'a str>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .split([' ', '\n'].as_ref())
                .map(|key_value| split_once(key_value, ':').unwrap())
                .collect::<HashMap<_, _>>()
        })
        .filter(move |passport| {
            passport
                .keys()
                .collect::<HashSet<_>>()
                .is_superset(required_keys)
        })
}

#[timed::timed]
fn solve1(input: &str) -> usize {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .collect();
    filter_valid(input, &required_keys).count()
}

fn valid_byr(byr: &str) -> bool {
    match byr.parse() {
        Ok(yr) => byr.len() == 4 && 1920 <= yr && yr <= 2002,
        Err(_) => false,
    }
}

fn valid_iyr(iyr: &str) -> bool {
    match iyr.parse() {
        Ok(yr) => iyr.len() == 4 && 2010 <= yr && yr <= 2020,
        Err(_) => false,
    }
}

fn valid_eyr(eyr: &str) -> bool {
    match eyr.parse() {
        Ok(yr) => eyr.len() == 4 && 2020 <= yr && yr <= 2030,
        Err(_) => false,
    }
}

fn valid_hgt(hgt: &str) -> bool {
    if let Some(cm) = hgt.strip_suffix("cm").and_then(|cm| cm.parse().ok()) {
        150 <= cm && cm <= 193
    } else if let Some(inches) = hgt
        .strip_suffix("in")
        .and_then(|inches| inches.parse().ok())
    {
        59 <= inches && inches <= 76
    } else {
        false
    }
}

fn valid_hcl(hcl: &str) -> bool {
    match hcl.strip_prefix('#') {
        Some(col) => col.len() == 6 && col.chars().all(|c| "0123456789abcdef".contains(c)),
        None => false,
    }
}

fn valid_ecl(ecl: &str) -> bool {
    match ecl {
        "amb" | "blu" | "brn" | "gry" | "grn" | "hzl" | "oth" => true,
        _ => false,
    }
}

fn valid_pid(pid: &str) -> bool {
    pid.len() == 9 && pid.chars().all(|c| c.is_ascii_digit())
}

#[timed::timed]
fn solve2(input: &str) -> usize {
    let required_keys = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"]
        .iter()
        .collect();
    let validations: HashMap<_, _> = {
        let mut map = HashMap::new();
        map.insert("byr", valid_byr as for<'r> fn(&'r str) -> bool);
        map.insert("iyr", valid_iyr as for<'r> fn(&'r str) -> bool);
        map.insert("eyr", valid_eyr as for<'r> fn(&'r str) -> bool);
        map.insert("hgt", valid_hgt as for<'r> fn(&'r str) -> bool);
        map.insert("hcl", valid_hcl as for<'r> fn(&'r str) -> bool);
        map.insert("ecl", valid_ecl as for<'r> fn(&'r str) -> bool);
        map.insert("pid", valid_pid as for<'r> fn(&'r str) -> bool);
        map
    };

    filter_valid(input, &required_keys)
        .filter(|passport| {
            passport
                .iter()
                .all(|(key, value)| match validations.get(key) {
                    Some(validate) => validate(value),
                    None => true,
                })
        })
        .count()
}

fn main() {
    println!("Solution 1: {}", solve1(INPUT));
    println!("Solution 2: {}", solve2(INPUT));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        assert_eq!(solve1(INPUT), 226)
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(INPUT), 160)
    }
}
