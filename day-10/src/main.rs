use std::{cmp::max, collections::HashMap, fs};

use shared::vector::Vector;

enum TrailResult {}

type Tile = u32;

type Map = HashMap<Vector, Tile>;

const DIRECTIONS: [Vector; 4] = [
    Vector::new(-1, 0),
    Vector::new(0, -1),
    Vector::new(0, 1),
    Vector::new(1, 0),
];

fn walk(map: &Map, trailheads: &mut Vec<Vector>, position: Vector, previous_height: Option<Tile>) {
    let Some(current_height) = map.get(&position) else {
        return;
    };

    if let Some(prev) = previous_height {
        if *current_height != prev + 1 || trailheads.contains(&position) {
            return;
        }
    }

    if *current_height == 9 {
        trailheads.push(position);
        return;
    }

    for direction in DIRECTIONS {
        walk(map, trailheads, position + direction, Some(*current_height));
    }
}

fn find_trailheads(map: &Map) -> usize {
    let starting_positions = map.iter().filter(|x| *x.1 == 0);

    let mut trailheads = 0usize;
    for (position, _) in starting_positions {
        let mut found = vec![];
        walk(map, &mut found, *position, None);
        trailheads += found.len();
    }
    trailheads
}

fn parse_map(input: &str) -> Map {
    let mut map = Map::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            map.insert(
                Vector::new(x as isize, y as isize),
                char.to_digit(10).unwrap_or(10),
            );
        }
    }
    map
}

fn main() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let map = parse_map(&input);

    let trailheads = find_trailheads(&map);
    println!("Part one result {}", trailheads);
}

#[test]
fn test_example_part_one() {
    let example = fs::read_to_string("./src/example.txt").unwrap();
    let map = parse_map(&example);

    let trailheads = find_trailheads(&map);
    assert_eq!(trailheads, 36);
}
#[test]
fn test_example2() {
    let example = fs::read_to_string("./src/example2.txt").unwrap();
    let map = parse_map(&example);

    let trailheads = find_trailheads(&map);
    assert_eq!(trailheads, 4);
}
