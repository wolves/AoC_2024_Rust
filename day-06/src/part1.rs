use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

pub fn process(input: &str) -> miette::Result<String> {
    let (input, output) = parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    todo!("day 00 - part 1");
}

type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, token) = one_of(".#^")(input)?;

    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), token)))
}

fn parse(input: Span) -> IResult<Span, ()> {
    let (input, items) = separated_list1(line_ending, many1(token))(input)?;

    let obstacles = items
        .iter()
        .flatten()
        .filter(|(position, value)| value != &'.')
        .collect::<Vec<_>>();
    dbg!(obstacles);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";
        assert_eq!("41", process(input)?);
        Ok(())
    }
}
