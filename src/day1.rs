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
        let sum: usize = combination.clone().into_iter().sum();
        if sum == 2020 {
            result = combination.into_iter().product();
            break;
        }
    }

    result
}

#[aoc(day1, part2, alt1)]
fn part2_alt1(entries: &[usize]) -> usize {
    let mut result: usize = 0;

    for combination in entries.into_iter().combinations(3) {
        let sum: usize = combination.clone().into_iter().sum();
        if sum == 2020 {
            result = combination.into_iter().product();
            break;
        }
    }

    result
}

#[aoc(day1, part2, alt2)]
fn part2_alt2(entries: &[usize]) -> usize {
    entries
        .into_iter()
        .combinations(3)
        .find(|combination| {
            let sum: usize = combination.clone().into_iter().sum();
            sum == 2020
        })
        .unwrap()
        .into_iter()
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result1 = part1_alt1(&input);
        let result2 = part1_alt2(&input);
        let expected = 514579;
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn part2_test() {
        let input = [1721, 979, 366, 299, 675, 1456];
        let result1 = part2_alt1(&input);
        let result2 = part2_alt2(&input);
        let expected = 241861950;
        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }
}
