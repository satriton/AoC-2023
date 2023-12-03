use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut result: i32 = 0;
    for line in lines {
        result += extract_code_from_string(line);
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

#[cfg(test)]
mod tests {
    use crate::extract_code_from_string;

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

}