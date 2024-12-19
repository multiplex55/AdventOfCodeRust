//https://adventofcode.com/2024/day/1
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

pub fn parse(input: &str) -> (Vec<usize>, Vec<usize>) {
    let (left, right): (Vec<_>, Vec<_>) = input
        .lines()
        .filter_map(|line| {
            let mut parts = line.split_whitespace();
            match (parts.next(), parts.next()) {
                (Some(l), Some(r)) => {
                    if let (Ok(l_num), Ok(r_num)) = (l.parse::<usize>(), r.parse::<usize>()) {
                        Some((l_num, r_num))
                    } else {
                        None
                    }
                }
                _ => None,
            }
        })
        .unzip();

    (left, right)
}

pub fn part1(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (mut left, mut right) = input.clone();

    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(l, r)| calculate_distance(*l, *r))
        .sum()
}

fn calculate_distance(l: usize, r: usize) -> usize {
    usize::abs_diff(l, r)
}

pub fn part2(input: &(Vec<usize>, Vec<usize>)) -> usize {
    let (mut left, mut right) = input.clone();
    left.sort_unstable();
    right.sort_unstable();

    //Bad way of doing it 6094 ish microseconds to 196 microseconds
    // let mut sum: usize = left
    //     .into_iter()
    //     .map(|l| l * right.clone().into_iter().filter(|x| *x == l).count())
    //     .sum();

    left.iter()
        .map(|l| l * right.iter().filter(|&&x| x == *l).count())
        .sum()
}
