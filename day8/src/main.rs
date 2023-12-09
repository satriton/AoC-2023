use std::{collections::HashMap, fs::read_to_string};

use num_integer::lcm;

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
    let mut starting_points = Vec::new();

    for line in lines {
        if !line.is_empty() {
            let point = store_line_in_map(&mut map, line);
            if point.ends_with("A") {
                starting_points.push(point);
            }
        }
    }

    // part 1
    println!("part 1: {}", solve_part_1(&instructions, &map));

    // part 2
    println!(
        "part 2: {}",
        solve_part_2(number_foreach_starting_point(
            &starting_points,
            &instructions,
            &map
        ))
    );
}

fn solve_part_1(instructions: &std::str::Chars<'_>, map: &HashMap<String, Point>) -> u64 {
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

    move_number
}

fn number_foreach_starting_point(
    starting_points: &Vec<String>,
    instructions: &std::str::Chars<'_>,
    map: &HashMap<String, Point>,
) -> Vec<u64> {
    let current_locations = starting_points;
    let mut moves = Vec::new();

    for current_location in current_locations {
        let mut move_number = 0;
        let mut current_location = current_location.clone();
        while !current_location.ends_with("Z") {
            for movement in instructions.clone() {
                compute_new_location(&map, &mut current_location, movement);
                move_number += 1;
                if current_location.ends_with("Z") {
                    break;
                }
            }
        }
        moves.push(move_number);
    }
    moves
}

fn solve_part_2(starting_points: Vec<u64>) -> u64 {
    starting_points.into_iter().fold(1, |a, b| lcm(a, b))
}

fn store_line_in_map(map: &mut HashMap<String, Point>, input: &String) -> String {
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

    map.insert(key.clone(), Point { left, right });
    key
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

fn compute_new_locations(
    map: &HashMap<String, Point>,
    current_locations: &mut Vec<String>,
    movement: char,
) {
    current_locations
        .iter_mut()
        .for_each(|current_location| compute_new_location(map, current_location, movement));
}

fn check_all_end_z(current_locations: &mut Vec<String>) -> bool {
    current_locations
        .iter()
        .fold(true, |acc, current_location| {
            acc && current_location.ends_with("Z")
        })
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        check_all_end_z, compute_new_location, compute_new_locations, store_line_in_map, Point,
    };

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

    #[test]
    fn should_compute_new_locations() {
        let mut map: HashMap<String, Point> = HashMap::new();
        map.insert(
            "AAA".to_string(),
            Point {
                left: "BBB".to_string(),
                right: "CCC".to_string(),
            },
        );
        map.insert(
            "BBB".to_string(),
            Point {
                left: "ZZZ".to_string(),
                right: "CCC".to_string(),
            },
        );

        let mut current_locations = vec!["AAA".to_string(), "BBB".to_string()];
        let expected = vec!["BBB".to_string(), "ZZZ".to_string()];
        let movement = 'L';

        compute_new_locations(&map, &mut current_locations, movement);
        assert_eq!(current_locations, expected);
    }

    #[test]
    fn should_all_end_z() {
        let mut input = vec!["AAZ".to_string(), "HSZ".to_string(), "PPZ".to_string()];

        assert_eq!(check_all_end_z(&mut input), true);
    }

    #[test]
    fn should_not_end_z() {
        let mut input = vec!["AAZ".to_string(), "HSN".to_string(), "PPZ".to_string()];

        assert_eq!(check_all_end_z(&mut input), false);
    }
}
