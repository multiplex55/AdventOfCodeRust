//https://adventofcode.com/2023/day/3
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]
use core::{fmt, hash};
use std::collections::{self, hash_map, HashMap, HashSet};
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;
use std::{cell, vec};

#[derive(Clone, Hash, Eq, PartialEq, Debug)]
struct MergedCell {
    number: u32,
    row: u32,
    col: u32,
}

#[derive(PartialEq, Eq, Hash)]
struct Cell {
    letter: char,
    row: u32,
    col: u32,
}

impl fmt::Display for MergedCell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "MergedCell {{ number: {}, row: {}, col: {} }}",
            self.number, self.row, self.col
        )
    }
}

impl fmt::Display for Cell {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "Cell {{ Letter: {}, row: {}, col: {} }}",
            self.letter, self.row, self.col
        )
    }
}
pub fn gear_ratio_part1(file_path: String) {
    let mut sum: u32 = 0;
    let mut engine_schematic: Vec<Vec<String>> = Vec::new();
    let mut line_counter = 0;
    let mut merged_cells: Vec<Vec<MergedCell>> = Vec::new();

    if let Ok(lines) = read_lines(file_path) {
        for line in lines.map_while(Result::ok) {
            let mut current_line_array: Vec<String> = Vec::new();

            let owned_line = line.to_string();
            for ch in owned_line.chars() {
                current_line_array.push(ch.clone().to_string());
            }
            engine_schematic.push(current_line_array);
        }
    }
    //DEBUGGING
    for x in engine_schematic.iter() {
        for y in x.iter() {
            println!("{}", y)
        }
    }

    //DEBUGGING

    for (current_row_counter, row_el) in engine_schematic.iter().enumerate() {
        for (current_col_counter, col_el) in row_el.iter().enumerate() {
            if engine_schematic[current_row_counter][current_col_counter] != "."
                && !(engine_schematic[current_row_counter][current_col_counter]
                    .chars()
                    .next()
                    .unwrap())
                .is_ascii_digit()
            {
                merged_cells.push(check_for_numbers(
                    &engine_schematic,
                    current_row_counter.try_into().unwrap(),
                    current_col_counter.try_into().unwrap(),
                ));

                println!("DEDUPLICATE Called");
                // slightly expensive but simple way to deduplicate, yolo
                merged_cells = vec![merged_cells
                    .into_iter()
                    .flatten()
                    .collect::<HashSet<_>>()
                    .into_iter()
                    .collect()];

                //DEBUGGING
                for row in merged_cells.iter() {
                    for cell in row.iter() {
                        println!(
                            "MergedCell {{ number: {}, row: {}, col: {} }}",
                            cell.number, cell.row, cell.col
                        );
                    }
                }
                println!("=======================")
            }
        }
    }

    //DEBUGGING
    for row in merged_cells.iter() {
        for cell in row.iter() {
            println!(
                "MergedCell {{ number: {}, row: {}, col: {} }}",
                cell.number, cell.row, cell.col
            );
        }
    }

    sum = merged_cells
        .iter()
        .flat_map(|row| row.iter())
        .map(|cell| cell.number)
        .sum();

    println!("Day 3 Part 1 Sum is {}", sum);
}

