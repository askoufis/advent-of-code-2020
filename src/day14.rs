use std::collections::HashMap;

#[derive(PartialEq, Debug, Clone)]
enum Operation {
    Mask(String),
    Mem { address: usize, value: usize },
}

impl Operation {
    fn get_mask(&self) -> &str {
        match self {
            Operation::Mask(m) => m,
            _ => panic!("Not a mask"),
        }
    }
}

#[aoc_generator(day14)]
fn input_generator(input: &str) -> Vec<Operation> {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split('=');

            let left = line_split.next().unwrap().trim();
            let right = line_split.next().unwrap().trim();

            if left.starts_with("mem") {
                let address = left
                    .chars()
                    .skip(4)
                    .take_while(|c| *c != ']')
                    .collect::<String>();
                return Operation::Mem {
                    address: address.parse().unwrap(),
                    value: right.parse().unwrap(),
                };
            } else {
                return Operation::Mask(right.to_string());
            };
        })
        .collect()
}

fn apply_bitmask(bitmask: &str, value: usize) -> usize {
    let mut target = value;
    let mask: Vec<(usize, char)> = bitmask.chars().rev().enumerate().collect();

    mask.iter().for_each(|(bit_index, c)| {
        if *c != 'X' {
            if *c == '1' {
                target |= 1 << bit_index;
            } else {
                target &= !0 ^ (1 << bit_index);
            }
        }
    });

    target
}

#[aoc(day14, part1)]
fn part1(input: &[Operation]) -> usize {
    let mut memory: HashMap<usize, usize> = HashMap::new();
    let mut current_mask = input[0].clone();

    input[1..].iter().for_each(|op| match op {
        m @ Operation::Mask(_) => current_mask = m.clone(),
        Operation::Mem { address, value } => {
            let bitmask = current_mask.get_mask();
            let masked_value = apply_bitmask(bitmask, *value);
            memory.insert(*address, masked_value);
        }
    });

    memory.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_generator_test() {
        let input = r"mask = 0X10110X1001000X10X00X01000X01X01101
mem[49559] = 97
mem[18692] = 49438791";
        let generated_input = input_generator(&input);
        let expected = vec![
            Operation::Mask(String::from("0X10110X1001000X10X00X01000X01X01101")),
            Operation::Mem {
                address: 49559,
                value: 97,
            },
            Operation::Mem {
                address: 18692,
                value: 49438791,
            },
        ];
        assert_eq!(generated_input, expected);
    }

    #[test]
    fn apply_bitmask_test() {
        let bitmask = "XX1XXXX0X";
        let value = 11;
        let result = apply_bitmask(bitmask, value);
        let expected = 73;

        assert_eq!(result, expected);
    }
}
