use std::str::FromStr;

fn contains_brackets(s: &str) -> bool {
    s.contains('(') || s.contains(')')
}

fn is_symbol(s: char) -> bool {
    s == '+' || s == '*'
}

fn is_mul(s: &str) -> bool {
    s.contains('*')
}

fn is_add(s: &str) -> bool {
    s.contains('+')
}

fn remove_spaces(s: &str) -> String {
    s.trim().chars().filter(|&c| c != ' ').collect()
}

#[derive(Clone, Debug, PartialEq)]
enum Equation {
    Expression(Expression),
    BracketedExpression(Expression),
    Value(usize),
}

impl FromStr for Equation {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let clean_str: String = s.trim().chars().filter(|&c| c != ' ').collect();
        if is_add(&clean_str) || is_mul(&clean_str) {
            if contains_brackets(&clean_str) {
                todo!()
            } else {
                todo!()
            }
        } else {
            Ok(Equation::Value(s.parse().unwrap()))
        }
    }
}

impl Equation {
    fn evaluate(&self) -> usize {
        match self {
            Equation::Expression(e) => e.evaluate(),
            Equation::BracketedExpression(e) => e.evaluate(),
            Equation::Value(v) => *v,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
enum Expression {
    Add(Box<Equation>, Box<Equation>),
    Mul(Box<Equation>, Box<Equation>),
}

impl Expression {
    fn evaluate(&self) -> usize {
        match self {
            Expression::Add(l, r) => l.evaluate() + r.evaluate(),
            Expression::Mul(l, r) => l.evaluate() * r.evaluate(),
        }
    }

    fn parse_no_brackets(s: &str) -> Self {
        let first_symbol_index = s.find(|c| is_symbol(c)).unwrap();
        let l_value = &s[0..first_symbol_index];
        let r = &s[first_symbol_index + 1..];
        println!("l: {}, r: {}", l_value, r);
        let l = Box::new(Equation::Value(l_value.parse().unwrap()));
        let r = Box::new(Equation::from_str(&r).unwrap());
        if is_mul(s) {
            Expression::Mul(l, r)
        } else {
            Expression::Add(l, r)
        }
    }
}

#[aoc_generator(day18)]
fn input_generator(input: &str) -> Vec<Equation> {
    todo!()
}

#[aoc(day18, part1)]
fn part1(input: &[Equation]) -> usize {
    input.iter().map(Equation::evaluate).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn statement_test() {
        let s1 = Equation::Value(3);
        let s2 = Equation::Expression(Expression::Add(Box::new(s1.clone()), Box::new(s1.clone())));
        let s3 = Equation::Expression(Expression::Mul(Box::new(s1.clone()), Box::new(s1.clone())));
        let s4 = Equation::Expression(Expression::Mul(Box::new(s2.clone()), Box::new(s3.clone())));

        assert_eq!(s1.evaluate(), 3);
        assert_eq!(s2.evaluate(), 6);
        assert_eq!(s3.evaluate(), 9);
        assert_eq!(s4.evaluate(), 54);
    }

    #[test]
    fn parse_simple_expression_test() {
        let ex1 = "4*5";
        let result1 = Expression::parse_no_brackets(ex1);
        let expected1 = Expression::Mul(Box::new(Equation::Value(4)), Box::new(Equation::Value(5)));
        assert_eq!(expected1, result1);

        let ex2 = "3+4*5";
        let result2 = Expression::parse_no_brackets(ex2);
        let expected2 = Expression::Add(
            Box::new(Equation::Value(3)),
            Box::new(Equation::Expression(expected1.clone())),
        );

        assert_eq!(expected2, result2);
    }
}
