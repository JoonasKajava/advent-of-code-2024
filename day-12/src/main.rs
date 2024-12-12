use std::{collections::HashMap, fs};

use shared::vector::Vector;

type Garden = HashMap<Vector, char>;

#[derive(Debug)]
struct Region {
    char: char,
    area: usize,
    perimeter: usize,
    sides: usize,
    plots: Vec<Vector>,
}

impl Region {
    fn price(&self, use_sides: bool) -> usize {
        match use_sides {
            true => self.area * self.sides,
            false => self.area * self.perimeter,
        }
    }
}

const TOP: Vector = Vector::new(0, -1);
const BOTTOM: Vector = Vector::new(0, 1);
const RIGHT: Vector = Vector::new(1, 0);
const LEFT: Vector = Vector::new(-1, 0);

const TOPRIGHT: Vector = Vector::new(1, -1);
const BOTTOMRIGHT: Vector = Vector::new(1, 1);
const TOPLEFT: Vector = Vector::new(-1, -1);
const BOTTOMLEFT: Vector = Vector::new(-1, 1);

const DIRECTIONS: [Vector; 4] = [TOP, BOTTOM, RIGHT, LEFT];

// This is pretty bad
fn corner_count(garden: &Garden, position: Vector) -> usize {
    let current_char = garden.get(&position).unwrap();

    let is_not_same_region = |pos| match garden.get(&pos) {
        Some(c) if c != current_char => true,
        Some(_) => false,
        None => true,
    };

    let top = is_not_same_region(position + TOP);
    let bottom = is_not_same_region(position + BOTTOM);
    let right = is_not_same_region(position + RIGHT);
    let left = is_not_same_region(position + LEFT);

    let top_right = is_not_same_region(position + TOPRIGHT);
    let bottom_right = is_not_same_region(position + BOTTOMRIGHT);
    let top_left = is_not_same_region(position + TOPLEFT);
    let bottom_left = is_not_same_region(position + BOTTOMLEFT);

    let tests = [
        left && top,
        top && right,
        left && bottom,
        right && bottom,
        !right && !bottom && bottom_right,
        !left && !bottom && bottom_left,
        !top && !right && top_right,
        !top && !left && top_left,
    ];

    tests.iter().filter(|x| **x).count()
}

fn walk_region(garden: &Garden, compiled_region: &mut Region, char: &char, position: &Vector) {
    let Some(current_plot) = garden.get(position) else {
        // trying to walk out of the garden
        compiled_region.perimeter += 1;
        return;
    };

    if current_plot != char {
        // Now walking in other region
        compiled_region.perimeter += 1;
        return;
    }
    // Still in same region
    compiled_region.area += 1;

    compiled_region.sides += corner_count(garden, *position);

    compiled_region.plots.push(*position);

    for direction in DIRECTIONS {
        let new_position = *position + direction;
        // Do now loop around in the same plot
        if compiled_region.plots.contains(&new_position) {
            continue;
        }

        walk_region(garden, compiled_region, char, &new_position);
    }
}

fn find_region(garden: &Garden, position: &Vector) -> Region {
    let char = garden.get(position).expect("should not be possible");

    let mut region = Region {
        char: *char,
        area: 0,
        perimeter: 0,
        sides: 0,
        plots: vec![],
    };
    walk_region(garden, &mut region, char, position);
    region
}

fn find_regions(garden: &Garden) -> Vec<Region> {
    let mut regions: Vec<Region> = vec![];

    for position in garden.keys() {
        let is_mapped = regions.iter().any(|x| x.plots.contains(position));
        if is_mapped {
            continue;
        }
        regions.push(find_region(garden, position));
    }

    regions
}

fn parse_garden(input: &str) -> Garden {
    let mut result: Garden = Garden::new();
    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            result.insert(Vector::new(x as isize, y as isize), char);
        }
    }
    result
}

fn main() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);
    let total_price: usize = regions.iter().map(|x| x.price(false)).sum();

    println!("first part {}", total_price);
    let total_price: usize = regions.iter().map(|x| x.price(true)).sum();

    println!("second part {}", total_price);
}

#[test]
fn test_region_find() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);

    assert_eq!(regions.len(), 5);
}

#[test]
fn test_region_find_exact() {
    let input = fs::read_to_string("./src/example2.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);

    let a_region = regions.iter().find(|x| x.char == 'A').unwrap();
    let b_region = regions.iter().find(|x| x.char == 'B').unwrap();
    let c_region = regions.iter().find(|x| x.char == 'C').unwrap();
    let d_region = regions.iter().find(|x| x.char == 'D').unwrap();
    let e_region = regions.iter().find(|x| x.char == 'E').unwrap();

    assert_eq!(a_region.area, 4);
    assert_eq!(b_region.area, 4);
    assert_eq!(c_region.area, 4);
    assert_eq!(d_region.area, 1);
    assert_eq!(e_region.area, 3);

    assert_eq!(a_region.perimeter, 10);
    assert_eq!(b_region.perimeter, 8);
    assert_eq!(c_region.perimeter, 10);
    assert_eq!(d_region.perimeter, 4);
    assert_eq!(e_region.perimeter, 8);

    assert_eq!(a_region.sides, 4);
    assert_eq!(b_region.sides, 4);
    assert_eq!(c_region.sides, 8);
    assert_eq!(d_region.sides, 4);
    assert_eq!(e_region.sides, 4);
}

#[test]
fn test_pricing() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);
    let total_price: usize = regions.iter().map(|x| x.price(false)).sum();

    assert_eq!(total_price, 772);

    let input = fs::read_to_string("./src/example2.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);
    let total_price: usize = regions.iter().map(|x| x.price(false)).sum();
    assert_eq!(total_price, 140);
}
