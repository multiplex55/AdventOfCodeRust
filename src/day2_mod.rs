//https://adventofcode.com/2023/day/1
#![allow(unused_variables)]
#![allow(dead_code)]
use core::hash;
use std::collections::{self, hash_map, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::vec;

pub fn cube_conundrum_part1(file_path: String) {
    let mut sum: u32 = 0;
    let line = "";

    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let split_line: Vec<&str> = line.split(":").collect();
            let game_num = &split_line[0].split(" ").last();
            let mut valid_game = true;

            for x in split_line[1].trim().replace(";", ",").split(", ") {
                let num = x
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .parse::<u32>()
                    .unwrap();

                let color = x.split(" ").collect::<Vec<&str>>().last().unwrap() as &str;

                match color {
                    "green" => {
                        if num > GREEN_LIMIT {
                            valid_game = false;
                        }
                    }
                    "red" => {
                        if num > RED_LIMIT {
                            valid_game = false;
                        }
                    }
                    "blue" => {
                        if num > BLUE_LIMIT {
                            valid_game = false;
                        }
                    }
                    _ => println!("Unknown color: {}", color),
                }
            }
            if valid_game {
                let current_num = game_num.unwrap().parse::<u32>().unwrap();
                sum += game_num.unwrap().parse::<u32>().unwrap();
            }
        }
    }
    println!("Day 2 Part 1 Sum is {}", sum);
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
