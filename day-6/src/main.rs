use std::{
    collections::HashMap,
    fs,
    ops::{self, Add},
};

enum Tile {
    Empty,
    Obstruction,
    OutOfBounds,
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum PartolResult {
    Loop,
    Escape,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
struct Vector {
    x: isize,
    y: isize,
}

type VisitedTiles = HashMap<Vector, Vec<Vector>>;

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

impl ops::AddAssign<Vector> for Vector {
    fn add_assign(&mut self, rhs: Vector) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Add<Vector> for Vector {
    type Output = Self;

    fn add(self, rhs: Vector) -> Self::Output {
        Self {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

#[derive(Clone)]
struct Map {
    map: HashMap<Vector, char>,
    guard_position: Vector,
    guard_direction: Vector,
    distinct_points_visited: VisitedTiles,
}

impl Map {
    fn from(input: &str) -> Map {
        let mut result: HashMap<Vector, char> = HashMap::new();
        let mut guard_position = Vector::new(0, 0);
        let line_num = input.lines().count();
        for (y, line) in input.lines().enumerate() {
            let y = line_num - y;
            for (x, char) in line.chars().enumerate() {
                if char == '^' {
                    guard_position = Vector::new(x as isize, y as isize);
                }
                result.insert(Vector::new(x as isize, y as isize), char);
            }
        }
        let guard_direction = Vector::new(0, 1);
        Map {
            map: result,
            guard_position,
            guard_direction,
            distinct_points_visited: HashMap::from([(guard_position, vec![guard_direction])]),
        }
    }

    fn upsert_visited_tile(&mut self, pos: Vector, dir: Vector) {
        match self.distinct_points_visited.get_mut(&pos) {
            Some(tile) => {
                tile.push(dir);
            }
            None => {
                self.distinct_points_visited.insert(pos, vec![dir]);
            }
        };
    }

    fn guard_partol(&mut self) -> PartolResult {
        loop {
            let next_pos = self.guard_position + self.guard_direction;
            let next_location = self.check_next_position(&next_pos);

            if let Some(directions) = self.distinct_points_visited.get(&next_pos) {
                if directions.contains(&self.guard_direction) {
                    return PartolResult::Loop;
                }
            }
            match next_location {
                Tile::Empty => {
                    self.guard_position = next_pos;

                    self.upsert_visited_tile(self.guard_position, self.guard_direction);
                }
                Tile::Obstruction => {
                    self.guard_direction.rotate_right();
                    self.upsert_visited_tile(self.guard_position, self.guard_direction);
                }
                Tile::OutOfBounds => {
                    return PartolResult::Escape;
                }
            }
        }
    }

    fn check_next_position(&self, next_pos: &Vector) -> Tile {
        let position = self.map.get(next_pos);
        match position {
            Some(char) if *char == '#' => Tile::Obstruction,
            Some(_) => Tile::Empty,
            None => Tile::OutOfBounds,
        }
    }
}

fn main() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let map = Map::from(&input);
    let mut count = 0;
    for tile in map.map.iter() {
        if *tile.1 != '.' {
            continue;
        }
        let mut simulation_map = map.clone();
        simulation_map.map.insert(*tile.0, '#');
        if simulation_map.guard_partol() == PartolResult::Loop {
            count += 1;
        }
    }
    println!("count: {}", count);
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
