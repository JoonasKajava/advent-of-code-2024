use std::{collections::HashMap, fs};

type Point = (isize, isize);

enum Tile {
    Empty,
    Obstruction,
    OutOfBounds,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone)]
struct Vector {
    x: isize,
    y: isize,
}

impl Vector {
    fn rotate_right(&mut self) {
        let x = self.x;
        let y = self.y;
        self.x = y;
        self.y = -x;
    }

    fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }
}

struct Map {
    map: HashMap<Point, char>,
    guard_position: Vector,
    guard_direction: Vector,
    distinct_points_visited: Vec<Vector>,
}

impl Map {
    fn from(input: &str) -> Map {
        let mut result: HashMap<Point, char> = HashMap::new();
        let mut guard_position = Vector::new(0, 0);
        let line_num = input.lines().count();
        for (y, line) in input.lines().enumerate() {
            let y = line_num - y;
            for (x, char) in line.chars().enumerate() {
                if char == '^' {
                    guard_position = Vector::new(x as isize, y as isize);
                }
                result.insert((x as isize, y as isize), char);
            }
        }
        Map {
            map: result,
            guard_position,
            guard_direction: Vector::new(0, 1),
            distinct_points_visited: vec![guard_position],
        }
    }

    fn guard_partol(&mut self) {
        loop {
            let next_pos = self.get_next_pos();
            let next_location = self.check_next_position(&next_pos);

            match next_location {
                Tile::Empty => {
                    self.guard_position = next_pos;
                    if !self.distinct_points_visited.contains(&next_pos) {
                        self.distinct_points_visited.push(next_pos);
                    }
                }
                Tile::Obstruction => self.guard_direction.rotate_right(),
                Tile::OutOfBounds => break,
            }
        }
    }

    fn get_next_pos(&self) -> Vector {
        Vector::new(
            self.guard_position.x + self.guard_direction.x,
            self.guard_position.y + self.guard_direction.y,
        )
    }

    fn check_next_position(&self, next_pos: &Vector) -> Tile {
        let position = self.map.get(&(next_pos.x, next_pos.y));
        match position {
            Some(char) if *char == '#' => Tile::Obstruction,
            Some(_) => Tile::Empty,
            None => Tile::OutOfBounds,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let mut map = Map::from(&input);
    map.guard_partol();
    println!("Tiles visited {}", map.distinct_points_visited.len())
}

#[test]
fn test_rotate_right() {
    let mut vector = Vector::new(1, 0);
    vector.rotate_right();
    assert_eq!(vector, Vector::new(0, -1));

    vector.rotate_right();
    assert_eq!(vector, Vector::new(-1, 0));

    vector.rotate_right();
    assert_eq!(vector, Vector::new(0, 1));

    vector.rotate_right();
    assert_eq!(vector, Vector::new(1, 0));
}

#[test]
fn test_example() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let mut example = Map::from(&input);
    example.guard_partol();
    assert_eq!(example.distinct_points_visited.len(), 41);
}
