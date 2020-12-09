#[aoc_generator(day9)]
fn input_generator(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.parse().unwrap()).collect()
}

fn is_value_sum(value: usize, window: &[usize]) -> bool {
    (0..window.len())
        .filter(|i| {
            let current_item = window[*i];
            if value > current_item {
                let target = value - current_item;

                return window.contains(&target);
            }
            false
        })
        .count()
        > 0
}

fn real_part1(data: &[usize], window_size: usize) -> usize {
    *data
        .windows(window_size)
        .zip(data.iter().skip(window_size))
        .filter(|(window, value)| !is_value_sum(**value, window))
        .next()
        .unwrap()
        .1
}

#[aoc(day9, part1)]
fn part1(data: &[usize]) -> usize {
    real_part1(data, 25)
}

fn real_part2(value: usize, data: &[usize]) -> usize {
    let mut window_size = 2;

    loop {
        if let Some(w) = data
            .windows(window_size)
            .filter(|window| window.into_iter().sum::<usize>() == value)
            .next()
        {
            break w.iter().min().unwrap() + w.iter().max().unwrap();
        }

        window_size += 1;
    }
}

#[aoc(day9, part2)]
fn part2(data: &[usize]) -> usize {
    real_part2(real_part1(data, 25), data)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn is_value_sum_false_test() {
        let window = [95, 102, 117, 150, 182];
        let value = 127;

        let result = is_value_sum(value, &window);
        assert!(!result);
    }

    #[test]
    fn is_value_sum_true_test() {
        let window = [35, 20, 15, 25, 47];
        let value = 40;

        let result = is_value_sum(value, &window);
        assert!(result);
    }

    #[test]
    fn part1_test() {
        let data = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let result = real_part1(&data, 5);
        let expected = 127;

        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test() {
        let data = [
            35, 20, 15, 25, 47, 40, 62, 55, 65, 95, 102, 117, 150, 182, 127, 219, 299, 277, 309,
            576,
        ];

        let weakness = real_part1(&data, 5);
        let result = real_part2(weakness, &data);
        let expected = 62;

        assert_eq!(result, expected);
    }
}