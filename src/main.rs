#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![allow(clippy::use_debug)]

use advent_of_code_rust::util::ansi::*;
use advent_of_code_rust::util::parse::*;
use advent_of_code_rust::*;
use regex::Regex;
use std::env;
use std::env::args;
use std::fs::read_to_string;
use std::fs::File;
use std::io::{self, BufRead};
use std::iter::empty;
use std::path::{Path, PathBuf};
use std::process::exit;
use std::time::{Duration, Instant, SystemTime};

fn main() {
    env::set_var("RUST_BACKTRACE", "1");
    let year_day_pattern = Regex::new(r"year(\d{4})::day(\d{2})").unwrap();
    let year_pattern = Regex::new(r"year(\d{4})").unwrap();

    let (year, day) = args()
        .skip(1) // Skip the program name
        .find_map(|arg| {
            year_day_pattern
                .captures(&arg)
                .map(|caps| (Some(caps[1].to_string()), Some(caps[2].to_string())))
                .or_else(|| {
                    year_pattern
                        .captures(&arg)
                        .map(|caps| (Some(caps[1].to_string()), None))
                })
        })
        .unwrap_or((None, None));

    println!("Year: {year:?}");
    println!("Day: {day:?}");

    // Filter solutions
    let solutions = empty()
        .chain(year2020())
        .chain(year2024())
        .filter(|solution| match &year {
            Some(y) => y
                .parse::<u32>()
                .map_or(false, |y_parsed| y_parsed == solution.year),
            None => true,
        })
        .filter(|solution| match &day {
            Some(d) => d
                .parse::<u32>()
                .map_or(false, |d_parsed| d_parsed == solution.day),
            None => true,
        });

    // Pretty print output and timing for each solution
    let mut solved = 0;
    let mut duration = Duration::ZERO;

    for Solution {
        year,
        day,
        path,
        wrapper,
    } in solutions
    {
        if let Ok(data) = read_to_string(&path) {
            let instant = Instant::now();
            let (part1, part2) = wrapper(data);
            let elapsed = instant.elapsed();

            solved += 2;
            duration += elapsed;

            println!("{BOLD}{YELLOW}{year} Day {day:02}{RESET}");
            println!("    Part 1: {part1}");
            println!("    Part 2: {part2}");
            println!("    Elapsed: {} μs", elapsed.as_micros());
        } else {
            eprintln!("{BOLD}{RED}{year} Day {day:02}{RESET}");
            eprintln!("    Missing input!");
            eprintln!(
                "    Place input file in {BOLD}{WHITE}{}{RESET}",
                path.display()
            );
        }
    }

    // Optionally print totals
    if args().any(|a| a == "--totals") {
        println!("{BOLD}{YELLOW}⭐ {solved}{RESET}");
        println!("{BOLD}{WHITE}🕓 {} ms{RESET}", duration.as_millis());
    }
}

struct Solution {
    year: u32,
    day: u32,
    path: PathBuf,
    wrapper: fn(String) -> (String, String),
}

macro_rules! run {
    ($year:tt $($day:tt),*) => {
        fn $year() -> Vec<Solution> {
            vec![$({
                let year = stringify!($year);
                let day = stringify!($day);
                let path = Path::new("input").join(year).join(day).with_extension("txt");

                let wrapper = |data: String| {
                    use $year::$day::*;

                    let input = parse(&data);
                    let part1 = part1(&input);
                    let part2 = part2(&input);

                    (part1.to_string(), part2.to_string())
                };

                Solution { year: year.unsigned(), day: day.unsigned(), path, wrapper }
            },)*]
        }
    }
}

run!(year2020
    day01,day02
);

// run!(year2023
//     day01, day02, day03
// );

run!(year2024
    day01,day02,day03
);
