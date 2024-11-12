//https://adventofcode.com/2023/day/1
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
    println!("Sum is {}", sum);
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
