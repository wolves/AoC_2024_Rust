use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    bytes::complete::take_till, character::complete::satisfy, multi::many1, sequence::preceded,
    AsChar, IResult,
};
use nom_locate::{position, LocatedSpan};

pub type Span<'a> = LocatedSpan<&'a str>;

pub fn process(input: &str) -> miette::Result<String> {
    let height = input.lines().count();
    let width = input.lines().next().unwrap().len();

    let y_bound = 0..height as i32;
    let x_bound = 0..width as i32;

    let (input, mut result) = parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;
    result.sort_by(|a, b| a.1.cmp(&b.1));
    let results = result
        .chunk_by(|a, b| a.1 == b.1)
        .flat_map(|chunk| {
            chunk
                .iter()
                .combinations(2)
                .flat_map(|sats| {
                    let diff = sats[0].0 - sats[1].0;
                    [sats[0].0 + diff, sats[1].0 - diff]
                })
                .filter(|pos| x_bound.contains(&pos.x) && y_bound.contains(&pos.y))
        })
        .unique()
        .count();
    Ok(results.to_string())
}

fn alphanum_pos(input: Span) -> IResult<Span, (IVec2, char)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_alphanum())(input)?;
    Ok((input, (IVec2::new(x, y), c)))
}

fn parse(input: Span) -> IResult<Span, Vec<(IVec2, char)>> {
    many1(preceded(take_till(|c: char| c.is_alphanum()), alphanum_pos))(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";
        assert_eq!("14", process(input)?);
        Ok(())
    }
}
