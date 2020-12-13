use std::time::Instant;

const INPUT: &str = include_str!("../../../input/day13.txt");

fn parse(input: &str) -> (i64, Vec<(i64, i64)>) {
    let mut lines = input.lines();
    let earliest = lines.next().and_then(|dep| dep.parse().ok()).unwrap();
    let bus_lines = lines
        .next()
        .unwrap()
        .split(',')
        .enumerate()
        .filter_map(|(pos, id)| id.parse().ok().map(|id| (pos as i64, id)))
        .collect();
    (earliest, bus_lines)
}

fn earliest_departure(earliest: i64, bus_lines: &[(i64, i64)]) -> i64 {
    bus_lines
        .iter()
        .map(|(_, id)| (id, (earliest as f32 / *id as f32).ceil() as i64))
        .map(|(id, div)| (id, id * div))
        .min_by_key(|(_, next_departure)| *next_departure)
        .map(|(id, next_departure)| id * (next_departure - earliest))
        .unwrap()
}

fn inverse_modulo(x: i64, n: i64) -> Option<i64> {
    let (r, x, _) = exended_gcd(x, n);
    if r == 1 {
        Some((x % n + n) % n)
    } else {
        None
    }
}

fn exended_gcd(a: i64, b: i64) -> (i64, i64, i64) {
    if a == 0 {
        return (b, 0, 1);
    }
    let (r, m, n) = exended_gcd(b % a, a);
    (r, n - (b / a) * m, m)
}

fn chinese_remainder(bus_lines: &[(i64, i64)]) -> i64 {
    // from: https://rosettacode.org/wiki/Chinese_remainder_theorem#Rust
    let lines: Vec<_> = bus_lines
        .iter()
        .map(|(a, n)| (n-a, *n))
        .collect();
    let prod = lines.iter().fold(1, |acc, (_, n)| acc * n);
    lines
        .iter()
        .map(|(a, n)| {
            let p = prod / n;
            a * inverse_modulo(p, *n).unwrap() * p
        })
        .sum::<i64>()
        % prod
}

fn main() {
    let start = Instant::now();
    let (earliest, bus_lines) = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let solution1 = earliest_departure(earliest, &bus_lines);
    println!("Solution 1 took {}ns", start.elapsed().as_nanos());
    println!("Solution 1: {}", solution1);

    let start = Instant::now();
    let solution2 = chinese_remainder(&bus_lines);
    println!("Solution 2 took {}ns", start.elapsed().as_nanos());
    println!("Solution 2: {}", solution2);
}
