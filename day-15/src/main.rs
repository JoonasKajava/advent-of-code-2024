use std::{
    cmp::{max, min},
    collections::HashMap,
    fs,
};

use shared::vector::Vector;

#[derive(Default)]
struct Warehouse {
    tiles: HashMap<Vector, Tile>,
    robot_position: Vector,
    robot_movements: RobotMovements,
}

impl Warehouse {
    // returns true if box moved

    fn horizontal_move(&mut self, at: Vector, direction: Vector) -> bool {
        let mut first_free_position: Option<Vector> = None;
        let mut offset = at;
        loop {
            let next_pos = offset + direction;
            let Some(tile) = self.tiles.get(&next_pos) else {
                break;
            };

            if let Tile::Empty = tile {
                first_free_position = Some(next_pos);
                break;
            }
            offset += direction;
        }

        if let Some(v) = first_free_position {
            let start = min(at.x, v.x);
            let end = max(at.x, v.x);

            let mut counter = 0;
            for i in start..=end {
                match counter % 2 {
                    0 => self.tiles.insert((i, at.y).into(), Tile::RightBox),
                    1 => self.tiles.insert((i, at.y).into(), Tile::LeftBox),
                    _ => unreachable!(),
                };
                counter += 1;
            }
            self.tiles.insert(at, Tile::Empty);
        }
        first_free_position.is_some()
    }

    fn move_box(&mut self, at: Vector, direction: Vector, is_first: bool) -> bool {
        let next_pos = at + direction;
        let next_tile = self.tiles.get(&next_pos).expect("should always return");
        let current_tile = *self.tiles.get(&at).expect("should always return");

        let mut box_moved = false;
        match next_tile {
            Tile::Wall => box_moved = false,
            Tile::LeftBox => {
                box_moved = self.move_box(next_pos, direction, false)
                    && self.move_box(next_pos + RIGHT, direction, false)
            }
            Tile::RightBox => {
                box_moved = self.move_box(next_pos, direction, false)
                    && self.move_box(next_pos + LEFT, direction, false)
            }
            Tile::Empty => {
                if direction == LEFT || direction == RIGHT {
                    let next_next_position = next_pos + direction;
                    let next_next_tile = self
                        .tiles
                        .get(&next_next_position)
                        .expect("Should always return");
                    if let Tile::Empty = next_next_tile {
                        self.tiles.insert(next_pos, Tile::RightBox);
                        self.tiles.insert(next_next_position, Tile::LeftBox);
                        box_moved = true;
                    }
                } else {
                    box_moved = true;
                    match &current_tile {
                        Tile::LeftBox => {
                            self.tiles.insert(next_pos + RIGHT, Tile::RightBox);
                            self.tiles.insert(next_pos, Tile::LeftBox);
                        }
                        Tile::RightBox => {
                            self.tiles.insert(next_pos + LEFT, Tile::LeftBox);
                            self.tiles.insert(next_pos, Tile::RightBox);
                        }
                        _ => unreachable!(),
                    };
                }
            }
            _ => unreachable!(),
        };

        if box_moved && is_first {
            if direction == LEFT || direction == RIGHT {
                self.tiles.insert(at, Tile::Empty);
                self.tiles.insert(at + direction, Tile::Empty);
            }
            match current_tile {
                Tile::LeftBox => {
                    self.tiles.insert(at + RIGHT, Tile::Empty);
                    self.tiles.insert(at, Tile::Empty);
                }
                Tile::RightBox => {
                    self.tiles.insert(at + LEFT, Tile::Empty);
                    self.tiles.insert(at, Tile::Empty);
                }
                _ => unreachable!(),
            };
        }
        box_moved
    }

    fn robot_move(&mut self, movement: Vector) {
        let new_pos = self.robot_position + movement;
        let tile = self
            .tiles
            .get(&new_pos)
            .expect("Robot cannot escape the warehouse");

        let is_horizontal_move = movement == LEFT || movement == RIGHT;
        match tile {
            Tile::Wall => (),
            Tile::LeftBox => {
                if is_horizontal_move {
                    if self.horizontal_move(new_pos, movement) {
                        self.robot_position = new_pos;
                    }
                } else if self.move_box(new_pos, movement, true)
                    && self.move_box(new_pos + RIGHT, movement, true)
                {
                    self.robot_position = new_pos;
                }
            }
            Tile::RightBox => {
                if is_horizontal_move {
                    if self.horizontal_move(new_pos, movement) {
                        self.robot_position = new_pos;
                    }
                } else if self.move_box(new_pos, movement, true)
                    && self.move_box(new_pos + LEFT, movement, true)
                {
                    self.robot_position = new_pos;
                }
            }
            Tile::Empty => self.robot_position = new_pos,
            _ => unreachable!(),
        };
    }

