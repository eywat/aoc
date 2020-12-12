use std::collections::HashMap;
use std::time::Instant;

use regex::Regex;
use util::split_once;

const INPUT: &str = include_str!("../../../input/day07.txt");

#[derive(Debug, Eq, PartialEq, Hash)]
struct Bag<'a>(&'a str);
type Contents<'a> = Vec<(usize, Bag<'a>)>;
type Rules<'a> = HashMap<Bag<'a>, Contents<'a>>;

fn parse(input: &str) -> Rules {
    let bag = Regex::new(r"^\w+\s\w+").unwrap();
    let contents = Regex::new(r"(?P<count>\d+)\s(?P<color>\w+\s\w+)").unwrap();
    input
        .lines()
        .map(|rule| {
            let bag = bag.find(rule).unwrap().as_str();
            let bag = Bag(bag);
            let contents = contents
                .find_iter(rule)
                .map(|cap| {
                    let cap = cap.as_str();
                    let (count, color) = split_once(cap, ' ').unwrap();
                    (count.parse().unwrap(), Bag(color))
                })
                .collect();

            (bag, contents)
        })
        .collect::<Rules>()
}

fn contains_bag(bag: &Bag, rule: &Bag, rules: &Rules) -> bool {
    match rules.get(rule) {
        Some(content) if content.iter().any(|(_, b)| b == bag) => true,
        Some(content) => content
            .iter()
            .any(|(_, rule)| contains_bag(bag, rule, rules)),
        None => false,
    }
}

fn count_bags(bag: &Bag, rules: &Rules) -> usize {
    match rules.get(bag) {
        Some(content) => content
            .iter()
            .map(|(count, bag)| count + count * count_bags(bag, rules))
            .sum(),
        None => 0,
    }
}

fn main() {
    const BAG: Bag = Bag("shiny gold");

    let start = Instant::now();
    let input = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());
    
    let start = Instant::now();
    let solution1 = input
        .keys()
        .filter(|rule| contains_bag(&BAG, rule, &input))
        .count();
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);

    let start = Instant::now();
    let solution2 = count_bags(&BAG, &input);
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        const BAG: Bag = Bag("shiny gold");
        let input = parse(INPUT);
        let solution1 = input
            .keys()
            .filter(|rule| contains_bag(&BAG, rule, &input))
            .count();
        assert_eq!(solution1, 211)
    }

    #[test]
    fn test_solve2() {
        const BAG: Bag = Bag("shiny gold");
        let input = parse(INPUT);
        let solution2 = count_bags(&BAG, &input);
        assert_eq!(solution2, 12414)
    }
}
