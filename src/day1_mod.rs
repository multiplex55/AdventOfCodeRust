//https://adventofcode.com/2023/day/1
#![allow(unused_variables)]
#![allow(dead_code)]
use core::hash;
use std::collections::{hash_map, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::thread::current;

pub fn trebuchet_part1(file_path: String) {
    let mut sum: u32 = 0;

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut first = 0;
            let mut last = 0;
            let mut found_first = false;
            let mut found_last = false;

            let current_line = line;
            for x in current_line.chars() {
                if x.is_numeric() {
                    if !found_first {
                        first = x.to_digit(10).unwrap();
                        found_first = true;
                    } else {
                        last = x.to_digit(10).unwrap();
                        found_last = true;
                    }
                }
            }

            if found_last {
                sum += (first.to_string() + &last.to_string())
                    .parse::<u32>()
                    .unwrap();
            } else {
                sum += (first.to_string() + &first.to_string())
                    .parse::<u32>()
                    .unwrap();
            }
        }
    }
    println!("Day 1 Part 1 Sum is {}", sum);
}

fn table_contains_key_parse_int(digit_table: HashMap<&str, u32>, string_to_find: &str) -> u32 {
    for (key, val) in digit_table.iter() {
        if string_to_find.contains(key) {
            return *val;
        }
    }
    0
}

///  
///
/// # Panics
///
/// Panics if .
pub fn trebuchet_part2(file_path: String) {
    let mut sum: u32 = 0;
    let digit_table = HashMap::from([
        ("one", 1),
        ("two", 2),
        ("three", 3),
        ("four", 4),
        ("five", 5),
        ("six", 6),
        ("seven", 7),
        ("eight", 8),
        ("nine", 9),
    ]);

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut first: u32 = 0;
            let mut last: u32 = 0;
            let mut found_first = false;
            let mut found_last = false;
            let mut potential_string_number = String::new();
            let mut current_digit: u32;
            let mut backup_letter = '0';

            let current_line = line;
            for x in current_line.chars() {
                if !x.is_numeric() {
                    potential_string_number.push(x);
                    current_digit =
                        table_contains_key_parse_int(digit_table.clone(), &potential_string_number);

                    if current_digit == 0 {
                        current_digit = table_contains_key_parse_int(
                            digit_table.clone(),
                            format!("{}{}", backup_letter, &potential_string_number).as_str(),
                        );
                    } else {
                        backup_letter = potential_string_number.chars().last().unwrap();
                        potential_string_number = "".to_string();
                    }
                } else {
                    current_digit = x.to_digit(10).unwrap();
                    potential_string_number = "".to_string();
                }

                if current_digit != 0 {
                    if !found_first {
                        first = current_digit;
                        found_first = true;
                    } else {
                        last = current_digit;
                        found_last = true;
                    }
                }
            }

            if found_last {
                sum += (first.to_string() + &last.to_string())
                    .parse::<u32>()
                    .unwrap();
            } else {
                sum += (first.to_string() + &first.to_string())
                    .parse::<u32>()
                    .unwrap();
            }
        }
    }
    println!("Day 1 Part 2 Sum is {}", sum);
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
