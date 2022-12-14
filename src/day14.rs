type Grid = Vec<Vec<bool>>;
type Point = (usize, usize);

const GRID_SIZE: usize = 1024; // TODO: make this dynamic?

// input is a list of lines in the format x,y -> x,y [-> x,y]*
pub fn generator(input: &str) -> Grid {
    use aoc_parse::{parser, prelude::*};
    let point = parser!((x: usize) ',' * (y: usize) => (x, y));
    let parser = parser!(lines(repeat_sep(point, " -> ")));
    let data = parser.parse(input).unwrap();
    let mut grid = vec![vec![false; GRID_SIZE]; GRID_SIZE];
    for line in data {
        for ((sx, sy), (ex, ey)) in line.windows(2).map(|w| (w[0], w[1])) {
            assert!(
                sx == ex || sy == ey,
                "diagonal line: {:?} -> {:?}",
                (sx, sy),
                (ex, ey)
            );
            let (sx, ex) = if sx < ex { (sx, ex) } else { (ex, sx) };
            let (sy, ey) = if sy < ey { (sy, ey) } else { (ey, sy) };
            for x in sx..=ex {
                for y in sy..=ey {
                    grid[x][y] = true;
                }
            }
        }
    }
    grid
}

fn print_grid(grid: &Grid) {
    // find the min and max x and y
    let mut min_x = GRID_SIZE;
    let mut max_x = 0;
    let mut min_y = GRID_SIZE;
    let mut max_y = 0;
    for x in 0..GRID_SIZE {
        for y in 0..GRID_SIZE {
            if grid[x][y] {
                min_x = min_x.min(x);
                max_x = max_x.max(x);
                min_y = min_y.min(y);
                max_y = max_y.max(y);
            }
        }
    }
    for y in min_y..=max_y {
        for x in min_x..=max_x {
            if grid[x][y] {
                print!("#");
            } else {
                print!(".");
            }
        }
        println!();
    }
}

pub fn part_1(input: &Grid) -> i64 {
    let mut grid = input.clone();
    let mut count = 0;
    'outer: loop {
        // print_grid(&grid);
        // println!("Simulating sand ({})", count);
        let mut x = 500;
        let mut y = 0;
        loop {
            if y >= GRID_SIZE - 1 {
                break 'outer;
            }
            // try to move down, down left, down right
            if !grid[x][y + 1] {
                y += 1;
            } else if !grid[x - 1][y + 1] {
                x -= 1;
                y += 1;
            } else if !grid[x + 1][y + 1] {
                x += 1;
                y += 1;
            } else {
                grid[x][y] = true;
                count += 1;
                break;
            }
        }
    }
    count
}

// part 2: there's a floor at max_y + 2. Run the simulation until the start point (500, 0) is blocked
pub fn part_2(input: &Grid) -> i64 {
    let mut grid = input.clone();
    let mut count = 0;
    let floor = grid
        .iter()
        .map(|col| {
            col.iter()
                .enumerate()
                .flat_map(|(idx, solid)| if *solid { Some(idx) } else { None })
                .max()
                .unwrap_or(0)
        })
        .max()
        .unwrap()
        + 2;
    'outer: loop {
        // print_grid(&grid);
        // println!("Simulating sand ({})", count);
        let mut x = 500;
        let mut y = 0;
        loop {
            if y == floor - 1 {
                grid[x][y] = true;
                count += 1;
                break;
            }
            // try to move down, down left, down right
            if !grid[x][y + 1] {
                y += 1;
            } else if !grid[x - 1][y + 1] {
                x -= 1;
                y += 1;
            } else if !grid[x + 1][y + 1] {
                x += 1;
                y += 1;
            } else {
                grid[x][y] = true;
                count += 1;
                if x == 500 && y == 0 {
                    break 'outer;
                }
                break;
            }
        }
    }
    count
}
