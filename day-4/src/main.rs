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

fn is_mas(map: &HashMap<(isize, isize), char>, starting_point: (isize, isize)) -> bool {
    let mut corner_chars = vec![];

    for i in 0..=8 {
        let vec_x = i % 3 - 1;
        let vec_y = i / 3 - 1;
        let vector = (vec_x, vec_y);
        if vector.0 == 0 || vector.1 == 0 {
            continue;
        }
        let mut cursor = vector;
        cursor.0 += starting_point.0;
        cursor.1 += starting_point.1;

        corner_chars.push(map.get(&cursor));
    }

    let corner_chars: Vec<char> = corner_chars.into_iter().flatten().copied().collect();

    if corner_chars.len() < 4 {
        return false;
    }

    // Almost 1 AM,
    let possible_configs: Vec<Vec<char>> = vec![
        vec!['M', 'M', 'S', 'S'],
        vec!['S', 'S', 'M', 'M'],
        vec!['S', 'M', 'S', 'M'],
        vec!['M', 'S', 'M', 'S'],
    ];

    possible_configs.contains(&corner_chars)
}

fn find_xmas_part2(input: &str) -> usize {
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
            if *char != 'A' {
                continue;
            }
            if is_mas(&map, (x, y)) {
                count += 1;
            }
        }
    }

    count
}
fn find_xmas(input: &str, word_to_find: &str) -> usize {
    let map = transform(input);
    let mut count = 0usize;
    let lines: Vec<String> = input.lines().map(String::from).collect();
    let height = input.lines().count();
    let width = lines.first().unwrap().chars().count();

    let first_char = word_to_find.chars().next().unwrap();

    for y in 0..height {
        for x in 0..width {
            let x = x as isize;
            let y = y as isize;

            let char = map.get(&(x, y)).unwrap();
            if *char != first_char {
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
    let count = find_xmas_part2(&test);
    println!("count = {}", count);
}

#[test]
fn test_find_xmas() {
    let test = fs::read_to_string("./src/test-input.txt").unwrap();
    assert_eq!(find_xmas(&test, "XMAS"), 18);
}

#[test]
fn test_find_xmas_part2() {
    let test = fs::read_to_string("./src/test-part2.txt").unwrap();
    assert_eq!(find_xmas_part2(&test), 9);
}

#[test]
fn test_simple() {
    let test = fs::read_to_string("./src/test-simple.txt").unwrap();
    assert_eq!(find_xmas(&test, "XMAS"), 4);
}
