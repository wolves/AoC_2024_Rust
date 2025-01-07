//! [Solution Credit](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2015/day08.rs)
//!
//! Nom parsing or my shallow depth of understanding of it did not work for me for this day
//!
//! Explanation from maneatingape original solution:
//!
//! Part two is even more straightforward with no need for statekeeping. Quotes and backslashes
//! need to be escaped so increase the difference by one. As before each newline increases by the
//! difference by two.

const NEWLINE: u8 = 10;
const QUOTE: u8 = 34;
const SLASH: u8 = 92;
const ESCAPE: u8 = 120;

pub fn process(input: &str) -> miette::Result<String> {
    let result: usize = input
        .bytes()
        .map(|b| match b {
            QUOTE | SLASH => 1,
            NEWLINE => 2,
            _ => 0,
        })
        .sum();

    Ok(result.to_string())
}
