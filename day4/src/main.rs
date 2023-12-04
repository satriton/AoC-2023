use std::{fs::read_to_string, cmp};
const NUMBER_OF_CARD_IN_INPUT: usize = 202;
fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result = 0;
    let mut cards = [1; NUMBER_OF_CARD_IN_INPUT];
    for (card_number, line) in lines.iter().enumerate() {
        // part 1
        let nb_winning = count_winning_number(line.split(": ").last().unwrap().to_string());
        result += count_card_value(nb_winning);

        // part 2
        update_copy_number(&mut cards, card_number, nb_winning as usize);
    }

    let nb_cards: i64 = cards.iter().fold(0, |acc, &x| acc + x);

    println!("Value of cards = {result}");
    println!("Number of cards = {nb_cards}");

}

fn update_copy_number(cards: &mut [i64; NUMBER_OF_CARD_IN_INPUT], current_card: usize, nb_winning: usize){
    for i in current_card+1..cmp::min(current_card+nb_winning+1, NUMBER_OF_CARD_IN_INPUT) {
        cards[i] += 1 * cards[current_card];
    }
}

fn count_card_value(nb_winning: u32) -> u32 {
    if nb_winning <= 1 {
        nb_winning
    } else {
        2_u32.pow(nb_winning-1)
    }
}

fn count_winning_number(card: String) -> u32 {
    let mut parts = card.split("|");
    let winning_numbers = string_numbers_to_vec_number(parts.next().unwrap());
    let numbers = string_numbers_to_vec_number(parts.next().unwrap());

    let correct_number = numbers.iter().fold(0, |acc, number| {
        if winning_numbers.contains(number) {
            acc + 1
        } else {
            acc
        }
    });
    correct_number
}

fn string_numbers_to_vec_number(number_str: &str) -> Vec<u32> {
    number_str.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use crate::{count_card_value, count_winning_number, update_copy_number, NUMBER_OF_CARD_IN_INPUT};

    #[test]
    fn should_count_card_value() {
        let input = "41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        assert_eq!(count_card_value(count_winning_number(input)), 8);
        
        let input = "41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string();
        assert_eq!(count_card_value(count_winning_number(input)), 1);

        let input = "31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        assert_eq!(count_card_value(count_winning_number(input)), 0);
    }

    #[test]
    fn should_update_number_of_card() {
        let mut expected = [1; NUMBER_OF_CARD_IN_INPUT];
        for i in 2..5 {
            expected[i] = 2;
        }
         

        let mut cards = [1; NUMBER_OF_CARD_IN_INPUT];
        update_copy_number(&mut cards, 1, 3);

        assert_eq!(cards, expected);
    }

    #[test]
    fn should_not_overflow() {
        let mut expected = [1; NUMBER_OF_CARD_IN_INPUT];
        for i in NUMBER_OF_CARD_IN_INPUT-1..NUMBER_OF_CARD_IN_INPUT {
            expected[i] = 2;
        }
         

        let mut cards = [1; NUMBER_OF_CARD_IN_INPUT];
        update_copy_number(&mut cards, NUMBER_OF_CARD_IN_INPUT-2, 3);

        assert_eq!(cards, expected);
    }
}