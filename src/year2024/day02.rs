//https://adventofcode.com/2024/day/2
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use std::thread::current;

pub fn parse(input: &str) -> Vec<Vec<usize>> {
    let x: Vec<Vec<usize>> = input
        .lines()
        .map(|x| {
            x.split(' ')
                .map(|n| n.parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect();

    x
}
fn is_safe(current_input: &[usize]) -> bool {
    let sorted_status = current_input.is_sorted();
    let sorted_status_descending = current_input
        .iter()
        .as_slice()
        .windows(2)
        // .inspect(|w| println!("{:?}", w))
        .all(|w| w[0] >= w[1]);

    let adjacent_differences = current_input
        .iter()
        .as_slice()
        .windows(2)
        .map(|w| w[0].abs_diff(w[1]))
        .collect::<Vec<usize>>();

    (sorted_status || sorted_status_descending)
        && *adjacent_differences
            .iter()
            .min()
            .unwrap()
            >= 1
        && *adjacent_differences
            .iter()
            .max()
            .unwrap()
            <= 3
}

pub fn part1(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .map(|x| is_safe(x))
        .filter(|y| *y)
        .count()
}

pub fn part2(input: &[Vec<usize>]) -> usize {
    input
        .iter()
        .filter(|x| {
            (0..x.len()).any(|i| {
                let mut subarray = (*x).clone();
                subarray.remove(i);
                is_safe(&subarray)
            })
        })
        .count()
}
