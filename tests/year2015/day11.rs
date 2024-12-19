use advent_of_code_rust::year2015::day11::*;

const EXAMPLE: &str = "ghijklmn";

#[test]
fn part1_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part1(&input), "ghjaabcc");
}

#[test]
fn part2_test() {
    let input = parse(EXAMPLE);
    assert_eq!(part2(&input), "ghjbbcdd");
}
