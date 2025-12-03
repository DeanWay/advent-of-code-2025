use std::io::{stdin, BufRead};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

type Input = Vec<i32>;

fn parse_input(mut input: impl BufRead) -> Input {
    let mut buf = String::new();
    input.read_to_string(&mut buf).unwrap();
    buf.lines()
        .map(|line| {
            let (direction_str, value_str) = line.split_at(1);
            let direction = match direction_str {
                "L" => -1,
                "R" => 1,
                _ => panic!(),
            };
            let value: i32 = value_str.parse().unwrap();
            value * direction
        })
        .collect()
}

fn part_1(input: &Input) -> u64 {
    input
        .iter()
        .scan(50, |state, &x| {
            *state = (*state + x).rem_euclid(100);
            Some(*state)
        })
        .map(|state| if state == 0 { 1 } else { 0 })
        .sum()
}

fn part_2(input: &Input) -> u64 {
    input
        .iter()
        .scan(50, |state, &x| {
            let started_at_zero = *state == 0;
            let next_absolute = *state + x;
            *state = (next_absolute).rem_euclid(100);
            let adjustment = if !started_at_zero && next_absolute <= 0 {
                1
            } else {
                0
            };
            let div = next_absolute.abs().div_euclid(100) as u64;
            Some(div + adjustment)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    macro_rules! part_2_tests {
        ($($name:ident: $value:expr,)*) => {
        $(
            #[test]
            fn $name() {
                let (input, expected) = $value;
                let input = parse_input(input.as_bytes());
                assert_eq!(expected, part_2(&input));
            }
        )*
        }
    }

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 3);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 6);
    }

    part_2_tests! {
        test_part_2_right_end_at_zero: ("R50", 1),
        test_part_2_right_past_zero: ("R150", 2),
        test_part_2_left_end_at_zero: ("L50", 1),
        test_part_2_left_past_zero: ("L150", 2),
        test_part_2_left_zero_to_zero: ("L50\nL100", 2),
        test_part_2_right_zero_to_zero: ("R50\nR100", 2),
        test_part_2_right_left_zero_to_zero: ("R50\nL100", 2),
        test_part_2_left_right_zero_to_zero: ("L50\nR100", 2),
    }
}
