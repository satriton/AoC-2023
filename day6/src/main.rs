use std::{fs::read_to_string, ops::Range};

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let mut lines = lines.iter();

    // part 1
    // let times: Vec<u64> = lines.next().unwrap().split(": ").last().unwrap().split_whitespace().collect::<Vec<_>>().iter().map(|num| num.parse::<u64>().unwrap()).collect();
    // let distances: Vec<u64> = lines.next().unwrap().split(": ").last().unwrap().split_whitespace().collect::<Vec<_>>().iter().map(|num| num.parse::<u64>().unwrap()).collect();

    // part 2 
    let times = vec![lines.next().unwrap().split(": ").last().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap()];
    let distances = vec![lines.next().unwrap().split(": ").last().unwrap().chars().filter(|c| !c.is_whitespace()).collect::<String>().parse::<u64>().unwrap()];

    let mut answer = 1;
    for i in 0..times.len() {
        answer *= compute_number_of_possibility_to_win(times[i], distances[i])
    }

    println!("{}", answer);
}

fn compute_number_of_possibility_to_win(time_limit: u64, distance: u64) -> u64 {
    let mut number_of_possibility = 0;
    for acceleration_time in 1..time_limit {
        if (time_limit - acceleration_time) * acceleration_time > distance {
            number_of_possibility += 1;
        }
    }
    
    number_of_possibility
}

#[cfg(test)]
mod tests {
    use crate::compute_number_of_possibility_to_win;


    #[test]
    fn should_extract_schematic_number_from_line() {
        let time = 15;
        let distance = 40;

        assert_eq!(compute_number_of_possibility_to_win(time, distance), 8);
    }
}