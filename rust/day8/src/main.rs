use std::collections::HashSet;
use util::split_once;

const INPUT: &str = include_str!("../../../input/day8.txt");

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

struct Program {
    src: Vec<Instruction>,
    instruction_ptr: isize,
    accumulator: isize,
    loop_detection: HashSet<usize>,
}

impl Program {
    pub fn new(input: &str) -> Self {
        use Instruction::*;
        let src = input
            .lines()
            .map(|instruction| match split_once(instruction, ' ') {
                Some(("acc", value)) => ACC(value.parse().unwrap()),
                Some(("jmp", value)) => JMP(value.parse().unwrap()),
                Some(("nop", value)) => NOP(value.parse().unwrap()),
                _ => panic!("Invalid program!"),
            })
            .collect();

        Self {
            src,
            instruction_ptr: 0,
            accumulator: 0,
            loop_detection: HashSet::new(),
        }
    }

    pub fn run(&mut self) -> Result<isize, isize> {
        use Instruction::*;
        let src_len = self.src.len();
        while self.loop_detection.insert(self.instruction_ptr as usize) {
            let instruction_ptr = self.instruction_ptr as usize;
            let next_instruction_ptr = match self.src.get(instruction_ptr) {
                Some(NOP(_)) => self.instruction_ptr + 1,
                Some(ACC(value)) => {
                    self.accumulator += value;
                    self.instruction_ptr + 1
                }
                Some(JMP(value)) => self.instruction_ptr + value,
                None => panic!("Tried executing out of bounds!"),
            } as usize;
            if instruction_ptr == src_len - 1 && next_instruction_ptr == src_len {
                return Ok(self.accumulator);
            }
            self.instruction_ptr = next_instruction_ptr as isize;
        }
        Err(self.accumulator)
    }

    pub fn fix(&mut self) -> Result<isize, ()> {
        use Instruction::*;
        self.reset();
        if let Ok(acc) = self.run() {
            return Ok(acc);
        }

        let trace: Vec<(usize, Instruction, Instruction)> = self
            .loop_detection
            .iter()
            .filter_map(|instr_ptr| match self.src[*instr_ptr] {
                JMP(value) => Some((*instr_ptr, JMP(value), NOP(value))),
                NOP(value) => Some((*instr_ptr, NOP(value), JMP(value))),
                _ => None,
            })
            .collect();

        for (instr_ptr, orig_instr, new_instr) in trace.into_iter() {
            self.src[instr_ptr] = new_instr;
            let acc = self.run();
            self.src[instr_ptr] = orig_instr;
            self.reset();
            if let Ok(acc) = acc {
                return Ok(acc);
            }
        }
        Err(())
    }

    pub fn reset(&mut self) {
        self.instruction_ptr = 0;
        self.accumulator = 0;
        self.loop_detection = HashSet::new();
    }
}

fn main() {
    let mut prog = Program::new(INPUT);
    let solution1 = prog.run().expect_err("The programm should loop");
    println!("Solution 1: {}", solution1);
    prog.reset();
    let solution2 = prog.fix().unwrap();
    println!("Solution 2: {}", solution2);
}


#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_solve1() {
        let mut prog = Program::new(INPUT);
        assert_eq!(prog.run(), Err(1928))
    }

    #[test]
    fn test_solve2() {
        let mut prog = Program::new(INPUT);
        assert_eq!(prog.fix(), Ok(1319))
    }
}
