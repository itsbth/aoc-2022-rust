use std::{
    collections::{HashMap, HashSet},
    time::Instant,
};

use rayon::max_num_threads;

pub struct Valve {
    flow: usize,
    edges: HashSet<String>,
}

const MAX_VALVES: usize = 64;
pub struct IndexValve {
    flow: usize,
    // adjacency list
    edges: [bool; MAX_VALVES],
    costs: [usize; MAX_VALVES],
}

// Valve AA has flow rate=0; tunnels lead to valves DD, II, BB
pub fn generator(input: &str) -> Vec<IndexValve> {
    use aoc_parse::{parser, prelude::*};
    let line = parser!(
        "Valve " (name: alpha+)
        " has flow rate=" (flow: usize)
        "; tunnel lead to valve " (edges: repeat_sep(alpha+, ", "))
        => (name.iter().collect::<String>(), Valve { flow, edges: edges.iter().map(|s| s.iter().collect()).collect() })
    );
    let map: HashMap<String, Valve> = parser!(lines(line))
        .parse(input)
        .unwrap()
        .into_iter()
        .collect();
    let mut names = map.keys().map(|s| s.as_str()).collect::<Vec<_>>();
    names.sort_unstable();
    let names_map: HashMap<&str, usize> = names.iter().enumerate().map(|(i, s)| (*s, i)).collect();

    // for name in names.iter() {
    //     println!("{}: {}", name, names_map.get(name).unwrap());
    // }

    names
        .iter()
        .map(|name| map.get(*name).unwrap())
        .map(|valve| {
            let mut edges = [false; MAX_VALVES];
            for edge in &valve.edges {
                let idx = names_map.get(edge.as_str()).unwrap();
                edges[*idx] = true;
            }
            IndexValve {
                flow: valve.flow,
                edges,
                costs: [1; MAX_VALVES],
            }
        })
        .collect()
}

fn floyd_warshall(nodes: &Vec<IndexValve>) -> Vec<Vec<usize>> {
    let mut costs = vec![vec![std::usize::MAX; nodes.len()]; nodes.len()];
    for (i, node) in nodes.iter().enumerate() {
        for (j, edge) in node.edges.iter().enumerate() {
            if *edge {
                costs[i][j] = node.costs[j];
            }
        }
    }
    for k in 0..nodes.len() {
        for i in 0..nodes.len() {
            for j in 0..nodes.len() {
                if costs[i][k] != std::usize::MAX && costs[k][j] != std::usize::MAX {
                    costs[i][j] = std::cmp::min(costs[i][j], costs[i][k] + costs[k][j]);
                }
            }
        }
    }
    costs
}

