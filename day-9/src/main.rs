use std::{char, fs};

fn compress(input: &str) -> String {
    let mut charaters: Vec<char> = input.chars().collect();
    let mut head_index = 0usize;
    loop {
        let Some(char) = charaters.get(head_index) else {
            break;
        };
        if *char == '.' {
            let mut tail_counter = 0usize;
            loop {
                let tail_index = (charaters.len() - 1) - tail_counter;

                if tail_index <= head_index {
                    break;
                }

                let Some(tail_char) = charaters.get(tail_index) else {
                    break;
                };
                if tail_char.is_ascii_digit() {
                    charaters.swap(head_index, tail_index);
                    break;
                }
                tail_counter += 1;
            }
        }
        head_index += 1;
    }
    charaters.iter().collect()
}

fn create(input: &str) -> String {
    let mut id = 0;
    let mut switch = true;
    let mut result = "".to_owned();
    for char in input.chars() {
        let num = char.to_digit(10).unwrap();
        for _ in 0..num {
            match switch {
                true => result += &id.to_string(),
                false => result += ".",
            };
        }
        if switch {
            id += 1;
        }
        switch = !switch;
    }
    result.to_owned()
}

fn checksum(input: &str) -> usize {
    let mut result = 0;
    for (i, char) in input.chars().enumerate() {
        let Some(num) = char.to_digit(10) else {
            break;
        };

        result += i * num as usize;
    }
    result
}

fn part_one() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let create = create(input.trim());
    let compress = compress(&create);
    let checksum = checksum(&compress);

    println!("Part one Checksum = {}", checksum);
}

fn main() {
    part_one();
}

#[test]
fn test_example() {
    let example = "2333133121414131402";
    let result = create(example);

    assert_eq!(result, "00...111...2...333.44.5555.6666.777.888899");
}

#[test]
fn test_compress() {
    let test = "0..111....22222";
    let result = compress(test);

    assert_eq!(result, "022111222......");
}

#[test]
fn test_checksum() {
    let example = "0099811188827773336446555566..............";
    let result = checksum(example);

    assert_eq!(result, 1928)
}

#[test]
fn test_part_one_example() {
    let example = "2333133121414131402";
    let create = create(example.trim());
    println!("create = {}", create);
    let compress = compress(&create);
    println!("compress = {}", compress);
    let checksum = checksum(&compress);
    assert_eq!(checksum, 1928)
}
