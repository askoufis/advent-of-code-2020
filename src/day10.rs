#[aoc_generator(day10)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

#[aoc(day10, part1)]
fn part1(data: &[usize]) -> usize {
    let mut data_vec: Vec<usize> = data.iter().cloned().collect();
    data_vec.sort();
    let mut jumps_1 = 0;
    // Initialized to 1 since the last jump will always be a 3 jump
    let mut jumps_3 = 1;

    match data_vec[0] {
        1 => jumps_1 += 1,
        3 => jumps_3 += 1,
        _ => {}
    }

    data_vec.windows(2).for_each(|window| {
        if window[1] - window[0] == 1 {
            jumps_1 += 1;
        } else if window[1] - window[0] == 3 {
            jumps_3 += 1;
        }
    });

    jumps_1 * jumps_3
}

fn get_num_combinations(window: &[(usize, usize)]) -> usize {
    let v = window[0].0;
    window
        .iter()
        .skip(1)
        .filter(|(jolts, _com)| *jolts - v <= 3)
        .map(|(_jolts, com)| *com)
        .sum()
}

#[aoc(day10, part2)]
fn part2(data: &[usize]) -> usize {
    let mut data_vec: Vec<usize> = data.iter().cloned().collect();
    data_vec.push(data_vec.iter().max().unwrap() + 3);
    data_vec.insert(0, 0);
    data_vec.sort();
    let data_length = data_vec.len();
    // First value is the jolts, second is the number of combinations
    let mut combinations: Vec<(usize, usize)> = data_vec
        .iter()
        .zip([1].iter().cycle())
        .map(|(a, b)| (*a, *b))
        .collect();

    // Initialize last two slots
    combinations[data_length - 1].1 = 1;
    combinations[data_length - 2].1 = 1;

    // Length 3
    let first_one = &combinations[data_length - 3..data_length];
    let third_comb = get_num_combinations(first_one);
    // Initialize 3rd last slot
    combinations[data_length - 3].1 = third_comb;

    for i in (0..data_length - 3).rev() {
        let window = &combinations[i..i + 4];
        let comb = get_num_combinations(window);
        combinations[i].1 = comb;
    }

    combinations[0].1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result = part1(&input);
        let expected = 35;
        assert_eq!(result, expected);
    }

    #[test]
    fn get_num_combinations_test() {
        let input = [(4, 1), (5, 2), (6, 1), (8, 1)];
        let result = get_num_combinations(&input);
        let expected = 3;

        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test1() {
        let input1 = [16, 10, 15, 5, 1, 11, 7, 19, 6, 12, 4];
        let result1 = part2(&input1);
        let expected1 = 8;
        assert_eq!(result1, expected1);
    }

    #[test]
    fn part2_test2() {
        let input2 = [
            28, 33, 18, 42, 31, 14, 46, 20, 48, 47, 24, 23, 49, 45, 19, 38, 39, 11, 1, 32, 25, 35,
            8, 17, 7, 9, 4, 2, 34, 10, 3,
        ];
        let result2 = part2(&input2);
        let expected2 = 19208;
        assert_eq!(result2, expected2);
    }
}
