fn contains_brackets(s: &str) -> bool {
    s.contains('(') || s.contains(')')
}

fn is_symbol(c: char) -> bool {
    c == '+' || c == '*'
}

fn is_mul(s: &str) -> bool {
    s.contains('*')
}

fn is_add(s: &str) -> bool {
    s.contains('+')
}

fn clean_string(s: &str) -> String {
    s.trim().chars().filter(|&c| c != ' ').collect()
}

#[derive(Clone, Debug, PartialEq)]
enum Equation {
    BracketedEquation(Box<Equation>),
    Add(Box<Equation>, Box<Equation>),
    Mul(Box<Equation>, Box<Equation>),
    Value(usize),
}

fn consume_start(s: &str) -> (String, String, String) {
    let first_symbol_index = s.find(|c| is_symbol(c)).unwrap();
    let l = &s[0..first_symbol_index];
    let symbol = &s[first_symbol_index..first_symbol_index + 1];
    let r = &s[first_symbol_index + 1..];
    (l.to_string(), symbol.to_string(), r.to_string())
}

fn consume_end(s: &str) -> (String, String, String) {
    let last_symbol_index = s.rfind(|c| is_symbol(c)).unwrap();
    let l = &s[0..last_symbol_index];
    let symbol = &s[last_symbol_index..last_symbol_index + 1];
    let r = &s[last_symbol_index + 1..];
    (l.to_string(), symbol.to_string(), r.to_string())
}

fn consume_at(s: &str, index: usize) -> (String, String, String) {
    let l = &s[0..index];
    let symbol = &s[index..index + 1];
    let r = &s[index + 1..];
    (l.to_string(), symbol.to_string(), r.to_string())
}

fn consume_brackets(s: &str) -> String {
    s[1..s.len() - 1].to_string()
}

fn find_bracket_part(s: &str) -> (String, Option<(String, String)>) {
    let s_string = s.to_string();
    let s_chars: Vec<char> = s_string.chars().collect();
    let mut bracket_tally = 1;
    // We already know the first characters is a bracket
    let mut index = 1;

    while bracket_tally != 0 {
        let c = s_chars[index];

        if c == '(' {
            bracket_tally += 1;
        } else if c == ')' {
            bracket_tally -= 1;
        }
        index += 1;
    }

    let l = &s.to_string()[0..index];
    let rest = if index == s.len() {
        None
    } else {
        let symbol = &s_string[index..index + 1];
        let r = &s_string[index + 1..];
        Some((symbol.to_string(), r.to_string()))
    };

    (l.to_string(), rest)
}

impl Equation {
    fn new_add_mul(l: Equation, symbol: &str, r: Equation) -> Self {
        if is_mul(symbol) {
            Equation::Mul(Box::new(l), Box::new(r))
        } else {
            Equation::Add(Box::new(l), Box::new(r))
        }
    }

    fn evaluate(&self) -> usize {
        match self {
            Equation::BracketedEquation(e) => e.evaluate(),
            Equation::Add(l, r) => l.evaluate() + r.evaluate(),
            Equation::Mul(l, r) => l.evaluate() * r.evaluate(),
            Equation::Value(v) => *v,
        }
    }

    fn parse(s: &str) -> Equation {
        if contains_brackets(s) {
            Equation::parse_brackets(s)
        } else {
            Equation::parse_no_brackets(s)
        }
    }

    fn parse_no_brackets(s: &str) -> Self {
        if is_mul(s) || is_add(s) {
            let (l_string, symbol, r_string) = consume_end(s);
            let l = Equation::parse_no_brackets(&l_string);
            let r = Equation::parse_no_brackets(&r_string);
            Equation::new_add_mul(l, &symbol, r)
        } else {
            Equation::Value(s.parse().unwrap())
        }
    }

