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
                let num_results = x
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .parse::<u32>();
                let num = match num_results {
                    Ok(nr) => nr,
                    Err(e) => {
                        println!("Parse Error for Num_results Raw Value: {} {}", x, e);
                        0
                    }
                };

                let color_res = x.split(" ").collect::<Vec<&str>>();
                //have to do this cause value is freed while still in use
                let color_res = color_res.last();

                let color = match color_res {
                    Some(cr) => cr, // `cr` is already of type `&str`
                    None => {
                        println!("Parse Error for color_res Raw Value: {}", x);
                        ""
                    }
                };

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

pub fn cube_conundrum_part2(file_path: String) {
    let mut sum: u32 = 0;
    let line = "";

    const RED_LIMIT: u32 = 12;
    const GREEN_LIMIT: u32 = 13;
    const BLUE_LIMIT: u32 = 14;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut min_green = 0;
            let mut min_blue = 0;
            let mut min_red = 0;

            let split_line: Vec<&str> = line.split(":").collect();

            for x in split_line[1].trim().replace(";", ",").split(", ") {
                let num_results = x
                    .split(" ")
                    .collect::<Vec<&str>>()
                    .first()
                    .unwrap()
                    .parse::<u32>();
                let num = match num_results {
                    Ok(nr) => nr,
                    Err(e) => {
                        println!("Parse Error for Num_results Raw Value: {} {}", x, e);
                        0
                    }
                };

                let color_res = x.split(" ").collect::<Vec<&str>>();
                //have to do this cause value is freed while still in use
                let color_res = color_res.last();

                let color = match color_res {
                    Some(cr) => cr, // `cr` is already of type `&str`
                    None => {
                        println!("Parse Error for color_res Raw Value: {}", x);
                        ""
                    }
                };

                match color {
                    "green" => {
                        if min_green < num {
                            min_green = num
                        }
                    }
                    "red" => {
                        if min_red < num {
                            min_red = num
                        }
                    }
                    "blue" => {
                        if min_blue < num {
                            min_blue = num
                        }
                    }
                    _ => println!("Unknown color: {}", color),
                }
            }
            sum += min_red * min_blue * min_green
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
