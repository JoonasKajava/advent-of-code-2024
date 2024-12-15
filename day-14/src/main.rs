use std::{fs, usize};

use regex::Regex;
use shared::vector::Vector;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum Quadrant {
    TopLeft,
    TopRight,
    BottomLeft,
    BottomRight,
    NotCounted,
}

struct Map {
    width: isize,
    height: isize,
}

impl Map {
    fn robot_quadrant(&self, robot: &Robot) -> Quadrant {
        let middle_x = self.width / 2;
        let middle_y = self.height / 2;
        if robot.position.x == middle_x || robot.position.y == middle_y {
            Quadrant::NotCounted
        } else if robot.position.x < middle_x {
            if robot.position.y < middle_y {
                return Quadrant::TopLeft;
            } else {
                return Quadrant::BottomLeft;
            }
        } else if robot.position.y < middle_y {
            return Quadrant::TopRight;
        } else {
            return Quadrant::BottomRight;
        }
    }
    fn count_robots_in_quadrant(&self, robots: &[Robot], quad: Quadrant) -> usize {
        robots
            .iter()
            .map(|x| self.robot_quadrant(x))
            .filter(|x| *x == quad)
            .count()
    }
}

#[derive(Debug)]
struct Robot {
    position: Vector,
    velocity: Vector,
}

impl Robot {
    fn navigate(&mut self, map: &Map, for_seconds: isize) {
        let new_position = self.position + (self.velocity * for_seconds);
        let new_x = new_position.x.rem_euclid(map.width);
        let new_y = new_position.y.rem_euclid(map.height);
        self.position = (new_x, new_y).into();
    }
}

impl From<&str> for Robot {
    fn from(input: &str) -> Self {
        let robot_regex = Regex::new(r"=([-\d]+),([-\d]+).+?=([-\d]+),([-\d]+)").unwrap();
        let (_, [p_x, p_y, v_x, v_y]) = robot_regex
            .captures_iter(input)
            .map(|c| c.extract())
            .next()
            .unwrap();
        let p_x = p_x.parse().unwrap();
        let p_y = p_y.parse().unwrap();
        let v_x = v_x.parse().unwrap();
        let v_y = v_y.parse().unwrap();
        Robot {
            position: (p_x, p_y).into(),
            velocity: (v_x, v_y).into(),
        }
    }
}

fn parse_robots(input: &str) -> Vec<Robot> {
    input.lines().map(Robot::from).collect()
}
fn output(robots: &[Robot]) -> String {
    let max_x = robots.iter().map(|x| x.position.x).max().unwrap();
    let max_y = robots.iter().map(|y| y.position.y).max().unwrap();

    let mut result = "".to_string();
    for y in 0..max_y {
        for x in 0..max_x {
            let robots_here = robots
                .iter()
                .filter(|robot| robot.position == Vector::new(x, y))
                .count();
            result = match robots_here {
                0 => result + ".",
                count => result + &count.to_string(),
            };
        }
        result += "\n";
    }
    result.to_owned()
}

fn get_safety_factor(robots: &[Robot], map: &Map) -> usize {
    let top_left = map.count_robots_in_quadrant(robots, Quadrant::TopLeft);
    let top_right = map.count_robots_in_quadrant(robots, Quadrant::TopRight);
    let bottom_left = map.count_robots_in_quadrant(robots, Quadrant::BottomLeft);
    let bottom_right = map.count_robots_in_quadrant(robots, Quadrant::BottomRight);
    top_left * top_right * bottom_left * bottom_right
}

fn main() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let mut robots = parse_robots(&input);
    let map = Map {
        width: 101,
        height: 103,
    };

    robots.iter_mut().for_each(|x| x.navigate(&map, 100));

    println!("part one {}", get_safety_factor(&robots, &map));

    println!("part two start");

    let mut robots = parse_robots(&input);

    for i in 0..10000 {
        println!("i {}", i + 1);
        robots.iter_mut().for_each(|x| x.navigate(&map, 1));
        println!("{}", output(&robots));
    }
}

#[test]
fn test_robot_teleport() {
    let mut robot = Robot {
        position: (2, 4).into(),
        velocity: (2, -3).into(),
    };
    let map = Map {
        width: 11,
        height: 7,
    };
    robot.navigate(&map, 1);

    assert_eq!(robot.position, (4, 1).into());

    robot.navigate(&map, 1);
    assert_eq!(robot.position, (6, 5).into());

    robot.navigate(&map, 1);
    assert_eq!(robot.position, (8, 2).into());

    robot.navigate(&map, 1);
    assert_eq!(robot.position, (10, 6).into());

    robot.navigate(&map, 1);
    assert_eq!(robot.position, (1, 3).into());
}

#[test]
fn test_example_part1() {
    let example = fs::read_to_string("./src/example.txt").unwrap();
    let mut robots = parse_robots(&example);
    let map = Map {
        width: 11,
        height: 7,
    };

    robots.iter_mut().for_each(|x| x.navigate(&map, 100));

    let top_left = map.count_robots_in_quadrant(&robots, Quadrant::TopLeft);
    let top_right = map.count_robots_in_quadrant(&robots, Quadrant::TopRight);
    let bottom_left = map.count_robots_in_quadrant(&robots, Quadrant::BottomLeft);
    let bottom_right = map.count_robots_in_quadrant(&robots, Quadrant::BottomRight);

    assert_eq!(top_left, 1);
    assert_eq!(top_right, 3);
    assert_eq!(bottom_left, 4);
    assert_eq!(bottom_right, 1);

    assert_eq!(get_safety_factor(&robots, &map), 12);
}

#[test]
fn test_modulo() {
    assert_eq!((-2isize).rem_euclid(5), 3);
}
