use std::{
    io::{stdin, BufRead},
    ops::RangeInclusive,
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<RangeInclusive<u64>>;

fn parse_input(mut input: impl BufRead) -> Input {
    let mut text = String::new();
    input.read_to_string(&mut text).unwrap();
    text.split(",")
        .map(|raw_range| {
            let (low, high) = raw_range.split_once('-').unwrap();
            let low = low.parse().unwrap();
            let high = high.parse().unwrap();
            low..=high
        })
        .collect()
}

fn part_1(input: &Input) -> u64 {
    input
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .filter(|x| id_is_invalid(*x))
        .sum()
}

fn id_is_invalid(id: u64) -> bool {
    let id_string = id.to_string();
    if id_string.len() % 2 != 0 {
        return false;
    }
    let (lhs, rhs) = id_string.split_at(id_string.len() / 2);
    lhs == rhs
}

fn part_2(input: &Input) -> u64 {
    input
        .iter()
        .flat_map(|range| range.clone().into_iter())
        .filter(|x| id_is_invalid_part_2(*x))
        .sum()
}

fn id_is_invalid_part_2(id: u64) -> bool {
    let id_string = id.to_string();
    for pattern_len in 1..=id_string.len() / 2 {
        let bytes = id_string.as_bytes();
        let first = &bytes[0..pattern_len];
        if bytes.chunks(pattern_len).all(|chunk| chunk == first) {
            return true;
        }
    }
    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 1227775554);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 4174379265);
    }
}
