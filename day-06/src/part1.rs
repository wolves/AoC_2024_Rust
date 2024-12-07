use std::collections::{HashMap, HashSet};

use glam::IVec2;
use itertools::Itertools;
use miette::miette;
use nom::{
    character::complete::{line_ending, one_of},
    multi::{many1, separated_list1},
    IResult,
};
use nom_locate::{position, LocatedSpan};

pub fn process(input: &str) -> miette::Result<String> {
    let (_input, ((mut player_pos, _), obstacles)) =
        parse(Span::new(input)).map_err(|e| miette!("parse failed {}", e))?;

    let x_minmax = obstacles
        .iter()
        .map(|(pos, _)| pos.x)
        .minmax()
        .into_option()
        .unwrap();
    let y_minmax = obstacles
        .iter()
        .map(|(pos, _)| pos.y)
        .minmax()
        .into_option()
        .unwrap();

    let mut direction = Direction::North;

    let mut visited_pos: HashSet<IVec2> = HashSet::from([player_pos]);

    while (x_minmax.0..=x_minmax.1).contains(&player_pos.x)
        && (y_minmax.0..=y_minmax.1).contains(&player_pos.y)
    {
        let next_pos = player_pos + direction.to_ivec2();
        if obstacles.get(&next_pos).is_some() {
            direction = direction.turn_right();
        } else {
            player_pos = next_pos;
            visited_pos.insert(player_pos);
        }
    }

    dbg!(visited_pos.len());
    Ok((visited_pos.len() - 1).to_string())
}

#[derive(Debug)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn turn_right(&self) -> Direction {
        match self {
            Direction::North => Direction::East,
            Direction::South => Direction::West,
            Direction::East => Direction::South,
            Direction::West => Direction::North,
        }
    }

    fn to_ivec2(&self) -> IVec2 {
        match self {
            Direction::North => IVec2::NEG_Y,
            Direction::South => IVec2::Y,
            Direction::East => IVec2::X,
            Direction::West => IVec2::NEG_X,
        }
    }
}

type Span<'a> = LocatedSpan<&'a str>;

fn token(input: Span) -> IResult<Span, (IVec2, char)> {
    let y = input.location_line();
    let x = input.get_column();
    let (input, token) = one_of(".#^")(input)?;

    Ok((input, (IVec2::new(x as i32 - 1, y as i32 - 1), token)))
}

fn parse(input: Span) -> IResult<Span, ((IVec2, char), HashMap<IVec2, char>)> {
    let (input, items) = separated_list1(line_ending, many1(token))(input)?;

    let player = items
        .iter()
        .flatten()
        .find(|(_, value)| value == &'^')
        .cloned()
        .expect("should have a player");

    let obstacles = items
        .into_iter()
        .flatten()
        .filter(|(_, value)| value == &'#')
        .collect::<HashMap<IVec2, char>>();

    Ok((input, (player, obstacles)))
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
