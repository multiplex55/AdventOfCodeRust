//https://adventofcode.com/2024/day/3
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
use core::{fmt, hash};
use regex::Regex;
use std::collections::{self, hash_map, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::iter::zip;
use std::path::{absolute, Path};
use std::{cell, vec};

pub fn mull_it_over(file_path: String) {
    let mut input_vec: Vec<String> = lines_from_file(file_path);
    let mut captures: Vec<String> = vec![];

    let mut answer: i32 = 0;
    for iseq in input_vec {
        let re = Regex::new(r"(mul\([0-9]+,[0-9]+\))|(don't)|(do)");

        let mut results = vec![];
        for (_, [line]) in re
            .expect("Something went wrong")
            .captures_iter(&iseq)
            .map(|c| c.extract())
        {
            results.push(line);
        }
        let mut enable_instructions: bool = true;
        for r in results {
            match r {
                "don't" => {
                    enable_instructions = false;
                }
                "do" => {
                    enable_instructions = true;
                }
                _ => {
                    if enable_instructions {
                        let working_string = r.replace("mul(", "").replace(")", "");
                        let split_string = working_string.split(",").collect::<Vec<&str>>();
                        answer += split_string.first().unwrap().parse::<i32>().unwrap()
                            * split_string.last().unwrap().parse::<i32>().unwrap();
                    }
                }
            }
        }
        // for r in results {
        //     let working_string = r.replace("mul(", "").replace(")", "");
        //     let split_string = working_string.split(",").collect::<Vec<&str>>();
        //     answer += split_string.first().unwrap().parse::<i32>().unwrap()
        //         * split_string.last().unwrap().parse::<i32>().unwrap();
        // }
    }

    println!("Day 3 Part 2 answer is {}", answer);
}

fn lines_from_file(filename: impl AsRef<Path>) -> Vec<String> {
    let file = File::open(filename).expect("no such file");
    let buf = BufReader::new(file);
    buf.lines()
        .map(|l| l.expect("Could not parse line"))
        .collect()
}
