const INPUT: &str = include_str!("../../../input/day09.txt");

fn parse(input: &str) -> Vec<isize> {
    input
        .lines()
        .map(|n| n.parse().expect("Invalid input"))
        .collect()
}

fn sum_of_ancestors_exists(ancestors: &[isize], target: isize) -> bool {
    // See https://stackoverflow.com/questions/4720271/find-a-pair-of-elements-from-an-array-whose-sum-equals-a-given-number
    let set: Vec<isize> = ancestors.iter().copied().collect();
    set.iter()
        .any(|number| set.contains(&(target-number)))
}

#[timed::timed]
fn invalid_number(input: &[isize]) -> Option<isize> {
    input.windows(26).find_map(|window| {
        if !sum_of_ancestors_exists(&window[..25], window[25]) {
            Some(window[25])
        } else {
            None
        }
    })
}

#[timed::timed]
fn invalid_number_summands<'a>(input: &'a [isize], target: isize) -> Option<&'a [isize]> {
    (2..input.len()).find_map(|i| {
        input
            .windows(i)
            .find(|&window| window.iter().sum::<isize>() == target)
    })
}

fn main() {
    let input = parse(INPUT);
    let solution1 = invalid_number(&input).expect("No invalid number found!");
    println!("Solution 1: {}", solution1);
    let solution2 = {
        let summands =
            invalid_number_summands(&input, solution1).expect("No continuous slice found");
        let min = summands.iter().min();
        let max = summands.iter().max();
        min.zip(max)
            .map(|(min, max)| min + max)
            .expect("Empty list")
    };
    println!("Solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let input = parse(INPUT);
        assert_eq!(invalid_number(&input), Some(104054607))
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        let solution1 = invalid_number(&input).expect("No invalid number found!");
        let solution2 = {
            let summands =
                invalid_number_summands(&input, solution1).expect("No continuous slice found");
            let min = summands.iter().min();
            let max = summands.iter().max();
            min.zip(max)
                .map(|(min, max)| min + max)
                .expect("Empty list")
        };
        assert_eq!(solution2, 13935797)
    }
}
