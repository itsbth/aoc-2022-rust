pub enum Push {
    Left,
    Right,
}

pub fn generator(input: &str) -> Vec<Push> {
    use aoc_parse::{parser, prelude::*};
    let parser = parser!({
        ("<" => Push::Left),
        (">" => Push::Right)
    }+);
    parser.parse(input).unwrap()
}

/*


####

.#.
###
.#.

..#
..#
###

#
#
#
#

##
##
*/

const SHAPES: [[[bool; 4]; 4]; 5] = [
    [
        [false, false, false, false],
        [false, false, false, false],
        [false, false, false, false],
        [true, true, true, true],
    ],
    [
        [false, false, false, false],
        [false, true, false, false],
        [true, true, true, false],
        [false, true, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, true, false],
        [false, false, true, false],
        [true, true, true, false],
    ],
    [
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
        [true, false, false, false],
    ],
    [
        [false, false, false, false],
        [false, false, false, false],
        [true, true, false, false],
        [true, true, false, false],
    ],
];
pub fn part_1(input: &[Push]) -> usize {
    // calculate length, bottom shape, top shape
    let shape_data = SHAPES
        .iter()
        .map(|&shape| {
            // length is max(x) (min x is always 0)
            let length = shape
                .iter()
                .map(|row| row.iter().position(|&x| x).unwrap_or(4))
                .max()
                .unwrap();
            // if the shape is resting on the floor, bottom shape is how high above the floor each column starts
            let bottom_shape = (0..4)
                .map(|x| (0..4).rev().find(|&y| shape[y][x]).unwrap_or(0))
                .collect::<Vec<_>>();
            // top_shape is the highest point of each column
            let top_shape = shape
                .iter()
                .map(|row| row.iter().position(|&x| x).unwrap_or(0))
                .collect::<Vec<_>>();
            (length, bottom_shape, top_shape)
        })
        .collect::<Vec<_>>();
    let mut floor = [0; 7];
    let mut shape_idx = 0;
    let mut push_idx = 0;
    for _ in 0..2022 {
        let shape = &SHAPES[shape_idx % SHAPES.len()];
        let (length, bottom_shape, top_shape) = &shape_data[shape_idx % SHAPES.len()];
        let mut x = 2;
        let mut y = floor.iter().max().unwrap() + 3;
        loop {
            let push = &input[push_idx % input.len()];
            let dir = match push {
                Push::Left => -1_isize,
                Push::Right => 1_isize,
            };
            x += dir;
            // clamp x to make the shape fit
            x = x.max(0).min(7 - *length as isize);
            // check if it rests on the floor
            let resting = (x..(x + *length as isize))
                .zip(bottom_shape.iter())
                .all(|(x, &sy)| floor[x as usize] >= y - sy as isize);
            if resting {
                // increase the floor height
                for (x, &sy) in (x..(x + *length as isize)).zip(top_shape.iter()) {
                    floor[x as usize] = y + sy as isize;
                }
                shape_idx += 1;
                break;
            } else {
                y -= 1;
            }
            push_idx += 1;
        }
        shape_idx += 1;
    }
    // return value is the height of the floor
    floor
        .iter()
        .max()
        .and_then(|v| {
            let res: Option<usize> = (*v).try_into().ok();
            res
        })
        .unwrap()
        + 1
}
