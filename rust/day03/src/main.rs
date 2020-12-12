use std::time::Instant;

const INPUT: &str = include_str!("../../../input/day03.txt");

fn solve1(input: &str, (right, down): (usize, usize)) -> usize {
    const TREE: char = '#';
    let mut movement = right;
    input
        .lines()
        .skip(down)
        .step_by(down)
        .map(|line| {
            let pos = line.chars().nth(movement).unwrap();
            movement = (movement + right) % line.len();
            pos
        })
        .filter(|pos| *pos == TREE)
        .count()
}

fn solve2(input: &str) -> usize {
    const SLOPES: [(usize, usize); 5] = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];
    SLOPES
        .iter()
        .map(|slope| solve1(input, *slope))
        .fold(1, |acc, n_trees| acc * n_trees)
}

fn main() {
    let start = Instant::now();
    let solution1 = solve1(INPUT, (3,1));
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);
    
    let start = Instant::now();
    let solution2 = solve2(INPUT);
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        assert_eq!(solve1(INPUT, (3, 1)), 274)
    }

    #[test]
    fn test_solve2() {
        assert_eq!(solve2(INPUT), 6050183040)
    }
}
