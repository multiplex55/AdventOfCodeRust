//https://adventofcode.com/2024/day/3
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

pub fn parse(input: &str) -> Vec<String> {
    input
        .lines()
        .map(String::from)
        .collect()
}

pub fn part1(input: &Vec<String>) -> usize {
    for i in input {
        dbg!(i);
    }
    123
}

pub fn part2(input: &Vec<String>) -> usize {
    for i in input {
        dbg!(i);
    }

    456
}
