//https://adventofcode.com/2020/day/2
#![allow(unused_variables)]
#![allow(dead_code)]
#![allow(unused_mut)]

use crate::util::iter::*;
use crate::util::parse::*;

pub struct Rule<'a> {
    start: usize,
    end: usize,
    letter: &'a str,
    password: &'a str,
}

//just as a reference
/*
https://doc.rust-lang.org/reference/trait-bounds.html
Lifetime bounds can be applied to types or to other lifetimes. The bound 'a: 'b is usually read as 'a outlives 'b.
'a: 'b means that 'a lasts at least as long as 'b, so a reference &'a () is valid whenever &'b () is valid.
 */

impl Rule<'_> {
    fn from([a, b, c, d]: [&str; 4]) -> Rule<'_> {
        let start = a.unsigned();
        let end = b.unsigned();
        let letter = c;
        let password = d;
        Rule {
            start,
            end,
            letter,
            password,
        }
    }
}

pub fn parse(input: &str) -> Vec<Rule<'_>> {
    input
        .split(['-', ':', ' ', '\n'])
        .filter(|s| !s.is_empty())
        .chunk::<4>()
        .map(Rule::from)
        .collect()
}

pub fn part1(input: &[Rule<'_>]) -> usize {
    input
        .iter()
        .filter(|rule| {
            let letter_count = rule
                .password
                .chars()
                .filter(|letter| *letter == rule.letter.chars().next().unwrap())
                .count();
            rule.start <= letter_count && letter_count <= rule.end
        })
        .count()
}

pub fn part2(input: &[Rule<'_>]) -> usize {
    // iterative debug for records
    // let mut count: usize = 0;
    // for rule in input {
    //     let first_letter = rule.password.char_indices().nth(rule.start - 1).unwrap();
    //     let second_letter = rule.password.char_indices().nth(rule.end - 1).unwrap();
    //     println!(
    //         "{} {} {} {} {} {}",
    //         rule.start, rule.end, rule.letter, rule.password, first_letter.1, second_letter.1
    //     );

    //     if (first_letter.1 != second_letter.1) {
    //         println!("They are different?");
    //     }
    //     if first_letter.1 != second_letter.1
    //         && (first_letter.1 == rule.letter.chars().next().unwrap()
    //             || second_letter.1 == rule.letter.chars().next().unwrap())
    //     {
    //         println!("HIT");
    //         count += 1;
    //     }
    // }
    // count
    input
        .iter()
        .filter(|rule| {
            let first_letter = rule.password.char_indices().nth(rule.start - 1).unwrap();
            let second_letter = rule.password.char_indices().nth(rule.end - 1).unwrap();

            first_letter.1 != second_letter.1
                && (first_letter.1 == rule.letter.chars().next().unwrap()
                    || second_letter.1 == rule.letter.chars().next().unwrap())
        })
        .count()
}
