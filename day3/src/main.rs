use std::{fs::read_to_string, ops::Range};

#[derive(Debug, PartialEq)]
struct SchematicNumber {
    number: u16,
    range: Range<u8>,
}


fn main() {
    let lines: Vec<String> = read_to_string("src/exemple.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();


    println!("");
}

fn extract_schematic_number_from_line(string: String) -> Vec<SchematicNumber>{
    for char in string.chars() {
        
    }
    Vec::new()
}

#[cfg(test)]
mod tests {
    use crate::{SchematicNumber, extract_schematic_number_from_line};


    #[test]
    fn should_extract_schematic_number_from_line() {
        let expected = vec![SchematicNumber {number: 467, range: 0..2}, SchematicNumber {number: 114, range: 5..6}];
        let input = "467..11...".to_string();
        assert_eq!(extract_schematic_number_from_line(input), expected);
    }
}