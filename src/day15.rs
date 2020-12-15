#[aoc_generator(day15)]
fn input_generator(input: &str) -> Vec<usize> {
    input.split(',').map(|num| num.parse().unwrap()).collect()
}

#[aoc(day15, part1)]
fn part1(input: &[usize]) -> usize {
    let mut spoken_numbers: Vec<usize> = Vec::with_capacity(2020);
    spoken_numbers.extend_from_slice(input);

    let input_length = input.len();

    for _ in input_length..2020 {
        let last_spoken = spoken_numbers.last().unwrap();
        let last_spoken_count = spoken_numbers
            .iter()
            .filter(|number| *number == last_spoken)
            .count();

        if last_spoken_count == 1 {
            spoken_numbers.push(0);
        } else {
            let last_spoken_indexes: Vec<(usize, &usize)> = spoken_numbers
                .iter()
                .enumerate()
                .rev()
                .filter(|(_, number)| *number == last_spoken)
                .collect();
            let last_spoken_index = last_spoken_indexes[0].0;
            let second_last_spoken_index = last_spoken_indexes[1].0;
            spoken_numbers.push(last_spoken_index - second_last_spoken_index);
        }
    }
    *spoken_numbers.last().unwrap()
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
}
