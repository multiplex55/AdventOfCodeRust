//https://adventofcode.com/2020/day/1
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use core::{fmt, hash, panic};
use std::collections::{self, hash_map, HashMap, HashSet};
use std::fs::{self, File};
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use std::{cell, vec};

pub fn report_repair_part1(file_path: String) {
    let mut sum: u32 = 0;
    let mut lines;

    let line_res = lines_from_file_to_vector(file_path);
    match line_res {
        Some(i) => lines = i,
        None => panic!("Something went wrong reading the input file"),
    }

    for i in 0..lines.len() {
        let first = lines[i].parse::<u32>().unwrap();
        for j in i..lines.len() - 1 {
            let second = lines[j].parse::<u32>().unwrap();
            if first + second == 2020 {
                sum += first * second;
            }
        }
    }
    println!("2020 Day 1 Part 1 Sum is {}", sum);
}

pub fn report_repair_part2(file_path: String) {
    let mut sum: u32 = 0;
    let mut lines;

    let line_res = lines_from_file_to_vector(file_path);
    match line_res {
        Some(i) => lines = i,
        None => panic!("Something went wrong reading the input file"),
    }

    for i in 0..lines.len() {
        let first = lines[i].parse::<u32>().unwrap();

        for j in i..lines.len() - 1 {
            let second = lines[j].parse::<u32>().unwrap();

            for k in j..lines.len() - 2 {
                let third = lines[k].parse::<u32>().unwrap();
                if first + second + third == 2020 {
                    sum += first * second * third;
                }
            }
        }
    }
    println!("2020 Day 1 Part 2 Sum is {}", sum);
}

fn lines_from_file_to_vector(file_path: String) -> Option<Vec<String>> {
    let file = File::open(file_path).expect("Can't read file");
    let buf = BufReader::new(file);
    Some(
        buf.lines()
            .map(|line| line.expect("Can't parse line somehow"))
            .collect(),
    )
}

// The output is wrapped in a Result to allow matching on errors.
// Returns an Iterator to the Reader of the lines of the file.
fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
