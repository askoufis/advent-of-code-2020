#[derive(Debug)]
struct Position {
    x: usize,
    y: usize,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim().chars().collect::<Vec<char>>())
        .collect()
}

fn tree_map_solver_alt1(tree_map: &[Vec<char>], right_step: usize, down_step: usize) -> usize {
    let width = tree_map[0].len();
    let height = tree_map.len();
    let mut pos = Position {
        x: right_step,
        y: down_step,
    };

    let mut tree_count = 0;

    while pos.y < height {
        let map_x = pos.x % width;
        let map_y = pos.y;

        let map_row = &tree_map[map_y];
        let map_character = map_row[map_x];

        if map_character == '#' {
            tree_count += 1;
        }

        pos.x += right_step;
        pos.y += down_step;
    }

    tree_count
}

#[aoc(day3, part1, alt1)]
fn part1_alt1(tree_map: &[Vec<char>]) -> usize {
    tree_map_solver_alt1(tree_map, 3, 1)
}

#[aoc(day3, part2, alt1)]
fn part2_alt1(tree_map: &[Vec<char>]) -> usize {
    let solves = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    solves
        .iter()
        .map(|(right_step, down_step)| tree_map_solver_alt1(tree_map, *right_step, *down_step))
        .product()
}

fn tree_map_solver_alt2(tree_map: &[Vec<char>], right_step: usize, down_step: usize) -> usize {
    let width = tree_map[0].len();
    let height = tree_map.len();

    [down_step]
        .iter()
        .cycle()
        .enumerate()
        .map(|(index, down_step)| Position {
            x: ((index + 1) * right_step) % width,
            y: (index + 1) * down_step,
        })
        .take_while(|position| position.y < height)
        .map(|position| tree_map[position.y][position.x])
        .filter(|character| *character == '#')
        .count()
}

#[aoc(day3, part1, alt2)]
fn part1_alt2(tree_map: &[Vec<char>]) -> usize {
    tree_map_solver_alt2(tree_map, 3, 1)
}

#[aoc(day3, part2, alt2)]
fn part2_alt2(tree_map: &[Vec<char>]) -> usize {
    let solves = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    solves
        .iter()
        .map(|(right_step, down_step)| tree_map_solver_alt2(tree_map, *right_step, *down_step))
        .product()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#";
        let generated_input = input_generator(input);

        let result1 = part1_alt1(&generated_input);
        let result2 = part1_alt2(&generated_input);
        let expected = 7;

        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }

    #[test]
    fn part2_test() {
        let input = r"..##.......
            #...#...#..
            .#....#..#.
            ..#.#...#.#
            .#...##..#.
            ..#.##.....
            .#.#.#....#
            .#........#
            #.##...#...
            #...##....#
            .#..#...#.#";
        let generated_input = input_generator(input);

        let result1 = part2_alt1(&generated_input);
        let result2 = part2_alt2(&generated_input);
        let expected = 336;

        assert_eq!(result1, expected);
        assert_eq!(result2, expected);
    }
}
