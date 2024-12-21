// Portable SIMD API is enabled by "simd" feature.
#![cfg_attr(feature = "simd", allow(unstable_features), feature(portable_simd))]
use regex as _; // This is temp for right now

/* May switch it to this later on
pub fn compile_regex(pattern: &str) -> regex::Regex {
    regex::Regex::new(pattern).expect("Invalid regex pattern")
}
    -----------------------
use advent_of_code_rust::compile_regex;

let year_day_pattern = compile_regex(r"year(\d{4})::day(\d{2})");
let year_pattern = compile_regex(r"year(\d{4})");


*/

macro_rules! library {
    ($year:tt $description:literal $($day:tt),*) => {
        #[doc = concat!("# ", $description)]
        pub mod $year {$(pub mod $day;)*}
    }
}

library!(util "Utility modules to handle common recurring Advent of Code patterns."
    ansi, bitset, grid, hash, heap, integer, iter, math, md5, parse, point, slice, thread
);

library!(year2020 "What could go wrong trying to enjoy a well deserved vacation?"
    day01,day02
);
// library!(year2023 "Restore global snow production."
//     day01, day02, day03
// );

library!(year2024 "Locate the Chief Historian in time for the big Christmas sleigh launch."
    day01,day02,day03
);
