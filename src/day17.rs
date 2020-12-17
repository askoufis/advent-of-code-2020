use std::{ops::Add, str::FromStr};

const OFFSETS: [Pos; 26] = [
    Pos {
        x: -1,
        y: -1,
        z: -1,
    },
    Pos { x: -1, y: -1, z: 0 },
    Pos { x: -1, y: -1, z: 1 },
    Pos { x: -1, y: 0, z: -1 },
    Pos { x: -1, y: 0, z: 0 },
    Pos { x: -1, y: 0, z: 1 },
    Pos { x: -1, y: 1, z: -1 },
    Pos { x: -1, y: 1, z: 0 },
    Pos { x: -1, y: 1, z: 1 },
    Pos { x: 0, y: -1, z: -1 },
    Pos { x: 0, y: -1, z: 0 },
    Pos { x: 0, y: -1, z: 1 },
    Pos { x: 0, y: 0, z: -1 },
    Pos { x: 0, y: 0, z: 1 },
    Pos { x: 0, y: 1, z: -1 },
    Pos { x: 0, y: 1, z: 0 },
    Pos { x: 0, y: 1, z: 1 },
    Pos { x: 1, y: -1, z: -1 },
    Pos { x: 1, y: -1, z: 0 },
    Pos { x: 1, y: -1, z: 1 },
    Pos { x: 1, y: 0, z: -1 },
    Pos { x: 1, y: 0, z: 0 },
    Pos { x: 1, y: 0, z: 1 },
    Pos { x: 1, y: 1, z: -1 },
    Pos { x: 1, y: 1, z: 0 },
    Pos { x: 1, y: 1, z: 1 },
];

#[derive(Debug, Copy, Clone)]
struct Pos {
    x: isize,
    y: isize,
    z: isize,
}

impl Add for Pos {
    type Output = Self;

    fn add(self, other: Self) -> Self::Output {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Pos {
    fn get_neighbours(&self) -> Vec<Pos> {
        OFFSETS
            .iter()
            .zip([self].iter().cycle())
            .map(|(&o, &&s)| o + s)
            .collect()
    }
}

#[derive(Debug)]
enum State {
    Active,
    Inactive,
}

impl FromStr for State {
    type Err = core::convert::Infallible;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "." => Ok(State::Inactive),
            "#" => Ok(State::Active),
            _ => panic!("Bad input"),
        }
    }
}

#[derive(Debug)]
struct Cube {
    position: Pos,
    state: State,
}

#[derive(Debug)]
struct Universe {
    state: Vec<Cube>,
}

#[aoc_generator(day17)]
fn input_generator(input: &str) -> Universe {
    let state = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(|c| State::from_str(&c.to_string()).unwrap())
                .enumerate()
                .map(|(x, state)| Cube {
                    position: Pos {
                        x: x as isize,
                        y: y as isize,
                        z: 0,
                    },
                    state,
                })
                .collect::<Vec<Cube>>()
        })
        .flatten()
        .collect();

    Universe { state }
}

#[aoc(day17, part1)]
fn part1(input: &Universe) -> usize {
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"class: 1-3 or 5-7
row: 6-11 or 33-44
seat: 13-40 or 45-50

your ticket:
7,1,14

nearby tickets:
7,3,47
40,4,50
55,2,20
38,6,12";
        let generated_input = input_generator(input);
        let result = part1(&generated_input);
        let expected = 71;
        assert_eq!(result, expected);
    }
}
