use std::{cmp::max, fs};

#[derive(Debug)]
struct Equation {
    result: usize,
    nums: Vec<usize>,
}

fn concat(a: usize, b: usize) -> usize {
    (a.to_string() + &b.to_string()).parse().unwrap()
}

fn calculate(eq: &Equation, cumulative_result: usize, at: usize, part2: bool) -> bool {
    if cumulative_result > eq.result {
        return false;
    }

    let Some(num) = eq.nums.get(at) else {
        return cumulative_result == eq.result;
    };

    if calculate(eq, cumulative_result + num, at + 1, part2) {
        return true;
    }

    if calculate(eq, max(cumulative_result, 1) * num, at + 1, part2) {
        return true;
    }
    if part2 && calculate(eq, concat(cumulative_result, *num), at + 1, part2) {
        return true;
    }

    false
}

fn parse(input: &str) -> Vec<Equation> {
    let mut result = vec![];

    for line in input.lines() {
        let (e_result, numbers) = line.split_once(':').unwrap();
        result.push(Equation {
            result: e_result.parse::<usize>().unwrap(),
            nums: numbers
                .trim()
                .split(' ')
                .map(|x| x.parse::<usize>())
                .filter_map(|x| x.ok())
                .collect(),
        });
    }
    result
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let parsed = parse(&input);
    let results: usize = parsed
        .iter()
        .filter_map(|x| {
            if calculate(x, 0, 0, false) {
                Some(x.result)
            } else {
                None
            }
        })
        .sum();

    println!("part one results = {}", results);

    let part_2_results: usize = parsed
        .iter()
        .filter_map(|x| {
            if calculate(x, 0, 0, true) {
                Some(x.result)
            } else {
                None
            }
        })
        .sum();

    println!("part two results = {}", part_2_results);
}

#[test]
fn test_equations() {
    assert!(calculate(
        &Equation {
            result: 190,
            nums: vec![10, 19]
        },
        0,
        0,
        false
    ));

    assert!(calculate(
        &Equation {
            result: 3267,
            nums: vec![81, 40, 27]
        },
        0,
        0,
        false
    ))
}

#[test]
fn test_concat() {
    assert_eq!(concat(214, 324), 214324);
}

#[test]
fn test_part1() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let parsed = parse(&input);
    println!("parsed {:?}", parsed);
    let results: usize = parsed
        .iter()
        .map(|x| {
            if calculate(x, 0, 0, false) {
                x.result
            } else {
                0
            }
        })
        .sum();

    assert_eq!(results, 3749);
}

#[test]
fn test_part2() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    let parsed = parse(&input);
    println!("parsed {:?}", parsed);
    let results: usize = parsed
        .iter()
        .filter_map(|x| {
            if calculate(x, 0, 0, true) {
                Some(x.result)
            } else {
                None
            }
        })
        .sum();

    assert_eq!(results, 11387);
}
