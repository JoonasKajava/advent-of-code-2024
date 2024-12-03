use std::fs;

use regex::Regex;

fn find_muls(string: &str) -> Vec<(usize, usize)> {
    let regex = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    let mut results = vec![];

    for (_, [first, second]) in regex.captures_iter(string).map(|c| c.extract()) {
        results.push((
            first.parse::<usize>().unwrap(),
            second.parse::<usize>().unwrap(),
        ));
    }
    results
}

fn add_up(values: Vec<(usize, usize)>) -> usize {
    values.iter().fold(0usize, |acc, e| acc + (e.0 * e.1))
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let muls = find_muls(&input);
    let sum = add_up(muls);
    dbg!(sum);
}

#[test]
fn test_find_muls() {
    let result =
        find_muls("xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))");

    assert_eq!(result, vec![(2, 4), (5, 5), (11, 8), (8, 5)])
}

#[test]
fn test_add_up() {
    let values = vec![(2, 4), (5, 5), (11, 8), (8, 5)];
    assert_eq!(add_up(values), 161);
}
