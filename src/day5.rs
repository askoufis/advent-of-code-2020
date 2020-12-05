#[derive(PartialEq, Debug)]
struct Seat {
    row: String,
    column: String,
}

#[aoc_generator(day5)]
fn input_generator(input: &str) -> Vec<Seat> {
    input
        .lines()
        .map(|line| {
            let row = line.clone().chars().take(7).collect();
            let column = line.clone().chars().skip(7).take(3).collect();

            Seat { row, column }
        })
        .collect()
}

fn get_row_or_column(row_or_column: &String, lower: usize, upper: usize) -> usize {
    let mut lower = lower.clone();
    let mut upper = upper.clone();
    let mut range = 2usize.pow(row_or_column.len() as u32) / 2usize;

    let mut row_chars = row_or_column.chars();

    while lower != upper {
        let c = row_chars.next().unwrap();
        if c == 'F' || c == 'L' {
            upper -= range;
        } else {
            lower += range;
        }

        range /= 2;
    }

    lower
}

fn get_seat_id(seat: &Seat) -> usize {
    let row = get_row_or_column(&seat.row, 0, 127);
    let column = get_row_or_column(&seat.column, 0, 7);

    row * 8 + column
}

#[aoc(day5, part1)]
fn part1(seats: &[Seat]) -> usize {
    seats.iter().map(|seat| get_seat_id(seat)).max().unwrap()
}

#[aoc(day5, part2)]
fn part2(seats: &[Seat]) -> usize {
    let mut seat_ids: Vec<usize> = seats.iter().map(|seat| get_seat_id(seat)).collect();
    seat_ids.sort();

    let num_ids = seat_ids.len();

    for index in 0..num_ids {
        if index > 0 && index < num_ids - 1 {
            let first = seat_ids[index];
            let second = seat_ids[index + 1];
            if second - first > 1 {
                return second - 1;
            }
        }
    }

    0
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn input_generator_test() {
        let input = "BFFFBBFRRR\nFFFBBBFRRR";
        let expected = vec![
            Seat {
                row: String::from("BFFFBBF"),
                column: String::from("RRR"),
            },
            Seat {
                row: String::from("FFFBBBF"),
                column: String::from("RRR"),
            },
        ];

        let result = input_generator(&input);
        assert_eq!(expected, result);
    }

    #[test]
    fn get_row_or_column_test() {
        let seat = Seat {
            row: String::from("BFFFBBF"),
            column: String::from("RRR"),
        };

        let result1 = get_row_or_column(&seat.row, 0, 127);
        let result2 = get_row_or_column(&seat.column, 0, 7);
        let expected1 = 70;
        let expected2 = 7;

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }

    #[test]
    fn test_get_seat_id() {
        let seat1 = Seat {
            row: String::from("BFFFBBF"),
            column: String::from("RRR"),
        };

        let seat2 = Seat {
            row: String::from("FFFBBBF"),
            column: String::from("RRR"),
        };

        let result1 = get_seat_id(&seat1);
        let expected1 = 567;
        let result2 = get_seat_id(&seat2);
        let expected2 = 119;

        assert_eq!(result1, expected1);
        assert_eq!(result2, expected2);
    }
}
