mod day11;
mod day12;

aoc_main::main! {
    year 2022;
    day11 : generator => part_1, part_2, part_2_rayon;
    day12 : generator => part_1, part_1_dijkstras, part_2;
}
