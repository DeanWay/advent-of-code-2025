use std::{
    io::{stdin, BufRead},
    ops::RangeInclusive,
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Clone, Debug)]
struct Input {
    fresh_id_ranges: Vec<RangeInclusive<u64>>,
    available_ids: Vec<u64>,
}

fn parse_input(mut input: impl BufRead) -> Input {
    let mut s = String::new();
    input.read_to_string(&mut s).unwrap();
    let (fresh_section, avail_section) = s.split_once("\n\n").unwrap();
    let fresh_id_ranges = fresh_section
        .lines()
        .map(|line| {
            let (low, high) = line.split_once('-').unwrap();
            (low.parse().unwrap())..=(high.parse().unwrap())
        })
        .collect();
    let available_ids = avail_section
        .lines()
        .map(|line| line.parse().unwrap())
        .collect();
    return Input {
        fresh_id_ranges,
        available_ids,
    };
}

fn part_1(input: &Input) -> u64 {
    input
        .available_ids
        .iter()
        .filter(|id| input.fresh_id_ranges.iter().any(|range| range.contains(id)))
        .count() as u64
}

fn part_2(input: &Input) -> u64 {
    let mut fresh_id_ranges = input.fresh_id_ranges.clone();
    fresh_id_ranges.sort_by_key(|item| *item.start());
    let mut minimal_ranges = Vec::<RangeInclusive<u64>>::new();
    for range in fresh_id_ranges.iter() {
        let current = minimal_ranges.last().cloned();
        match current {
            None => minimal_ranges.push(range.clone()),
            Some(current) => {
                if current.end() >= range.start() {
                    let start = *current.start();
                    let end = *range.end().max(current.end());
                    minimal_ranges.pop();
                    minimal_ranges.push(start..=end)
                } else {
                    minimal_ranges.push(range.clone())
                }
            }
        }
    }
    minimal_ranges
        .into_iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 14);
    }
}
