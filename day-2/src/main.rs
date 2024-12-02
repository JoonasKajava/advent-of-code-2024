use std::fs;

#[derive(PartialEq, Eq, Debug)]
enum Report {
    Safe,
    Unsafe,
}

fn validate_pair(previous: Option<&usize>, first: usize, second: usize) -> bool {
    if first == second {
        return false;
    }
    let diff = first.abs_diff(second);
    if !(1..=3).contains(&diff) {
        return false;
    }

    if let Some(previous) = previous {
        if (*previous < first) == (first > second) {
            return false;
        }
    }

    true
}

fn classify_reportv2(mut report: Vec<usize>, mut allowed_removals: usize) -> Report {
    let mut i = 0;
    println!("report {:?}", report);
    while i < report.len() {
        let mut skip = false;
        let previous = if i > 0 { report.get(i - 1) } else { None };
        let current = report[i];
        let next = report.get(i + 1);
        if let Some(next) = next {
            println!("previous {:?} current {} next {}", previous, current, next);
            let result = validate_pair(previous, current, *next);
            // I do not understand why this skipping does not work when using real input
            // Otherwise it works perfectly
            // It's too late, I will just use the brute_classify_report
            if !result && allowed_removals > 0 {
                report.remove(i);
                println!("skip {}", current);
                i = i.saturating_sub(1);
                skip = true;
                allowed_removals -= 1;
            } else if !result {
                return Report::Unsafe;
            }
        }
        if !skip {
            i += 1;
        }
    }

    Report::Safe
}

fn brute_classify_report(report: Vec<usize>) -> Report {
    let mut rep = Report::Unsafe;
    for i in 0..report.len() {
        let mut clone = report.clone();
        clone.remove(i);
        if classify_reportv2(clone, 0) == Report::Safe {
            rep = Report::Safe;
            break;
        }
    }
    rep
}
fn main() {
    let _example = fs::read_to_string("./src/example-data.txt").unwrap();
    let _real = fs::read_to_string("./src/puzzle-input.txt").unwrap();
    let count = _real
        .lines()
        .map(|i| {
            let numbers = i
                .split(' ')
                .filter_map(|x| x.parse::<usize>().ok())
                .collect::<Vec<usize>>();
            brute_classify_report(numbers)
        })
        .filter(|f| *f == Report::Safe)
        .count();
    println!("Safe reports: {}", count);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_classify_part1() {
        assert_eq!(classify_reportv2(vec![7, 6, 4, 2, 1], 0), Report::Safe);
        assert_eq!(classify_reportv2(vec![1, 2, 7, 8, 9], 0), Report::Unsafe);
        assert_eq!(classify_reportv2(vec![9, 7, 6, 2, 1], 0), Report::Unsafe);
        assert_eq!(classify_reportv2(vec![1, 3, 2, 4, 5], 0), Report::Unsafe);
        assert_eq!(classify_reportv2(vec![8, 6, 4, 4, 1], 0), Report::Unsafe);
        assert_eq!(classify_reportv2(vec![1, 3, 6, 7, 9], 0), Report::Safe);
    }
    #[test]
    fn test_classify_part2() {
        assert_eq!(classify_reportv2(vec![7, 6, 4, 2, 1], 1), Report::Safe);
        assert_eq!(classify_reportv2(vec![1, 2, 7, 8, 9], 1), Report::Unsafe);
        assert_eq!(classify_reportv2(vec![9, 7, 6, 2, 1], 1), Report::Unsafe);
        assert_eq!(classify_reportv2(vec![1, 3, 2, 4, 5], 1), Report::Safe);
        assert_eq!(classify_reportv2(vec![8, 6, 4, 4, 1], 1), Report::Safe);
        assert_eq!(classify_reportv2(vec![1, 3, 6, 7, 9], 1), Report::Safe);
    }
    #[test]
    fn test_brute() {
        assert_eq!(brute_classify_report(vec![7, 6, 4, 2, 1]), Report::Safe);
        assert_eq!(brute_classify_report(vec![1, 2, 7, 8, 9]), Report::Unsafe);
        assert_eq!(brute_classify_report(vec![9, 7, 6, 2, 1]), Report::Unsafe);
        assert_eq!(brute_classify_report(vec![1, 3, 2, 4, 5]), Report::Safe);
        assert_eq!(brute_classify_report(vec![8, 6, 4, 4, 1]), Report::Safe);
        assert_eq!(brute_classify_report(vec![1, 3, 6, 7, 9]), Report::Safe);
    }
    #[test]
    fn test_validate_pair() {
        assert_eq!(validate_pair(Some(&1), 3, 2), false);
        assert_eq!(validate_pair(Some(&6), 4, 4), false);
        assert_eq!(validate_pair(None, 1, 5), false);
        assert_eq!(validate_pair(None, 5, 1), false);
        assert_eq!(validate_pair(None, 2, 1), true);
        assert_eq!(validate_pair(Some(&1), 1, 1), false);
    }
}