pub fn part_1(input: &Vec<IndexValve>) -> usize {
    #[derive(Debug, Clone, PartialEq, Eq, Hash)]
    struct State {
        pos: usize,
        flow: usize,
        open: [bool; MAX_VALVES],
        time: usize,
        // for debugging
        // path: Vec<usize>,
    }
    impl Ord for State {
        fn cmp(&self, other: &Self) -> std::cmp::Ordering {
            self.flow.cmp(&other.flow)
        }
    }
    impl PartialOrd for State {
        fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
            Some(self.cmp(other))
        }
    }
    let costs = floyd_warshall(input);
    let non_zero_nodes = input
        .iter()
        .enumerate()
        .filter(|(_, node)| node.flow != 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    let mut queue = std::collections::BinaryHeap::new();
    queue.push(State {
        pos: 0,
        flow: 0,
        open: [false; MAX_VALVES],
        time: 0,
        // path: vec![0],
    });
    // let mut max_flow = 0;
    let mut max_state = State {
        pos: 0,
        flow: 0,
        open: [false; MAX_VALVES],
        time: 0,
        // path: vec![0],
    };
    let mut visited = HashSet::new();
    let mut iterations = 0; // just used for printing progress
    fn fact(n: usize) -> usize {
        (1..=n).product()
    }
    // theoretical upper bound on the number of states
    let start = Instant::now();
    let limit = fact(non_zero_nodes.len());
    while let Some(state) = queue.pop() {
        // if iterations % 100_000 == 0 {
        //     let elapsed = start.elapsed().as_secs_f64();
        //     let eta = elapsed * (limit as f64 / iterations as f64);
        //     let seconds = eta % 60.0;
        //     let minutes = (eta / 60.0) % 60.0;
        //     let hours = eta / 3600.0;

        //     println!(
        //         "iterations: {} ({:.4}%, eta {}h {}m {:.4}s)",
        //         iterations,
        //         (iterations as f64 * 100.0) / (limit as f64),
        //         hours.floor(),
        //         minutes.floor(),
        //         seconds
        //     );
        //     println!("max_flow: {}", max_state.flow);
        //     println!("queue: {}", queue.len());
        //     println!("visited: {}", visited.len());
        // }
        // iterations += 1;
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        if state.flow > max_state.flow {
            max_state = state.clone();
        }
        let State {
            pos,
            flow,
            open,
            time,
            // path,
        } = state;
        if time == 30 {
            // out of time
            continue;
        }
        // nodes that have a non-zero flow rate and are not open
        let remaining_nodes = non_zero_nodes.iter().filter(|&&i| !open[i]);
        for &node in remaining_nodes {
            let new_time = time + costs[pos][node] + 1; // +1 for the time it takes to open the valve
            if new_time > 30 {
                continue;
            }
            let mut new_open = open;
            new_open[node] = true;
            let new_flow = flow + input[node].flow * (30 - new_time);
            // let mut path = path.clone();
            // path.push(node);
            queue.push(State {
                pos: node,
                flow: new_flow,
                open: new_open,
                time: new_time,
                // path,
            });
        }
    }
    // try to recreate the path to verify that it's correct
    // {
    //     let mut time = 0;
    //     let mut flow = 0;
    //     let mut pos = 0;

    //     for &node in max_state.path[1..].iter() {
    //         let cost = costs[pos][node];
    //         time += cost;
    //         println!("{} -> {} (cost {})", pos, node, cost);
    //         pos = node;
    //         time += 1; // time to open the valve
    //         flow += input[node].flow * (30 - time);
    //     }
    //     // assert_eq!(time, 30); // time might be less than 30 if there are no reachable valves left
    //     assert_eq!(flow, max_state.flow);
    // }
    max_state.flow
}

