use std::fs;

#[derive(Debug)]
struct WholeFile {
    start_index: usize,
    end_index: usize,
    size: usize,
    id: Option<Id>,
}

fn compile_whole_files(disk: &Disk) -> Vec<WholeFile> {
    let mut result: Vec<WholeFile> = vec![];
    let mut previous_block: Option<Id> = *disk.first().unwrap();
    let mut previous_block_start = 0;
    for (i, block) in disk.iter().enumerate() {
        let Some(next_block) = disk.get(i) else {
            break;
        };
        if previous_block != *next_block {
            result.push(WholeFile {
                start_index: previous_block_start,
                end_index: i,
                size: i - previous_block_start,
                id: previous_block,
            });
            previous_block = *next_block;
            previous_block_start = i;
        }
    }
    result.push(WholeFile {
        start_index: previous_block_start,
        end_index: disk.len(),
        size: disk.len() - previous_block_start,
        id: previous_block,
    });
    result
}

fn compress_part2(mut disk: Disk) -> Disk {
    let mut static_compiled = compile_whole_files(&disk);
    static_compiled.sort_by(|a, b| a.id.cmp(&b.id));
    let mut dynamic_compiled = compile_whole_files(&disk);

    let mut i: isize = static_compiled.len() as isize - 1isize;

    loop {
        if i <= 0 {
            break;
        }
        let Some(file) = static_compiled.get(i as usize) else {
            break;
        };
        let Some(file_id) = file.id else {
            i -= 1;
            continue;
        };

        let Some(free_position) = dynamic_compiled
            .iter()
            .find(|x| x.id.is_none() && x.size >= file.size && file.start_index > x.start_index)
        else {
            i -= 1;
            continue;
        };

        for x in free_position.start_index..free_position.start_index + file.size {
            disk[x] = Some(file_id);
        }
        for x in file.start_index..file.end_index {
            disk[x] = None;
        }
        i -= 1;
        dynamic_compiled = compile_whole_files(&disk);
    }
    disk
}
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
            result += i * *num;
        } else {
            continue;
        }
    }
    result
}

fn process(input: &str, part2: bool) -> usize {
    let disk = create_disk(input);
    let compress = match part2 {
        true => compress_part2(disk),
        false => compress(disk),
    };
    checksum(compress)
}

fn part_two() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let result = process(input.trim(), true);

    println!("Part two Checksum = {}", result);
}
fn part_one() {
    let input = fs::read_to_string("./src/puzzle.txt").unwrap();
    let result = process(input.trim(), false);

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
    part_two();
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
    let checksum = process(example, false);
    assert_eq!(checksum, 1928)
}

#[test]
fn test_part_two_example() {
    let example = "2333133121414131402";
    let checksum = process(example, true);
    assert_eq!(checksum, 2858)
}
#[test]
fn test_edge_case() {
    let edge_case = "1010101010101010101010";
    let checksum = process(edge_case, false);
    assert_eq!(checksum, 385);
}

#[test]
fn test_edge_case2() {
    let edge_case = "12345";
    let checksum = process(edge_case, false);
    assert_eq!(checksum, 60);

    let checksum = process(edge_case, true);
    assert_eq!(checksum, 132);
}

#[test]
fn test_edge_case3() {
    let edge_case = "14113";
    let checksum = process(edge_case, true);
    assert_eq!(checksum, 16);
}
