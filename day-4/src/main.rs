use std::{collections::HashMap, fs};

fn transform(input: &str) -> HashMap<(isize, isize), char> {
    let mut map = HashMap::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert((x as isize, y as isize), char);
        }
    }
    map
}

fn collect_string(
    map: &HashMap<(isize, isize), char>,
    starting_point: (isize, isize),
    vector: (isize, isize),
) -> String {
    let mut result: Vec<char> = vec![*map.get(&starting_point).unwrap()];

    let mut found_char: Option<&char> = result.first();
    let mut cursor = starting_point;
    while found_char.is_some() {
        cursor.0 += vector.0;
        cursor.1 += vector.1;
        found_char = map.get(&cursor);
        if let Some(next_char) = map.get(&cursor) {
            result.push(*next_char);
        }
    }
    return result.iter().collect();
}

fn find_xmas(input: &str, word_to_find: &str) -> usize {
    let map = transform(input);
    let mut count = 0usize;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let height = input.lines().count();
    let width = lines.first().unwrap().chars().count();
    for y in 0..height {
        for x in 0..width {
            let x = x as isize;
            let y = y as isize;

            let char = map.get(&(x, y)).unwrap();
            if *char != 'X' {
                continue;
            }
            for i in 0..=8 {
                let vec_x = i % 3 - 1;
                let vec_y = i / 3 - 1;
                let vector = (vec_x, vec_y);
                if vector == (0, 0) {
                    continue;
                }
                let result = collect_string(&map, (x, y), vector);

                if result.starts_with(word_to_find) {
                    count += 1;
                }
            }
        }
    }

    count
}

fn main() {
    let test = fs::read_to_string("./src/puzzle-input.txt").unwrap();
    let count = find_xmas(&test, "XMAS");
    println!("count = {}", count);
}

#[test]
fn test_find_xmas() {
    let test = fs::read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(find_xmas(&test, "XMAS"), 18);
}

#[test]
fn test_simple() {
    let test = fs::read_to_string("./src/test-simple.txt").unwrap();
    assert_eq!(find_xmas(&test, "XMAS"), 4);
}
