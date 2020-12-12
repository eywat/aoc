use std::time::Instant;
use std::collections::HashSet;

const INPUT: &str = include_str!("../../../input/day01.txt");

fn parse(input: &str) -> HashSet<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn solve1(input: &HashSet<isize>) -> Option<isize> {
    input.iter().find_map(|i| match input.get(&(2020 - *i)) {
        Some(j) => Some(i*j),
        None => None
    })
}

fn solve2(input: &HashSet<isize>) -> Option<isize> {
    input
        .iter()
        .find_map(|x| {
            input
                .iter()
                .find_map(|y| match input.get(&(2020 - x - y)) {
                    Some(z) => Some(x*y*z),
                    None => None
                })
        })
}

fn main() {
    let start = Instant::now();
    let input = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let solution1 = solve1(&input).unwrap();
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);
    
    let start = Instant::now();
    let solution2 = solve2(&input).unwrap();
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let input = parse(INPUT);
        assert_eq!(solve1(&input), Some(988771))
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        assert_eq!(solve2(&input), Some(171933104))
    }
}
