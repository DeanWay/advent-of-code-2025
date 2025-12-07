use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Debug, Clone, Copy)]
enum Op {
    Add,
    Mul,
}

#[derive(Debug, Clone)]
struct Expression {
    numbers: Vec<u64>,
    op: Op,
}

type Input = String;

fn parse_input(mut input: impl BufRead) -> Input {
    let mut text = String::new();
    input.read_to_string(&mut text).unwrap();
    text
}

fn parse_expressions_part_1(input: &Input) -> Vec<Expression> {
    let mut lines: Vec<&str> = input.lines().collect();
    let last_line = lines.pop().unwrap();

    let operations: Vec<Op> = last_line
        .split_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("unexpected op: \"{s}\""),
        })
        .collect();
    let number_grid: Vec<Vec<u64>> = lines
        .into_iter()
        .map(|line| {
            line.split_whitespace()
                .map(|word| word.parse().unwrap())
                .collect()
        })
        .collect();
    let num_rows = number_grid.len();
    let num_cols = operations.len();
    (0..num_cols)
        .map(|col| {
            let numbers = (0..num_rows).map(|row| number_grid[row][col]).collect();
            Expression {
                numbers,
                op: operations[col],
            }
        })
        .collect()
}

fn part_1(input: &Input) -> u64 {
    let expressions = parse_expressions_part_1(input);
    return resolve_expressions(&expressions);
}

fn parse_expressions_part_2(input: &Input) -> Vec<Expression> {
    let mut lines: Vec<&str> = input.lines().collect();
    let last_line = lines.pop().unwrap();

    let mut operations: Vec<Op> = last_line
        .split_whitespace()
        .map(|s| match s {
            "+" => Op::Add,
            "*" => Op::Mul,
            _ => panic!("unexpected op: \"{s}\""),
        })
        .rev()
        .collect();
    let digits_grid: Vec<Vec<char>> = lines
        .into_iter()
        .map(|line| line.chars().collect())
        .collect();
    let num_rows = digits_grid.len();
    let num_cols = digits_grid[0].len();
    let mut expressions = Vec::new();
    let mut current_numbers: Vec<u64> = Vec::new();
    for col in 0..num_cols {
        let mut current_digits = String::new();
        for row in 0..num_rows {
            let current_digit = digits_grid[row][col];
            if !current_digit.is_whitespace() {
                current_digits.push(current_digit);
            }
        }
        if current_digits.is_empty() {
            let numbers = current_numbers;
            current_numbers = Vec::new();
            expressions.push(Expression {
                numbers,
                op: operations.pop().unwrap(),
            });
        } else {
            current_numbers.push(current_digits.parse().unwrap());
        }
    }
    if !current_numbers.is_empty() {
        expressions.push(Expression {
            numbers: current_numbers,
            op: operations.pop().unwrap(),
        });
    }
    expressions
}

fn part_2(input: &Input) -> u64 {
    let expressions = parse_expressions_part_2(input);
    return resolve_expressions(&expressions);
}

fn resolve_expressions(expressions: &[Expression]) -> u64 {
    expressions.iter().map(|exp| resolve_expression(exp)).sum()
}

fn resolve_expression(exp: &Expression) -> u64 {
    let init: u64 = match exp.op {
        Op::Add => 0,
        Op::Mul => 1,
    };
    exp.numbers.iter().fold(init, |acc, x| match exp.op {
        Op::Add => acc + x,
        Op::Mul => acc * x,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 4277556);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 3263827);
    }
}
