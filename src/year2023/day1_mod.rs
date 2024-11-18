//https://adventofcode.com/2023/day/1

#![allow(unused_variables)]
#![allow(dead_code)]
use core::hash;
use std::collections::{self, hash_map, HashMap};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

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

/// Searches for a key within a provided string and returns the associated value if found.
///
/// This function iterates over a `HashMap` of string keys and numeric values. If any key is found
/// within the `string_to_find`, it returns the associated value. If no key is found, the function
/// returns `0` as a default.
///
/// # Parameters
/// - `digit_table`: A reference to a `HashMap` where keys are `&str` and values are `u32`.
///   The keys represent words (e.g., "one", "two") that map to numeric values.
/// - `string_to_find`: A `&str` representing the input string to search for occurrences of the keys.
///
/// # Returns
/// - `u32`: The value associated with the first key found in `string_to_find`, or `0` if no match is found.
///
/// # Behavior
/// - The function stops at the first match and returns the corresponding value.
/// - Returns `0` if no keys from `digit_table` are present in `string_to_find`.

fn table_contains_key_parse_int(digit_table: &HashMap<&str, u32>, string_to_find: &str) -> u32 {
    // Iterate through the entries of the digit_table to find a match
    for (key, val) in digit_table.iter() {
        // Check if the string_to_find contains the current key
        if string_to_find.contains(key) {
            // Return the associated value if a match is found
            return *val;
        }
    }

    // Return 0 if no match is found
    0
}

/// Processes the contents of a file and calculates a sum based on specific rules.
///
/// This function reads a file line by line, extracting numeric values represented
/// as both digits and words. It sums these values based on specific logic, which involves
/// finding the first and last numeric value in each line, concatenating them, and adding
/// them to a running total.
///
/// # Parameters
/// - `file_path`: A `String` representing the path to the input file.
///
/// # Behavior
/// - Reads the file line by line, parsing characters.
/// - Builds a potential numeric value from alphanumeric words or digits.
/// - Uses a `HashMap` to translate word representations of numbers into actual digits.
/// - If no "last" digit is found, duplicates the first digit in the final sum.
///
/// # Panics
/// This function will panic if the `parse::<u32>()` call fails, which can occur if the parsed
/// string is not a valid number.
///
/// # Example Usage
/// ```
/// trebuchet_part2("input.txt".to_string());
/// ```

pub fn trebuchet_part2(file_path: String) {
    let mut sum: u32 = 0;
    // Initializing a HashMap for number word conversion
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

    // Read lines from the specified file path
    if let Ok(lines) = read_lines(file_path) {
        // Iterate over each line, handling only valid results
        for line in lines.map_while(Result::ok) {
            let mut first: u32 = 0;
            let mut last: u32 = 0;
            let mut found_first = false;
            let mut found_last = false;
            let mut potential_string_number = String::new();
            let mut current_digit: u32;
            let mut backup_letter = '0'; // Last character used for backup processing

            // Iterate through each character in the line
            for x in line.chars() {
                if !x.is_numeric() {
                    potential_string_number.push(x);
                    current_digit =
                        table_contains_key_parse_int(&digit_table, &potential_string_number);

                    if current_digit == 0 {
                        // avoid cloning with an updated helper function signature
                        current_digit = table_contains_key_parse_int(
                            &digit_table,
                            format!("{}{}", backup_letter, &potential_string_number).as_str(),
                        );
                    } else {
                        // Update the backup letter with the last character of the current string
                        backup_letter = potential_string_number.chars().last().unwrap();
                        potential_string_number.clear();
                    }
                } else {
                    // Safely handle digit parsing
                    current_digit = x.to_digit(10).expect("Character is not a valid digit");
                    potential_string_number.clear();
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

            // If the last digit was not found, duplicate the first digit for summation
            if found_last {
                sum += (first.to_string() + &last.to_string())
                    .parse::<u32>()
                    .expect("Failed to parse concatenated string to u32");
            } else {
                sum += (first.to_string() + &first.to_string())
                    .parse::<u32>()
                    .expect("Failed to parse concatenated string to u32");
            }
        }
    }
    // Print the final sum
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
