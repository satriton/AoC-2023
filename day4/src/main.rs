use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

        let mut result = 0;
    for line in lines {
        result += count_card_value(line.split(": ").last().unwrap().to_string())
    }

    println!("{result}");
}

fn count_card_value(card: String) -> u32 {
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

    if correct_number <= 1 {
        correct_number
    } else {
        2_u32.pow(correct_number-1)
    }
}

fn string_numbers_to_vec_number(number_str: &str) -> Vec<u32> {
    number_str.split_whitespace().map(|s| s.parse::<u32>().unwrap()).collect::<Vec<u32>>()
}

#[cfg(test)]
mod tests {
    use crate::count_card_value;

    #[test]
    fn should_count_card_value() {
        let input = "41 48 83 86 17 | 83 86  6 31 17  9 48 53".to_string();
        assert_eq!(count_card_value(input), 8);
        
        let input = "41 92 73 84 69 | 59 84 76 51 58  5 54 83".to_string();
        assert_eq!(count_card_value(input), 1);

        let input = "31 18 13 56 72 | 74 77 10 23 35 67 36 11".to_string();
        assert_eq!(count_card_value(input), 0);
    }
}