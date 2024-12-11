fn digit_count(num: usize) -> usize {
    num.to_string().len()
}

fn split_number(num: usize) -> Vec<usize> {
    let string = num.to_string();
    let split = string.split_at(string.len() / 2);
    vec![split.0.parse().unwrap(), split.1.parse().unwrap()]
}

fn blink_stone(current_blink: usize, to_blink: usize, number: usize) -> Vec<usize> {
    let result: Vec<usize> = {
        if number == 0 {
            vec![1]
        } else if digit_count(number) % 2 == 0 {
            split_number(number)
        } else {
            vec![number * 2024]
        }
    };

    if current_blink < to_blink - 1 {
        result
            .iter()
            .flat_map(|x| blink_stone(current_blink + 1, to_blink, *x))
            .collect()
    } else {
        result
    }
}

fn blink_stones(input: &str, blink_amount: usize) -> Vec<usize> {
    let numbers: Vec<usize> = input
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    numbers
        .iter()
        .flat_map(|x| blink_stone(0, blink_amount, *x))
        .collect()
}

fn main() {
    let input = "64554 35 906 6 6960985 5755 975820 0";

    let result = blink_stones(input, 25);

    println!("Part 1 = {}", result.len());
}

#[test]
fn test_split_number() {
    assert_eq!(split_number(99), vec![9, 9]);
    assert_eq!(split_number(253000), vec![253, 0]);
}

#[test]
fn test_blink_stone() {
    assert_eq!(blink_stone(0, 1, 0), vec![1]);

    assert_eq!(blink_stone(0, 1, 1), vec![2024]);

    assert_eq!(blink_stone(0, 1, 10), vec![1, 0]);

    assert_eq!(blink_stone(0, 1, 99), vec![9, 9]);

    assert_eq!(blink_stone(0, 1, 999), vec![2021976]);
}

#[test]
fn test_blick_stones() {
    let result = blink_stones("125 17", 6);

    let result_string = result
        .iter()
        .map(|x| x.to_string())
        .collect::<Vec<String>>()
        .join(" ");

    assert_eq!(
        result_string,
        "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2"
    );
}
