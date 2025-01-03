use std::collections::HashMap;

use glam::IVec2;
use itertools::Itertools;
use nom::{
    branch::alt,
    character::complete::{self, line_ending},
    combinator::value,
    multi::{many1, separated_list1},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, directions) = parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    let mut visited: Vec<IVec2> = vec![IVec2::new(0, 0)];
    let mut location_stops = HashMap::from([(visited[0], 1)]);

    for direction in directions {
        let next_location = visited.last().unwrap() + direction;
        visited.push(next_location);

        location_stops
            .entry(next_location)
            .and_modify(|count| *count += 1)
            .or_insert(1);
    }

    let result = visited.iter().unique().count();
    Ok(result.to_string())
}

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    let (input, directions) = separated_list1(
        line_ending,
        many1(alt((
            value(IVec2::NEG_Y, complete::char('^')),
            value(IVec2::Y, complete::char('v')),
            value(IVec2::NEG_X, complete::char('<')),
            value(IVec2::X, complete::char('>')),
        ))),
    )(input)?;

    Ok((input, directions.into_iter().flatten().collect()))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(">", "2")]
    #[case("^>v<", "4")]
    #[case("^v^v^v^v^v", "2")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
