use std::fmt;
use std::str::FromStr;

#[derive(Debug, PartialEq, Copy, Clone)]
enum Seat {
    Floor,
    Empty,
    Occupied,
}

impl Seat {
    fn is_not_floor(&self) -> bool {
        *self != Seat::Floor
    }
}

impl FromStr for Seat {
    type Err = core::convert::Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s == "L" {
            return Ok(Seat::Empty);
        }
        if s == "#" {
            return Ok(Seat::Occupied);
        }
        return Ok(Seat::Floor);
    }
}

impl fmt::Display for Seat {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        if self == &Seat::Empty {
            write!(f, "L")
        } else if self == &Seat::Occupied {
            write!(f, "#")
        } else {
            write!(f, ".")
        }
    }
}

#[derive(Clone)]
struct Universe {
    width: usize,
    height: usize,
    seats: Vec<Seat>,
}

impl fmt::Display for Universe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s: String = self
            .seats
            .chunks(self.width)
            .into_iter()
            .map(|chunk| {
                let mut new_chunk: String =
                    chunk.clone().iter().map(|seat| seat.to_string()).collect();
                new_chunk.push_str("\n");
                new_chunk
            })
            .collect();
        write!(f, "{}", s)
    }
}

#[derive(PartialEq)]
enum State {
    Stable,
    Unstable,
}

impl Universe {
    fn get_index(&self, row: isize, column: isize) -> isize {
        if row < 0 || row >= self.height as isize {
            return -1;
        } else if column < 0 || column >= self.width as isize {
            return -1;
        }

        row * self.width as isize + column
    }

    fn occupied_neighbour_count(&self, row: isize, column: isize) -> usize {
        let north = row - 1;
        let south = row + 1;
        let west = column - 1;
        let east = column + 1;

        let nw = self.get_index(north, west);
        let n = self.get_index(north, column);
        let ne = self.get_index(north, east);
        let w = self.get_index(row, west);
        let e = self.get_index(row, east);
        let sw = self.get_index(south, west);
        let s = self.get_index(south, column);
        let se = self.get_index(south, east);
        let seat_indexes = vec![nw, n, ne, w, e, sw, s, se];

        let seats = seat_indexes
            .iter()
            .filter(|i| **i != -1)
            .map(|i| self.seats[*i as usize])
            .collect::<Vec<Seat>>();

        count_occupied_seats(&seats)
    }

    fn seat_in_direction(&self, row: isize, column: isize, direction: (isize, isize)) -> Seat {
        match [direction]
            .iter()
            .cycle()
            .enumerate()
            .skip(1)
            .map(|(i, d)| {
                let rc = (row + (i as isize * d.0), column + (i as isize * d.1));
                self.get_index(rc.0, rc.1)
            })
            .take_while(|index| *index != -1)
            .map(|index| self.seats[index as usize])
            .find(|seat| seat.is_not_floor())
        {
            Some(seat) => seat,
            None => Seat::Floor,
        }
    }

    fn occupied_visible_count(&self, row: isize, column: isize) -> usize {
        // (row transform, column transform)
        let north = (-1, 0);
        let nw = (-1, -1);
        let ne = (-1, 1);
        let south = (1, 0);
        let sw = (1, -1);
        let se = (1, 1);
        let west = (0, -1);
        let east = (0, 1);
        let directions = [north, nw, ne, south, sw, se, west, east];

        directions
            .iter()
            .map(|d| self.seat_in_direction(row, column, *d))
            .filter(|seat| *seat == Seat::Occupied)
            .count()
    }

    fn get_next_seat_state(&self, seat: &Seat, row: isize, column: isize) -> Seat {
        if *seat == Seat::Floor {
            return Seat::Floor;
        } else {
            let num_occupied = self.occupied_neighbour_count(row, column);

            if *seat == Seat::Empty && num_occupied == 0 {
                return Seat::Occupied;
            } else if *seat == Seat::Occupied && num_occupied >= 4 {
                return Seat::Empty;
            }
        }

        return *seat;
    }

    fn get_next_seat_state2(&self, seat: &Seat, row: isize, column: isize) -> Seat {
        if *seat == Seat::Floor {
            return Seat::Floor;
        } else {
            let num_occupied_visible = self.occupied_visible_count(row, column);

            if *seat == Seat::Empty && num_occupied_visible == 0 {
                return Seat::Occupied;
            } else if *seat == Seat::Occupied && num_occupied_visible >= 5 {
                return Seat::Empty;
            }
        }

        return *seat;
    }

    fn tick(&mut self) -> State {
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row as isize, column as isize);
                let current_seat = self.seats[index as usize];
                let next_seat_state =
                    self.get_next_seat_state(&current_seat, row as isize, column as isize);

                next[index as usize] = next_seat_state;
            }
        }

        if next == self.seats {
            return State::Stable;
        } else {
            self.seats = next;
            return State::Unstable;
        }
    }

    fn tick2(&mut self) -> State {
        let mut next = self.seats.clone();

        for row in 0..self.height {
            for column in 0..self.width {
                let index = self.get_index(row as isize, column as isize);
                let current_seat = self.seats[index as usize];
                let next_seat_state =
                    self.get_next_seat_state2(&current_seat, row as isize, column as isize);

                next[index as usize] = next_seat_state;
            }
        }

        if next == self.seats {
            return State::Stable;
        } else {
            self.seats = next;
            return State::Unstable;
        }
    }
}

fn count_occupied_seats(seats: &[Seat]) -> usize {
    seats.iter().filter(|seat| seat == &&Seat::Occupied).count()
}

#[aoc_generator(day11)]
fn input_generator(input: &str) -> Universe {
    let input_lines = input.lines();
    let seats = input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| Seat::from_str(&c.to_string()).unwrap())
                .collect::<Vec<Seat>>()
        })
        .flatten()
        .collect();
    let width = input.lines().next().unwrap().chars().count();
    let height = input.lines().count();

    Universe {
        width,
        height,
        seats,
    }
}

#[aoc(day11, part1)]
fn part1(state: &Universe) -> usize {
    let mut u = state.clone();

    loop {
        let tick_result = &mut u.tick();
        if tick_result == &State::Stable {
            break;
        }
    }

    count_occupied_seats(&u.seats)
}

#[aoc(day11, part2)]
fn part2(state: &Universe) -> usize {
    let mut u = state.clone();

    loop {
        let tick_result = &mut u.tick2();
        if tick_result == &State::Stable {
            break;
        }
    }

    count_occupied_seats(&u.seats)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let generated_input = input_generator(&input);
        let result = part1(&generated_input);
        let expected = 37;
        assert_eq!(result, expected);
    }

    #[test]
    fn occupied_visible_count_test() {
        let input = r".......#.
...#.....
.#.......
.........
..#L....#
....#....
.........
#........
...#.....";
        let generated_input = input_generator(&input);
        let result = generated_input.occupied_visible_count(4, 3);
        let expected = 8;

        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let input = r"L.LL.LL.LL
LLLLLLL.LL
L.L.L..L..
LLLL.LL.LL
L.LL.LL.LL
L.LLLLL.LL
..L.L.....
LLLLLLLLLL
L.LLLLLL.L
L.LLLLL.LL";
        let generated_input = input_generator(&input);
        let result = part2(&generated_input);
        let expected = 26;
        assert_eq!(result, expected);
    }
}
