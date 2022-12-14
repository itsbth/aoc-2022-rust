use std::collections::HashMap;
use std::fmt::Write;

pub type Point = (usize, usize);

pub type Input = (Point, Point, HashMap<Point, usize>);

pub fn generator(input: &str) -> Input {
    use aoc_parse::{parser, prelude::*};

    let data = parser!(lines(alpha+)).parse(input).unwrap();
    let mut map = HashMap::new();
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (i, line) in data.iter().enumerate() {
        for (j, c) in line.iter().enumerate() {
            if c == &'S' {
                start = (i, j);
                map.insert((i, j), 0);
            } else if c == &'E' {
                end = (i, j);
                map.insert((i, j), 25);
            } else {
                // elevation is a..z, where z is the highest
                map.insert((i, j), *c as usize - 'a' as usize);
            }
        }
    }
    (start, end, map)
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Node {
    i: usize,
    j: usize,
    dist: usize,
    estimated_dist: usize,
}

impl std::cmp::Ord for Node {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.estimated_dist.cmp(&self.estimated_dist)
    }
}

impl std::cmp::PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

// visited = grey, heap = green (using ANSI codes)
fn print_grid(
    input: &HashMap<(usize, usize), usize>,
    visited: &std::collections::HashSet<(usize, usize)>,
    heap: &std::collections::BinaryHeap<Node>,
) {
    // return;
    let heap_set: std::collections::HashSet<_> = heap.iter().cloned().collect();
    let mut buffer = String::new();
    // clear screen
    buffer.push_str("\x1b[2J");
    for i in 0..input.keys().map(|(i, _)| i).max().unwrap() + 1 {
        for j in 0..input.keys().map(|(_, j)| j).max().unwrap() + 1 {
            let c = input
                .get(&(i, j))
                .map(|&z| (z + 'a' as usize) as u8 as char)
                .unwrap_or(' ');
            if heap_set
                .iter()
                .find(|&&Node { i: i2, j: j2, .. }| i == i2 && j == j2)
                .is_some()
            {
                let _ = write!(buffer, "\x1b[32m{}\x1b[0m", c);
            } else if visited.contains(&(i, j)) {
                let _ = write!(buffer, "\x1b[90m{}\x1b[0m", c);
            } else {
                let _ = write!(buffer, "{}", c);
            }
        }
        let _ = write!(buffer, "\n");
    }
    print!("{}", buffer);
    // sleep 1/24th of a second
    std::thread::sleep(std::time::Duration::from_millis(1000 / 24));
}

// find the length of the shortest path from (0, 0) to the highest point (z).
// we can only move to a point that is at most 1 higher than our current point.
pub fn part_1((start, end, input): &Input) -> usize {
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(Node {
        i: start.0,
        j: start.1,
        dist: 0,
        estimated_dist: 0,
    });
    let heuristic = |Node { i, j, .. }| {
        ((i as isize - end.0 as isize).abs() + (j as isize - end.1 as isize).abs()) as usize
    };
    let mut visited = std::collections::HashSet::new();
    visited.insert(*start);
    while let Some(Node { i, j, dist, .. }) = heap.pop() {
        print_grid(input, &visited, &heap);
        if (i, j) == *end {
            return dist;
        }
        // let neighbours = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        let neighbours = [
            (i.wrapping_sub(1), j + 1),
            (i + 1, j + 1),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ];
        for (i2, j2) in neighbours.into_iter() {
            if visited.contains(&(i2, j2)) {
                continue;
            }
            if let Some(&z) = input.get(&(i2, j2)) {
                if z <= input[&(i, j)] + 1 {
                    visited.insert((i2, j2));
                    heap.push(Node {
                        i: i2,
                        j: j2,
                        dist: dist + 1,
                        estimated_dist: dist
                            + 1
                            + heuristic(Node {
                                i: i2,
                                j: j2,
                                dist: 0,
                                estimated_dist: 0,
                            }),
                    });
                }
            }
        }
    }
    panic!("no path found")
}

