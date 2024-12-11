use std::fs;

fn classify(
    numbers: &Vec<isize>,
    at: usize,
    prev_number: Option<isize>,
    mut ascending: Option<bool>,
    mut can_use_skip: bool,
) -> bool {
    let mut num = match numbers.get(at) {
        Some(n) => *n,
        None => return true,
    };

    let mut skip = false;
    if let Some(prev) = prev_number {
        let asc = num > prev;
        let diff = prev.abs_diff(num);
        let diff = !(1..=3).contains(&diff);
        let mut result = true;
        if diff {
            result = false;
        }
        if let Some(ascending) = ascending {
            if asc != ascending {
                result = false;
            }
        } else {
            ascending = Some(asc);
        }

        if can_use_skip && !result {
            skip = true;
        } else if !result {
            return false;
        }
    }

    if skip
        && classify(
            numbers,
            at + 1,
            Some(prev_number.unwrap()),
            ascending,
            false,
        )
    {
        return true;
    }

    if classify(numbers, at + 1, Some(num), ascending, can_use_skip) {
        return true;
    }
    false
}

fn parse(input: &str) -> Vec<Vec<isize>> {
    let mut result = vec![];
    for line in input.lines() {
        result.push(
            line.split(' ')
                .map(|x| x.parse::<isize>().unwrap())
                .collect(),
        );
    }
    result
}

fn main() {
    let input = fs::read_to_string("./src/puzzle-input.txt").unwrap();
    let parsed = parse(&input);

    let count = parsed
        .iter()
        .map(|x| classify(x, 0, None, None, false))
        .filter(|x| *x)
        .count();
    let count2 = parsed
        .iter()
        .map(|x| classify(x, 0, None, None, true))
        .filter(|x| *x)
        .count();

    println!("Safe reports part 1: {}", count);
    println!("Safe reports part 2: {}", count2);
}
