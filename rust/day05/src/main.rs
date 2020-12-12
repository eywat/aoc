use std::time::Instant;

const INPUT: &str = include_str!("../../../input/day05.txt");

fn seat_id(s: &str) -> u16 {
    s.chars().fold(0, |acc, c| match c {
        'B' | 'R' => (acc << 1) | 1,
        _ => acc << 1,
    })
}

fn solve1(input: &[u16]) -> u16 {
    *input.iter().max().unwrap()
}

fn solve2(input: &mut [u16]) -> u16 {
    input.sort_unstable();
    input
        .windows(2)
        .find(|seats| seats[1] - seats[0] == 2)
        .map(|seats| seats[0] + 1)
        .unwrap()
}

fn main() {
    let start = Instant::now();

    let mut input: Vec<_> = INPUT.lines().map(seat_id).collect();
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let solution1 = solve1(&input);
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);

    let start = Instant::now();
    let solution2 = solve2(&mut input);
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_row() {
        assert_eq!(seat_id("BFFFBBF"), 70);
        assert_eq!(seat_id("FFFBBBF"), 14);
        assert_eq!(seat_id("BBFFBBF"), 102)
    }

    #[test]
    fn test_column() {
        assert_eq!(seat_id("RRR"), 7);
        assert_eq!(seat_id("RLL"), 4)
    }

    #[test]
    fn test_solve1() {
        let input: Vec<_> = INPUT.lines().map(seat_id).collect();
        assert_eq!(solve1(&input), 818)
    }

    #[test]
    fn test_solve2() {
        let mut input: Vec<_> = INPUT.lines().map(seat_id).collect();
        assert_eq!(solve2(&mut input), 559)
    }
}