fn check_for_numbers(
    engine_schematic: &Vec<Vec<String>>,
    original_row_index: isize,
    original_col_index: isize,
) -> Vec<MergedCell> {
    let mut numbers_encountered: Vec<u32> = Vec::new();
    let mut cells: Vec<Cell> = Vec::new();
    let mut merged_cells: Vec<MergedCell> = Vec::new();

    let max_row: isize = (engine_schematic.len() - 1).try_into().unwrap();
    let max_col: isize = (engine_schematic.first().unwrap().len() - 1)
        .try_into()
        .unwrap();

    const ZERO: isize = 0;
    const ONE: isize = 1;

    // Check up
    if original_row_index - 1 >= ZERO {
        let char_at_index = engine_schematic[original_row_index as usize - 1]
            [original_col_index as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index - 1).try_into().unwrap(),
                original_col_index.try_into().unwrap(),
            ));
            println!("DEBUG: direction UP");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check right
    if original_col_index <= max_col {
        let char_at_index = engine_schematic[original_row_index as usize]
            [(original_col_index + 1) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index).try_into().unwrap(),
                (original_col_index + 1).try_into().unwrap(),
            ));
            println!("DEBUG: direction right");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check down
    if original_row_index + 1 <= max_row {
        let char_at_index = engine_schematic[(original_row_index + 1) as usize]
            [(original_col_index) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index + 1).try_into().unwrap(),
                (original_col_index).try_into().unwrap(),
            ));
            println!("DEBUG: direction down");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check left
    if original_col_index - 1 >= ZERO {
        let char_at_index = engine_schematic[(original_row_index) as usize]
            [(original_col_index - 1) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index).try_into().unwrap(),
                (original_col_index - 1).try_into().unwrap(),
            ));
            println!("DEBUG: direction left");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check up right
    if original_row_index - 1 >= ZERO && original_col_index + 1 <= max_col {
        let char_at_index = engine_schematic[(original_row_index - 1) as usize]
            [(original_col_index + 1) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index - 1).try_into().unwrap(),
                (original_col_index + 1).try_into().unwrap(),
            ));
            println!("DEBUG: direction up right");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check up left
    if original_row_index - 1 >= ZERO && original_col_index - 1 >= ZERO {
        let char_at_index = engine_schematic[(original_row_index - 1) as usize]
            [(original_col_index - 1) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index - 1).try_into().unwrap(),
                (original_col_index - 1).try_into().unwrap(),
            ));
            println!("DEBUG: direction up left");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check down right
    if original_row_index + 1 <= max_row && original_col_index + 1 <= max_col {
        let char_at_index = engine_schematic[(original_row_index + 1) as usize]
            [(original_col_index + 1) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index + 1).try_into().unwrap(),
                (original_col_index + 1).try_into().unwrap(),
            ));
            println!("DEBUG: direction down right");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }

    // Check down left
    if original_row_index + 1 <= max_row && original_col_index - 1 >= ZERO {
        let char_at_index = engine_schematic[(original_row_index + 1) as usize]
            [(original_col_index - 1) as usize]
            .chars()
            .next()
            .unwrap();

        if char_at_index.is_ascii_digit() {
            merged_cells.push(walk_and_find_numbers(
                engine_schematic,
                (original_row_index + 1).try_into().unwrap(),
                (original_col_index - 1).try_into().unwrap(),
            ));
            println!("DEBUG: direction down left");
            for cell in merged_cells.iter() {
                println!(
                    "MergedCell {{ number: {}, row: {}, col: {} }}",
                    cell.number, cell.row, cell.col
                );
            }
        }
    }
    println!("DEBUG Exiting check for numbers");
    println!("@@@@@@@@@@@@@@@@@@@@@@@@@@@");
    merged_cells
}

fn walk_and_find_numbers(
    engine_schematic: &[Vec<String>],
    original_row_index: usize,
    original_col_index: usize,
) -> MergedCell {
    let mut cells: Vec<Cell> = Vec::new();
    let merged_cells: Vec<MergedCell> = Vec::new();
    let max_row = engine_schematic.len();
    let max_col = engine_schematic.first().unwrap().len();

    let mut current_row_index = original_row_index;
    let mut current_col_index = original_col_index;
    let mut current_char = engine_schematic[current_row_index][current_col_index]
        .chars()
        .next()
        .unwrap();

    println!(
        "Current row {} col {} current char {}",
        current_row_index, current_col_index, current_char
    );
    println!("Walk left");
    while current_char.is_ascii_digit() {
        current_char = engine_schematic[current_row_index][current_col_index]
            .chars()
            .next()
            .unwrap();
        cells.push(Cell {
            letter: current_char,
            row: current_row_index.try_into().unwrap(),
            col: current_col_index.try_into().unwrap(),
        });

        if current_col_index == 0 {
            break;
        }
        current_col_index -= 1;
    }

    //Reset indexes
    current_row_index = original_row_index;
    current_col_index = original_col_index;
    current_char = engine_schematic[current_row_index][current_col_index]
        .chars()
        .next()
        .unwrap();

    println!("Walk right");
    while current_char.is_ascii_digit() {
        current_char = engine_schematic[current_row_index][current_col_index]
            .chars()
            .next()
            .unwrap();
        cells.push(Cell {
            letter: current_char,
            row: current_row_index.try_into().unwrap(),
            col: current_col_index.try_into().unwrap(),
        });

        current_col_index += 1;
        if current_col_index >= max_col {
            break;
        }
    }
    println!("Cells before sorting");

    for c in &cells {
        println!("{}", c);
    }
    if cells.is_empty() {
        println!("brekapoint")
    }

    cells.sort_unstable_by_key(|k| k.col);
    println!("Cells after sorting");
    for c in &cells {
        println!("{}", c);
    }
    let chars_to_filter = ['.', '*', '$', '#', '+', '=', '@', '/', '%', '-', '&'];

    for ch in chars_to_filter {
        cells.retain(|c| c.letter != ch);
    }

    //deduplicate cells
    //keeping order w/o using Hashset
    cells.sort_by_key(|c| (c.letter, c.row, c.col));
    cells.sort_by_key(|c| (c.col));
    cells.dedup();

    let mut merged_num_string = String::new();
    let mut merged_row_string = String::new();
    let mut merged_col_string = String::new();

    cells
        .iter()
        .for_each(|item| merged_num_string.push(item.letter));

    cells
        .iter()
        .for_each(|item| merged_row_string.push_str(&item.row.to_string()));

    cells
        .iter()
        .for_each(|item| merged_col_string.push_str(&item.col.to_string()));

    MergedCell {
        number: merged_num_string.parse::<u32>().unwrap(),
        row: merged_row_string.parse::<u32>().unwrap(),
        col: merged_col_string.parse::<u32>().unwrap(),
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
