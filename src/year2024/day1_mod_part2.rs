//https://adventofcode.com/2024/day/1
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
use core::{fmt, hash};
use std::collections::{self, hash_map, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;
use std::path::{absolute, Path};
use std::{cell, vec};

pub fn historian_hysteria(file_path: String) {
    let mut sum: u32 = 0;
    let mut location_ids: Vec<u32> = Vec::new();
    let mut location_ids_compare: Vec<u32> = Vec::new();

    let mut lines = lines_from_file(file_path);
    for l in lines {
        let mut x = l.split(" ").collect::<Vec<&str>>();
        location_ids.push(x.first().unwrap().parse::<u32>().unwrap());
        location_ids_compare.push(x.last().unwrap().parse::<u32>().unwrap());
    }
    location_ids.sort();
    location_ids_compare.sort();

    sum = calculate_differences(location_ids, location_ids_compare);
    println!("Day 1 Part 1 Sum is {}", sum);
}

fn calculate_differences(location_ids: Vec<u32>, location_ids_compare: Vec<u32>) -> u32 {
    let mut difference_total: u32 = 0;
    for (l, c) in zip(location_ids, location_ids_compare.clone()) {
        difference_total +=
            l * u32::try_from(location_ids_compare.iter().filter(|&n| *n == l).count()).unwrap();
    }
    difference_total
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
