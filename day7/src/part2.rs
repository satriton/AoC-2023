use std::collections::HashMap;

use crate::hand::{Hand, Kind};

pub fn solve_part2(lines: &Vec<String>) {
    let mut hands = vec![];

    for line in lines {
        hands.push(detect_hand_part2(line))
    }

    hands.sort();

    let mut result = 0;
    for (index, hand) in hands.iter().enumerate() {
        let rank = index + 1;
        result += rank * hand.bet;
    }

    println!("Part2: {result}")
}

pub fn detect_hand_part2(line: &String) -> Hand {
    let parts: Vec<&str> = line.split(' ').collect();
    let hand = parts[0].to_string();
    Hand {
        kind: detect_kind_part2(&hand),
        bet: parts[1].parse().unwrap(),
        hand,
    }
}

pub fn detect_kind_part2(hand: &String) -> Kind {
    let mut hand_cards = HashMap::new();
    for card in hand.chars() {
        let number_of_card = hand_cards.entry(card).or_insert(0);
        *number_of_card += 1;
    }

    update_hand_with_joker(&mut hand_cards);

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

fn update_hand_with_joker(hand: &mut HashMap<char, usize>) {
    let joker_numbers = hand.remove(&'J');
    match joker_numbers {
        Some(joker_numbers) => {
            if joker_numbers == 5 {
                hand.insert('J', 5);
            }
            *hand.values_mut().max().unwrap() += joker_numbers;
        }
        None => {}
    }
}

#[cfg(test)]
mod tests {
    use crate::{hand::Kind, part2::detect_kind_part2};

    #[test]
    fn should_detect_hand_kind() {
        let line = String::from("T55J5");
        let result = detect_kind_part2(&line);
        assert_eq!(result, Kind::Four);

        let line = String::from("K3JJT");
        let result = detect_kind_part2(&line);
        assert_eq!(result, Kind::Three);

        let line = String::from("JJJJK");
        let result = detect_kind_part2(&line);
        assert_eq!(result, Kind::Five);
    }
}
