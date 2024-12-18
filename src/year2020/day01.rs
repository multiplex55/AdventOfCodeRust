//https://adventofcode.com/2020/day/1
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::util::parse::*;

pub fn part1(input: &[usize]) -> usize {
    let mut sum: usize = 0;

    for i in 0..input.len() {
        let first = input[i];
        for j in input.iter().take(input.len() - 1).skip(i) {
            let second = *j;
            if first + second == 2020 {
                sum += first * second;
            }
        }
    }
    sum
}

pub fn part2(input: &[usize]) -> usize {
    let mut sum: usize = 0;

    for i in 0..input.len() {
        let first = input[i];
        for j in i..input.len() - 1 {
            let second = input[j];
            for k in input.iter().take(input.len() - 2).skip(j) {
                let third = *k;
                if first + second + third == 2020 {
                    sum += first * second * third;
                }
            }
        }
    }
    sum
}

pub fn parse(input: &str) -> Vec<usize> {
    input.lines().map(|line| line.unsigned()).collect()
}
