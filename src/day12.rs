use std::num::ParseIntError;
use std::str::FromStr;

const RIGHT_ROTATION_ORDER: [Direction; 4] =
    [Direction::N, Direction::E, Direction::S, Direction::W];
const LEFT_ROTATION_ORDER: [Direction; 4] =
    [Direction::N, Direction::W, Direction::S, Direction::E];

#[derive(Clone, Copy, Debug)]
enum Action {
    F { steps: usize },
    N { steps: usize },
    S { steps: usize },
    E { steps: usize },
    W { steps: usize },
    L { degrees: usize },
    R { degrees: usize },
}

impl FromStr for Action {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut iter = s.chars();
        let c = iter.next().unwrap();

        let num: usize = iter.collect::<String>().parse().unwrap();

        match c {
            'F' => Ok(Action::F { steps: num }),
            'N' => Ok(Action::N { steps: num }),
            'S' => Ok(Action::S { steps: num }),
            'E' => Ok(Action::E { steps: num }),
            'W' => Ok(Action::W { steps: num }),
            'L' => Ok(Action::L { degrees: num }),
            'R' => Ok(Action::R { degrees: num }),
            _ => panic!("Bad input"),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum Direction {
    N,
    E,
    S,
    W,
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

struct Ship {
    position: Point,
    facing: Direction,
    waypoint: Point,
}

impl Ship {
    fn new() -> Self {
        Ship {
            position: Point { x: 0, y: 0 },
            facing: Direction::E,
            waypoint: Point { x: 10, y: 1 },
        }
    }

    fn execute_action(&mut self, action: Action) {
        match action {
            Action::N { steps } => self.move_n(steps),
            Action::S { steps } => self.move_s(steps),
            Action::E { steps } => self.move_e(steps),
            Action::W { steps } => self.move_w(steps),
            Action::F { steps } => self.move_f(steps),
            Action::L { degrees } => self.rotate_l(degrees),
            Action::R { degrees } => self.rotate_r(degrees),
        }
    }

    fn execute_waypoint_action(&mut self, action: Action) {
        match action {
            Action::N { steps } => self.move_waypoint_n(steps),
            Action::S { steps } => self.move_waypoint_s(steps),
            Action::E { steps } => self.move_waypoint_e(steps),
            Action::W { steps } => self.move_waypoint_w(steps),
            Action::F { steps } => self.move_f2(steps),
            Action::L { degrees } => self.rotate_waypoint_l(degrees),
            Action::R { degrees } => self.rotate_waypoint_r(degrees),
        }
    }

    fn move_f(&mut self, steps: usize) {
        match self.facing {
            Direction::N => self.move_n(steps),
            Direction::S => self.move_s(steps),
            Direction::E => self.move_e(steps),
            Direction::W => self.move_w(steps),
        }
    }

    fn move_f2(&mut self, steps: usize) {
        self.position.x += self.waypoint.x * steps as isize;
        self.position.y += self.waypoint.y * steps as isize;
    }

    fn move_n(&mut self, steps: usize) {
        self.position.y += steps as isize;
    }

    fn move_waypoint_n(&mut self, steps: usize) {
        self.waypoint.y += steps as isize;
    }

    fn move_s(&mut self, steps: usize) {
        self.position.y -= steps as isize;
    }

    fn move_waypoint_s(&mut self, steps: usize) {
        self.waypoint.y -= steps as isize;
    }

    fn move_e(&mut self, steps: usize) {
        self.position.x += steps as isize;
    }

    fn move_waypoint_e(&mut self, steps: usize) {
        self.waypoint.x += steps as isize;
    }

    fn move_w(&mut self, steps: usize) {
        self.position.x -= steps as isize;
    }

    fn move_waypoint_w(&mut self, steps: usize) {
        self.waypoint.x -= steps as isize;
    }

    fn rotate_l(&mut self, degrees: usize) {
        let turns = degrees / 90;
        let current_order_index = LEFT_ROTATION_ORDER
            .iter()
            .position(|f| self.facing == *f)
            .unwrap();
        let new_index = (current_order_index + turns) % 4;
        self.facing = LEFT_ROTATION_ORDER[new_index];
    }

    fn rotate_waypoint_l(&mut self, degrees: usize) {
        let turns = degrees / 90;
        let mut new_waypoint = self.waypoint.clone();
        for _ in 0..turns {
            let new_x = -new_waypoint.y;
            let new_y = new_waypoint.x;
            new_waypoint = Point { x: new_x, y: new_y };
        }
        self.waypoint = new_waypoint;
    }

    fn rotate_r(&mut self, degrees: usize) {
        let turns = degrees / 90;
        let current_order_index = RIGHT_ROTATION_ORDER
            .iter()
            .position(|f| self.facing == *f)
            .unwrap();
        let new_index = (current_order_index + turns) % 4;
        self.facing = RIGHT_ROTATION_ORDER[new_index];
    }

    fn rotate_waypoint_r(&mut self, degrees: usize) {
        let turns = degrees / 90;
        let mut new_waypoint = self.waypoint.clone();
        for _ in 0..turns {
            let new_x = new_waypoint.y;
            let new_y = -new_waypoint.x;
            new_waypoint = Point { x: new_x, y: new_y };
        }
        self.waypoint = new_waypoint;
    }

    fn get_manhattan_distance(&self) -> usize {
        (self.position.x.abs() + self.position.y.abs()) as usize
    }
}

#[aoc_generator(day12)]
fn input_generator(input: &str) -> Vec<Action> {
    input
        .lines()
        .map(|line| Action::from_str(line).unwrap())
        .collect()
}

#[aoc(day12, part1)]
fn part1(actions: &[Action]) -> usize {
    let mut ship = Ship::new();

    actions
        .iter()
        .for_each(|action| ship.execute_action(*action));

    ship.get_manhattan_distance()
}

#[aoc(day12, part2)]
fn part2(actions: &[Action]) -> usize {
    let mut ship = Ship::new();

    actions.iter().for_each(|action| {
        ship.execute_waypoint_action(*action);
    });

    ship.get_manhattan_distance()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"F10
N3
F7
R90
F11";
        let generated_input = input_generator(&input);
        let result = part1(&generated_input);
        let expected = 25;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = r"F10
N3
F7
R90
F11";
        let generated_input = input_generator(&input);
        let result = part2(&generated_input);
        let expected = 286;
        assert_eq!(result, expected);
    }
}
