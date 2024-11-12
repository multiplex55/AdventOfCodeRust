#![allow(unused_imports)]
mod day1_mod;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::os;
use std::path::Path;
use std::time::{Duration, Instant, SystemTime};

fn main() {
    // Get the current directory as a PathBuf
    let mut path = env::current_dir().expect("Failed to get current directory");

    // Append the relative path to the existing path
    path.push("inputFiles\\day1.txt");

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

        // Call the function with the String
        day1_mod::trebuchet_part1(path_string);

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
        eprintln!("Error: The file 'inputFiles\\day1.txt' could not be found.");
    }
}
