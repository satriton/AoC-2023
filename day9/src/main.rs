use std::fs::read_to_string;

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    solve_part1(&lines);
    solve_part2(lines);
}

fn solve_part2(lines: Vec<String>) {
    let mut result = 0;
    for line in lines {
        let history: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        result += compute_history_part2(history)
    }

    println!("Part 2: {result}");
}

fn compute_history_part2(history: Vec<i32>) -> i32 {
    let mut new_line = history;
    let mut beginning_values = vec![];

    while !new_line.iter().all(|&number| number == 0) {
        beginning_values.push(new_line.first().unwrap().clone());
        new_line = compute_new_line(new_line);
    }

    let mut history_value = 0;
    for value in beginning_values.iter().rev() {
        history_value = value - history_value;
    }

    history_value
}

fn solve_part1(lines: &Vec<String>) {
    let mut result = 0;
    for line in lines {
        let history: Vec<i32> = line
            .split_whitespace()
            .map(|s| s.parse::<i32>().unwrap())
            .collect();

        result += compute_history(history)
    }

    println!("Part 1: {result}");
}

fn compute_history(history: Vec<i32>) -> i32 {
    let mut new_line = history;
    let mut history_value = 0;

    while !new_line.iter().all(|&number| number == 0) {
        history_value += new_line.last().unwrap();
        new_line = compute_new_line(new_line);
    }

    history_value
}

fn compute_new_line(line: Vec<i32>) -> Vec<i32> {
    let mut new_line = vec![];
    for i in 0..(line.len() - 1) {
        new_line.push(line[i + 1] - line[i])
    }

    new_line
}

#[cfg(test)]
mod tests {
    use crate::{compute_history, compute_history_part2, compute_new_line};

    #[test]
    fn compute_difference_to_new_line() {
        let line = vec![1, 3, 6, 10, 15, 21];
        let expected_new_line = vec![2, 3, 4, 5, 6];

        let new_line = compute_new_line(line);

        assert_eq!(new_line, expected_new_line);
    }

    #[test]
    fn compute_value() {
        let line = vec![10, 13, 16, 21, 30, 45];

        let history_value = compute_history(line);

        assert_eq!(history_value, 68);
    }

    #[test]
    fn check_all_element_are_0() {
        let line = vec![0, 4, -4, 0, 0, 0];
        assert!(!line.iter().all(|&number| number == 0));

        let line2 = vec![0, 0, 0, 0, 0, 0];
        assert!(line2.iter().all(|&number| number == 0));
    }

    #[test]
    fn compute_value_part2() {
        let line = vec![10, 13, 16, 21, 30, 45];

        let history_value = compute_history_part2(line);

        assert_eq!(history_value, 5);
    }
}