    fn parse_brackets(s: &str) -> Equation {
        println!("s: {}", s);
        let starts_with_bracket = s.starts_with('(');
        let ends_with_bracket = s.ends_with(')');

        if starts_with_bracket && ends_with_bracket {
            let (l_string, rest) = find_bracket_part(s);
            let l = Equation::BracketedEquation(Box::new(Equation::parse(&consume_brackets(
                &l_string,
            ))));
            match rest {
                Some((symbol, r_string)) => {
                    let r = Equation::parse(&r_string);
                    Equation::new_add_mul(l, &symbol, r)
                }
                None => l,
            }
        } else if starts_with_bracket {
            let (l_string, symbol, r_string) = consume_end(s);
            let l = Equation::parse_brackets(&l_string);
            let r = Equation::parse_no_brackets(&r_string);
            Equation::new_add_mul(l, &symbol, r)
        } else {
            let (l_string, symbol, r_string) = consume_start(s);
            let l = Equation::parse_no_brackets(&l_string);
            let r = Equation::parse_brackets(&r_string);
            println!("l: {}, r: {}", l_string, r_string);
            Equation::new_add_mul(l, &symbol, r)
        }
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Equation> {
    input
        .lines()
        .map(clean_string)
        .map(|s| Equation::parse(&s))
        .collect()
}

#[aoc(day18, part1)]
fn part1(input: &[Equation]) -> usize {
    input.iter().map(Equation::evaluate).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn equation_evaluate_test() {
        let s1 = Equation::Value(3);
        let s2 = Equation::Add(Box::new(s1.clone()), Box::new(s1.clone()));
        let s3 = Equation::Mul(Box::new(s1.clone()), Box::new(s1.clone()));
        let s4 = Equation::Mul(Box::new(s2.clone()), Box::new(s3.clone()));

        assert_eq!(s1.evaluate(), 3);
        assert_eq!(s2.evaluate(), 6);
        assert_eq!(s3.evaluate(), 9);
        assert_eq!(s4.evaluate(), 54);
    }

    #[test]
    fn consume_start_test() {
        let s = "3+4*5";
        let result = consume_start(s);

        assert_eq!(result.0, "3");
        assert_eq!(result.1, "+");
        assert_eq!(result.2, "4*5");
    }

    #[test]
    fn consume_end_test() {
        let s = "3+4*5";
        let result = consume_end(s);

        assert_eq!(result.0, "3+4");
        assert_eq!(result.1, "*");
        assert_eq!(result.2, "5");
    }

    #[test]
    fn consume_at_test() {
        let s = "(3+4)*(4+5)";
        let result = consume_at(s, 5);

        assert_eq!(result.0, "(3+4)");
        assert_eq!(result.1, "*");
        assert_eq!(result.2, "(4+5)");
    }

    #[test]
    fn consume_brackets_test() {
        let s = "(4+5)";
        let result = consume_brackets(s);
        assert_eq!(result, "4+5");
    }

    #[test]
    fn parse_no_brackets_test() {
        let e1 = "4*5";
        let result1 = Equation::parse_no_brackets(e1);
        let expected1 = Equation::Mul(Box::new(Equation::Value(4)), Box::new(Equation::Value(5)));
        assert_eq!(expected1, result1);
        assert_eq!(result1.evaluate(), 20);

        let e2 = "3+4*5";
        let result2 = Equation::parse_no_brackets(e2);
        let expected2 = Equation::Mul(
            Box::new(Equation::Add(
                Box::new(Equation::Value(3)),
                Box::new(Equation::Value(4)),
            )),
            Box::new(Equation::Value(5)),
        );

        assert_eq!(expected2, result2);
        assert_eq!(result2.evaluate(), 35);
    }

    #[test]
    fn parse_brackets_whole_test() {
        let e = "(4*5)";
        let result = Equation::parse_brackets(e);
        let expected = Equation::BracketedEquation(Box::new(Equation::Mul(
            Box::new(Equation::Value(4)),
            Box::new(Equation::Value(5)),
        )));
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_brackets_right_test() {
        let e = "(4*5)+3";
        let result = Equation::parse_brackets(e);
        let left = Equation::BracketedEquation(Box::new(Equation::Mul(
            Box::new(Equation::Value(4)),
            Box::new(Equation::Value(5)),
        )));
        let expected = Equation::Add(Box::new(left), Box::new(Equation::Value(3)));
        assert_eq!(expected, result);
    }

    #[test]
    fn parse_brackets_left_test() {
        let e = "3+(4*5)";
        let result = Equation::parse_brackets(e);
        let right = Equation::BracketedEquation(Box::new(Equation::Mul(
            Box::new(Equation::Value(4)),
            Box::new(Equation::Value(5)),
        )));
        let expected = Equation::Add(Box::new(Equation::Value(3)), Box::new(right));
        assert_eq!(expected, result);
        assert_eq!(result.evaluate(), 23);
    }

    #[test]
    fn parse_brackets_double_test() {
        let e = "(3+4)*(4*5)";
        let result = Equation::parse_brackets(e);
        let left = Equation::BracketedEquation(Box::new(Equation::Add(
            Box::new(Equation::Value(3)),
            Box::new(Equation::Value(4)),
        )));
        let right = Equation::BracketedEquation(Box::new(Equation::Mul(
            Box::new(Equation::Value(4)),
            Box::new(Equation::Value(5)),
        )));
        let expected = Equation::Mul(Box::new(left), Box::new(right));
        assert_eq!(expected, result);
        assert_eq!(result.evaluate(), 140);
    }

    #[test]
    fn eq_test1() {}

    #[test]
    fn part1_test() {
        let input = r"2 * 3 + (4 * 5)
5 + (8 * 3 + 9 + 3 * 4 * 3)
5 * 9 * (7 * 3 * 3 + 9 * 3 + (8 + 6 * 4))
((2 + 4 * 9) * (6 + 9 * 8 + 6) + 6) + 2 + 4 * 2";
        let generated_input = input_generator(input);
        println!("{:?}", generated_input);
        let result = part1(&generated_input);
        let expected = 26 + 437 + 12240 + 13632;
        let equations: Vec<_> = generated_input.iter().map(Equation::evaluate).collect();
        println!("{:?}", equations);

        assert_eq!(result, expected);
    }
}
