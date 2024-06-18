use std::{fs::read_to_string, ops::Range, cmp};

#[derive(Debug, PartialEq)]
struct MapEntry {
    destination: i64,
    source: i64,
    range_length: i64,
}

fn main() {
    let mut lines: Vec<String> = read_to_string("src/input.txt") 
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    // Part 1
    let min_location = compute_part_1(&mut lines.clone());
    
    // Part 2
    let ranges: Vec<Range<i64>> = extract_ranges_part_2(lines.remove(0));
    let maps = extract_maps(&mut lines);
    
    let mut min_location_2 = i64::MAX;
    for range in ranges {
        min_location_2 = cmp::min(min_location_of_range(range, &maps), min_location_2);
        println!("min of range: {min_location_2}");
    } 

    println!("Part 1: {}", min_location);
    println!("Part 2: {}", min_location_2);
}

fn min_location_of_range(range: Range<i64>, maps: &Vec<Vec<MapEntry>>) -> i64 {
    let mut min_location = i64::MAX;
    for seed in range {
        let mut number = seed as i64;
        for map in maps {
            number = convert_number_with_map(number, map);
        }
        min_location = cmp::min(number, min_location);
    }

    min_location
}

fn compute_part_1(lines: &mut Vec<String>) -> i64 {
    let seeds: Vec<i64> = extract_seeds(lines.remove(0));

    let maps = extract_maps(lines);

    let mut locations = Vec::new();
    for seed in seeds {
        let mut number = seed as i64;
        for map in &maps {
            number = convert_number_with_map(number, map);
        }
        locations.push(number);
    }

    *locations.iter().min().unwrap()
}

fn extract_maps(lines: &mut Vec<String>) -> Vec<Vec<MapEntry>> {
    let mut current_map = 0;
    
    let mut maps = Vec::new();
    let mut lines = lines.iter_mut();
    while let Some(line) = lines.next() {
        if line.contains("map:") {
            maps.push(Vec::new());
            current_map += 1;
        }
        if line.chars().any(|c| c.is_numeric()) {
            maps[current_map - 1].push(extract_map_entry(line.clone()));
        }
    }

    maps
}

fn extract_seeds(string: String) -> Vec<i64> {
    let mut parts = string.split(": ").last().unwrap().split_whitespace();

    let mut seeds = Vec::new();
    while let Some(seed) = parts.next() {
        seeds.push(seed.parse().unwrap())
    }

    seeds
}

fn extract_ranges_part_2(string: String) -> Vec<Range<i64>> {
    let parts = string.split(": ").last().unwrap().split_whitespace();

    let seeds: Vec<i64> = parts.map(|elem| elem.parse::<i64>().unwrap()).collect();
    let mut ranges = Vec::new();
    for i in (0..seeds.len()).step_by(2) {
        let range = seeds[i]..seeds[i] + seeds[i+1] - 1;
        ranges.push(range);
    }

    ranges
}

fn extract_map_entry(string: String) -> MapEntry {
    let mut parts = string.split_whitespace();
    MapEntry { destination: parts.next().unwrap().parse().unwrap(), source: parts.next().unwrap().parse().unwrap(), range_length: parts.next().unwrap().parse().unwrap() }
}

fn convert_number_with_map(source_number: i64, map: &Vec<MapEntry>) -> i64{
    for map_entry in map {
        if (source_number > map_entry.source) && (source_number < map_entry.source + map_entry.range_length){
            return source_number + (map_entry.destination - map_entry.source)
        }
    }

    source_number
}

#[cfg(test)]
mod tests {
    use crate::{MapEntry, extract_map_entry, extract_seeds, convert_number_with_map, extract_ranges_part_2};

    #[test]
    fn should_extract_map_entry() {
        let expect= MapEntry { destination: 50, source: 98, range_length: 2 };

        let input = "50 98 2".to_string();

        assert_eq!(extract_map_entry(input), expect);
    }

    #[test]
    fn should_extract_seeds() {
        let expect= vec![79, 14, 55, 13];

        let input = "seeds: 79 14 55 13".to_string();

        assert_eq!(extract_seeds(input), expect);
    }

    #[test]
    fn should_convert_number_with_map_if_no_mapping_exist() {
        let map = vec![MapEntry{destination: 50, source: 98, range_length:2}, MapEntry{destination: 52, source: 50, range_length:48}];

        assert_eq!(convert_number_with_map(10, &map), 10);
    }

    #[test]
    fn should_convert_number_with_map_if_mapping_exist() {
        let map = vec![MapEntry{destination: 50, source: 98, range_length:2}, MapEntry{destination: 52, source: 50, range_length:48}];

        assert_eq!(convert_number_with_map(79, &map), 81);
    }

    #[test]
    fn should_extract_ranges_part_2() {
        let expect= vec![79..81, 12..16];

        let input = "seeds: 79 3 12 5".to_string();

        assert_eq!(extract_ranges_part_2(input), expect);
    }
}