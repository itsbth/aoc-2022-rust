use std::ops::RangeInclusive;

pub type Point = (i64, i64);
// position of the sensor, position of the closest beacon
pub struct Sensor(Point, Point);

// "Sensor at x=2, y=18: closest beacon is at x=-2, y=15"
pub fn generator(input: &str) -> Vec<Sensor> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!(lines("Sensor at x=" (x: i64) ", y=" (y: i64) ": closest beacon is at x=" (x2: i64) ", y=" (y2: i64) => Sensor((x, y), (x2, y2))));
    parser.parse(input).unwrap()
}

// nb: manhattan distance
struct Circle(Point, i64); // position of the beacon, area (nb: manhattan distance)

// return the range of x values that are in the circle
fn range_in(y: i64, circle: &Circle) -> RangeInclusive<i64> {
    let Circle((cx, cy), radius) = circle;
    let ydiff = (y - cy).abs();
    // ydiff + x = cx - radius
    let x1 = cx - radius - ydiff;
    let x2 = cx + radius - ydiff;
    x1..=x2
}

fn overlapping_intervals(intervals: &[RangeInclusive<i64>]) -> Vec<RangeInclusive<i64>> {
    let mut intervals = intervals.to_vec();
    intervals.sort_by_key(|r| *r.start());
    // visualize them as lines
    let minx = intervals.iter().map(|r| *r.start()).min().unwrap();
    for it in intervals.iter() {
        let start = it.start() - minx;
        let end = it.end() - minx;
        println!(
            "{} {}",
            ".".repeat(start as usize),
            "#".repeat((end - start) as usize)
        );
    }
    let mut result = vec![];
    let mut current = intervals[0].clone();
    for interval in intervals[1..].iter() {
        if current.start() > current.end() {
            panic!("current interval is empty: {:?}", current)
        }
        if interval.start() <= current.end() {
            let start = current.start();
            let end = std::cmp::max(current.end(), interval.end());
            current = *start..=*end;
        } else {
            result.push(current);
            current = interval.clone();
        }
    }
    result.push(current);
    for it in result.iter() {
        let start = it.start() - minx;
        let end = it.end() - minx;
        println!(
            "{} {}",
            ".".repeat(start as usize),
            "X".repeat((end - start) as usize)
        );
    }
    result
}

pub fn part_1(input: &[Sensor]) -> i64 {
    // let y = 20_000;
    // sample is y = 10, real input is y = 20_000. Try to guess which one we have based on the input length
    let y = if input.len() == 14 { 10 } else { 2_000_000 };
    // let's convert (pos, pos) to (pos, radius)
    let circles = input.iter().map(|Sensor(pos, closest)| {
        let radius =
            (pos.0 as i64 - closest.0 as i64).abs() + (pos.1 as i64 - closest.1 as i64).abs();
        Circle(*pos, radius)
    });
    // let's try printing the whole map from -5..25
    // for y in -5..25 {
    //     print!("{:2} ", y);
    //     let ranges =
    //         overlapping_intervals(&circles.clone().map(|c| range_in(y, &c)).collect::<Vec<_>>());
    //     for x in -5..25 {
    //         // let c = circles.clone().find(|c| range_in(y, c).contains(&x));
    //         let c = ranges.iter().find(|r| r.contains(&x));
    //         print!(
    //             "{}",
    //             match c {
    //                 Some(_) => '#',
    //                 None => '.',
    //             }
    //         );
    //     }
    //     println!();
    // }
    let ranges = overlapping_intervals(&circles.map(|c| range_in(y, &c)).collect::<Vec<_>>());
    ranges.iter().map(|r| r.end() - r.start()).sum::<i64>()
        - (input.iter().filter(|Sensor(_, pos)| pos.1 == y).count() as i64)
}
