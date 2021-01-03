use std::collections::VecDeque;

#[derive(Debug, PartialEq)]
struct Player {
    deck: VecDeque<usize>,
}

impl Player {
    fn new(deck: VecDeque<usize>) -> Self {
        Player { deck }
    }

    fn get_top_card(&mut self) -> usize {
        self.deck.pop_front().unwrap()
    }

    fn place_on_bottom(&mut self, winning_card: usize, losing_card: usize) {
        self.deck.push_back(winning_card);
        self.deck.push_back(losing_card);
    }

    fn is_empty(&self) -> bool {
        self.deck.len() == 0
    }

    fn get_deck_score(&self) -> usize {
        self.deck
            .iter()
            .rev()
            .enumerate()
            .map(|(i, &card)| (i + 1) * card)
            .sum()
    }
}

#[aoc_generator(day22)]
fn input_generator(input: &str) -> Vec<VecDeque<usize>> {
    let players = input
        .split("\n\n")
        .map(|p| p.lines().skip(1).map(|v| v.parse().unwrap()).collect())
        .take(2)
        .collect();

    players
}

fn play_game(p1_deck: VecDeque<usize>, p2_deck: VecDeque<usize>) -> usize {
    let mut p1 = Player::new(p1_deck);
    let mut p2 = Player::new(p2_deck);

    while !(p1.is_empty() || p2.is_empty()) {
        let p1_card = p1.get_top_card();
        let p2_card = p2.get_top_card();

        match p1_card.cmp(&p2_card) {
            std::cmp::Ordering::Greater => p1.place_on_bottom(p1_card, p2_card),
            std::cmp::Ordering::Less => p2.place_on_bottom(p2_card, p1_card),
            std::cmp::Ordering::Equal => unreachable!("oops"),
        }
    }

    if p1.is_empty() {
        p2.get_deck_score()
    } else {
        p1.get_deck_score()
    }
}

#[aoc(day22, part1)]
fn part1(input: &[VecDeque<usize>]) -> usize {
    play_game(input[0].clone(), input[1].clone())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_test() {
        let input = r"Player 1:
9
2
6
3
1

Player 2:
5
8
4
7
10";
        let generated_input = input_generator(&input);
        let result = part1(&generated_input);
        let expected = 306;

        assert_eq!(result, expected);
    }
}
