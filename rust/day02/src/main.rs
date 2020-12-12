use std::time::Instant;

use util::split_once;

const INPUT: &str = include_str!("../../../input/day02.txt");

type PWRules<'a> = (usize, usize, char, &'a str);

fn parse(input: &str) -> Vec<PWRules> {
    input
        .lines()
        .map(|line| {
            let (rules, pw) = split_once(line, ':').unwrap();
            let (counts, letter) = split_once(rules, ' ').unwrap();
            let (first, second) = split_once(counts, '-').unwrap();
            let first = first.parse().unwrap();
            let second = second.parse().unwrap();
            let letter = letter.chars().next().unwrap();
            let pw = pw.trim();
            (first, second, letter, pw)
        })
        .collect()
}

fn solve1(input: &[PWRules]) -> usize {
    input
        .iter()
        .filter(|(min, max, letter, pw)| {
            let count = pw.chars().filter(|c| c == letter).count();
            (*min..=*max).contains(&count)
        })
        .count()
}

fn solve2(input: &[PWRules]) -> usize {
    input
        .iter()
        .filter(|(pos1, pos2, letter, pw)| {
            pw.match_indices(*letter)
                .filter(|(ind, _)| ind + 1 == *pos1 || ind + 1 == *pos2)
                .count()
                == 1
        })
        .count()
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
        assert_eq!(solve1(&input), 625)
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        assert_eq!(solve2(&input), 391)
    }
}
