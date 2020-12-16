use std::collections::HashMap;

#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

fn get_spoken_number(input: &[usize], turn: usize) -> usize {
    let mut spoken_numbers: HashMap<usize, Vec<usize>> = HashMap::new();
    input.iter().enumerate().for_each(|(index, number)| {
        let n = spoken_numbers.entry(*number).or_insert(vec![]);
        n.push(index);
    });

    let mut last_spoken = *input.last().unwrap();

    let input_length = input.len();

    for i in input_length..turn {
        let last_spoken_indexes = spoken_numbers.get_mut(&last_spoken).unwrap();
        let num_indexes = last_spoken_indexes.len();

        if last_spoken_indexes.len() == 1 {
            last_spoken = 0;
            spoken_numbers.get_mut(&0).unwrap().push(i);
        } else {
            let last_spoken_index = last_spoken_indexes[num_indexes - 1];
            let second_last_spoken_index = last_spoken_indexes[num_indexes - 2];
            last_spoken = last_spoken_index - second_last_spoken_index;
            let current_last_spoken_indexes = spoken_numbers.entry(last_spoken).or_insert(vec![]);
            current_last_spoken_indexes.push(i);
        }
    }

    last_spoken
}

#[aoc(day15, part1)]
fn part1(input: &[usize]) -> usize {
    get_spoken_number(input, 2020)
}

#[aoc(day15, part2)]
fn part2(input: &[usize]) -> usize {
    get_spoken_number(input, 30000000)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = vec![0, 3, 6];
        let result = part1(&input);
        let expected = 436;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = vec![0, 3, 6];
        let result = part2(&input);
        let expected = 175594;
        assert_eq!(result, expected);
    }
}
