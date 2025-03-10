use std::collections::{HashMap, HashSet};

use glam::IVec2;
use miette::miette;
use nom::{
    character::complete::{line_ending, satisfy},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

const DIRECTIONS: [IVec2; 4] = [IVec2::X, IVec2::NEG_X, IVec2::Y, IVec2::NEG_Y];

pub fn process(input: &str) -> miette::Result<String> {
    let data = input.trim();
    let (_input, map) = parse(Span::new(data)).map_err(|e| miette!("parse failed {}", e))?;

    let counts: usize = map
        .iter()
        .filter(|(_, height)| height == &&0)
        .map(|(pos, _)| search_path(&map, pos))
        .sum();

    Ok(counts.to_string())
}

fn search_path(map: &HashMap<IVec2, u32>, position: &IVec2) -> usize {
    let mut visited: HashSet<IVec2> = HashSet::from([]);
    let mut new_locations: HashSet<IVec2> = HashSet::from([*position]);
    loop {
        if new_locations.is_empty() {
            break;
        }

        let newer_locations = new_locations
            .iter()
            .flat_map(|starting_location| {
                DIRECTIONS
                    .iter()
                    .zip(std::iter::repeat(starting_location))
                    .map(|(dir, location)| (dir + location, location))
                    .filter(|(new_location, location)| {
                        !visited.contains(new_location)
                            && !new_locations.contains(new_location)
                            && map.get(new_location).is_some_and(|h| {
                                let current_height = map.get(location).unwrap();
                                *h == current_height + 1
                            })
                    })
                    .map(|(new, _)| new)
            })
            .collect::<HashSet<IVec2>>();

        visited = visited
            .union(&newer_locations)
            .cloned()
            .collect::<HashSet<IVec2>>();
        new_locations = newer_locations;
    }

    visited
        .iter()
        .filter(|pos| map.get(pos).unwrap() == &9)
        .count()
}

pub type Span<'a> = LocatedSpan<&'a str>;
fn alphanum_pos(input: Span) -> IResult<Span, (IVec2, u32)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, c) = satisfy(|c| c.is_numeric())(input)?;

    Ok((input, (IVec2::new(x, y), c.to_digit(10).unwrap())))
}

fn parse(input: Span) -> IResult<Span, HashMap<IVec2, u32>> {
    let (input, lines) = separated_list1(line_ending, many1(alphanum_pos))(input)?;

    let hashmap = lines
        .iter()
        .flatten()
        .copied()
        .collect::<HashMap<IVec2, u32>>();

    Ok((input, hashmap))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";
        assert_eq!("36", process(input)?);
        Ok(())
    }
}
