use std::collections::HashSet;

const INPUT: &str = include_str!("../input.txt");

#[timed::timed]
fn parse<'a>(input: &'a str) -> Vec<&'a str> {
    input.split("\n\n").collect()
}

#[timed::timed]
fn solve1(input: &[&str]) -> usize {
    input
        .iter()
        .map(|group| {
            group
                .chars()
                .filter(|c| *c != '\n')
                .collect::<HashSet<_>>()
                .len()
        })
        .sum::<usize>()
}

#[timed::timed]
fn solve2<'a>(input: &[&str]) -> usize {
    input.iter().map(|group| {
        let mut group = group
            .split('\n')
            .map(|individual| individual.chars().collect::<HashSet<_>>());
        let first = group.next().unwrap();
        group.fold(first, |acc, set| &acc & &set)
            .len()
    })
    .sum()
}

fn main() {
    let input = parse(INPUT);
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}
