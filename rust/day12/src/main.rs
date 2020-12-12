use std::{ops::Add, time::Instant};

const INPUT: &str = include_str!("../../../input/day12.txt");

#[repr(i8)]
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Direction {
    North = 0,
    East = 1,
    South = 2,
    West = 3,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Instruction {
    North(i32),
    East(i32),
    South(i32),
    West(i32),
    Left(i8),
    Right(i8),
    Forward(i32),
}

#[derive(Debug)]
struct BoatState1 {
    x: i32,
    y: i32,
    direction: Direction,
}

impl Default for BoatState1 {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            direction: Direction::East,
        }
    }
}

impl BoatState1 {
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn rotate(self, rotation: i8) -> (i32, i32, Direction) {
        let direction = match (self.direction as i8 + (rotation + 4)) % 4 {
            0 => Direction::North,
            1 => Direction::East,
            2 => Direction::South,
            3 => Direction::West,
            _ => unreachable!(),
        };
        (self.x, self.y, direction)
    }
}

impl Add<&Instruction> for BoatState1 {
    type Output = BoatState1;

    fn add(self, rhs: &Instruction) -> Self::Output {
        use Instruction::*;
        let Self { x, y, direction } = self;
        let (x, y, direction) = match *rhs {
            North(val) => (x, y + val, direction),
            East(val) => (x + val, y, direction),
            South(val) => (x, y - val, direction),
            West(val) => (x - val, y, direction),
            Left(val) => self.rotate(-val),
            Right(val) => self.rotate(val),
            Forward(val) => match direction {
                Direction::North => (x, y + val, direction),
                Direction::East => (x + val, y, direction),
                Direction::South => (x, y - val, direction),
                Direction::West => (x - val, y, direction),
            },
        };
        BoatState1 { x, y, direction }
    }
}

#[derive(Debug)]
struct Boatstate2 {
    x: i32,
    y: i32,
    wx: i32,
    wy: i32,
}

impl Default for Boatstate2 {
    fn default() -> Self {
        Self {
            x: 0,
            y: 0,
            wx: 10,
            wy: 1,
        }
    }
}

impl Boatstate2 {
    pub fn pos(&self) -> (i32, i32) {
        (self.x, self.y)
    }

    fn rotate(&self, rotation: i8) -> (i32, i32, i32, i32) {
        let rotation = (rotation - 4).abs() % 4;
        let (sin, cos) = match rotation {
            0 => (0, 1),
            1 => (1, 0),
            2 => (0, -1),
            3 => (-1, 0),
            _ => unreachable!(),
        };
        (self.x, self.y, self.wx * cos - self.wy * sin, self.wx * sin + self.wy * cos)
    }
}

impl Add<&Instruction> for Boatstate2 {
    type Output = Boatstate2;

    fn add(self, rhs: &Instruction) -> Self::Output {
        use Instruction::*;
        let Self { x, y, wx, wy } = self;
        let (x, y, wx, wy) = match *rhs {
            North(val) => (x, y, wx, wy + val),
            East(val) => (x, y, wx + val, wy),
            South(val) => (x, y, wx, wy - val),
            West(val) => (x, y, wx - val, wy),
            Left(val) => self.rotate(-val),
            Right(val) => self.rotate(val),
            Forward(val) => (x + wx * val, y + wy * val, wx, wy),
        };
        Self { x, y, wx, wy }
    }
}

fn parse(input: &str) -> Vec<Instruction> {
    use Instruction::*;
    input
        .lines()
        .map(|line| {
            let (instr, val) = line.split_at(1);
            match instr {
                "N" => North(val.parse().unwrap()),
                "E" => East(val.parse().unwrap()),
                "S" => South(val.parse().unwrap()),
                "W" => West(val.parse().unwrap()),
                "L" => Left((val.parse::<u16>().unwrap() / 90) as i8),
                "R" => Right((val.parse::<u16>().unwrap() / 90) as i8),
                "F" => Forward(val.parse().unwrap()),
                _ => unreachable!(),
            }
        })
        .collect()
}

fn main() {
    let start = Instant::now();
    let input = parse(INPUT);
    println!("Parsing took {}µs", start.elapsed().as_micros());

    let start = Instant::now();
    let (x, y) = input
        .iter()
        .fold(BoatState1::default(), |state, instr| state + instr)
        .pos();
    println!("Solutixon 1 took {}µs", start.elapsed().as_micros());
    println!("Solution 1: {}", x.abs() + y.abs());

    let start = Instant::now();
    let (x, y) = input
        .iter()
        .fold(Boatstate2::default(), |state, instr| state + instr)
        .pos();
    println!("Solution 2 took {}µs", start.elapsed().as_micros());
    println!("Solution 2: {}", x.abs() + y.abs());
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve1() {
        let input = parse(INPUT);
        let (x, y) = input
            .iter()
            .fold(BoatState1::default(), |state, instr| state + instr)
            .pos();
        assert_eq!(x.abs() + y.abs(), 757)
    }

    #[test]
    fn test_solve2() {
        let input = parse(INPUT);
        let (x, y) = input
            .iter()
            .fold(Boatstate2::default(), |state, instr| state + instr)
            .pos();
        assert_eq!(x.abs() + y.abs(), 51249)
    }
}
