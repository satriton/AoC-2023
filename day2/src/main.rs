use std::{cmp, collections::HashMap, fs::read_to_string};

fn main() {
    let lines: Vec<String> = read_to_string("src/input.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let actual = HashMap::from([
        ("red".to_string(), 12),
        ("blue".to_string(), 14),
        ("green".to_string(), 13),
    ]);

    let mut sum = 0;
    let mut power_sum = 0;
    for line in lines {
        let mut colors_count = HashMap::from([
            ("red".to_string(), 0),
            ("blue".to_string(), 0),
            ("green".to_string(), 0),
        ]);

        let mut parts = line.split(": ");
        let game_number = extract_game_number(parts.next().unwrap().to_string());
        process_sets(parts.next().unwrap().to_string(), &mut colors_count);

        // part 1
        if check_possible_bag(&actual, &colors_count) {
            sum += game_number;
        }

        // part 2
        power_sum += colors_count.get("red").unwrap()
            * colors_count.get("blue").unwrap()
            * colors_count.get("green").unwrap();
    }

    println!("Sum = {sum}");
    println!("Power Sum = {power_sum}");
}

fn check_possible_bag(actual: &HashMap<String, i32>, colors_count: &HashMap<String, i32>) -> bool {
    let red = colors_count.get("red").unwrap() <= actual.get("red").unwrap();
    let blue = colors_count.get("blue").unwrap() <= actual.get("blue").unwrap();
    let green = colors_count.get("green").unwrap() <= actual.get("green").unwrap();
    red && blue && green
}

fn process_sets(sets: String, colors_count: &mut HashMap<String, i32>) {
    let splited_set = sets.split("; ");
    for set in splited_set {
        change_max_number_foreach_color_in_set(set.to_string(), colors_count);
    }
}

fn extract_game_number(game: String) -> i32 {
    game.split_whitespace()
        .last()
        .unwrap()
        .parse::<i32>()
        .unwrap()
}

fn change_max_number_foreach_color_in_set(set: String, colors_count: &mut HashMap<String, i32>) {
    let draws = set.split(", ");

    for draw in draws {
        let number = draw.split(" ").next().unwrap().parse::<i32>().unwrap();
        let color = draw.split(" ").last().unwrap();

        *colors_count.get_mut(color).unwrap() =
            cmp::max(number, colors_count.get(color).unwrap().clone());
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use crate::{
        change_max_number_foreach_color_in_set, check_possible_bag, extract_game_number,
        process_sets,
    };

    #[test]
    fn should_extract_game_number() {
        let input = "Game 4".to_string();
        assert_eq!(extract_game_number(input), 4);
    }

    #[test]
    fn should_extract_number_for_color_in_set() {
        let expected = HashMap::from([
            ("red".to_string(), 4),
            ("blue".to_string(), 3),
            ("green".to_string(), 0),
        ]);

        let mut color_count = HashMap::from([
            ("red".to_string(), 0),
            ("blue".to_string(), 0),
            ("green".to_string(), 0),
        ]);

        let input = "3 blue, 4 red".to_string();
        change_max_number_foreach_color_in_set(input, &mut color_count);
        assert_eq!(color_count, expected);
    }

    #[test]
    fn should_extract_number_from_sets() {
        let expected = HashMap::from([
            ("red".to_string(), 20),
            ("blue".to_string(), 6),
            ("green".to_string(), 13),
        ]);

        let mut color_count = HashMap::from([
            ("red".to_string(), 0),
            ("blue".to_string(), 0),
            ("green".to_string(), 0),
        ]);

        let input = "8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string();
        process_sets(input, &mut color_count);
        assert_eq!(color_count, expected);
    }

    #[test]
    fn should_validate_bag() {
        let actual = HashMap::from([
            ("red".to_string(), 20),
            ("blue".to_string(), 6),
            ("green".to_string(), 13),
        ]);

        let computed = HashMap::from([
            ("red".to_string(), 7),
            ("blue".to_string(), 4),
            ("green".to_string(), 13),
        ]);

        assert_eq!(check_possible_bag(&actual, &computed), true);
    }

    #[test]
    fn should_invalidate_bag() {
        let actual = HashMap::from([
            ("red".to_string(), 20),
            ("blue".to_string(), 6),
            ("green".to_string(), 13),
        ]);

        let computed = HashMap::from([
            ("red".to_string(), 25),
            ("blue".to_string(), 4),
            ("green".to_string(), 13),
        ]);

        assert_eq!(check_possible_bag(&actual, &computed), false);
    }
}
