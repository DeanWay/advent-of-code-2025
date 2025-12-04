use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(input));
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Location {
    Empty,
    Roll,
}

type Input = Vec<Vec<Location>>;
type Position = (usize, usize);

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            line.chars()
                .map(|c| match c {
                    '@' => Location::Roll,
                    '.' => Location::Empty,
                    _ => panic!("unexpected location char: {c}"),
                })
                .collect()
        })
        .collect()
}

fn adjacent(pos: Position, bounds: Position) -> Vec<Position> {
    let row = pos.0 as i32;
    let col = pos.1 as i32;
    let offsets: [i32; 3] = [-1, 0, 1];
    offsets
        .iter()
        .flat_map(|x| offsets.iter().map(|y| (*x, *y)))
        .filter_map(|(off_r, off_c)| {
            let possible = (row + off_r, col + off_c);
            if possible.0 >= 0
                && possible.1 >= 0
                && (possible.0 as usize) < bounds.0
                && (possible.1 as usize) < bounds.1
                && possible != (row, col)
            {
                Some((possible.0 as usize, possible.1 as usize))
            } else {
                None
            }
        })
        .collect()
}

fn part_1(input: &Input) -> u64 {
    find_removable_positions(&input).count() as u64
}

fn find_removable_positions(input: &Input) -> impl Iterator<Item = Position> {
    let bounds = (input.len(), input[0].len());
    let positions = (0..input.len()).flat_map(|r| (0..input[r].len()).map(move |c| (r, c)));
    positions.filter(move |pos| {
        input[pos.0][pos.1] == Location::Roll
            && (adjacent(*pos, bounds)
                .into_iter()
                .filter(|adjacent| input[adjacent.0][adjacent.1] == Location::Roll)
                .count()
                < 4)
    })
}

fn part_2(mut input: Input) -> u64 {
    let mut total_removed = 0;
    loop {
        let positions_to_remove: Vec<Position> = find_removable_positions(&input).collect();
        if positions_to_remove.len() == 0 {
            break;
        }
        total_removed += positions_to_remove.len() as u64;
        positions_to_remove
            .into_iter()
            .for_each(|(r, c)| input[r][c] = Location::Empty);
    }
    total_removed
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 13);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(input), 43);
    }
}
