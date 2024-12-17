use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::is_a,
    character::complete::{self, line_ending, multispace1},
    combinator::{opt, value},
    multi::{many1, separated_list1},
    sequence::preceded,
    IResult,
};
use nom_locate::{position, LocatedSpan};
use std::collections::HashMap;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let new_input = input
        .chars()
        .map(|c| match c {
            '#' => "##".to_string(),
            'O' => "[]".to_string(),
            '.' => "..".to_string(),
            '@' => "@.".to_string(),
            other => other.to_string(),
        })
        .collect::<String>();

    let (_, (mut map, directions)) =
        parse(Span::new(&new_input)).map_err(|e| {
            miette::miette!("parse failed {}", e)
        })?;

    for direction in directions {
        let robot = *map
            .iter()
            .find(|(_, obj)| obj == &&Object::Robot)
            .expect("a robot")
            .0;
        let next_pos = robot + direction;
        let Some(next) = map.get(&next_pos) else {
            let bot = map
                .remove(&robot)
                .expect("robot to exist when removing");
            map.insert(next_pos, bot);

            continue;
        };

        match next {
            Object::Wall => continue,
            Object::Box => {
                // check all objects until wall or
                // space
                let mut items = vec![next_pos];
                while Some(&Object::Box)
                    == map.get(
                        &(items.iter().last().unwrap()
                            + direction),
                    )
                {
                    items.push(
                        items.iter().last().unwrap()
                            + direction,
                    );
                }

                // because of other checks next will be a
                // wall here
                let Some(_next) = map.get(
                    &(items.iter().last().unwrap()
                        + direction),
                ) else {
                    let bot = map.remove(&robot).expect(
                        "robot to exist when removing",
                    );

                    let mut it = items.iter();
                    let next_item_location =
                        it.next().unwrap();
                    let next_ = map
                        .remove(next_item_location)
                        .expect(
                            "robot to exist when removing",
                        );
                    map.insert(*next_item_location, bot);
                    match it.last() {
                        Some(location) => {
                            map.insert(
                                *location + direction,
                                next_,
                            );
                        }
                        None => {
                            map.insert(
                                next_item_location
                                    + direction,
                                next_,
                            );
                        }
                    }
                    continue;
                };
            }
            Object::Robot => {
                unreachable!(
                    "should never see a second robot"
                );
            }
        }
    }

    let result: i32 = map
        .iter()
        .filter(|(_, obj)| obj == &&Object::Box)
        .map(|(pos, _)| 100 * pos.y + pos.x)
        .sum();

    Ok(result.to_string())
}

pub type Span<'a> = LocatedSpan<&'a str>;

#[derive(Debug, Clone, PartialEq, Eq)]
enum Object {
    Wall,
    BoxLeft,
    BoxRight,
    Robot,
}
fn object_pos(
    input: Span,
) -> IResult<Span, (IVec2, Object)> {
    let (input, pos) = position(input)?;
    let x = pos.get_column() as i32 - 1;
    let y = pos.location_line() as i32 - 1;
    let (input, obj) = alt((
        value(Object::Wall, complete::char('#')),
        value(Object::BoxLeft, complete::char('[')),
        value(Object::BoxRight, complete::char(']')),
        value(Object::Robot, complete::char('@')),
    ))(input)?;
    Ok((input, (IVec2::new(x, y), obj)))
}

fn parse(
    input: Span,
) -> IResult<Span, (HashMap<IVec2, Object>, Vec<IVec2>)> {
    let (input, lines) = separated_list1(
        line_ending,
        many1(preceded(opt(is_a(".")), object_pos)),
    )(input)?;

    let (input, directions) = preceded(
        multispace1,
        separated_list1(
            line_ending,
            many1(alt((
                value(IVec2::NEG_Y, complete::char('^')),
                value(IVec2::Y, complete::char('v')),
                value(IVec2::X, complete::char('>')),
                value(IVec2::NEG_X, complete::char('<')),
            ))),
        ),
    )(input)?;

    let hashmap = lines.into_iter().flatten().collect();
    Ok((
        input,
        (
            hashmap,
            directions.into_iter().flatten().collect(),
        ),
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(
        "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
        "no answer"
    )]
    #[case(
        "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
        "9021"
    )]

    fn test_process(
        #[case] input: &str,
        #[case] result: &str,
    ) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
