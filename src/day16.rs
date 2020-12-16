use std::collections::VecDeque;
use std::ops::RangeInclusive;
use std::str::FromStr;

type Ranges = (RangeInclusive<usize>, RangeInclusive<usize>);

#[derive(Debug, Clone, PartialEq)]
struct Field {
    name: String,
    ranges: Ranges,
}

#[derive(Debug)]
struct Puzzle {
    fields: Vec<Field>,
    your_ticket: Vec<usize>,
    nearby_tickets: Vec<Vec<usize>>,
}

fn validate_ranges(value: usize, ranges: &Ranges) -> bool {
    ranges.0.contains(&value) || ranges.1.contains(&value)
}

fn parse_ticket(ticket_line: &str) -> Vec<usize> {
    ticket_line
        .split(',')
        .map(usize::from_str)
        .map(Result::unwrap)
        .collect()
}

#[aoc_generator(day16)]
fn input_generator(input: &str) -> Puzzle {
    let mut iter = input.split("\n\n");
    let fields_str = iter.next().unwrap().trim();
    let your_ticket_str = iter.next().unwrap().trim();
    let nearby_tickets_str = iter.next().unwrap().trim();

    let fields = fields_str
        .lines()
        .map(|line| {
            let mut line_iter = line.split(": ");
            let name = line_iter.next().unwrap().to_string();
            let mut ranges_iter = line_iter.next().unwrap().split(" or ").map(|r| {
                let min_max: Vec<usize> = r
                    .split('-')
                    .map(usize::from_str)
                    .map(Result::unwrap)
                    .collect();
                RangeInclusive::new(min_max[0], min_max[1])
            });
            let ranges = (ranges_iter.next().unwrap(), ranges_iter.next().unwrap());

            Field { name, ranges }
        })
        .collect();

    let your_ticket = parse_ticket(your_ticket_str.lines().nth(1).unwrap());

    let nearby_tickets = nearby_tickets_str
        .lines()
        .skip(1)
        .map(parse_ticket)
        .collect();

    Puzzle {
        fields,
        your_ticket,
        nearby_tickets,
    }
}

#[aoc(day16, part1)]
fn part1(input: &Puzzle) -> usize {
    input
        .nearby_tickets
        .iter()
        .map(|ticket| {
            ticket
                .iter()
                .filter(|&&value| {
                    !input
                        .fields
                        .iter()
                        .any(|field| validate_ranges(value, &field.ranges))
                })
                .sum::<usize>()
        })
        .sum()
}

fn get_valid_tickets(tickets: &[Vec<usize>], fields: &[Field]) -> Vec<Vec<usize>> {
    tickets
        .iter()
        .filter(|ticket| {
            ticket.iter().all(|&value| {
                fields
                    .iter()
                    .any(|field| validate_ranges(value, &field.ranges))
            })
        })
        .cloned()
        .collect()
}

fn get_field_order(valid_tickets: &[Vec<usize>], fields: &[Field]) -> Vec<String> {
    let num_fields = fields.len();
    let mut mut_fields: Vec<&Field> = Vec::new();
    fields.iter().for_each(|f| mut_fields.push(f));
    let mut field_order: Vec<(usize, String)> = Vec::with_capacity(num_fields);
    let mut field_values: VecDeque<(usize, Vec<usize>)> = (0..num_fields)
        .map(|i| valid_tickets.iter().map(|ticket| ticket[i]).collect())
        .enumerate()
        .collect();
    while field_values.len() > 0 {
        let values = field_values.pop_front().unwrap();
        let matching_fields: Vec<_> = mut_fields
            .iter()
            .filter(|field| {
                values
                    .1
                    .iter()
                    .all(|&value| validate_ranges(value, &field.ranges))
            })
            .collect();
        match matching_fields.len() {
            1 => {
                field_order.push((values.0, matching_fields[0].name.clone()));
                mut_fields = mut_fields
                    .iter()
                    .filter(|&field| field.name != matching_fields[0].name)
                    .cloned()
                    .collect();
            }
            _ => field_values.push_back(values),
        }
    }
    field_order.sort_by_key(|k| k.0);
    field_order.iter().map(|f| f.1.clone()).collect()
}

#[aoc(day16, part2)]
fn part2(input: &Puzzle) -> usize {
    let valid_tickets = get_valid_tickets(&input.nearby_tickets, &input.fields);
    let field_order = get_field_order(&valid_tickets, &input.fields);
    field_order
        .iter()
        .enumerate()
        .filter_map(|field| match field.1.starts_with("departure") {
            true => Some(input.your_ticket[field.0]),
            false => None,
        })
        .product()
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
    #[test]
    fn get_field_order_test() {
        let input = r"class: 0-1 or 4-19
row: 0-5 or 8-19
seat: 0-13 or 16-19

your ticket:
11,12,13

nearby tickets:
3,9,18
15,1,5
5,14,9";
        let generated_input = input_generator(input);
        let result = get_field_order(&generated_input.nearby_tickets, &generated_input.fields);
        let expected = vec!["row", "class", "seat"];
        assert_eq!(result, expected);
    }
}
