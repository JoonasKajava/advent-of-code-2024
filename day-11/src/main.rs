use cached::proc_macro::cached;

fn digit_count(num: usize) -> usize {
    num.to_string().len()
}

fn split_number(num: usize) -> Vec<usize> {
    let string = num.to_string();
    let split = string.split_at(string.len() / 2);
    vec![split.0.parse().unwrap(), split.1.parse().unwrap()]
}

#[cached]
fn blink_stone(current_blink: usize, to_blink: usize, number: usize) -> usize {
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
            .fold(0, |a, x| a + blink_stone(current_blink + 1, to_blink, *x))
    } else {
        result.len()
    }
}

fn blink_stones(input: &str, blink_amount: usize) -> usize {
    let numbers: Vec<usize> = input
        .split(' ')
        .map(|x| x.parse::<usize>().unwrap())
        .collect();

    numbers
        .iter()
        .fold(0usize, |a, x| a + blink_stone(0, blink_amount, *x))
}

fn main() {
    let input = "64554 35 906 6 6960985 5755 975820 0";

    let result = blink_stones(input, 25);

    println!("Part 1 = {}", result);

    let result = blink_stones(input, 75);

    println!("Part 2 = {}", result);
}

#[test]
fn test_split_number() {
    assert_eq!(split_number(99), vec![9, 9]);
    assert_eq!(split_number(253000), vec![253, 0]);
}
