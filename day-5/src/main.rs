use std::{collections::HashMap, fs};

fn parse_pages(input: &str) -> Vec<Vec<usize>> {
    let mut result: Vec<Vec<usize>> = vec![];
    for line in input.lines() {
        if !line.contains(',') {
            continue;
        }

        let pages: Vec<usize> = line
            .split(',')
            .map(|x| x.parse::<usize>().unwrap())
            .collect();
        result.push(pages);
    }
    result
}

fn parse_ordering_rules(input: &str) -> HashMap<usize, Vec<usize>> {
    let mut result: HashMap<usize, Vec<usize>> = HashMap::new();
    for line in input.lines() {
        if !line.contains('|') {
            break;
        }

        let mut split = line.split('|');
        let first = split.next().unwrap().parse::<usize>().unwrap();
        let second = split.next().unwrap().parse::<usize>().unwrap();

        if let Some(rule) = result.get_mut(&first) {
            rule.push(second);
        } else {
            result.insert(first, vec![second]);
        }
    }
    result
}

fn reorder(rules: &HashMap<usize, Vec<usize>>, pages: &Vec<usize>) -> Vec<usize> {
    let mut pages = pages.clone();
    pages.sort_by(|a, b| {
        if let Some(rule) = rules.get(a) {
            if rule.contains(b) {
                return std::cmp::Ordering::Less;
            } else {
                return std::cmp::Ordering::Greater;
            }
        }
        std::cmp::Ordering::Equal
    });
    pages
}

fn part_one(input: &str) -> usize {
    let rules = parse_ordering_rules(&input);
    let all_pages = parse_pages(&input);
    let mut count = 0usize;

    for page in all_pages {
        let reordered = reorder(&rules, &page);
        let middle = reordered.get(reordered.len() / 2).unwrap();
        if reordered == page {
            count += middle;
        }
    }
    count
}

fn part_two(input: &str) -> usize {
    let rules = parse_ordering_rules(&input);
    let all_pages = parse_pages(&input);
    let mut count = 0usize;

    for page in all_pages {
        let reordered = reorder(&rules, &page);
        let middle = reordered.get(reordered.len() / 2).unwrap();
        if reordered != page {
            count += middle;
        }
    }
    count
}

fn main() {
    let input = fs::read_to_string("./src/puzzle-input.txt").unwrap();
    let count = part_one(&input);
    println!("\n Part one result {}", count);

    let count = part_two(&input);
    println!("\n Part two result {}", count);
}

#[test]
fn test_count() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    assert_eq!(part_one(&input), 143);
}

#[test]
fn test_part_two() {
    let input = fs::read_to_string("./src/example.txt").unwrap();
    assert_eq!(part_two(&input), 123);
}
