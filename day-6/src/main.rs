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
    new_obstructions: Vec<Vector>,
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
            new_obstructions: vec![],
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

    fn does_rotating_here_cause_loop(&self, pos: Vector, guard_dir: Vector) -> bool {
        let mut simulation_map = self.clone();

        simulation_map.map.insert(pos + guard_dir, '#');
        simulation_map.guard_direction.rotate_right();

        if simulation_map.guard_partol(false) == PartolResult::Loop {
            return true;
        }

        false
    }

    fn guard_partol(&mut self, create_obstructions: bool) -> PartolResult {
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
                    if create_obstructions
                        && self.does_rotating_here_cause_loop(
                            self.guard_position,
                            self.guard_direction,
                        )
                    {
                        self.new_obstructions.push(next_pos);
                    }

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
    let mut map = Map::from(&input);
    map.guard_partol(true);
    println!("Tiles visited {}", map.distinct_points_visited.len());
    println!("New Obstructions {}", map.new_obstructions.len());
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
    example.guard_partol(false);
    assert_eq!(example.distinct_points_visited.len(), 41);
}

#[test]
fn test_new_obstructions() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let mut example = Map::from(&input);

    println!("Guard position {:?}", example.guard_position);
    example.guard_partol(true);
    println!("new_obstructions = {:?}", example.new_obstructions);

    assert!(
        example.new_obstructions.contains(&Vector::new(3, 4)),
        "Option one"
    );

    assert!(
        example.new_obstructions.contains(&Vector::new(6, 3)),
        "Option two"
    );
    assert!(
        example.new_obstructions.contains(&Vector::new(6, 3)),
        "Option three"
    );
    assert!(
        example.new_obstructions.contains(&Vector::new(1, 2)),
        "Option four"
    );
    assert!(
        example.new_obstructions.contains(&Vector::new(3, 2)),
        "Option five"
    );
    assert!(
        example.new_obstructions.contains(&Vector::new(7, 1)),
        "Option six"
    );
}
