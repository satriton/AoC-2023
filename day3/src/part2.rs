use std::cmp::min;

use crate::{extract_number_from_line, Number};

pub fn solve_part2(lines: &Vec<String>) {
    let mut previous_line_numbers = vec![];

    let mut result = 0;
    for (index, line) in lines.iter().enumerate() {
        let numbers = extract_number_from_line(&line);
        let next_line_numbers = extract_number_from_line(&lines[min(index + 1, lines.len() - 1)]);

        result += compute_gear(line, &previous_line_numbers, &numbers, &next_line_numbers);

        previous_line_numbers = numbers;
    }

    println!("Part2: {result}")
}

fn compute_gear(
    line: &String,
    previous_line_numbers: &Vec<Number>,
    line_numbers: &Vec<Number>,
    next_line_numbers: &Vec<Number>,
) -> u32 {
    let mut sum = 0;
    for (index, character) in line.char_indices() {
        if character.eq(&'*') {
            let gear_numbers = numbers_in_range(
                index,
                previous_line_numbers.to_vec(),
                line_numbers.to_vec(),
                next_line_numbers.to_vec(),
            );

            if gear_numbers.len() == 2 {
                sum += gear_numbers[0].value * gear_numbers[1].value;
            }
        }
    }
    sum
}

fn numbers_in_range(
    index: usize,
    previous_line_numbers: Vec<Number>,
    mut line_numbers: Vec<Number>,
    mut next_line_numbers: Vec<Number>,
) -> Vec<Number> {
    let mut numbers = previous_line_numbers;
    numbers.append(&mut line_numbers);
    numbers.append(&mut next_line_numbers);

    let mut gear_numbers = vec![];
    for number in numbers {
        for index_within_range in (index - 1)..=(index + 1) {
            if (&number.first_digit_index..=&number.last_digit_index).contains(&&index_within_range)
            {
                gear_numbers.push(number);
                break;
            }
        }
    }

    gear_numbers
}

#[cfg(test)]
mod tests {
    use crate::{
        part2::{compute_gear, numbers_in_range},
        Number,
    };

    #[test]
    fn should_get_all_number_next_to_number() {
        //"467..114.."
        //"...*3....."
        //"..35..633."
        let previous_line_numbers = vec![
            Number {
                value: 467,
                first_digit_index: 0,
                last_digit_index: 2,
            },
            Number {
                value: 114,
                first_digit_index: 5,
                last_digit_index: 7,
            },
        ];
        let line_numbers = vec![Number {
            value: 3,
            first_digit_index: 4,
            last_digit_index: 4,
        }];
        let next_line_numbers = vec![
            Number {
                value: 35,
                first_digit_index: 2,
                last_digit_index: 3,
            },
            Number {
                value: 633,
                first_digit_index: 6,
                last_digit_index: 8,
            },
        ];

        let result = numbers_in_range(3, previous_line_numbers, line_numbers, next_line_numbers);
        assert_eq!(
            result,
            vec![
                Number {
                    value: 467,
                    first_digit_index: 0,
                    last_digit_index: 2,
                },
                Number {
                    value: 3,
                    first_digit_index: 4,
                    last_digit_index: 4,
                },
                Number {
                    value: 35,
                    first_digit_index: 2,
                    last_digit_index: 3,
                }
            ]
        );
    }

    #[test]
    fn should_compte_gear_if_next_to_two_number() {
        //"467..114.."
        let line = &"...*......".to_string();
        //"..35..633."
        let previous_line_numbers = vec![
            Number {
                value: 467,
                first_digit_index: 0,
                last_digit_index: 2,
            },
            Number {
                value: 114,
                first_digit_index: 5,
                last_digit_index: 7,
            },
        ];
        let line_numbers = vec![];
        let next_line_numbers = vec![
            Number {
                value: 35,
                first_digit_index: 2,
                last_digit_index: 3,
            },
            Number {
                value: 633,
                first_digit_index: 6,
                last_digit_index: 8,
            },
        ];

        let sum = compute_gear(
            line,
            &previous_line_numbers,
            &line_numbers,
            &next_line_numbers,
        );
        assert_eq!(sum, 16345);
    }
}
