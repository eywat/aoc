use std::{collections::HashSet, time::Instant};

const INPUT: &str = include_str!("../../../input/day06.txt");

fn parse<'a>(input: &'a str) -> Vec<&'a str> {
    input.split("\n\n").collect()
}

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
    let start = Instant::now();
    let input = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let solution1 = solve1(&input);
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);
    
    let start = Instant::now();
    let solution2 = solve2(&input);
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}



#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let input = parse(INPUT);
        assert_eq!(solve1(&input), 6735)
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        assert_eq!(solve2(&input), 3221)
    }
}