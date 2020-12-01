use itertools::Itertools;
use std::iter;

#[aoc_generator(day1)]
fn input_generator(input: &str) -> Vec<usize> {
    input
        .lines()
        .map(|l| {
            let entry = l.trim().parse().unwrap();
            entry
        })
        .collect()
}

#[aoc(day1, part1, alt1)]
fn part1_alt1(entries: &[usize]) -> usize {
    let mut result: usize = 0;

    for entry in entries {
        let entry_pair_sums: Vec<(usize, usize, usize)> = iter::repeat(entry)
            .zip(entries)
            .map(|(&left, &right)| (left + right, left, right))
            .filter(|(sum, _, _)| sum == &2020)
            .collect();

        if entry_pair_sums.len() > 0 {
            result = entry_pair_sums[0].1 * entry_pair_sums[0].2;
            break;
        }
    }

    result
}

#[aoc(day1, part1, alt2)]
fn part1_alt2(entries: &[usize]) -> usize {
    let mut result: usize = 0;

    for combination in entries.into_iter().combinations(2) {
        let sum: usize = combination.into_iter().sum();
        if sum == 2020 {
            result = sum;
            break;
        }
    }

    result
}

#[aoc(day1, part2)]
fn part2(entries: &[usize]) -> usize {
    let mut result: usize = 0;

    for combination in entries.into_iter().combinations(3) {
        let sum: usize = combination.into_iter().sum();
        if sum == 2020 {
            result = sum;
            break;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_alt1_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result = part1_alt1(&input);
        let expected = 514579;
        assert_eq!(result, expected);
    }

    #[test]
    fn part1_alt2_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result = part1_alt2(&input);
        let expected = 514579;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result = part2(&input);
        let expected = 241861950;
        assert_eq!(result, expected);
    }
}
