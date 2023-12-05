use std::{fs::read_to_string, collections::HashMap};

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    

    println!("");
}

#[cfg(test)]
mod tests {

    #[test]
    fn should_extract_number_from_line() {
        let input = "1abc2".to_string();
        assert_eq!(12, 12);
    }
}