// instead of a graph search, find each valid permutation of valves and calculate the flow
pub(crate) fn part_1_permutations(input: &Vec<IndexValve>) -> usize {
    let costs = floyd_warshall(input);
    let non_zero_nodes = input
        .iter()
        .enumerate()
        .filter(|(_, node)| node.flow != 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();
    fn paths(
        costs: &Vec<Vec<usize>>,
        from: usize,
        nodes: &[usize],
        time: usize,
    ) -> Vec<Vec<usize>> {
        if nodes.is_empty() {
            return vec![vec![]];
        }
        let mut result = Vec::new();
        for (i, &node) in nodes.iter().enumerate() {
            let cost = costs[from][node];
            if time + cost + 1 > 30 {
                continue;
            }
            let mut new_nodes = nodes.to_vec();
            new_nodes.remove(i);
            let subpaths = paths(costs, node, &new_nodes, time + cost + 1);
            let path = vec![node];
            result.push(path.clone());
            for subpath in subpaths {
                let mut path = path.clone();
                path.extend(subpath);
                result.push(path);
            }
        }
        result
    }
    let max_flow = paths(&costs, 0, &non_zero_nodes, 0)
        .iter()
        .map(|path| {
            let mut time = 0;
            let mut flow = 0;
            let mut pos = 0;
            for &node in path.iter() {
                let cost = costs[pos][node];
                time += cost;
                pos = node;
                time += 1; // time to open the valve
                flow += input[node].flow * (30 - time);
            }
            flow
        })
        .max()
        .unwrap();
    max_flow
}

fn cartesian_product<T: Clone>(a: &[T], b: &[T]) -> Vec<(T, T)> {
    let mut result = Vec::new();
    for x in a {
        for y in b {
            result.push((x.clone(), y.clone()));
        }
    }
    result
}

// same as part 1, but with an elephant (that we spent 4 minutes to teach how to open valves)
pub fn part_2(input: &Vec<IndexValve>) -> usize {
    let costs = floyd_warshall(input);

    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    struct Agent {
        pos: usize,
        // when the agent will be able to open the next valve
        next_action: usize,
    }

    #[derive(Clone, Debug, Eq, PartialEq, Hash)]
    struct State {
        me: Agent,
        elephant: Agent,
        flow: usize,
        open: [bool; MAX_VALVES],
        time: usize,
    }

    // as we're exploring the whole state space, we can use a simple BFS
    let mut queue = vec![State {
        me: Agent {
            pos: 0,
            next_action: 0,
        },
        elephant: Agent {
            pos: 0,
            next_action: 0,
        },
        flow: 0,
        open: [false; MAX_VALVES],
        time: 0,
    }];

    let non_zero_nodes = input
        .iter()
        .enumerate()
        .filter(|(_, node)| node.flow != 0)
        .map(|(i, _)| i)
        .collect::<Vec<_>>();

    let mut visited = HashSet::new();
    let mut max_flow = 0;

    let max_time = 26; // 30 - 4 (time to teach the elephant)

    while let Some(state) = queue.pop() {
        if visited.contains(&state) {
            continue;
        }
        visited.insert(state.clone());
        let State {
            me,
            elephant,
            flow,
            open,
            time,
        } = state;
        if flow > max_flow {
            max_flow = flow;
            // println!("max_flow: {}", max_flow);
            // println!("queue: {}", queue.len());
            // println!("visited: {}", visited.len());
        }
        let remaining_nodes = non_zero_nodes
            .iter()
            .filter(|&&i| !open[i])
            .collect::<Vec<_>>();
        // bail early if there's no way to improve the flow (ie sum of remaining flows is less max - current)
        let remaining_flow = remaining_nodes
            .iter()
            .map(|&&i| input[i].flow * (max_time - time))
            .sum::<usize>();

        if remaining_flow + flow <= max_flow {
            continue;
        }

        match (me.next_action <= time, elephant.next_action <= time) {
            (true, true) => {
                for (&me_node, &elephant_node) in
                    cartesian_product(&remaining_nodes, &remaining_nodes)
                {
                    // no point in opening the same valve twice
                    if me_node == elephant_node {
                        continue;
                    }
                    let me_cost = costs[me.pos][me_node];
                    let elephant_cost = costs[elephant.pos][elephant_node];
                    // new_time is the time when the first agent will be able to open the next valve
                    let new_time = time + me_cost.min(elephant_cost) + 1;
                    if new_time > max_time {
                        continue;
                    }
                    let mut new_open = open;
                    new_open[me_node] = true;
                    new_open[elephant_node] = true;
                    let me_flow = input[me_node].flow * (max_time - time - me_cost - 1);
                    let elephant_flow =
                        input[elephant_node].flow * (max_time - time - elephant_cost - 1);
                    let new_flow = flow + me_flow + elephant_flow;
                    queue.push(State {
                        me: Agent {
                            pos: me_node,
                            next_action: time + me_cost + 1,
                        },
                        elephant: Agent {
                            pos: elephant_node,
                            next_action: time + elephant_cost + 1,
                        },
                        flow: new_flow,
                        open: new_open,
                        time: new_time,
                    });
                }
            }
            (true, false) => {
                for &me_node in remaining_nodes.iter() {
                    let me_cost = costs[me.pos][*me_node];
                    let new_time = elephant.next_action.min(time + me_cost + 1);
                    if new_time > max_time {
                        continue;
                    }
                    let mut new_open = open;
                    new_open[*me_node] = true;
                    let me_flow = input[*me_node].flow * (max_time - time - me_cost - 1);
                    let new_flow = flow + me_flow;
                    queue.push(State {
                        me: Agent {
                            pos: *me_node,
                            next_action: time + me_cost + 1,
                        },
                        elephant: elephant.clone(),
                        flow: new_flow,
                        open: new_open,
                        time: new_time,
                    });
                }
            }
            (false, true) => {
                for &elephant_node in remaining_nodes.iter() {
                    let elephant_cost = costs[elephant.pos][*elephant_node];
                    let new_time = me.next_action.min(time + elephant_cost + 1);
                    if new_time > max_time {
                        continue;
                    }
                    let mut new_open = open;
                    new_open[*elephant_node] = true;
                    let elephant_flow =
                        input[*elephant_node].flow * (max_time - time - elephant_cost - 1);
                    let new_flow = flow + elephant_flow;
                    queue.push(State {
                        me: me.clone(),
                        elephant: Agent {
                            pos: *elephant_node,
                            next_action: time + elephant_cost + 1,
                        },
                        flow: new_flow,
                        open: new_open,
                        time: new_time,
                    });
                }
            }
            (false, false) => {
                unreachable!("both agents are busy");
            }
        }
    }
    max_flow
}

mod tests {
    use super::*;

    const SAMPLE_INPUT: &str = include_str!("../day16.sample");

    #[test]
    fn test_generator() {
        fn edges(inp: &[usize]) -> [bool; MAX_VALVES] {
            let mut ret = [false; MAX_VALVES];
            for &i in inp {
                ret[i] = true;
            }
            ret
        }
        let input = generator(SAMPLE_INPUT);
        for (idx, node) in input.iter().enumerate() {
            let edges = node
                .edges
                .iter()
                .enumerate()
                .filter(|(_, &b)| b)
                .map(|(i, _)| i.to_string())
                .collect::<Vec<_>>();
            println!("{} => {}", idx, edges.join(", "));
        }
        let expected_edges = vec![
            edges(&[3, 8, 1]), // 0
            edges(&[2, 0]),    // 1
            edges(&[3, 1]),    // 2
            edges(&[2, 0, 4]), // 3
            edges(&[5, 3]),    // 4
            edges(&[4, 6]),    // 5
            edges(&[5, 7]),    // 6
            edges(&[6]),       // 7
            edges(&[0, 9]),    // 8
            edges(&[8]),       // 9
        ];
        for (i, node) in input.iter().enumerate() {
            assert_eq!(node.edges, expected_edges[i], "node {}", i);
        }
    }

    #[test]
    fn test_floyd_warshall() {
        // helper function to avoid having to write out the full array
        fn extend<T>(a: T, b: T, c: T) -> [T; MAX_VALVES]
        where
            T: Default + Copy,
        {
            let mut ret = [T::default(); MAX_VALVES];
            ret[0] = a;
            ret[1] = b;
            ret[2] = c;
            ret
        }
        // three nodes, arranged in a ring (a -> b -> c -> a)
        let input = vec![
            IndexValve {
                flow: 0,
                edges: extend(false, true, false),
                costs: extend(1, 1, 1),
            },
            IndexValve {
                flow: 0,
                edges: extend(false, false, true),
                costs: extend(1, 1, 1),
            },
            IndexValve {
                flow: 0,
                edges: extend(true, false, false),
                costs: extend(1, 1, 1),
            },
        ];
        let costs = floyd_warshall(&input);
        for i in 0..3 {
            for j in 0..3 {
                print!("{:3} ", costs[i][j])
            }
            println!();
        }
        assert_eq!(costs[0][0], 3);
        assert_eq!(costs[0][1], 1);
        assert_eq!(costs[0][2], 2);
    }
}
