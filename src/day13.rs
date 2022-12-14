use std::cmp::PartialOrd;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum List {
    Int(i64),
    List(Vec<List>),
}

// lexical sort, but ints are treated as a single-element list when compared to another list
impl PartialOrd for List {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (List::Int(a), List::Int(b)) => a.partial_cmp(b),
            (List::Int(a), List::List(b)) => {
                let al = List::List(vec![List::Int(*a)]);
                al.partial_cmp(other)
            }
            (List::List(a), List::Int(b)) => {
                let bl = List::List(vec![List::Int(*b)]);
                self.partial_cmp(&bl)
            }
            (List::List(a), List::List(b)) => {
                for (a, b) in a.iter().zip(b.iter()) {
                    match a.partial_cmp(b) {
                        Some(std::cmp::Ordering::Equal) => continue,
                        Some(order) => return Some(order),
                        None => return None,
                    }
                }
                a.len().partial_cmp(&b.len())
            }
        }
    }
}

pub type Input = Vec<(List, List)>;

// parse nested lists of integers, eg: [[1,2],[3,4]]
fn parse_list(input: &str) -> (List, &str) {
    let mut input = input.trim();
    let mut list = Vec::new();
    while !input.is_empty() {
        let (value, rest) = if input.starts_with('[') {
            let (value, rest) = parse_list(&input[1..]);
            (value, &rest[1..])
        } else if input.starts_with(']') {
            break;
        } else {
            let (value, rest) = input.split_at(
                input
                    .find(|c: char| !c.is_ascii_digit())
                    .unwrap_or(input.len()),
            );
            (List::Int(value.parse().unwrap()), rest)
        };
        list.push(value);
        input = rest.trim_start_matches(',');
    }
    (List::List(list), input)
}

pub fn generator(input: &str) -> Input {
    // onput is two lists separated by a newline, then a blank line (repeating)

    input
        .split("\n")
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|chunk| {
            let (list1, _) = parse_list(chunk[0]);
            let (list2, _) = parse_list(chunk[1]);
            (list1, list2)
        })
        .collect()
}

pub fn part_1(input: &Input) -> i64 {
    // sum of the indices of pairs that are in order
    let mut sum = 0;
    for (i, (a, b)) in input.iter().enumerate() {
        if a < b {
            sum += (i + 1) as i64;
            // debug: sample should have 1, 2, 4, and 6
            // println!("{:?} < {:?} at index {}", a, b, i + 1)
        }
    }
    sum
}

pub fn part_2(input: &Input) -> i64 {
    let marker1 = parse_list("[[2]]").0;
    let marker2 = parse_list("[[6]]").0;
    // first combine all the pairs to one list
    let mut list = vec![marker1.clone(), marker2.clone()];
    for (a, b) in input {
        list.push(a.clone());
        list.push(b.clone());
    }
    // sort the list
    list.sort_by(|a, b| a.partial_cmp(b).unwrap());
    // find the indices of [[2]] and [[6]]
    let mut prod = 1;
    for (i, item) in list.iter().enumerate() {
        println!("{:?} ", item);
        if item == &marker1 {
            prod *= (i + 1) as i64;
        }
        if item == &marker2 {
            prod *= (i + 1) as i64;
        }
    }
    prod
}
