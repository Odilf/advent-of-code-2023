//! Library for holding all the boilerplate code for the advent of code solutions
//!
//! It provides the [`christmas_tree::day`] macro which picks up the solutions for each part and
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

mod data;

const YEAR: usize = 2023;

pub type Part<T> = fn(&str) -> T;
pub struct Solution<T, U> {
    pub part1: Part<T>,
    pub part2: Part<U>,
}

#[derive(Parser, Debug)]
struct Args {
    #[clap(short, long, value_parser = clap::value_parser!(u32).range(1..=2))]
    part: Option<u32>,
}

/// Runs the solution as a binary
///
/// # Panics
pub fn run_as_main<T, U>(solution: &Solution<T, U>, day: u32)
where
    T: std::fmt::Display,
    U: std::fmt::Display,
{
    let args = Args::parse();

    match args.part {
        Some(1) => println!("{}", (solution.part1)(&data::get(day).input)),
        Some(2) => println!("{}", (solution.part2)(&data::get(day).input)),
        None => {
            println!("Part 1: {}", (solution.part1)(&data::get(day).input));
            println!("Part 2: {}", (solution.part2)(&data::get(day).input));
        }
        _ => unreachable!("Handled by clap"),
    }
}

#[macro_export]
macro_rules! day {
    ($day:literal) => {
        fn main() {
            $crate::run_as_main(&$crate::Solution { part1, part2 }, $day);
        }
    };
}

/// Macro for making it easier to write examples
///
/// # Examples
///
/// The main way you use it for part 1 and 2 is like this:
///
/// ```rust
/// christmas_tree::examples! {
///    r"
///         example input
///    " => 4,
///
///    r"
///         Example input for part 2
///     " => 4,
/// }
/// ```
///
/// The indentation is handled by the [`indoc`] macro.
///
/// If part 1 and 2 have the same input, you can combine them:
///
/// ```rust
/// christmas_tree::examples! {
///   r"
///         example input
///         for both part 1 and 2
///    " => 4, 8,
/// }
/// ```
///
/// If there are multiple examples for a part, you can do this:
///
/// ```rust
/// christmas_tree::examples! {
///     part1 {
///         r"
///             example input
///             for part 1
///         " => 4,
///
///         r"
///             another example input
///             for part 1
///         " => 8,
///     }
///
///     part2 {
///         r"
///             example input
///             for part 2
///         " => 4,
///     }
/// }
/// ```
///
/// You can also be more granular:
///
/// ```rust
/// #[cfg(test)]
/// mod tests {
///     christmas_tree::examples!(part1, test_simple, r"example input" => 4);
/// }
#[macro_export]
macro_rules! examples {
    // This rule needs to be first because otherwise it messes up the priorities
    // 5. Part 1 and 2 with different inputs (calls 3)
    (
        $example1:literal => $expected1:expr,
        $example2:literal => $expected2:expr $(,)?
    ) => {
        #[cfg(test)]
        mod tests {
            $crate::examples!(super::part1, part1: $example1 => $expected1);
            $crate::examples!(super::part2, part2: $example2 => $expected2);
        }
    };

    // 1. Single test with name
    ($part:path, $test_name:ident : $example:literal => $expected:expr $(,)?) => {
        #[test]
        fn $test_name() {
            assert_eq!($part($crate::indoc! { $example }), $expected);
        }
    };

    // 2. Multiple named tests (calls 1)
    ($part:path, $($test_name:ident: $example:literal => $expected:expr),+ $(,)?) => {
        $(
            $crate::examples!($part, $test_name: $example => $expected);
        )+
    };

    // 3. Single test with part as name (calls 1)
    ($part:path, $example:literal => $expected:expr $(,)?) => {
        $crate::examples!($part, $part: $example => $expected);
    };

    // 4. Only part 1 (calls 3)
    ($example:literal => $expected:expr $(,)?) => {
        #[cfg(test)]
        mod tests {
            $crate::examples!(part1, $example => $expected);
        }
    };


    // 6. Part 1 and 2 with same input (calls 5)
    ($example:literal => $expected1:expr, $expected2:expr $(,)?) => {
        $crate::examples!($example => $expected1, $example => $expected2);
    };


    // 7. Multiple tests (calls 2)
    (
        $(part1 { $($tests1:tt)* })? $(,)?
        $(part2 { $($tests2:tt)* })? $(,)?
    ) => {
        #[cfg(test)]
        mod tests {
            mod test_part1 {
                $(
                    $crate::examples!(super::super::part1, $($tests1)*);
                )?
            }

            mod part2 {
                $(
                    $crate::examples!(super::super::part2, $($tests2)*);
                )?
            }
        }
    };

    // 0. Empty
    () => {};


}
