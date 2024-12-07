use std::{collections::HashMap, fs};

struct PermutationGenerator {
    permutations: Vec<Vec<Operator>>,
}

impl PermutationGenerator {
    fn new() -> Self {
        Self {
            permutations: vec![],
        }
    }
    fn generate(&mut self, set: &Vec<Operator>, len: usize) {
        self.generate_rec(set, vec![], set.len(), len)
    }
    fn generate_rec(
        &mut self,
        set: &Vec<Operator>,
        permutation: Vec<Operator>,
        n: usize,
        k: usize,
    ) {
        if k == 0 {
            self.permutations.push(permutation);
            return;
        }
        for i in 0..n {
            let mut perm = permutation.clone();
            perm.push(set.get(i).unwrap().clone());
            self.generate_rec(set, perm, n, k - 1);
        }
    }
}

struct Equation {
    result: isize,
    numbers: Vec<isize>,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenation,
}

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug)]
enum CalculationResult {
    Success {
        result: isize,
        operator_configurations: Vec<Vec<Operator>>,
    },
    Impossible,
}

fn parse_line(line: &str) -> Equation {
    let (result, numbers) = line.split_once(':').unwrap();
    Equation {
        result: result.parse::<isize>().unwrap(),
        numbers: numbers
            .trim()
            .split(' ')
            .map(|x| x.parse::<isize>().unwrap())
            .collect(),
    }
}

fn concat_nums(a: isize, b: isize) -> isize {
    (a.to_string() + &b.to_string()).parse::<isize>().unwrap()
}

fn calculate(line: &str) -> CalculationResult {
    let parsed = parse_line(line);

    let mut generator = PermutationGenerator::new();

    generator.generate(
        &vec![Operator::Add, Operator::Multiply, Operator::Concatenation],
        parsed.numbers.len() - 1,
    );

    let operator_configurations_to_test: Vec<Vec<Operator>> = generator.permutations;

    let mut successful_permutations: Vec<Vec<Operator>> = vec![];

    for operator_config in operator_configurations_to_test {
        let calculated_result = parsed
            .numbers
            .iter()
            .enumerate()
            .fold(0isize, |a, (index, b)| {
                if index == 0 {
                    return *b;
                }
                match operator_config.get(index - 1).unwrap() {
                    Operator::Add => a + b,
                    Operator::Multiply => a * b,
                    Operator::Concatenation => concat_nums(a, *b),
                }
            });

        if calculated_result == parsed.result {
            successful_permutations.push(operator_config);
        }
    }

    if !successful_permutations.is_empty() {
        return CalculationResult::Success {
            result: parsed.result,
            operator_configurations: successful_permutations,
        };
    }
    CalculationResult::Impossible
}

fn sum_succesful(input: &str) -> isize {
    let mut sum = 0isize;
    for line in input.lines() {
        match calculate(line) {
            CalculationResult::Success {
                result,
                operator_configurations: _,
            } => {
                sum += result;
            }
            CalculationResult::Impossible => continue,
        }
    }
    sum
}

fn main() {
    let input = fs::read_to_string("./src/input.txt").unwrap();
    let sum = sum_succesful(&input);
    println!("Sum: {}", sum);
}

#[test]
fn test_generate_configurations() {
    let operators = vec![Operator::Add, Operator::Multiply];

    let sorted = |mut x: Vec<Vec<Operator>>| {
        x.sort();
        x
    };

    let mut generator = PermutationGenerator::new();
    generator.generate(&operators, 1);
    assert_eq!(
        sorted(generator.permutations),
        sorted(vec![vec![Operator::Add], vec![Operator::Multiply]])
    );

    let mut generator = PermutationGenerator::new();
    generator.generate(&operators, 2);
    assert_eq!(
        sorted(generator.permutations),
        sorted(vec![
            vec![Operator::Add, Operator::Add],
            vec![Operator::Multiply, Operator::Multiply],
            vec![Operator::Add, Operator::Multiply],
            vec![Operator::Multiply, Operator::Add]
        ])
    );
}

#[test]
fn test_calculcations() {
    assert_eq!(
        calculate("190: 10 19"),
        CalculationResult::Success {
            result: 190,
            operator_configurations: vec![vec![Operator::Multiply]]
        }
    );
    assert_eq!(
        calculate("3267: 81 40 27"),
        CalculationResult::Success {
            result: 3267,
            operator_configurations: vec![
                vec![Operator::Add, Operator::Multiply],
                vec![Operator::Multiply, Operator::Add]
            ]
        }
    );
    assert_eq!(calculate("83: 17 5"), CalculationResult::Impossible);
    //assert_eq!(calculate("156: 15 6"), CalculationResult::Impossible);
    assert_eq!(calculate("161011: 16 10 13"), CalculationResult::Impossible);
    assert_eq!(calculate("21037: 9 7 18 13"), CalculationResult::Impossible);
    assert_eq!(
        calculate("292: 11 6 16 20"),
        CalculationResult::Success {
            result: 292,
            operator_configurations: vec![vec![Operator::Add, Operator::Multiply, Operator::Add],]
        }
    );

    assert_ne!(
        calculate("156: 15 6"),
        CalculationResult::Success {
            result: 156,
            operator_configurations: vec![vec![Operator::Concatenation]]
        }
    );

    assert_eq!(
        calculate("7290: 6 8 6 15"),
        CalculationResult::Success {
            result: 7290,
            operator_configurations: vec![vec![
                Operator::Multiply,
                Operator::Concatenation,
                Operator::Multiply
            ]]
        }
    );
    assert_eq!(
        calculate("192: 17 8 14"),
        CalculationResult::Success {
            result: 192,
            operator_configurations: vec![vec![Operator::Concatenation, Operator::Add]]
        }
    );
}

#[test]
fn test_concat() {
    assert_eq!(concat_nums(15, 6), 156);
}
