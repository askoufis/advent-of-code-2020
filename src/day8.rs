use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

#[derive(Debug, PartialEq, Clone)]
enum Instruction {
    ACC(isize),
    JMP(isize),
    NOP(isize),
}

impl FromStr for Instruction {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(' ').collect();

        let operation = parts[0];
        let argument = parts[1];

        let mut chars = argument.chars();
        let sign = chars.next().unwrap();
        let value = chars.collect::<String>().parse()?;

        let instruction_argument = if sign == '+' { value } else { -1 * value };

        if operation == "acc" {
            Ok(Instruction::ACC(instruction_argument))
        } else if operation == "jmp" {
            Ok(Instruction::JMP(instruction_argument))
        } else {
            Ok(Instruction::NOP(instruction_argument))
        }
    }
}

#[derive(Debug, PartialEq)]
enum ProgramResult {
    Terminate(isize),
    Loop(isize),
}

fn execute_instruction(instruction: &Instruction, acc: &mut isize, pc: &mut isize) {
    match instruction {
        Instruction::ACC(arg) => {
            *acc += arg;
            *pc += 1;
        }
        Instruction::JMP(arg) => {
            *pc += arg;
        }
        Instruction::NOP(_) => {
            *pc += 1;
        }
    }
}

fn analyse_program(program: &[Instruction]) -> ProgramResult {
    let mut acc: isize = 0;
    let mut pc: isize = 0;
    let mut executed: HashSet<isize> = HashSet::new();

    let result;

    loop {
        if executed.contains(&pc) {
            result = ProgramResult::Loop(acc);
            break;
        };

        executed.insert(pc);

        let instruction = program.get(pc as usize);
        match instruction {
            Some(i) => execute_instruction(i, &mut acc, &mut pc),
            None => {
                result = ProgramResult::Terminate(acc);
                break;
            }
        }
    }

    result
}

fn flip_instruction(instructions: &[Instruction], index: usize) -> Vec<Instruction> {
    let mut altered_instructions = instructions.to_vec();

    let instruction = altered_instructions.get(index).unwrap();
    match instruction {
        Instruction::JMP(v) => altered_instructions[index] = Instruction::NOP(*v),
        Instruction::NOP(v) => altered_instructions[index] = Instruction::JMP(*v),
        _ => {}
    }

    altered_instructions
}

#[aoc_generator(day8)]
fn input_generator(input: &str) -> Vec<Instruction> {
    input
        .lines()
        .map(|instruction| Instruction::from_str(instruction).unwrap())
        .collect()
}

#[aoc(day8, part1)]
fn part1(instructions: &[Instruction]) -> isize {
    match analyse_program(instructions) {
        ProgramResult::Loop(acc) => acc,
        _ => 0,
    }
}

#[aoc(day8, part2)]
fn part2(instructions: &[Instruction]) -> isize {
    let num_instructions = instructions.len();

    for i in 0..num_instructions {
        let current_instruction = instructions.get(i).unwrap();
        match current_instruction {
            Instruction::ACC(_) => {}
            _ => {
                let altered_instructions = flip_instruction(instructions, i);
                match analyse_program(&altered_instructions) {
                    ProgramResult::Loop(_) => {}
                    ProgramResult::Terminate(acc) => return acc,
                };
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parse_nop_test() {
        let input = "nop +69";
        let result = Instruction::from_str(&input).unwrap();

        let expected = Instruction::NOP(69);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_acc_positive_test() {
        let input = "acc +8";
        let result = Instruction::from_str(&input).unwrap();

        let expected = Instruction::ACC(8);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_acc_negative_test() {
        let input = "acc -23";
        let result = Instruction::from_str(&input).unwrap();

        let expected = Instruction::ACC(-23);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_jmp_positive_test() {
        let input = "jmp +132";
        let result = Instruction::from_str(&input).unwrap();

        let expected = Instruction::JMP(132);

        assert_eq!(result, expected);
    }

    #[test]
    fn parse_jmp_negative_test() {
        let input = "jmp -193";
        let result = Instruction::from_str(&input).unwrap();

        let expected = Instruction::JMP(-193);

        assert_eq!(result, expected);
    }

    #[test]
    fn execute_nop_test() {
        let instruction = Instruction::NOP(32);
        let mut acc = 0;
        let mut pc = 0;

        execute_instruction(&instruction, &mut acc, &mut pc);

        assert_eq!(acc, 0);
        assert_eq!(pc, 1);
    }

    #[test]
    fn execute_acc_test() {
        let instruction = Instruction::ACC(-23);
        let mut acc = 0;
        let mut pc = 0;

        execute_instruction(&instruction, &mut acc, &mut pc);

        assert_eq!(acc, -23);
        assert_eq!(pc, 1);
    }

    #[test]
    fn execute_jmp_test() {
        let instruction = Instruction::JMP(20);
        let mut acc = 0;
        let mut pc = 0;

        execute_instruction(&instruction, &mut acc, &mut pc);

        assert_eq!(acc, 0);
        assert_eq!(pc, 20);
    }
}
