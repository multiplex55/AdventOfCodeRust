//https://adventofcode.com/2024/day/2
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

pub fn red_nosed_reports(file_path: String) {
    let mut input_vec: Vec<String> = lines_from_file(file_path);
    let mut are_reports_safe: Vec<bool> = evaluate_reports(input_vec);
    let answer = are_reports_safe.into_iter().filter(|b| *b).count();

    println!("Day 2 Part 1 answer is {}", answer);
}

fn evaluate_reports(report_vec: Vec<String>) -> Vec<bool> {
    let mut ret_vec: Vec<bool> = vec![];

    for l in report_vec {
        let mut report_as_int: Vec<i32> = l
            .split(" ")
            .collect::<Vec<&str>>()
            .iter()
            .map(|&n| n.parse::<i32>().unwrap())
            .collect();
        let mut report_evaluation = evaluate_report(report_as_int);
        if report_evaluation {
            ret_vec.push(true)
        } else {
            ret_vec.push(false)
        }
    }
    ret_vec
}

fn evaluate_report(report_seq: Vec<i32>) -> bool {
    let diff_amount_min = 1;
    let diff_amount_max = 3;
    let sorted_ascending = match report_seq.as_slice() {
        seq if seq.windows(2).all(|w| w[0] <= w[1]) => true, //ascending
        seq if seq.windows(2).all(|w| w[0] >= w[1]) => true, //descending
        _ => false,
    };

    let mut differences: Vec<i32> = vec![];
    for (i, ri) in report_seq.iter().enumerate() {
        if i + 1 < report_seq.len() {
            differences.push((ri - report_seq[i + 1]).abs());
        }
    }

    let ret_status = differences.iter().max().unwrap() <= &diff_amount_max
        && differences.iter().min().unwrap() >= &diff_amount_min
        && sorted_ascending;

    ret_status
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
