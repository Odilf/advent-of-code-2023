//! Library for holding all the boilerplate code for the advent of code solutions
//!
//! It provides the [christmas_tree::day] macro which picks up the solutions for each part and
//! runs them based on command line arguments.
//!
//! You need to have two functions named `part1` and `part2` in the scope of the macro for it to
//! work.
//!
//! # Usage
//!
//! ```rust
//! use christmas_tree::day;
//!
//! day!(1);
//!
//! fn part1(input: &str) -> i32 {
//!     42
//! }
//!
//! fn part2(input: &str) -> i32 {
//!    69420
//! }
//! ```
//!

use clap::Parser;

pub use indoc::indoc;

pub mod data;

pub use data::get_data;

const YEAR: usize = 2023;

pub type Part<T = i32> = fn(&str) -> T;
pub struct Solution<T = i32> {
    pub part1: Part<T>,
    pub part2: Part<T>,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long)]
    part: Option<u32>,
}

pub fn run_as_main<T>(solution: Solution<T>, day: u32)
where
    T: std::fmt::Display,
{
    let args = Args::parse();

    match args.part {
        Some(1) => println!("{}", (solution.part1)(&get_data(day).input)),
        Some(2) => println!("{}", (solution.part2)(&get_data(day).input)),
        None => {
            println!("Part 1: {}", (solution.part1)(&get_data(day).input));
            println!("Part 2: {}", (solution.part2)(&get_data(day).input));
        }
        _ => panic!("Part should be either 1 or 2"),
    }
}

#[macro_export]
macro_rules! day {
    ($day:literal) => {
        fn main() {
            $crate::run_as_main($crate::Solution { part1, part2 }, $day);
        }
    };
}

#[macro_export]
macro_rules! examples {
    () => {};

    ($part:ident, $example:literal, $expected:expr) => {
        #[test]
        fn $part() {
            assert_eq!(super::$part($crate::indoc! { $example }), $expected);
        }
    };

    ($example:literal => $expected:expr $(,)?) => {
        #[cfg(test)]
        mod tests {
            $crate::examples!(part1, $example, $expected);
        }
    };

    ($example:literal => $expected1:expr, $expected2:expr $(,)?) => {
        $crate::examples!($example => $expected1, $example => $expected2);
    };

    ($example1:literal => $expected1:expr, $example2:literal => $expected2:expr $(,)?) => {
        #[cfg(test)]
        mod tests {
            $crate::examples!(part1, $example1, $expected1);
            $crate::examples!(part2, $example2, $expected2);
        }
    };
}
