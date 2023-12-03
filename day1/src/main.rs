use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result: i32 = 0;
    for line in lines {
        //part 1
        // result += extract_code_from_string(line);
        //part 2
        result += extract_code_from_string(convert_written_digits_to_number(line));
    }

    println!("{result}");
}

fn extract_code_from_string(string: String) -> i32 {
    let accepted_char = vec!['0','1','2','3','4','5','6','7','8','9'];
    let mut keeped_chars = Vec::new();
    for c in string.chars() {
        if accepted_char.contains(&c) {
            keeped_chars.push(c);
        }
    }

    extract_first_and_last_digits(keeped_chars)
}

fn extract_first_and_last_digits(chars: Vec<char>) -> i32 {
    let mut ans = String::new();
    ans.push(chars.first().unwrap().clone());
    ans.push(chars.last().unwrap().clone());

    ans.parse().unwrap()
}

fn convert_written_digits_to_number(string: String) -> String {
    let accepted_string = HashMap::from([("zero", "z0ero"),("one", "o1ne"),("two", "t2wo"),("three", "t3hree"),("four", "f4our"),("five", "f5ive"),("six", "s6ix"),("seven", "s7even"),("eight", "e8ight"),("nine", "n9ine")]);
    let mut converted_string = string.clone();
    for (written_number, converted_number) in accepted_string.iter() {
        converted_string = converted_string.replace(written_number, converted_number);
    }

    converted_string.clone()
}

#[cfg(test)]
mod tests {
    use crate::{extract_code_from_string, convert_written_digits_to_number};

    #[test]
    fn should_extract_number_from_line() {
        let input = "1abc2".to_string();
        assert_eq!(extract_code_from_string(input), 12);
    }

    #[test]
    fn should_extract_first_and_last_digits() {
        let input = "a1b2c3d4e5f".to_string();
        assert_eq!(extract_code_from_string(input), 15);
    }

    #[test]
    fn should_double_single_digits() {
        let input = "treb7uchet".to_string();
        assert_eq!(extract_code_from_string(input), 77);
    }

    #[test]
    fn should_convert_written_digits_to_number() {
        let input = "zoneight234".to_string();
        assert_eq!(convert_written_digits_to_number(input), "zo1ne8ight234");
    }

    #[test]
    fn should_extract_code_from_written_digits_string() {
        let input = "zoneight234".to_string();
        assert_eq!(extract_code_from_string(convert_written_digits_to_number(input)), 14);
    }
}