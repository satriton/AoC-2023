use std::cmp::Ordering;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Kind {
    Hight,
    One,
    Two,
    Three,
    Full,
    Four,
    Five,
}

#[derive(Debug)]
pub struct Hand {
    pub kind: Kind,
    pub bet: usize,
    pub hand: String,
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.kind.cmp(&other.kind) {
            Ordering::Equal => {
                let mut self_chars = self.hand.chars();
                let mut other_chars = other.hand.chars();

                for _ in 0..=4 {
                    let self_char = self_chars.next().unwrap_or('0');
                    let other_char = other_chars.next().unwrap_or('0');

                    match compare_card(self_char, other_char) {
                        Ordering::Equal => continue,
                        ord => return ord,
                    }
                }
                Ordering::Equal
            }
            ord => ord,
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.kind == other.kind && self.hand == other.hand
    }
}

impl Eq for Hand {}

fn compare_card(c1: char, c2: char) -> Ordering {
    // let values = "23456789TJQKA"; //part 1
    let values = "J23456789TQKA"; //part 2
    let c1_value = values.find(c1).unwrap_or(0);
    let c2_value = values.find(c2).unwrap_or(0);

    c1_value.cmp(&c2_value)
}

#[cfg(test)]
mod tests {
    use std::cmp::Ordering;

    use crate::hand::{compare_card, Hand, Kind};

    #[test]
    fn should_compare_two_card() {
        let card1 = 'K';
        let card2 = 'T';
        assert_eq!(compare_card(card1, card2), Ordering::Greater);

        let card1 = '5';
        let card2 = '5';
        assert_eq!(compare_card(card1, card2), Ordering::Equal);

        let card1 = '9';
        let card2 = 'A';
        assert_eq!(compare_card(card1, card2), Ordering::Less);
    }

    #[test]
    fn should_compare_two_hand() {
        let hand1 = Hand {
            kind: Kind::One,
            bet: 1,
            hand: "A72A4".to_string(),
        };

        let hand2 = Hand {
            kind: Kind::One,
            bet: 3,
            hand: "A72A4".to_string(),
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Equal);

        let hand2 = Hand {
            kind: Kind::One,
            bet: 3,
            hand: "A72A3".to_string(),
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Greater);

        let hand2 = Hand {
            kind: Kind::Full,
            bet: 3,
            hand: "AA4A4".to_string(),
        };
        assert_eq!(hand1.cmp(&hand2), Ordering::Less);
    }

    #[test]
    fn should_compare_hands() {
        let mut hands = vec![
            Hand {
                kind: Kind::Four,
                bet: 33,
                hand: "AAAA4".to_string(),
            },
            Hand {
                kind: Kind::Four,
                bet: 3,
                hand: "AAAA3".to_string(),
            },
            Hand {
                kind: Kind::Hight,
                bet: 2,
                hand: "49AT3".to_string(),
            },
        ];

        let expected_hands = vec![
            Hand {
                kind: Kind::Hight,
                bet: 2,
                hand: "49AT3".to_string(),
            },
            Hand {
                kind: Kind::Four,
                bet: 3,
                hand: "AAAA3".to_string(),
            },
            Hand {
                kind: Kind::Four,
                bet: 33,
                hand: "AAAA4".to_string(),
            },
        ];

        hands.sort();
        assert_eq!(hands, expected_hands);
    }
}
