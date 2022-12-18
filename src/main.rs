mod day11;
mod day12;
mod day13;
mod day14;
mod day15;
mod day16;
mod day17;
mod day18;

aoc_main::main! {
    year 2022;
    day11 : generator => part_1, part_2, part_2_rayon;
    day12 : generator => part_1, part_1_dijkstras, part_2;
    day13 : generator => part_1, part_2;
    day14 : generator => part_1, part_1_backtracking, part_2, part_2_backtracking;
    day15 : generator => part_1;
    day16 : generator => part_1, part_1_permutations, part_2;
    day17 : generator => part_1;
    day18 : generator => part_1, part_2;
}
