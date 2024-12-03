#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]

mod year2020;
mod year2023;
mod year2024;

use std::env;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::PathBuf;
use std::process::exit;
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
    println!();
}

fn main() {
    benchmark(
        "inputFiles\\2024\\day2.txt",
        year2024::day2_mod::red_nosed_reports,
    );
    benchmark(
        "inputFiles\\2024\\day2.txt",
        year2024::day2_mod_part2::red_nosed_reports,
    );
    exit(0);

    println!("Which function do you want to run");
    const AVAILABLE_FUNCTIONS_TO_RUN: [&str; 12] = [
        "2020 1 1 -> Day 1 2020 Part 1",
        "2020 1 2 -> Day 1 2020 Part 2",
        "2023 1 1 -> Day 1 2020 Part 1",
        "2023 1 2 -> Day 1 2023 Part 2",
        "2023 2 1 -> Day 2 2023 Part 1",
        "2023 2 2 -> Day 2 2023 Part 2",
        "2023 3 1 -> Day 3 2023 Part 1",
        "2023 3 2 -> Day 3 2023 Part 2",
        "2024 1 1 -> Day 1 2024 Part 1",
        "2024 1 2 -> Day 1 2024 Part 2",
        "2024 2 1 -> Day 2 2024 Part 1",
        "2024 2 2 -> Day 2 2024 Part 2",
    ];
    for avtr in AVAILABLE_FUNCTIONS_TO_RUN {
        println!("{}", avtr)
    }
    println!("------------\n\r\nEnter ID: ");

    let mut input = String::new();
    io::stdin()
        .read_line(&mut input)
        .expect("Error: unable to read input");

    launch_aoc_function(input);
}

fn launch_aoc_function(input: String) {
    match input.as_str().strip_suffix("\r\n").unwrap() {
        "2020 1 1" => {
            benchmark(
                "inputFiles\\2020\\day1.txt",
                year2020::day1_mod::report_repair_part1,
            );
        }
        "2020 1 2" => {
            benchmark(
                "inputFiles\\2020\\day1.txt",
                year2020::day1_mod::report_repair_part2,
            );
        }
        "2023 1 1" => {
            benchmark(
                "inputFiles\\2023\\day1.txt",
                year2023::day1_mod::trebuchet_part1,
            );
        }
        "2023 1 2" => {
            benchmark(
                "inputFiles\\2023\\day1.txt",
                year2023::day1_mod::trebuchet_part2,
            );
        }
        "2023 2 1" => {
            benchmark(
                "inputFiles\\2023\\day2.txt",
                year2023::day2_mod::cube_conundrum_part1,
            );
        }
        "2023 2 2" => {
            benchmark(
                "inputFiles\\2023\\day2.txt",
                year2023::day2_mod::cube_conundrum_part2,
            );
        }
        "2023 3 1" => {
            benchmark(
                "inputFiles\\day3.txt",
                year2023::day3_mod_part1::gear_ratio_part1,
            );
        }
        "2023 3 2" => {
            benchmark(
                "inputFiles\\2023\\day3.txt",
                year2023::day3_mod_part2::gear_ratio_part2,
            );
        }
        "2024 1 1" => {
            benchmark(
                "inputFiles\\2024\\day1.txt",
                year2024::day1_mod::historian_hysteria,
            );
        }
        "2024 1 2" => {
            benchmark(
                "inputFiles\\2024\\day1.txt",
                year2024::day1_mod_part2::historian_hysteria,
            );
        }
        "2024 2 1" => {
            benchmark(
                "inputFiles\\2024\\day2.txt",
                year2024::day2_mod::red_nosed_reports,
            );
        }
        "2024 2 2" => {
            benchmark(
                "inputFiles\\2024\\day2.txt",
                year2024::day2_mod::red_nosed_reports,
            );
        }
        _ => print!("Did not match anything"),
    }
}
