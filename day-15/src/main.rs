use std::{collections::HashMap, fs};

use shared::vector::Vector;

#[derive(Default)]
struct Warehouse {
    tiles: HashMap<Vector, Tile>,
    robot_position: Vector,
    robot_movements: RobotMovements,
}

impl Warehouse {
    // returns true if box moved
    fn move_box(&mut self, at: &Vector, direction: &Vector, is_first: bool) -> bool {
        let next_pos = at + direction;
        let next_tile = self.tiles.get(&next_pos).expect("should always return");

        let box_moved;
        match next_tile {
            Tile::Wall => box_moved = false,
            Tile::Box => box_moved = self.move_box(&next_pos, direction, false),
            Tile::Robot => unreachable!(),
            Tile::Empty => {
                box_moved = true;
                self.tiles.insert(next_pos, Tile::Box);
            }
        };

        if box_moved && is_first {
            self.tiles.insert(*at, Tile::Empty);
        }
        box_moved
    }

    fn robot_move(&mut self, movement: Vector) {
        let new_pos = self.robot_position + movement;
        let tile = self
            .tiles
            .get(&new_pos)
            .expect("Robot cannot escape the warehouse");

        match tile {
            Tile::Wall => (),
            Tile::Box => {
                if self.move_box(&new_pos, &movement, true) {
                    self.robot_position = new_pos;
                }
            }
            Tile::Robot => unreachable!(),
            Tile::Empty => self.robot_position = new_pos,
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
                        Tile::Box => "O",
                        Tile::Robot => "@",
                        Tile::Empty => ".",
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
            .filter(|x| *x.1 == Tile::Box)
            .map(|x| x.0.y * 100 + x.0.x)
            .sum()
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord)]
enum Tile {
    Wall,
    Box,
    Robot,
    Empty,
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
            if !warehouse_parsed {
                let tile = parse_tile(char);
                let pos = (x as isize, y as isize).into();
                if tile == Tile::Robot {
                    warehouse.robot_position = pos;
                    warehouse.tiles.insert(pos, Tile::Empty);
                } else {
                    warehouse.tiles.insert(pos, tile);
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
