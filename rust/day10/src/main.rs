use std::{collections::HashMap, time::Instant};

const INPUT: &str = include_str!("../../../input/day10.txt");

fn parse(input: &str) -> Vec<usize> {
    let mut output: Vec<usize> = input
        .lines()
        .map(|line| line.parse().expect("Invalid input!"))
        .collect();
    output.push(0); // Add source joltage
    output.sort_unstable();
    output.push(output.last().unwrap() + 3);
    output
}

fn joltage_diff(input: &[usize]) -> usize {
    let (ones, threes) = input.windows(2).fold((0, 0), |(ones, threes), window| {
        match window[1] - window[0] {
            3 => (ones, threes + 1),
            1 => (ones + 1, threes),
            _ => (ones, threes),
        }
    });
    ones * threes
}

fn reachable(input: &[usize]) -> HashMap<usize, Vec<usize>> {
    let mut map: HashMap<usize, Vec<usize>> = HashMap::with_capacity(input.len());
    for jolt in input.iter() {
        let keys: Vec<_> = map.keys().copied().collect();
        for key in keys.into_iter() {
            if jolt - key <= 3 {
                map.entry(key).and_modify(|reach| reach.push(*jolt));
            }
        }
        map.insert(*jolt, Vec::new());
    }
    map
}

fn dfs(
    graph: &HashMap<usize, Vec<usize>>,
    paths: &mut HashMap<usize, usize>,
    node: &usize,
    target: &usize,
) -> usize {
    if node == target {
        1
    } else if paths.contains_key(node) {
        *paths.get(node).unwrap()
    } else {
        let neighbors = graph.get(node).unwrap();
        let n_paths = neighbors
            .iter()
            .map(|nbr| dfs(graph, paths, nbr, target))
            .sum::<usize>();
        paths.insert(*node, n_paths);
        n_paths
    }
}

fn main() {
    let start = Instant::now();
    let input = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());
    
    let start = Instant::now();
    let solution1 = joltage_diff(&input);
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);
    
    let start = Instant::now();
    let graph = reachable(&input);
    let mut paths = HashMap::with_capacity(input.len());
    let solution2 = dfs(&graph, &mut paths, &0, &input.last().unwrap());
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2);
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let input = parse(INPUT);
        assert_eq!(joltage_diff(&input), 2030)
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        let graph = reachable(&input);
        let mut paths = HashMap::with_capacity(input.len());
        assert_eq!(dfs(&graph, &mut paths, &0, &input.last().unwrap()), 42313823813632)
    }
}
