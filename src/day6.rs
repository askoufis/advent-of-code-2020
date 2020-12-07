use std::collections::HashMap;

#[aoc_generator(day6)]
fn input_generator(input: &str) -> Vec<(HashMap<char, usize>, usize)> {
    input
        .split("\n\n")
        .map(|group_answers| {
            let mut answers = HashMap::new();
            let mut count = 0;
            group_answers.trim().lines().for_each(|line| {
                count += 1;
                line.chars().for_each(|c| {
                    if c.is_ascii_alphabetic() {
                        let count = answers.entry(c).or_insert(0);
                        *count += 1;
                    }
                })
            });
            (answers, count)
        })
        .collect()
}

#[aoc(day6, part1)]
fn part1(answers: &[(HashMap<char, usize>, usize)]) -> usize {
    answers.iter().map(|(answer, _)| answer.keys().len()).sum()
}

#[aoc(day6, part2)]
fn part2(answers: &[(HashMap<char, usize>, usize)]) -> usize {
    answers
        .iter()
        .map(|(answer, count)| answer.values().filter(|value| value == &count).count())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"
abc

a
b
c

ab
ac

a
a
a
a

b
        ";
        println!("{}", input);
        let generated_input = input_generator(&input);
        let result = part1(&generated_input);
        let expected = 11;

        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = r"
abc

a
b
c

ab
ac

a
a
a
a

b
        ";
        let generated_input = input_generator(&input);
        let result = part2(&generated_input);
        let expected = 6;

        assert_eq!(result, expected);
    }
}
