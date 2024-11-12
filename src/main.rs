use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

fn main() {
    println!("Hello, world!");
    do_something();
}

fn do_something() {
    let file_path = "InputFiles/day1_small.txt";
    if let Ok(lines) = read_lines(file_path) {
        for line in lines.flatten() {
            println!("{}", line);
            let current_line = line;
            for x in current_line.chars() {
                if (x.is_alphanumeric()) {}
                println!("{}", x)
            }
        }
    }
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
