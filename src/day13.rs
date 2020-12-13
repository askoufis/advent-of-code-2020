#[derive(Clone)]
struct Timetable {
    earliest: usize,
    buses: Vec<usize>,
}

#[derive(Copy, Clone, PartialEq, Debug)]
enum BusTime {
    Bus(usize),
    X,
}

impl BusTime {
    fn get_time(&self) -> usize {
        match self {
            BusTime::Bus(t) => *t,
            BusTime::X => panic!("oops"),
        }
    }
}

fn lcm(first: usize, second: usize) -> usize {
    first * second / gcd(first, second)
}

fn gcd(first: usize, second: usize) -> usize {
    let mut max = first;
    let mut min = second;
    if min > max {
        let val = max;
        max = min;
        min = val;
    }

    loop {
        let res = max % min;
        if res == 0 {
            return min;
        }

        max = min;
        min = res;
    }
}

#[aoc_generator(day13)]
fn input_generator(input: &str) -> (Timetable, Vec<BusTime>) {
    let mut lines = input.lines();
    let earliest = lines.next().unwrap().parse().unwrap();

    let buses = lines
        .next()
        .unwrap()
        .split(',')
        .filter(|s| *s != "x")
        .map(|s| s.parse().unwrap())
        .collect();

    let timetable = Timetable { earliest, buses };

    let bus_times_str = input.lines().skip(1).next().unwrap();
    let bus_times = bus_times_str
        .split(',')
        .map(|s| {
            if s == "x" {
                BusTime::X
            } else {
                BusTime::Bus(s.parse().unwrap())
            }
        })
        .collect();

    (timetable, bus_times)
}

#[aoc(day13, part1)]
fn part1(t: &(Timetable, Vec<BusTime>)) -> usize {
    let timetable = t.0.clone();
    let mut bus_times: Vec<(usize, usize)> = timetable
        .buses
        .iter()
        .map(|time| {
            [time]
                .iter()
                .cycle()
                .enumerate()
                .skip(1)
                .map(|(index, bus)| (index * **bus, **bus))
                .skip_while(|(bus_time, _)| *bus_time < timetable.earliest)
                .next()
                .unwrap()
        })
        .collect();

    bus_times.sort_by(|a, b| a.0.cmp(&b.0));
    (bus_times[0].0 - timetable.earliest) * bus_times[0].1
}

#[derive(Debug)]
struct Item {
    time: usize,
    lcm: usize,
}

#[aoc(day13, part2)]
fn part2(t: &(Timetable, Vec<BusTime>)) -> usize {
    let bus_times = t.1.clone();
    let delay = bus_times.len() - 1;
    let initial_value = Item {
        time: bus_times[0].get_time(),
        lcm: bus_times[0].get_time(),
    };
    let result = bus_times[1..]
        .iter()
        .fold(initial_value, |acc, bus_time| match bus_time {
            BusTime::X => Item {
                time: acc.time + 1,
                lcm: acc.lcm,
            },
            BusTime::Bus(t) => {
                let new_lcm = lcm(*t, acc.lcm);
                let multplier = (0..)
                    .skip_while(|&x| (acc.time + acc.lcm * x + 1) % t != 0)
                    .next()
                    .unwrap();
                Item {
                    time: acc.time + acc.lcm * multplier + 1,
                    lcm: new_lcm,
                }
            }
        });
    result.time - delay
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"939
7,13,x,x,59,x,31,19";
        let generated_input = input_generator(&input);
        let result = part1(&generated_input);
        let expected = 295;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test1() {
        let input = r"939
7,13,x,x,59,x,31,19";
        let generated_input = input_generator(&input);
        let result = part2(&generated_input);
        let expected = 1068781;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test2() {
        let input = r"939
2,7";
        let generated_input = input_generator(&input);
        let result = part2(&generated_input);
        let expected = 6;
        assert_eq!(result, expected);
    }

    #[test]
    fn part2_test3() {
        let input = r"939
1789,37,47,1889";
        let generated_input = input_generator(&input);
        let result = part2(&generated_input);
        let expected = 1202161486;
        assert_eq!(result, expected);
    }

    #[test]
    fn lcm_test() {
        assert_eq!(lcm(4, 6), 12);
        assert_eq!(lcm(15, 20), 60);
    }
}
