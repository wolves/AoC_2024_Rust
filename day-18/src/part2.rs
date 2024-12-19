use std::ops::Not;

use glam::IVec2;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    sequence::separated_pair,
    IResult, Parser,
};

use pathfinding::prelude::*;

const GRID_SIZE: IVec2 = if cfg!(test) {
    IVec2::splat(6)
} else {
    IVec2::splat(70)
};

const DIRECTIONS: [IVec2; 4] =
    [IVec2::X, IVec2::Y, IVec2::NEG_X, IVec2::NEG_Y];

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, falling_bytes) =
        parse(input).map_err(|e| {
            miette::miette!("parsing failed {}", e)
        })?;

    let mut lower = (GRID_SIZE - 2).x as usize;
    let mut upper = falling_bytes.len();

    let result = loop {
        let n = lower + (upper - lower) / 2;
        let end = falling_bytes.len().min(n);
        let start_node = IVec2::ZERO;

        let path = dijkstra(
            &start_node,
            |position| {
                DIRECTIONS
                    .iter()
                    .filter_map(|dir| {
                        let next_pos = position + dir;

                        if !((0..=GRID_SIZE.x)
                            .contains(&next_pos.x)
                            && (0..=GRID_SIZE.y)
                                .contains(&next_pos.y))
                        {
                            return None;
                        }

                        if falling_bytes[0..end]
                            .contains(&next_pos)
                            .not()
                        {
                            Some((next_pos, 1usize))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            },
            |&p| p == GRID_SIZE,
        );

        match path {
            Some(_) => lower = n + 1,
            None => {
                if n == lower {
                    break &falling_bytes[n - 1];
                }
                upper = n;
            }
        }
    };

    Ok(format!("{},{}", result.x, result.y))
}

fn parse(input: &str) -> IResult<&str, Vec<IVec2>> {
    separated_list1(
        line_ending,
        separated_pair(
            complete::i32,
            tag(","),
            complete::i32,
        )
        .map(|(x, y)| IVec2::new(x, y)),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";
        assert_eq!("6,1", process(input)?);
        Ok(())
    }
}
