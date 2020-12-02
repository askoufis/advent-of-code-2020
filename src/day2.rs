#[derive(Debug)]
struct Password {
    lower_bound: usize,
    upper_bound: usize,
    required_character: char,
    password: String,
}

#[aoc_generator(day2)]
fn input_generator(input: &str) -> Vec<Password> {
    input
        .lines()
        .map(|l| {
            let mut entry = l.trim().split(" ");
            let mut bounds = entry.next().unwrap().split("-");
            let lower_bound = bounds.next().unwrap().parse().unwrap();
            let upper_bound = bounds.next().unwrap().parse().unwrap();

            let required_character = entry.next().unwrap().chars().next().unwrap();
            let password = String::from(entry.next().unwrap());

            Password {
                lower_bound,
                upper_bound,
                required_character,
                password,
            }
        })
        .collect()
}

#[aoc(day2, part1)]
fn part1(entries: &[Password]) -> usize {
    let valid_entries = entries
        .iter()
        .filter(|entry| {
            let character_count = entry
                .password
                .chars()
                .filter(|character| character == &entry.required_character)
                .collect::<Vec<char>>()
                .len();

            character_count >= entry.lower_bound && character_count <= entry.upper_bound
        })
        .collect::<Vec<&Password>>();

    valid_entries.len()
}

#[aoc(day2, part2)]
fn part2(entries: &[Password]) -> usize {
    let valid_entries = entries
        .iter()
        .filter(|entry| {
            let lower_character = entry.password.chars().nth(entry.lower_bound - 1).unwrap();
            let upper_character = entry.password.chars().nth(entry.upper_bound - 1).unwrap();

            let a = lower_character == entry.required_character;
            let b = upper_character == entry.required_character;

            (a && !b) || (!a && b)
        })
        .collect::<Vec<&Password>>();

    valid_entries.len()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = [
            Password {
                lower_bound: 1,
                upper_bound: 3,
                required_character: 'a',
                password: String::from("abcde"),
            },
            Password {
                lower_bound: 1,
                upper_bound: 3,
                required_character: 'b',
                password: String::from("cdefg"),
            },
            Password {
                lower_bound: 2,
                upper_bound: 9,
                required_character: 'c',
                password: String::from("ccccccccc"),
            },
        ];
        let valid_passwords = part1(&input);
        let expected = 2;

        assert_eq!(valid_passwords, expected);
    }

    #[test]
    fn par2_test() {
        let input = [
            Password {
                lower_bound: 1,
                upper_bound: 3,
                required_character: 'a',
                password: String::from("abcde"),
            },
            Password {
                lower_bound: 1,
                upper_bound: 3,
                required_character: 'b',
                password: String::from("cdefg"),
            },
            Password {
                lower_bound: 2,
                upper_bound: 9,
                required_character: 'c',
                password: String::from("ccccccccc"),
            },
        ];
        let valid_passwords = part2(&input);
        let expected = 1;

        assert_eq!(valid_passwords, expected);
    }
}
