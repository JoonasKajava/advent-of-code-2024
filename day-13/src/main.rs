use std::fs;

use regex::Regex;

type Number = isize;

#[derive(Default, Clone)]
struct Machine {
    a_x: Number,
    a_y: Number,

    b_x: Number,
    b_y: Number,

    p_x: Number,
    p_y: Number,
}

impl Machine {
    fn calculate(&self) -> Option<Number> {
        let det = self.a_x * self.b_y - self.a_y * self.b_x;
        let a = (self.p_x * self.b_y - self.p_y * self.b_x) / det;
        let b = (self.a_x * self.p_y - self.a_y * self.p_x) / det;

        if (self.a_x * a + self.b_x * b, self.a_y * a + self.b_y * b) == (self.p_x, self.p_y) {
            Some(a * 3 + b)
        } else {
            None
        }
    }
}

fn parse(input: &str) -> Vec<Machine> {
    let mut result = vec![];

    let mut machine = Machine::default();

    let button_regex = Regex::new(r"\+(\d+).*\+(\d+)").unwrap();
    let prize_regex = Regex::new(r"=(\d+).*=(\d+)").unwrap();

    let parse_two = |s: &str, regex: &Regex| {
        let (_, [first, second]) = regex.captures_iter(s).map(|c| c.extract()).next().unwrap();
        (
            first.parse::<isize>().unwrap(),
            second.parse::<isize>().unwrap(),
        )
    };

    let mut y = 0;
    for line in input.lines() {
        if line.is_empty() {
            result.push(machine.clone());
            machine = Machine::default();
            y = 0;
            continue;
        }
        match y % 3 {
            0 => {
                let (x, y) = parse_two(line, &button_regex);
                machine.a_x = x;
                machine.a_y = y;
            }
            1 => {
                let (x, y) = parse_two(line, &button_regex);
                machine.b_x = x;
                machine.b_y = y;
            }
            2 => {
                let (x, y) = parse_two(line, &prize_regex);
                machine.p_x = x;
                machine.p_y = y;
            }
            _ => {}
        }
        y += 1;
    }

    result.push(machine);

    result
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let machines = parse(&input);

    let result: isize = machines.iter().filter_map(|x| x.calculate()).sum();
    println!("part one {}", result);
}

#[test]
fn test_calc() {
    let machine = Machine {
        a_x: 94,
        a_y: 34,
        b_x: 22,
        b_y: 67,
        p_x: 8400,
        p_y: 5400,
    };

    assert_eq!(machine.calculate(), Some(280));

    let machine = Machine {
        a_x: 26,
        a_y: 66,
        b_x: 67,
        b_y: 21,
        p_x: 12748,
        p_y: 12176,
    };

    assert_eq!(machine.calculate(), None);
}

#[test]
fn test_example() {
    let example = fs::read_to_string("./src/example.txt").unwrap();
    let machines = parse(&example);

    let result: isize = machines.iter().filter_map(|x| x.calculate()).sum();
    assert_eq!(result, 480);
}
