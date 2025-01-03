use std::collections::HashSet;

use glam::IVec2;
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

    let mut santa_pos = IVec2::new(0, 0);
    let mut robo_pos = IVec2::new(0, 0);
    let mut visited = HashSet::from([IVec2::new(0, 0)]);

    for (i, direction) in directions.iter().enumerate() {
        let pos = if i % 2 == 0 {
            santa_pos += *direction;
            santa_pos
        } else {
            robo_pos += *direction;
            robo_pos
        };

        visited.insert(pos);
    }

    let result = visited.len();

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
    #[case("^v", "3")]
    #[case("^>v<", "3")]
    #[case("^v^v^v^v^v", "11")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
