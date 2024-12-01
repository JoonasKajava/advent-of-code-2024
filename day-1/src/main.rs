use std::fs;

fn split_to_sides(str: String) -> (Vec<usize>, Vec<usize>) {
    let lines = str.lines();
    let mut left = vec![];
    let mut right = vec![];
    for line in lines {
        let sides: Vec<usize> = line
            .split(' ')
            .filter_map(|s| s.parse::<usize>().ok())
            .collect();
        left.push(sides[0]);
        right.push(sides[1]);
    }
    (left, right)
}

fn pair_and_calc_distance(left: Vec<usize>, right: Vec<usize>) -> Vec<usize> {
    let mut distances = vec![];
    for i in 0..left.len() {
        distances.push(left[i].abs_diff(right[i]))
    }
    distances
}

fn calc_similarity(left: Vec<usize>, right: Vec<usize>) -> usize {
    let mut score = 0usize;
    for num in left {
        score += right.iter().filter(|f| **f == num).sum::<usize>();
    }
    score
}

fn partone() {
    let input = fs::read_to_string("./src/real-input.txt").unwrap();

    let (mut left, mut right) = split_to_sides(input);
    left.sort();
    right.sort();

    let result: usize = pair_and_calc_distance(left, right).iter().sum();

    println!("{}", result)
}

fn parttwo() {
    let input = fs::read_to_string("./src/real-input.txt").unwrap();

    let (left, right) = split_to_sides(input);

    let result = calc_similarity(left, right);

    println!("{}", result)
}

fn main() {
    parttwo();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_split() {
        let input = fs::read_to_string("./src/test-input.txt").unwrap();
        let (left, right) = split_to_sides(input);

        assert_eq!(left, vec![3, 4, 2, 1, 3, 3]);
        assert_eq!(right, vec![4, 3, 5, 3, 9, 3]);
    }

    #[test]
    fn test_calc_similarity() {
        let input = fs::read_to_string("./src/test-input.txt").unwrap();
        let (left, right) = split_to_sides(input);

        let similarity = calc_similarity(left, right);

        assert_eq!(similarity, 31);
    }
}
