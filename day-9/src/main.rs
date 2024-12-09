use std::fs;

fn compress(mut disk: Disk) -> Disk {
    let mut i = 0usize;
    'outer: loop {
        let Some(item) = disk.get(i) else {
            break;
        };

        if item.is_some() {
            i += 1;
            continue;
        }

        let last: Id = {
            loop {
                if i > disk.len() - 1 {
                    break 'outer;
                }
                let Some(last_item) = disk.pop() else {
                    break 'outer;
                };
                match last_item {
                    Some(l) => break l,
                    None => continue,
                }
            }
        };

        disk[i] = Some(last);

        i += 1;
    }

    disk
}

type Id = usize;

type Disk = Vec<Option<Id>>;

fn create_disk(input: &str) -> Disk {
    let mut id = 0usize;
    let mut switch = true;
    let mut disk: Vec<Option<Id>> = vec![];

    for char in input.chars() {
        let num = char.to_digit(10).unwrap();
        for _ in 0..num {
            match switch {
                true => disk.push(Some(id)),
                false => disk.push(None),
            };
        }
        if switch {
            id += 1;
        }
        switch = !switch;
    }
    disk
}

fn checksum(input: Disk) -> usize {
    let mut result = 0;
    for (i, item) in input.iter().enumerate() {
        if let Some(num) = item {
            println!("id {}", num);
            result += i * *num;
        } else {
            continue;
        }
    }
    result
}

fn process(input: &str) -> usize {
    let disk = create_disk(input);
    println!("disk {}", disk_to_string(&disk));
    let compress = compress(disk);
    println!("compressed {}", disk_to_string(&compress));
    checksum(compress)
}

fn part_one() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let result = process(input.trim());

    println!("Part one Checksum = {}", result);
}

fn disk_to_string(disk: &Disk) -> String {
    let mut result = "".to_owned();
    for i in disk {
        match i {
            Some(num) => result += &num.to_string(),
            None => result += ".",
        }
    }
    result
}

fn main() {
    part_one();
}

#[test]
fn test_example() {
    let example = "2333133121414131402";
    let result = create_disk(example);

    assert_eq!(
        disk_to_string(&result),
        "00...111...2...333.44.5555.6666.777.888899"
    );
}

#[test]
fn test_part_one_example() {
    let example = "2333133121414131402";
    let checksum = process(example);
    assert_eq!(checksum, 1928)
}

#[test]
fn test_edge_case() {
    let edge_case = "1010101010101010101010";
    let checksum = process(edge_case);
    assert_eq!(checksum, 385);
}

#[test]
fn test_edge_case2() {
    let edge_case = "12345";
    let checksum = process(edge_case);
    assert_eq!(checksum, 60);
}
