use advent_of_code_rust::year2021::day20::*;

const EXAMPLE: &str = "\
..#.#..#####.#.#.#.###.##.....###.##.#..###.####..#####..#....#.\
.#..##..###..######.###...####..#..#####..##..#.#####...##.#.#..\
#.##..#.#......#.###.######.###.####...#.##.##..#..#..#####.....\
#.#....###..#.##......#.....#..#..#..##..#...##.######.####.####\
.#.#...#.......#..#.#.#...####.##.#......#..#...##.#.##..#...##.\
#.##..###.#......#.#.......#.#.#.####.###.##...#.....####.#..#..\
#.##.#....##..#.####....##...##..#...#......#.#.......#.......##\
..####..#...#.#.#...##..#.#..###..#####........#..####......#..#

#..#.
#....
##..#
..#..
..###";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), 35);
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), 3351);
}
