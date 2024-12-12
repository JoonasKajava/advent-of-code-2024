use std::{collections::HashMap, fs};

use shared::vector::Vector;

type Garden = HashMap<Vector, char>;

#[derive(Debug)]
struct Region {
    char: char,
    area: usize,
    perimeter: usize,
    plots: Vec<Vector>,
}

impl Region {
    fn price(&self) -> usize {
        self.area * self.perimeter
    }
}

const DIRECTIONS: [Vector; 4] = [
    Vector::new(-1, 0),
    Vector::new(0, -1),
    Vector::new(0, 1),
    Vector::new(1, 0),
];

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
    let total_price: usize = regions.iter().map(|x| x.price()).sum();

    println!("first part {}", total_price);
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
}

#[test]
fn test_pricing() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);
    let total_price: usize = regions.iter().map(|x| x.price()).sum();

    assert_eq!(total_price, 772);

    let input = fs::read_to_string("./src/example2.txt").unwrap();
    let garden = parse_garden(&input);
    let regions = find_regions(&garden);
    let total_price: usize = regions.iter().map(|x| x.price()).sum();
    assert_eq!(total_price, 140);
}
