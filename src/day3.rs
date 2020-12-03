struct Position {
    x: usize,
    y: usize,
}

#[aoc_generator(day3)]
fn input_generator(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|l| l.trim().chars().take(31).collect::<Vec<char>>())
        .collect()
}

fn tree_map_solver(tree_map: &[Vec<char>], right_step: &usize, down_step: &usize) -> usize {
    let width = tree_map[0].len();
    let height = tree_map.len();
    let mut pos = Position {
        x: *right_step,
        y: *down_step,
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

#[aoc(day3, part1)]
fn part1(tree_map: &[Vec<char>]) -> usize {
    tree_map_solver(tree_map, &3, &1)
}

#[aoc(day3, part2)]
fn part2(tree_map: &[Vec<char>]) -> usize {
    let solves = [(1, 1), (3, 1), (5, 1), (7, 1), (1, 2)];

    solves
        .iter()
        .map(|(right_step, down_step)| tree_map_solver(tree_map, right_step, down_step))
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

        let result = part1(&generated_input);
        let expected = 7;

        assert_eq!(result, expected);
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

        let result = part2(&generated_input);
        let expected = 336;

        assert_eq!(result, expected);
    }
}
