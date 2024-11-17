#![allow(unused_imports)]
#![allow(dead_code)]
mod day1_mod;
mod day2_mod;
mod day3_mod_part1;
mod day3_mod_part2;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::time::{Duration, Instant, SystemTime};

// Function to benchmark a specific function with an input file path
fn benchmark<F>(input_file: &str, func: F)
where
    F: Fn(String),
{
    // Get the current directory as a PathBuf
    let mut path = env::current_dir().expect("Failed to get current directory");
    path.push(input_file);

    // Check if the file exists
    if path.exists() {
        // Convert the PathBuf to a String
        let path_string = path
            .to_str()
            .expect("Failed to convert path to string")
            .to_string();

        // Print start time
        let start_time = SystemTime::now();
        println!("Start time: {:?}", start_time);

        // Start the performance timer
        let start = Instant::now();

        // Call the provided function with the input file path
        func(path_string);

        // End the performance timer
        let duration = start.elapsed();

        // Print end time
        let end_time = SystemTime::now();
        println!("End time: {:?}", end_time);

        // Format the duration into minutes, seconds, milliseconds, and microseconds
        let total_seconds = duration.as_secs();
        let minutes = total_seconds / 60;
        let seconds = total_seconds % 60;
        let milliseconds = duration.subsec_millis();
        let microseconds = duration.subsec_micros() % 1_000;

        // Print the formatted duration
        println!(
            "Duration: {} minutes, {} seconds, {} milliseconds, {} microseconds",
            minutes, seconds, milliseconds, microseconds
        );
    } else {
        // Print an error message if the file does not exist
        eprintln!("Error: The file '{}' could not be found.", input_file);
    }
}

fn main() {
    // Example usage for day 1
    // benchmark("inputFiles\\day1.txt", day1_mod::trebuchet_part1);
    // benchmark("inputFiles\\day1.txt", day1_mod::trebuchet_part2);
    // benchmark("inputFiles\\day2.txt", day2_mod::cube_conundrum_part1);
    // benchmark("inputFiles\\day2.txt", day2_mod::cube_conundrum_part2);

    //benchmark("inputFiles\\day3.txt", day3_mod_part1::gear_ratio_part1);
    benchmark("inputFiles\\day3.txt", day3_mod_part2::gear_ratio_part2);
}
