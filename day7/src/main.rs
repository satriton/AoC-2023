use std::{collections::HashMap, fs::read_to_string};

use hand::{Hand, Kind};
use part2::solve_part2;

mod hand;
mod part2;

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // solve_part1(&lines); // Change compare_card string in hand.rs
    solve_part2(&lines);
}

fn solve_part1(lines: &Vec<String>) {
    let mut hands = vec![];

    for line in lines {
        hands.push(detect_hand(line))
    }

    hands.sort();

    let mut result = 0;
    for (index, hand) in hands.iter().enumerate() {
        let rank = index + 1;
        result += rank * hand.bet;
    }

    println!("Part1: {result}")
}

fn detect_hand(line: &String) -> Hand {
    let parts: Vec<&str> = line.split(' ').collect();
    let hand = parts[0].to_string();
    Hand {
        kind: detect_kind(&hand),
        bet: parts[1].parse().unwrap(),
        hand,
    }
}

fn detect_kind(hand: &String) -> Kind {
    let mut hand_cards = HashMap::new();
    for card in hand.chars() {
        let number_of_card = hand_cards.entry(card).or_insert(0);
        *number_of_card += 1;
    }

    match hand_cards.len() {
        1 => Kind::Five,
        2 => {
            if hand_cards.values().max() == Some(&4) {
                return Kind::Four;
            }
            Kind::Full
        }
        3 => {
            if hand_cards.values().max() == Some(&3) {
                return Kind::Three;
            }
            Kind::Two
        }
        4 => Kind::One,
        5 => Kind::Hight,
        _ => panic!("Not possible"),
    }
}

#[cfg(test)]
mod tests {
    use crate::{detect_hand, detect_kind, Hand, Kind};

    #[test]
    fn should_detect_hand_kind() {
        let line = String::from("AAAAA");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::Five);

        let line = String::from("32T8K");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::Hight);

        let line = String::from("32T3K");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::One);

        let line = String::from("4444K");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::Four);

        let line = String::from("444AA");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::Full);

        let line = String::from("44K9K");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::Two);

        let line = String::from("444AT");
        let result = detect_kind(&line);
        assert_eq!(result, Kind::Three);
    }

    #[test]
    fn should_detect_hand() {
        let line = String::from("444AT 234");
        let expected_hand = Hand {
            kind: Kind::Three,
            bet: 234,
            hand: "444AT".to_string(),
        };
        let result = detect_hand(&line);
        assert_eq!(result, expected_hand);
    }
}
