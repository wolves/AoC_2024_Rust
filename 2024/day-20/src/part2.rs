use std::{collections::HashSet, ops::Not};

use glam::IVec2;
use itertools::Itertools;
use nom::{
    character::complete::{line_ending, one_of},
    combinator::{all_consuming, opt},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::LocatedSpan;
use pathfinding::prelude::*;

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, Map { start, end, walls }) =
        all_consuming(parse)(Span::new(input)).map_err(
            |e| miette::miette!("parsing failed {}", e),
        )?;

    let (orig_path, orig_cost) = dijkstra(
        &start,
        |position| {
            DIRECTIONS
                .iter()
                .filter_map(|dir| {
                    let next_pos = position + dir;
                    walls
                        .contains(&next_pos)
                        .not()
                        .then_some((next_pos, 1))
                })
                .collect::<Vec<_>>()
        },
        |&pos| pos == end,
    )
    .expect("a valid path");

    let result = orig_path
        .iter()
        .enumerate()
        .tuple_combinations()
        .filter_map(
            |(
                (start_cost, start_pos),
                (end_cost, end_pos),
            )| {
                let distance: usize = (start_pos - end_pos)
                    .abs()
                    .element_sum()
                    as usize;
                if distance > 20 {
                    return None;
                };
                let cheat_cost =
                    start_cost + distance + orig_cost
                        - end_cost;
                Some(orig_cost - cheat_cost)
            },
        )
        .filter(|savings| savings >= &100)
        .count();

    Ok(result.to_string())
}

type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, token) = one_of(".#SE")(input)?;

    Ok((
        input,
        (
            IVec2::new(x as i32 - 1, y as i32 - 1),
            token,
        ),
    ))
}

struct Map {
    start: IVec2,
    end: IVec2,
    walls: HashSet<IVec2>,
}

fn parse(input: Span) -> IResult<Span, Map> {
    let (input, items) =
        separated_list1(line_ending, many1(token))(input)?;

    let (input, _) = opt(line_ending)(input)?;

    let (starting_position, _) = items
        .iter()
        .flatten()
        .find(|(_, val)| val == &'S')
        .cloned()
        .expect("should have a starting position");
    let (ending_position, _) = items
        .iter()
        .flatten()
        .find(|(_, val)| val == &'E')
        .cloned()
        .expect("should have a ending position");
    let walls = items
        .into_iter()
        .flatten()
        .filter_map(|(pos, val)| {
            (val == '#').then_some(pos)
        })
        .collect::<HashSet<IVec2>>();

    Ok((
        input,
        Map {
            start: starting_position,
            end: ending_position,
            walls,
        },
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";
        assert_eq!("", process(input)?);
        Ok(())
    }
}
