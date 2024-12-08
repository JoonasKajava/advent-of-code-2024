use std::{
    collections::{HashMap, HashSet},
    fs,
    ops::{self, Add, Sub},
};

struct Bounds {
    x: isize,
    y: isize,
    height: isize,
    width: isize,
}

impl Bounds {
    fn from(input: &str) -> Bounds {
        let height = input.lines().count();
        let width = input.chars().count() / height - 1;
        Bounds {
            x: 0,
            y: 0,
            width: width as isize,
            height: height as isize,
        }
    }

    fn is_within(&self, vector: &Vector) -> bool {
        vector.x >= self.x && vector.x < self.width && vector.y >= self.y && vector.y < self.height
    }
}

impl Vector {
    fn new(x: isize, y: isize) -> Vector {
        Vector { x, y }
    }

    fn delta(&self, other: &Vector) -> Vector {
        other - self
    }
    fn mirror(&self) -> Vector {
        Vector {
            x: -self.x,
            y: -self.y,
        }
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
impl Sub<&Vector> for &Vector {
    type Output = Vector;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Copy, Clone, Hash)]
struct Vector {
    x: isize,
    y: isize,
}

type AntennaMap = HashMap<char, Vec<Vector>>;

fn is_antenna(ch: char) -> bool {
    ch.is_ascii_digit() || ch.is_ascii_alphabetic()
}

fn solution(input: &str, bounce: bool) -> usize {
    let mut map: AntennaMap = HashMap::new();

    let bounds = Bounds::from(input);

    let mut antinodes: HashSet<Vector> = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        for (x, char) in line.chars().enumerate() {
            if !is_antenna(char) {
                continue;
            }
            map.entry(char)
                .or_default()
                .push(Vector::new(x as isize, y as isize));
        }
    }

    for (_antenna_feq, positions) in map {
        for position in &positions {
            for second_antenna_position in positions.iter() {
                if second_antenna_position == position {
                    continue;
                }
                let delta = match bounce {
                    true => position.delta(second_antenna_position),
                    false => position.delta(second_antenna_position).mirror(),
                };
                let mut antinode_position = *position;
                loop {
                    antinode_position += delta;
                    if bounds.is_within(&antinode_position) {
                        antinodes.insert(antinode_position);
                        if !bounce {
                            break;
                        } else {
                            antinodes.insert(*position);
                            antinodes.insert(*second_antenna_position);
                        }
                    } else {
                        break;
                    }
                }
            }
        }
    }
    antinodes.len()
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let part_one_results = solution(&input, false);

    println!("Part One {}", part_one_results);

    let part_two_results = solution(&input, true);

    println!("Part two {}", part_two_results);
}

#[test]
fn test_is_antenna() {
    assert!(is_antenna('a'));
    assert!(is_antenna('D'));
    assert!(is_antenna('9'));
    assert!(is_antenna('0'));
    assert!(!is_antenna('.'));
}

#[test]
fn test_bounds() {
    let example = fs::read_to_string("./src/example.txt").unwrap();
    let bounds = Bounds::from(&example);
    assert_eq!(bounds.x, 0);
    assert_eq!(bounds.y, 0);
    assert_eq!(bounds.height, 12);
    assert_eq!(bounds.width, 12);

    assert!(bounds.is_within(&Vector::new(0, 0)));
    assert!(!bounds.is_within(&Vector::new(12, 12)));
    assert!(bounds.is_within(&Vector::new(11, 11)));
    assert!(bounds.is_within(&Vector::new(5, 5)));
}

#[test]
fn test_part_one_example() {
    let example = fs::read_to_string("./src/example.txt").unwrap();
    let count = solution(&example, false);
    assert_eq!(count, 14);
}

#[test]
fn test_part_two_example() {
    let example = fs::read_to_string("./src/example.txt").unwrap();
    let count = solution(&example, true);
    assert_eq!(count, 34);
}
#[test]
fn mirror() {
    let antenna1 = Vector::new(7, 7);
    let antenna2 = Vector::new(10, 10);

    let delta = antenna1.delta(&antenna2);

    let mirror = antenna1 + delta.mirror();

    assert_eq!(mirror, Vector::new(4, 4));
}
