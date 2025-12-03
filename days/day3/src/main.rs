use std::{
    collections::BinaryHeap,
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<Vec<u32>>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect()
}

fn part_1(input: &Input) -> u64 {
    input.iter().map(|bank| joltage_part_1(bank)).sum()
}

fn joltage_part_1(bank: &[u32]) -> u64 {
    let (max_first_digit_index, max_first_digit) = &bank[0..bank.len() - 1]
        .iter()
        .enumerate()
        .max_by(|x, y| x.1.cmp(y.1).then(y.0.cmp(&x.0)))
        .unwrap();
    let max_second_digit = &bank[*max_first_digit_index + 1..bank.len()]
        .iter()
        .max()
        .unwrap();
    (*max_first_digit * 10 + *max_second_digit) as u64
}

fn part_2(input: &Input) -> u64 {
    input.iter().map(|bank| joltage_part_2(bank)).sum()
}

fn joltage_part_2(bank: &[u32]) -> u64 {
    let mut heap =
        BinaryHeap::from_iter(bank.iter().enumerate().map(|(i, x)| (*x, bank.len() - i)));
    let mut batteries_needed = 12;
    let mut last_battery_rank = bank.len() + 1;
    let mut sum = 0;
    let mut skipped = Vec::new();
    while batteries_needed > 0 {
        let (current, rank_of_current) = heap.pop().unwrap();
        if rank_of_current > last_battery_rank {
            continue;
        }
        if rank_of_current < batteries_needed {
            skipped.push((current, rank_of_current));
            continue;
        }
        last_battery_rank = rank_of_current;
        batteries_needed -= 1;
        sum += current as u64 * 10u64.pow(batteries_needed as u32);
        skipped.drain(0..).for_each(|item| heap.push(item));
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");
    const INPUT: &str = include_str!("../input.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 357);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 3121910778619);
    }
}
