//! [Solution Credit](https://github.com/maneatingape/advent-of-code-rust/blob/main/src/year2015/day08.rs)
//!
//! Nom parsing or my shallow depth of understanding of it did not work for me for this day
//!
//! Explanation from maneatingape original solution:
//!
//! While [regular expressions](https://en.wikipedia.org/wiki/Regular_expression) may feel like a
//! natural choice, it's much faster and easier to simply treat the input as a stream of raw
//! ASCII `u8` bytes including newlines.
//!
//! For part one we run a small state machine using [`fold`] to keep track of the current and
//! previous characters. If we encounter a hexadecimal escape then four characters become one so the
//! difference increases by three. The sequences `\\` and `\"` both increase the difference by one.
//! Each newline increases the difference by two since every line is enclosed with two quotes.

const NEWLINE: u8 = 10;
const SLASH: u8 = 92;
const ESCAPE: u8 = 120;

pub fn process(input: &str) -> miette::Result<String> {
    let (_, result) = input
        .bytes()
        .fold((false, 0), |(flag, count), b| match (flag, b) {
            (true, ESCAPE) => (false, count + 3),
            (true, _) => (false, count + 1),
            (false, SLASH) => (true, count),
            (false, NEWLINE) => (false, count + 2),
            _ => (false, count),
        });

    Ok(result.to_string())
}
