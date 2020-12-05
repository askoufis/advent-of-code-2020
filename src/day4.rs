use std::collections::HashMap;

const REQUIRED_KEYS: [&str; 7] = ["byr", "iyr", "eyr", "hgt", "hcl", "ecl", "pid"];
const EYE_COLOURS: [&str; 7] = ["amb", "blu", "brn", "gry", "grn", "hzl", "oth"];

fn validate_byr(byr: &str) -> bool {
    byr.parse::<usize>()
        .map_or(false, |foo| foo >= 1920 && foo <= 2002)
}

fn validate_iyr(iyr: &str) -> bool {
    iyr.parse::<usize>()
        .map_or(false, |foo| foo >= 2010 && foo <= 2020)
}

fn validate_eyr(eyr: &str) -> bool {
    eyr.parse::<usize>()
        .map_or(false, |foo| foo >= 2020 && foo <= 2030)
}

fn validate_ecl(ecl: &str) -> bool {
    EYE_COLOURS.contains(&ecl)
}

fn validate_pid(pid: &str) -> bool {
    if pid.chars().all(|char| char.is_numeric()) {
        return pid.len() == 9;
    };

    false
}

fn validate_hgt(hgt: &str) -> bool {
    if hgt.ends_with("cm") {
        let height: Result<usize, _> = hgt.split_at(3).0.parse();
        match height {
            Ok(h) => return h >= 150 && h <= 193,
            Err(_) => return false,
        }
    }

    if hgt.ends_with("in") {
        let height: usize = hgt.split_at(2).0.parse().unwrap();
        return height >= 59 && height <= 76;
    }

    false
}

fn validate_hcl(hcl: &str) -> bool {
    let mut hcl_iter = hcl.chars();
    let first = hcl_iter.next().unwrap();
    if first != '#' {
        return false;
    }

    hcl_iter.all(|char| !char.is_uppercase() && char.is_ascii_hexdigit())
}

fn validate_value(key: &str, value: &str) -> bool {
    if key == "hcl" {
        return validate_hcl(value);
    }

    if key == "byr" {
        return validate_byr(value);
    }

    if key == "iyr" {
        return validate_iyr(value);
    }

    if key == "eyr" {
        return validate_eyr(value);
    }

    if key == "hgt" {
        return validate_hgt(value);
    }

    if key == "ecl" {
        return validate_ecl(value);
    }

    if key == "pid" {
        return validate_pid(value);
    }

    false
}

#[aoc_generator(day4)]
fn input_generator(input: &str) -> Vec<HashMap<String, String>> {
    input
        .split("\n\n")
        .map(|passport| {
            passport
                .replace('\n', " ")
                .trim()
                .split(' ')
                .map(|key_value| {
                    let pair: Vec<_> = key_value.split(':').collect();
                    let key = pair[0].to_string();
                    let value = pair[1].to_string();
                    (key, value)
                })
                .collect::<HashMap<String, String>>()
        })
        .collect()
}

#[aoc(day4, part1)]
fn part1(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|passport| {
            let filtered_passport: HashMap<&String, &String> =
                passport.iter().filter(|(k, _)| k != &"cid").collect();
            filtered_passport.keys().len() == 7
        })
        .count()
}

#[aoc(day4, part2)]
fn part2(passports: &[HashMap<String, String>]) -> usize {
    passports
        .iter()
        .filter(|passport| {
            let filtered_passport: HashMap<&String, &String> =
                passport.iter().filter(|(k, _)| k != &"cid").collect();
            if filtered_passport.keys().len() != 7 {
                return false;
            }

            REQUIRED_KEYS.iter().all(|key| {
                let value = passport.get(*key);
                match value {
                    Some(val) => validate_value(key, val),
                    None => false,
                }
            })
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test_4() {
        let input = r"
ecl:gry pid:860033327 eyr:2020 hcl:#fffffd
byr:1937 iyr:2017 cid:147 hgt:183cm

iyr:2013 ecl:amb cid:350 eyr:2023 pid:028048884
hcl:#cfa07d byr:1929

hcl:#ae17e1 iyr:2013
eyr:2024
ecl:brn pid:760753108 byr:1931
hgt:179cm

hcl:#cfa07d eyr:2025 pid:166559648
iyr:2011 ecl:brn hgt:59in
        ";
        let generated_input = input_generator(input);

        let result1 = part1(&generated_input);
        let expected = 2;

        assert_eq!(result1, expected);
    }

    #[test]
    fn part2_test_invalid() {
        let invalid_input = r"
eyr:1972 cid:100
hcl:#18171d ecl:amb hgt:170 pid:186cm iyr:2018 byr:1926

iyr:2019
hcl:#602927 eyr:1967 hgt:170cm
ecl:grn pid:012533040 byr:1946

hcl:dab227 iyr:2012
ecl:brn hgt:182cm pid:021572410 eyr:2020 byr:1992 cid:277

hgt:59cm ecl:zzz
eyr:2038 hcl:74454a iyr:2023
pid:3556412378 byr:2007n
        ";
        let invalid_generated_input = input_generator(invalid_input);

        let result1 = part2(&invalid_generated_input);
        let invalid_expected = 0;

        assert_eq!(result1, invalid_expected);
    }
    #[test]

    fn part2_test_valid() {
        let valid_input = r"
pid:087499704 hgt:74in ecl:grn iyr:2012 eyr:2030 byr:1980
hcl:#623a2f

eyr:2029 ecl:blu cid:129 byr:1989
iyr:2014 pid:896056539 hcl:#a97842 hgt:165cm

hcl:#888785
hgt:164cm byr:2001 iyr:2015 cid:88
pid:545766238 ecl:hzl
eyr:2022

iyr:2010 hgt:158cm hcl:#b6652a ecl:blu byr:1944 eyr:2021 pid:093154719
        ";
        let valid_generated_input = input_generator(valid_input);

        let result1 = part2(&valid_generated_input);
        let valid_expected = 4;

        assert_eq!(result1, valid_expected);
    }
}
