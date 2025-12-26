use std::{
    collections::{HashMap, HashSet, VecDeque},
    io::{stdin, BufRead},
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input));
    println!("part 2: {}", part_2(&input));
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Point2D {
    x: usize,
    y: usize,
}

type Input = Vec<Point2D>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut split = line.split(",");
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            Point2D { x, y }
        })
        .collect()
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Rect {
    a: Point2D,
    b: Point2D,
}

fn part_1(input: &Input) -> u128 {
    (0..input.len())
        .flat_map(|a| {
            (0..a).map(move |b| Rect {
                a: input[a].clone(),
                b: input[b].clone(),
            })
        })
        .map(|rect| area_between_points(&rect.a, &rect.b))
        .max()
        .unwrap()
}

fn part_2(input: &Input) -> u128 {
    let y_to_grid_row: HashMap<usize, usize> = {
        let mut rows = input
            .iter()
            .map(|point| point.y)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        rows.sort();
        rows.into_iter()
            .enumerate()
            .map(|(i, row)| (row, i + 1))
            .collect()
    };
    let x_to_grid_col: HashMap<usize, usize> = {
        let mut cols = input
            .iter()
            .map(|point| point.x)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect::<Vec<_>>();
        cols.sort();
        cols.into_iter()
            .enumerate()
            .map(|(i, col)| (col, i + 1))
            .collect()
    };
    let grid_row_to_y: HashMap<usize, usize> =
        y_to_grid_row.iter().map(|(y, row)| (*row, *y)).collect();
    let grid_col_to_x: HashMap<usize, usize> =
        x_to_grid_col.iter().map(|(x, col)| (*col, *x)).collect();

    let compressed_points: Vec<Point2D> = input
        .iter()
        .map(|point| Point2D {
            x: x_to_grid_col[&point.x],
            y: y_to_grid_row[&point.y],
        })
        .collect();
    let mut grid = vec![vec![false; x_to_grid_col.len() + 3]; y_to_grid_row.len() + 3];

    let by_row = {
        let mut by_row = HashMap::<usize, Vec<Point2D>>::new();
        for point in compressed_points.iter() {
            let entry = by_row.entry(point.y).or_default();
            entry.push(point.clone());
        }
        by_row
    };
    let by_col = {
        let mut by_col = HashMap::<usize, Vec<Point2D>>::new();
        for point in compressed_points.iter() {
            let entry = by_col.entry(point.x).or_default();
            entry.push(point.clone());
        }
        by_col
    };
    for point in compressed_points.iter() {
        grid[point.y][point.x] = true
    }
    for (row, pair) in by_row.iter() {
        let start = pair[0].x.min(pair[1].x);
        let end = pair[0].x.max(pair[1].x);
        let row = *row;
        for col in start..=end {
            grid[row][col] = true;
        }
    }
    for (col, pair) in by_col.iter() {
        let start = pair[0].y.min(pair[1].y);
        let end = pair[0].y.max(pair[1].y);
        let col = *col;
        for row in start..=end {
            grid[row][col] = true;
        }
    }
    // print_grid(&grid);
    flood_fill(&mut grid);
    for (row, pair) in by_row.iter() {
        let start = pair[0].x.min(pair[1].x);
        let end = pair[0].x.max(pair[1].x);
        let row = *row;
        for col in start..=end {
            grid[row][col] = false;
        }
    }
    for (col, pair) in by_col.iter() {
        let start = pair[0].y.min(pair[1].y);
        let end = pair[0].y.max(pair[1].y);
        let col = *col;
        for row in start..=end {
            grid[row][col] = false;
        }
    }
    // print_grid(&grid);
    let rect = (0..compressed_points.len())
        .flat_map(|a| {
            let points = &compressed_points;
            (0..a).map(move |b| Rect {
                a: points[a].clone(),
                b: points[b].clone(),
            })
        })
        .filter(|rect| {
            let bottom = rect.a.y.min(rect.b.y);
            let top = rect.a.y.max(rect.b.y);
            let left = rect.a.x.min(rect.b.x);
            let right = rect.a.x.max(rect.b.x);
            for row in bottom..=top {
                for col in left..=right {
                    if grid[row][col] {
                        return false;
                    }
                }
            }
            return true;
        })
        .max_by_key(|rect| area_between_points(&rect.a, &rect.b))
        .unwrap();
    // print_grid_with_rect(&grid, &rect);
    area_between_points(
        &Point2D {
            x: grid_col_to_x[&rect.a.x],
            y: grid_row_to_y[&rect.a.y],
        },
        &Point2D {
            x: grid_col_to_x[&rect.b.x],
            y: grid_row_to_y[&rect.b.y],
        },
    )
}

fn flood_fill(grid: &mut Vec<Vec<bool>>) {
    let mut stack = VecDeque::<(usize, usize)>::new();
    stack.push_back((0, 0));
    while let Some((row, col)) = stack.pop_back() {
        if grid[row][col] == true {
            continue;
        }
        grid[row][col] = true;
        let deltas = [(-1, 0), (0, -1), (1, 0), (0, 1)];
        for (dy, dx) in deltas.into_iter() {
            let row = row as i32 + dy;
            let col = col as i32 + dx;
            if row < 0 || col < 0 {
                continue;
            }
            let row = row as usize;
            let col = col as usize;
            if row >= grid.len() || col >= grid[row].len() {
                continue;
            }
            if grid[row][col] == true {
                continue;
            }
            stack.push_back((row, col));
        }
    }
}

#[allow(unused)]
fn print_grid(grid: &Vec<Vec<bool>>) {
    for row in grid.iter() {
        for cell in row.iter() {
            if *cell {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

#[allow(unused)]
fn print_grid_with_rect(grid: &Vec<Vec<bool>>, rect: &Rect) {
    let bottom = rect.a.y.min(rect.b.y);
    let top = rect.a.y.max(rect.b.y);
    let left = rect.a.x.min(rect.b.x);
    let right = rect.a.x.max(rect.b.x);
    for row in 0..grid.len() {
        for col in 0..grid[row].len() {
            let cell = &grid[row][col];
            if bottom <= row && row <= top && left <= col && col <= right {
                print!("#")
            } else if *cell {
                print!("X")
            } else {
                print!(".")
            }
        }
        println!()
    }
}

fn area_between_points(a: &Point2D, b: &Point2D) -> u128 {
    let x1 = a.x as i64;
    let x2 = b.x as i64;
    let y1 = a.y as i64;
    let y2 = b.y as i64;
    ((x2 - x1).abs() as u128 + 1) * ((y2 - y1).abs() as u128 + 1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input), 50);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 24);
    }
}
