const INPUT: &str = include_str!("../input.txt");

#[timed::timed]
fn parse(input: &str) -> Vec<isize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[timed::timed]
fn solve1(input: &[isize]) -> isize {
    for x in input.iter() {
        for y in input.iter() {
            if x + y == 2020 {
                return x * y;
            }
        }
    }
    0
}

#[timed::timed]
fn solve2(input: &[isize]) -> isize {
    for x in input.iter() {
        for y in input.iter() {
            for z in input.iter() {
                if x + y + z == 2020 {
                    return x * y * z;
                }
            }
        }
    }
    0
}

fn main() {
    let input = parse(INPUT);
    println!("Solution 1: {}", solve1(&input));
    println!("Solution 2: {}", solve2(&input));
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let input = parse(INPUT);
        assert_eq!(solve1(&input), 988771)
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        assert_eq!(solve2(&input), 171933104)
    }
}
