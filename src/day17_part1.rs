use std::{ops::RangeInclusive, str::FromStr};

use crate::vec::{Vec2, Vec3, OFFSETS_3};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum State {
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

#[derive(Debug, Clone, Copy)]
struct Cube {
    position: Vec3,
    state: State,
}

#[derive(Debug, Clone)]
struct Universe {
    state: Option<Vec<Cube>>,
    ranges: [RangeInclusive<isize>; 3],
    x_size: usize,
    y_size: usize,
    z_size: usize,
    dimension_extension: usize,
}

impl Universe {
    fn new(width: usize, height: usize, cycles: usize) -> Self {
        let dimension_extension = cycles as isize;
        let min_xyz = 0;
        let max_z = 0;
        let max_x = width as isize - 1;
        let max_y = height as isize - 1;

        let x_range = (min_xyz - dimension_extension)..=(max_x + dimension_extension);
        let y_range = (min_xyz - dimension_extension)..=(max_y + dimension_extension);
        let z_range = (min_xyz - dimension_extension)..=(max_z + dimension_extension);

        let x_size = x_range.clone().count();
        let y_size = y_range.clone().count();
        let z_size = z_range.clone().count();

        Universe {
            state: None,
            ranges: [x_range, y_range, z_range],
            x_size,
            y_size,
            z_size,
            dimension_extension: dimension_extension as usize,
        }
    }

    fn initialise_state(&mut self, initial_state: &[(Vec2, State)]) {
        let state_size = self.x_size * self.y_size * self.z_size;
        let mut state = Vec::with_capacity(state_size);
        for i in 0..state_size {
            let position = self.get_pos_from_index(i);
            state.push(Cube {
                position,
                state: State::Inactive,
            });
        }

        initial_state.iter().for_each(|c| {
            let pos = c.0.to_vec3();
            let index = self.get_cube_state_index(&pos);
            if let Some(i) = index {
                state[i] = Cube {
                    position: pos,
                    state: c.1,
                }
            }
        });

        self.state.replace(state);
    }

    fn num_active(&self) -> usize {
        match self.state.clone() {
            Some(s) => s.iter().filter(|c| c.state == State::Active).count(),
            None => 0,
        }
    }

    fn get_cube_state_index(&self, pos: &Vec3) -> Option<usize> {
        if !self.ranges[0].contains(&pos.x) {
            return None;
        }
        if !self.ranges[1].contains(&pos.y) {
            return None;
        }
        if !self.ranges[2].contains(&pos.z) {
            return None;
        }

        let shifted_pos = *pos + self.dimension_extension as isize;

        let index = shifted_pos.x
            + (shifted_pos.y * self.x_size as isize)
            + (shifted_pos.z * self.x_size as isize * self.y_size as isize);

        Some(index as usize)
    }

    fn get_pos_from_index(&self, index: usize) -> Vec3 {
        let xy_area = self.x_size * self.y_size;
        let z = index / (xy_area);
        let y = (index % (xy_area)) / self.x_size;
        let x = (index % (xy_area)) % self.x_size;

        Vec3 {
            x: x as isize,
            y: y as isize,
            z: z as isize,
        } + (-(self.dimension_extension as isize))
    }

    fn get_next_cube_state(&self, index: usize) -> State {
        let self_state = self.state.as_ref().unwrap();
        let cube = self_state[index];
        let active_neighbours = [cube.position]
            .iter()
            .cycle()
            .zip(OFFSETS_3.iter())
            .filter_map(|(&p, &o)| {
                let n_pos = p + o;
                let i = self.get_cube_state_index(&n_pos);
                match i {
                    Some(i_) => match self_state[i_].state {
                        State::Inactive => None,
                        s => Some(s),
                    },
                    None => None,
                }
            })
            .count();

        match cube.state {
            State::Active => {
                if (2..=3).contains(&active_neighbours) {
                    return State::Active;
                } else {
                    return State::Inactive;
                }
            }
            State::Inactive => {
                if active_neighbours == 3 {
                    return State::Active;
                }
            }
        }

        return cube.state;
    }

    fn tick(&mut self) {
        let mut next = self.state.clone().unwrap();

        for i in 0..next.len() {
            let next_cube_state = self.get_next_cube_state(i);
            next[i].state = next_cube_state;
        }

        self.state.replace(next);
    }
}

#[aoc_generator(day17)]
pub fn input_generator(input: &str) -> (Vec<(Vec2, State)>, (usize, usize)) {
    let initial_state = input
        .lines()
        .enumerate()
        .map(|(y, line)| {
            line.chars()
                .map(|c| State::from_str(&c.to_string()).unwrap())
                .enumerate()
                .map(|(x, state)| {
                    (
                        Vec2 {
                            x: x as isize,
                            y: y as isize,
                        },
                        state,
                    )
                })
                .collect::<Vec<(Vec2, State)>>()
        })
        .flatten()
        .collect();
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    (initial_state, (width, height))
}

#[aoc(day17, part1)]
fn part1(input: &(Vec<(Vec2, State)>, (usize, usize))) -> usize {
    let (width, height) = input.1;
    let cycles = 6;
    let mut u = Universe::new(width, height, cycles);
    u.initialise_state(&input.0);
    for _ in 0..cycles {
        u.tick();
    }

    u.num_active()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r".#.
..#
###";
        let generated_input = input_generator(input);
        let result = part1(&generated_input);
        let expected = 112;
        assert_eq!(result, expected);
    }
}
