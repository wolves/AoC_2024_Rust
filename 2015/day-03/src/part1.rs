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

    dbg!(directions);
    todo!("day 00 - part 1");
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
