use std::time::Instant;

const INPUT: &str = include_str!("../../../input/day11.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Tile {
    Empty,
    Occupied,
    Ground,
}

fn parse(input: &str) -> (Vec<Tile>, (usize, usize)) {
    let dim_x = input.lines().count();
    let mut dim_y = 0;
    let grid: Vec<Tile> = input
        .lines()
        .flat_map(|line| {
            dim_y = line.len();
            line.chars().map(|spot| match spot {
                'L' => Tile::Empty,
                '#' => Tile::Occupied,
                '.' => Tile::Ground,
                _ => unreachable!(),
            })
        })
        .collect();
    (grid, (dim_x, dim_y))
}

fn adjacent_occupancy(grid: &[Tile], (dim_x, dim_y): (usize, usize), pos: usize) -> usize {
    assert_eq!(grid.len(), dim_x * dim_y);
    let x = pos / dim_y;
    let y = pos - x * dim_y;
    let x0 = x.checked_sub(1).unwrap_or(0);
    let y0 = y.checked_sub(1).unwrap_or(0);
    let xn = (x + 1).min(dim_x - 1);
    let yn = (y + 1).min(dim_y - 1);
    (x0..=xn)
        .map(move |x| {
            (y0..=yn)
                .filter(move |&y| dim_y * x + y != pos && grid[dim_y * x + y] == Tile::Occupied)
                .count()
        })
        .sum()
}

fn is_occupied_seat(
    grid: &[Tile],
    (_, dim_y): (usize, usize),
    (x, y): (usize, usize),
) -> Option<usize> {
    let adj = dim_y * x + y;
    match grid[adj] {
        Tile::Occupied => Some(1),
        Tile::Empty => Some(0),
        Tile::Ground => None,
    }
}

fn view_occupancy(grid: &[Tile], (dim_x, dim_y): (usize, usize), pos: usize) -> usize {
    let dims = (dim_x, dim_y);
    let x = pos / dim_y;
    let y = pos - x * dim_y;

    let mut xminr = (0..x).rev();
    let mut yminr = (0..y).rev();
    let mut xmaxr = x + 1..dim_x;
    let mut ymaxr = y + 1..dim_y;

    let occ_diag = |x: &mut dyn Iterator<Item = usize>, y: &mut dyn Iterator<Item = usize>| {
        x.zip(y)
            .find_map(|xy| is_occupied_seat(grid, dims, xy))
            .unwrap_or(0)
    };

    occ_diag(&mut xminr.clone(), &mut yminr.clone())
        + occ_diag(&mut xminr.clone(), &mut ymaxr.clone())
        + occ_diag(&mut xmaxr.clone(), &mut yminr.clone())
        + occ_diag(&mut xmaxr.clone(), &mut ymaxr.clone())
        + xminr
            .find_map(|x| is_occupied_seat(grid, dims, (x, y)))
            .unwrap_or(0)
        + xmaxr
            .find_map(|x| is_occupied_seat(grid, dims, (x, y)))
            .unwrap_or(0)
        + yminr
            .find_map(|y| is_occupied_seat(grid, dims, (x, y)))
            .unwrap_or(0)
        + ymaxr
            .find_map(|y| is_occupied_seat(grid, dims, (x, y)))
            .unwrap_or(0)
}

fn first_rule(grid: &[Tile], dims: (usize, usize), pos: usize) -> Tile {
    let occupied = adjacent_occupancy(grid, dims, pos);
    match grid[pos] {
        Tile::Empty if occupied == 0 => Tile::Occupied,
        Tile::Occupied if occupied >= 4 => Tile::Empty,
        tile => tile,
    }
}

fn second_rule(grid: &[Tile], dims: (usize, usize), pos: usize) -> Tile {
    let occupied = view_occupancy(grid, dims, pos);
    match grid[pos] {
        Tile::Empty if occupied == 0 => Tile::Occupied,
        Tile::Occupied if occupied >= 5 => Tile::Empty,
        tile => tile,
    }
}

fn evolve(
    grid: &[Tile],
    dims: (usize, usize),
    rule: fn(&[Tile], (usize, usize), usize) -> Tile,
) -> Vec<Tile> {
    let n = grid.len();
    let new = (0..n).map(|pos| rule(grid, dims, pos)).collect();
    if new == grid {
        new
    } else {
        evolve(&new, dims, rule)
    }
}

fn main() {
    let start = Instant::now();
    let (grid, dims) = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let solution1 = evolve(&grid, dims, first_rule)
        .iter()
        .filter(|&seat| seat == &Tile::Occupied)
        .count();
    println!("Solution 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", solution1);

    let start = Instant::now();
    let solution2 = evolve(&grid, dims, second_rule)
        .iter()
        .filter(|&seat| seat == &Tile::Occupied)
        .count();
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", solution2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve1() {
        let (grid, dims) = parse(INPUT);
        let solution1 = evolve(&grid, dims, first_rule)
            .iter()
            .filter(|&seat| seat == &Tile::Occupied)
            .count();
        assert_eq!(solution1, 2489)
    }

    #[test]
    fn test_solve2() {
        let (grid, dims) = parse(INPUT);
        let solution2 = evolve(&grid, dims, second_rule)
            .iter()
            .filter(|&seat| seat == &Tile::Occupied)
            .count();
        assert_eq!(solution2, 2180)
    }
}