    fn print(&self) {
        let max_x = self.tiles.keys().map(|x| x.x).max().unwrap();
        let max_y = self.tiles.keys().map(|x| x.y).max().unwrap();
        let mut result = "".to_string();
        for y in 0..=max_y {
            for x in 0..=max_x {
                let pos = &(x, y).into();
                let Some(tile) = self.tiles.get(pos) else {
                    continue;
                };
                if *pos == self.robot_position {
                    result += "@";
                } else {
                    result += match tile {
                        Tile::Wall => "#",
                        Tile::Robot => "@",
                        Tile::Empty => ".",
                        Tile::LeftBox => "[",
                        Tile::RightBox => "]",
                        Tile::Box => unreachable!(),
                    }
                }
            }
            result += "\n";
        }
        println!("{}", result);
    }

    fn sum_gps_coords(&self) -> isize {
        self.tiles
            .iter()
            .filter(|x| *x.1 == Tile::LeftBox)
            .map(|x| x.0.y * 100 + x.0.x)
            .sum()
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Wall,
    LeftBox,
    RightBox,
    Robot,
    Empty,
    Box,
}

type RobotMovements = Vec<Vector>;

fn parse_direction(char: char) -> Vector {
    match char {
        '<' => LEFT,
        '^' => TOP,
        '>' => RIGHT,
        'v' => BOTTOM,
        _ => unreachable!(),
    }
}

fn parse_tile(char: char) -> Tile {
    match char {
        '#' => Tile::Wall,
        '.' => Tile::Empty,
        '@' => Tile::Robot,
        'O' => Tile::Box,
        '[' => Tile::LeftBox,
        ']' => Tile::RightBox,
        _ => unreachable!(),
    }
}

const TOP: Vector = Vector::new(0, -1);
const BOTTOM: Vector = Vector::new(0, 1);
const RIGHT: Vector = Vector::new(1, 0);
const LEFT: Vector = Vector::new(-1, 0);

fn parse(input: &str) -> Warehouse {
    let mut warehouse = Warehouse::default();
    let mut warehouse_parsed = false;
    for (y, line) in input.lines().enumerate() {
        if line.is_empty() {
            warehouse_parsed = true;
            continue;
        }
        for (x, char) in line.chars().enumerate() {
            let x = x * 2;
            if !warehouse_parsed {
                let tile = parse_tile(char);
                let pos = (x as isize, y as isize).into();
                if tile == Tile::Robot {
                    warehouse.robot_position = pos;
                    warehouse.tiles.insert(pos, Tile::Empty);
                    warehouse.tiles.insert(pos + RIGHT, Tile::Empty);
                } else if let Tile::Box = tile {
                    warehouse.tiles.insert(pos, Tile::LeftBox);
                    warehouse.tiles.insert(pos + RIGHT, Tile::RightBox);
                } else {
                    warehouse.tiles.insert(pos, tile);
                    warehouse.tiles.insert(pos + RIGHT, tile);
                }
            } else {
                warehouse.robot_movements.push(parse_direction(char));
            }
        }
    }
    warehouse
}

fn main() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let mut warehouse = parse(&input);

    let movements = warehouse.robot_movements.clone();

    for movement in movements {
        warehouse.robot_move(movement);
    }

    println!("first part {}", warehouse.sum_gps_coords());
}

#[test]
fn test_example() {
    let input = fs::read_to_string("./src/large-example.txt").unwrap();
    let mut warehouse = parse(&input);

    println!("initial state:");
    warehouse.print();

    let movements = warehouse.robot_movements.clone();

    for movement in movements {
        warehouse.robot_move(movement);
        println!("moving {:?}", movement);
        warehouse.print();
    }
    assert_eq!(warehouse.sum_gps_coords(), 10092);
}
