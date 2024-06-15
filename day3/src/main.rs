use std::{
    cmp::{max, min},
    fs::read_to_string,
};

use part2::solve_part2;
use regex::Regex;

mod part2;

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    solve_part1(&lines);
    solve_part2(&lines);
}

#[derive(Debug, PartialEq, Clone)]
struct Number {
    value: u32,
    first_digit_index: usize,
    last_digit_index: usize,
}

fn solve_part1(lines: &Vec<String>) {
    let mut previous_line = &String::new();

    let mut result = 0;
    for (index, line) in lines.iter().enumerate() {
        let numbers = extract_number_from_line(&line);
        let next_line = &lines[min(index + 1, lines.len() - 1)];

        result += sum_number_next_to_symbole(previous_line, line, next_line, numbers);
        previous_line = line;
    }

    println!("Part1: {result}")
}

fn sum_number_next_to_symbole(
    previous_line: &String,
    line: &String,
    next_line: &String,
    line_numbers: Vec<Number>,
) -> u32 {
    let mut sum = 0;
    for number in line_numbers {
        let range_start = number.first_digit_index as i32 - 1;
        let range_end = number.last_digit_index + 1;
        if is_symbole_in_range(&previous_line, range_start, range_end)
            || is_symbole_in_range(&line, range_start, range_end)
            || is_symbole_in_range(&next_line, range_start, range_end)
        {
            sum += number.value
        }
    }

    sum
}

fn is_symbole_in_range(line: &str, range_start: i32, range_end: usize) -> bool {
    let regex = Regex::new(r"[0-9.]*[^0-9.][0-9.]*").unwrap();
    if line.is_empty() {
        return false;
    }
    regex.is_match(&line[max(range_start, 0) as usize..min(range_end + 1, line.len()) as usize])
}

fn extract_number_from_line(line: &String) -> Vec<Number> {
    let mut current_number = String::new();
    let mut first: Option<usize> = None;

    let mut numbers = vec![];
    for (index, character) in line.char_indices() {
        let is_digit = character.is_ascii_digit();
        let is_last = index == line.len() - 1;

        if is_digit && is_last {
            match first {
                Some(first) => {
                    current_number.push(character);
                    numbers.push(Number {
                        value: current_number.parse::<u32>().unwrap(),
                        first_digit_index: first,
                        last_digit_index: index,
                    });
                    return numbers;
                }
                None => {
                    numbers.push(Number {
                        value: character.to_digit(10).unwrap(),
                        first_digit_index: index - 1,
                        last_digit_index: index - 1,
                    });
                    return numbers;
                }
            }
        }

        if first.is_none() && is_digit {
            first = Some(index);
            current_number.push(character);
        } else if first.is_some() && is_digit {
            current_number.push(character);
        } else if first.is_some() && !is_digit {
            numbers.push(Number {
                value: current_number.parse::<u32>().unwrap(),
                first_digit_index: first.unwrap(),
                last_digit_index: index - 1,
            });
            first = None;
            current_number = String::new();
        }
    }

    numbers
}

#[cfg(test)]
mod tests {
    use crate::{
        extract_number_from_line, is_symbole_in_range, sum_number_next_to_symbole, Number,
    };

    #[test]
    fn should_extract_number_from_line() {
        let line = &"467..11...7".to_string();
        let expected_number = vec![
            Number {
                value: 467,
                first_digit_index: 0,
                last_digit_index: 2,
            },
            Number {
                value: 11,
                first_digit_index: 5,
                last_digit_index: 6,
            },
            Number {
                value: 7,
                first_digit_index: 9,
                last_digit_index: 9,
            },
        ];

        let numbers = extract_number_from_line(line);
        assert_eq!(numbers, expected_number);
    }

    #[test]
    fn should_detect_if_symbole_in_range() {
        let line = "467..11.*7".to_string();

        assert!(!is_symbole_in_range(&line, -3, 5));
        assert!(is_symbole_in_range(&line, 7, 13));
    }

    #[test]
    fn should_compte_number_if_next_to_symbole() {
        let previous_line = &"/....0...7".to_string();
        let line = &"467..11.*7".to_string();
        let next_line = &"4.7....@..".to_string();
        let numbers = vec![
            Number {
                value: 467,
                first_digit_index: 0,
                last_digit_index: 2,
            },
            Number {
                value: 11,
                first_digit_index: 5,
                last_digit_index: 6,
            },
            Number {
                value: 7,
                first_digit_index: 9,
                last_digit_index: 9,
            },
        ];

        let sum = sum_number_next_to_symbole(previous_line, line, next_line, numbers);
        assert_eq!(sum, 485);
    }
}
