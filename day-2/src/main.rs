use std::fs;

#[derive(PartialEq, Eq)]
enum Report {
    Safe,
    Unsafe,
}

fn main() {
    let example = fs::read_to_string("./src/example-data.txt").unwrap();
    let real = fs::read_to_string("./src/puzzle-input.txt").unwrap();
    let count = real
        .lines()
        .map(|i| {
            let numbers = i
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            let mut list_asc: Option<bool> = None;
            for i in 0..numbers.len() {
                let current = numbers[i];
                let next = numbers.get(i + 1);

                match next {
                    Some(next) => {
                        let asc = *next > current;
                        let diff = current.abs_diff(*next);

                        match list_asc {
                            Some(list_asc) if asc != list_asc => {
                                return Report::Unsafe;
                            }
                            None if current != *next => list_asc = Some(asc),
                            None => (),
                            Some(_) => (),
                        }

                        if !(1..=3).contains(&diff) {
                            return Report::Unsafe;
                        }
                    }
                    None => break,
                };
            }
            Report::Safe
        })
        .filter(|f| *f == Report::Safe)
        .count();
    println!("Safe reports: {}", count);
}
