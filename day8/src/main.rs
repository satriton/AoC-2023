use std::{collections::HashMap, fs::read_to_string};

#[derive(Debug, PartialEq)]
struct Point {
    left: String,
    right: String,
}

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut lines = lines.iter();
    let instructions = lines.next().unwrap().chars();

    let mut map = HashMap::new();

    for line in lines {
        if !line.is_empty() {
            store_line_in_map(&mut map, line)
        }
    }

    let mut current_location = "AAA".to_string();
    let mut move_number = 0;
    while current_location != "ZZZ".to_string() {
        for movement in instructions.clone() {
            compute_new_location(&map, &mut current_location, movement);
            move_number += 1;
            if current_location == "ZZZ".to_string() {
                break;
            }
        }
    }
    println!("{move_number} {current_location}");
}

fn store_line_in_map(map: &mut HashMap<String, Point>, input: &String) {
    let mut parts = input.split(" = ");
    let key = parts.next().unwrap().to_string();

    let binding = parts
        .next()
        .unwrap()
        .to_string()
        .replace("(", "")
        .replace(")", "");
    let mut point_str = binding.split(", ");
    let left = point_str.next().unwrap().to_string();
    let right = point_str.next().unwrap().to_string();

    map.insert(key, Point { left, right });
}

fn compute_new_location(
    map: &HashMap<String, Point>,
    current_location: &mut String,
    movement: char,
) {
    match movement {
        'R' => *current_location = map.get(current_location).unwrap().right.clone(),
        'L' => *current_location = map.get(current_location).unwrap().left.clone(),
        _ => panic!(),
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{compute_new_location, store_line_in_map, Point};

    #[test]
    fn should_extract_code_from_written_digits_string() {
        let mut expected: HashMap<String, Point> = HashMap::new();
        expected.insert(
            "AAA".to_string(),
            Point {
                left: "BBB".to_string(),
                right: "BBB".to_string(),
            },
        );
        let input = "AAA = (BBB, BBB)".to_string();
        let mut map: HashMap<String, Point> = HashMap::new();

        store_line_in_map(&mut map, &input);

        assert_eq!(map, expected);
    }

    #[test]
    fn should_move_from_location_to_another() {
        let mut map: HashMap<String, Point> = HashMap::new();
        map.insert(
            "AAA".to_string(),
            Point {
                left: "BBB".to_string(),
                right: "CCC".to_string(),
            },
        );

        let mut current_location = "AAA".to_string();
        let movement = 'R';

        compute_new_location(&map, &mut current_location, movement);
        assert_eq!(current_location, "CCC".to_string());
    }
}
