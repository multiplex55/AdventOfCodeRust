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
    let sum: u32 = 0;
    let line = "";
    let game_index = 0;

    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 12;
    const BLUE_LIMIT: u32 = 12;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let split_line: Vec<&str> = line.split(":").collect();
            let game_num = &split_line[0].split(" ").last();
            let valid_game = true;
            println!("DEBUG: game num is: {}", game_num.unwrap());

            for x in split_line[1].trim().replace(";", ",").split(", ") {
                let num = x
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .chars()
                    .next()
                    .unwrap()
                    .to_digit(10)
                    .unwrap();
                let color = x.split(" ").collect::<Vec<&str>>().last().unwrap() as &str;

                println!("Num is {}", num);
                println!("Color is {}", color);
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