pub fn part_1_dijkstras((start, end, input): &Input) -> usize {
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(Node {
        i: start.0,
        j: start.1,
        dist: 0,
        estimated_dist: 0,
    });
    // a star with a zero heuristic is dijkstras
    let heuristic = |Node { i, j, .. }| 0;
    let mut visited = std::collections::HashSet::new();
    visited.insert(*start);
    while let Some(Node { i, j, dist, .. }) = heap.pop() {
        print_grid(input, &visited, &heap);
        if (i, j) == *end {
            return dist;
        }
        // let neighbours = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        let neighbours = [
            (i.wrapping_sub(1), j + 1),
            (i + 1, j + 1),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ];
        for (i2, j2) in neighbours.into_iter() {
            if visited.contains(&(i2, j2)) {
                continue;
            }
            if let Some(&z) = input.get(&(i2, j2)) {
                if z <= input[&(i, j)] + 1 {
                    visited.insert((i2, j2));
                    heap.push(Node {
                        i: i2,
                        j: j2,
                        dist: dist + 1,
                        estimated_dist: dist
                            + 1
                            + heuristic(Node {
                                i: i2,
                                j: j2,
                                dist: 0,
                                estimated_dist: 0,
                            }),
                    });
                }
            }
        }
    }
    panic!("no path found")
}

// part 2 is similar to part 1, but now we want to find the nearest tile to the end of height 0
pub fn part_2((_start, end, input): &Input) -> usize {
    let mut heap = std::collections::BinaryHeap::new();
    heap.push(Node {
        i: end.0,
        j: end.1,
        dist: 0,
        estimated_dist: 0,
    });
    let mut visited = std::collections::HashSet::new();
    visited.insert(*end);
    while let Some(Node { i, j, dist, .. }) = heap.pop() {
        print_grid(input, &visited, &heap);
        if input[&(i, j)] == 0 {
            return dist;
        }
        // let neighbours = [(i - 1, j), (i + 1, j), (i, j - 1), (i, j + 1)];
        let neighbours = [
            (i.wrapping_sub(1), j + 1),
            (i + 1, j + 1),
            (i, j.wrapping_sub(1)),
            (i, j + 1),
        ];
        for (i2, j2) in neighbours.into_iter() {
            if visited.contains(&(i2, j2)) {
                continue;
            }
            if let Some(&z) = input.get(&(i2, j2)) {
                // need to flip this. As we're running this in "reverse", it's down one or up any
                if z >= input[&(i, j)] - 1 {
                    visited.insert((i2, j2));
                    heap.push(Node {
                        i: i2,
                        j: j2,
                        dist: dist + 1,
                        estimated_dist: dist + 1,
                    });
                }
            }
        }
    }
    panic!("no path found")
}

mod test {
    use super::*;

    #[test]
    fn test_generator() {
        let input = "abcde\n\
        fghij\n\
        klmno\n";
        let expected: HashMap<(usize, usize), usize> = HashMap::from_iter(vec![
            ((0, 0), 0),
            ((1, 0), 1),
            ((2, 0), 2),
            ((3, 0), 3),
            ((4, 0), 4),
            ((0, 1), 5),
            ((1, 1), 6),
            ((2, 1), 7),
            ((3, 1), 8),
            ((4, 1), 9),
            ((0, 2), 10),
            ((1, 2), 11),
            ((2, 2), 12),
            ((3, 2), 13),
            ((4, 2), 14),
        ]);
        let keys = expected.keys().cloned().collect::<Vec<_>>();
        let (_, _, actual) = generator(input);
        assert_eq!(actual.len(), expected.len(), "lengths differ");
        for k in keys {
            // TODO: It's easier to flip the key, but I should really fix the expected hashmap
            let flipped = (k.1, k.0);
            assert_eq!(actual.get(&flipped), expected.get(&k), "key: {:?}", k);
        }
    }
}
