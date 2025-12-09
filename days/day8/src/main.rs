use std::{
    collections::{BinaryHeap, HashMap},
    io::{stdin, BufRead},
    u128,
};

fn main() {
    let input = parse_input(stdin().lock());
    println!("part 1: {}", part_1(&input, 1000));
    println!("part 2: {}", part_2(&input));
}

struct Point3D {
    x: u64,
    y: u64,
    z: u64,
}

type Input = Vec<Point3D>;

fn parse_input(input: impl BufRead) -> Input {
    input
        .lines()
        .map(|line| line.unwrap())
        .map(|line| {
            let mut split = line.split(",");
            let x = split.next().unwrap().parse().unwrap();
            let y = split.next().unwrap().parse().unwrap();
            let z = split.next().unwrap().parse().unwrap();
            Point3D { x, y, z }
        })
        .collect()
}

struct MinDistanceEdge {
    connection: (usize, usize),
    distance: u128,
}

impl PartialEq for MinDistanceEdge {
    fn eq(&self, other: &Self) -> bool {
        self.distance == other.distance
    }
}
impl Eq for MinDistanceEdge {}
impl PartialOrd for MinDistanceEdge {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        other.distance.partial_cmp(&self.distance)
    }
}
impl Ord for MinDistanceEdge {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.distance.cmp(&self.distance)
    }
}

struct DisjointSetUnion {
    rank: Vec<usize>,
    parent: Vec<usize>,
}

impl DisjointSetUnion {
    fn new(size: usize) -> Self {
        Self {
            rank: vec![0; size],
            parent: (0..size).collect(),
        }
    }

    fn build_sets(&self) -> Vec<Vec<usize>> {
        let mut map = HashMap::<usize, Vec<usize>>::new();
        for i in 0..self.parent.len() {
            let root = self.find_root(i);
            let entry = map.entry(root).or_insert_with(Vec::new);
            entry.push(i);
        }
        map.into_values().collect()
    }

    fn find_root(&self, x: usize) -> usize {
        let root = self.parent[x];
        if self.parent[root] != root {
            return self.find_root(root);
        }
        return root;
    }

    fn join(&mut self, x: usize, y: usize) {
        let x_root = self.find_root(x);
        let y_root = self.find_root(y);

        if x_root == y_root {
            return;
        }

        if self.rank[x_root] < self.rank[y_root] {
            self.parent[x_root] = y_root;
        } else if self.rank[y_root] < self.rank[x_root] {
            self.parent[y_root] = x_root;
        } else {
            self.parent[y_root] = x_root;

            self.rank[x_root] = self.rank[x_root] + 1;
        }
    }
}

fn part_1(input: &Input, n: usize) -> u64 {
    let distance_squared_matrix = calculate_square_distance_between_all_points(input);
    let mut heap_of_edges_by_min_distance = BinaryHeap::from_iter(
        distance_squared_matrix
            .into_iter()
            .map(|(connection, distance)| MinDistanceEdge {
                connection,
                distance,
            }),
    );
    let mut connected_edges: DisjointSetUnion = DisjointSetUnion::new(input.len());
    for _ in 0..n {
        let next_min = heap_of_edges_by_min_distance.pop().unwrap();
        connected_edges.join(next_min.connection.0, next_min.connection.1);
    }
    let mut clusters = connected_edges.build_sets();
    clusters.sort_by_key(|cluster| -1 * cluster.len() as i64);
    clusters
        .iter()
        .take(3)
        .map(|cluster| cluster.len() as u64)
        .product()
}

fn calculate_square_distance_between_all_points(input: &Input) -> HashMap<(usize, usize), u128> {
    let mut distance_squared_matrix = HashMap::new();
    for a in 0..input.len() {
        for b in 0..a {
            distance_squared_matrix
                .insert((a, b), calculate_distance_squared_3d(&input[a], &input[b]));
        }
    }
    distance_squared_matrix
}

fn calculate_distance_squared_3d(a: &Point3D, b: &Point3D) -> u128 {
    let x1 = a.x as i64;
    let x2 = b.x as i64;
    let y1 = a.y as i64;
    let y2 = b.y as i64;
    let z1 = a.z as i64;
    let z2 = b.z as i64;
    ((x2 - x1).abs().pow(2) + (y2 - y1).abs().pow(2) + (z2 - z1).abs().pow(2)) as u128
}

fn part_2(input: &Input) -> u64 {
    let distance_squared_matrix = calculate_square_distance_between_all_points(input);
    let mut heap_of_edges_by_min_distance = BinaryHeap::from_iter(
        distance_squared_matrix
            .into_iter()
            .map(|(connection, distance)| MinDistanceEdge {
                connection,
                distance,
            }),
    );
    let mut connected_edges: DisjointSetUnion = DisjointSetUnion::new(input.len());
    let mut edges_added = 0;
    while !heap_of_edges_by_min_distance.is_empty() {
        let next_min = heap_of_edges_by_min_distance.pop().unwrap();
        let a = next_min.connection.0;
        let b = next_min.connection.1;
        if connected_edges.find_root(a) != connected_edges.find_root(b) {
            edges_added += 1;
            if edges_added == input.len() - 1 {
                return input[a].x * input[b].x;
            }
            connected_edges.join(next_min.connection.0, next_min.connection.1);
        }
    }
    panic!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = include_str!("../example.txt");

    #[test]
    fn test_part_1_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_1(&input, 10), 40);
    }

    #[test]
    fn test_part_2_example() {
        let input = parse_input(EXAMPLE.as_bytes());
        assert_eq!(part_2(&input), 25272);
    }
}
