use std::{
    collections::{HashMap, HashSet},
    io::{stdin, BufRead},
    mem,
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

enum Location {
    Space,
    Splitter,
}

type Position = (usize, usize);

struct Input {
    start_position: Position,
    map: Vec<Vec<Location>>,
}

fn parse_input(input: impl BufRead) -> Input {
    let mut start_position: Position = (0, 0);
    let map = input
        .lines()
        .map(|line| line.unwrap())
        .enumerate()
        .map(|(row, line)| {
            line.chars()
                .enumerate()
                .map(|(col, char)| match char {
                    '.' => Location::Space,
                    '^' => Location::Splitter,
                    'S' => {
                        start_position = (row, col);
                        Location::Space
                    }
                    _ => panic!("unexpected char: '{char}'"),
                })
                .collect()
        })
        .collect();
    Input {
        start_position,
        map,
    }
}

fn part_1(input: &Input) -> u64 {
    let mut beams = HashSet::<usize>::from_iter([input.start_position.1]);
    let mut times_split = 0;
    for row in 1..input.map.len() {
        let prev_beams = mem::take(&mut beams);
        for beam_col in prev_beams.into_iter() {
            match &input.map[row][beam_col] {
                Location::Space => {
                    beams.insert(beam_col);
                }
                Location::Splitter => {
                    if let Some(col) = beam_col.checked_sub(1) {
                        beams.insert(col);
                    }
                    beams.insert(beam_col + 1);
                    times_split += 1
                }
            }
        }
    }
    times_split
}

fn part_2(input: &Input) -> u64 {
    let mut beams = HashMap::<usize, u64>::from_iter([(input.start_position.1, 1)]);
    for row in 1..input.map.len() {
        let prev_beams = mem::take(&mut beams);
        for (beam_col, paths_to_here) in prev_beams.into_iter() {
            match &input.map[row][beam_col] {
                Location::Space => {
                    let entry = beams.entry(beam_col).or_insert(0);
                    *entry += paths_to_here;
                }
                Location::Splitter => {
                    if let Some(col) = beam_col.checked_sub(1) {
                        let entry = beams.entry(col).or_insert(0);
                        *entry += paths_to_here;
                    }
                    let entry = beams.entry(beam_col + 1).or_insert(0);
                    *entry += paths_to_here;
                }
            }
        }
    }
    beams.values().sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 21);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 40);
    }
}
