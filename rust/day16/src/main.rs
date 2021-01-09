use regex::Regex;
use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

const INPUT: &str = include_str!("../../../input/day16.txt");

type Rule = HashMap<String, ((usize, usize), (usize, usize))>;

fn parse(input: &str) -> (Rule, Vec<Vec<usize>>) {
    let rule_regex = Regex::new(r"(?P<rule>(\w+\s?)+):\s(?P<first_one>\d+)-(?P<first_two>\d+)\sor\s(?P<sec_one>\d+)-(?P<sec_two>\d+)").unwrap();
    if let [rules, own_ticket, other_tickets] = input.split("\n\n").collect::<Vec<_>>().as_slice() {
        let rules = rule_regex
            .captures_iter(rules)
            .map(|cap| {
                (
                    cap["rule"].to_owned(),
                    (
                        (
                            *(&cap["first_one"].parse::<usize>().unwrap()),
                            *(&cap["first_two"].parse::<usize>().unwrap()),
                        ),
                        (
                            *(&cap["sec_one"].parse::<usize>().unwrap()),
                            *(&cap["sec_two"].parse::<usize>().unwrap()),
                        ),
                    ),
                )
            })
            .collect();
        let tickets = own_ticket
            .lines()
            .skip(1)
            .chain(other_tickets.lines().skip(1))
            .map(|line| line.split(',').map(|num| num.parse().unwrap()).collect())
            .collect();
        (rules, tickets)
    } else {
        unreachable!();
    }
}

fn breaks_rule(rules: &Rule, ticket: &[usize]) -> Option<usize> {
    ticket
        .iter()
        .find(|num| {
            !rules
                .values()
                .any(|(a, b)| (a.0..=a.1).contains(num) || (b.0..=b.1).contains(num))
        })
        .copied()
}

fn get<'a>(tickets: &'a [&'a Vec<usize>], index: usize) -> impl Iterator<Item = usize> + 'a {
    tickets.iter().map(move |ticket| ticket[index])
}

fn fullfills_rule<'a>(
    tickets: &'a [&'a Vec<usize>],
    a: (usize, usize),
    b: (usize, usize),
) -> HashSet<usize> {
    let n = tickets[0].len();
    (0..n)
        .filter(|idx| {
            get(tickets, *idx).all(|num| (a.0..=a.1).contains(&num) || (b.0..=b.1).contains(&num))
        })
        .collect()
}

fn main() {
    let start = Instant::now();
    let (rules, tickets) = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let solution1 = tickets
        .iter()
        .filter_map(|ticket| breaks_rule(&rules, ticket))
        .sum::<usize>();
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);

    let start = Instant::now();
    let filtered_tickets = tickets
        .iter()
        .filter(|ticket| breaks_rule(&rules, ticket).is_none())
        .collect::<Vec<_>>();
    let mut positions = rules
        .iter()
        .map(|(rule, (a, b))| {
            let pos = fullfills_rule(&filtered_tickets, *a, *b);
            (rule, pos)
        })
        .collect::<Vec<_>>();
    positions.sort_unstable_by_key(|p| p.1.len());
    positions.reverse();
    let mut positions = {
        let mut vec = Vec::new();
        while !positions.is_empty() {
            let (rule, pos) = positions.pop().unwrap();
            let pos = pos.iter().next().unwrap();
            vec.push((rule.clone(), *pos));
            positions.iter_mut().for_each(|(_, set)| {
                set.remove(pos);
            });
        }
        vec
    };
    positions.sort_unstable_by_key(|k| k.1);
    let solution2 = positions
        .iter()
        .filter(|(rule, _)| rule.starts_with("departure"))
        .take(6)
        .fold(1, |acc, (_, idx)| acc * tickets[0][*idx]);
    // println!("{:?}", positions);
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}
