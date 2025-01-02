use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{
        self, alphanumeric1, line_ending, multispace1,
        space1,
    },
    combinator::value,
    multi::separated_list1,
    sequence::{
        preceded, separated_pair, terminated, tuple,
    },
    IResult, Parser,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, (_map, gates)) =
        parse(input).map_err(|e| {
            miette::miette!("parsing failed {}", e)
        })?;

    let connection_cache: HashSet<_> = gates
        .iter()
        .flat_map(
            |Gate {
                 inputs,
                 output: _,
                 opertaion,
             }| {
                [
                    (inputs[0], opertaion),
                    (inputs[1], opertaion),
                ]
            },
        )
        .collect();

    let mut results: Vec<_> = gates
        .iter()
        .filter_map(
            |Gate {
                 inputs,
                 output,
                 opertaion,
             }| match opertaion {
                Operation::AND
                    if inputs[0] != "x00"
                        && inputs[1] != "x00"
                        && !connection_cache.contains(
                            &(output, &Operation::OR),
                        ) =>
                {
                    Some(*output)
                }
                Operation::XOR
                    if ((inputs[0].starts_with('x')
                        || inputs[1].starts_with('x'))
                        && inputs[0] != "x00"
                        && inputs[1] != "x00"
                        && !connection_cache.contains(
                            &(output, &Operation::XOR),
                        ))
                        || (!output.starts_with('z')
                            && !inputs[0]
                                .starts_with('x')
                            && !inputs[1]
                                .starts_with('x')) =>
                {
                    Some(*output)
                }
                Operation::OR
                    if output.starts_with('z')
                        && *output != "z45" =>
                {
                    Some(*output)
                }
                _ => None,
            },
        )
        .collect();

    results.sort();

    Ok(results.join(","))
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
enum Operation {
    AND,
    OR,
    XOR,
}

#[derive(Debug, Clone)]
struct Gate<'a> {
    inputs: Vec<&'a str>,
    output: &'a str,
    opertaion: Operation,
}

fn gate(input: &str) -> IResult<&str, Gate> {
    let (input, elements) = tuple((
        terminated(alphanumeric1, space1),
        alt((
            value(Operation::AND, tag("AND")),
            value(Operation::OR, tag("OR")),
            value(Operation::XOR, tag("XOR")),
        )),
        preceded(space1, alphanumeric1),
        preceded(tag(" -> "), alphanumeric1),
    ))(input)?;

    Ok((
        input,
        Gate {
            inputs: vec![elements.0, elements.2],
            output: elements.3,
            opertaion: elements.1,
        },
    ))
}

fn parse(
    input: &str,
) -> IResult<&str, (HashMap<&str, bool>, Vec<Gate>)> {
    let (input, map) = separated_list1(
        line_ending,
        separated_pair(
            alphanumeric1,
            tag(": "),
            complete::u8.map(|v| match v {
                0 => false,
                1 => true,
                _ => unreachable!(""),
            }),
        ),
    )(input)?;

    let (input, gates) = preceded(
        multispace1,
        separated_list1(line_ending, gate),
    )(input)?;

    let map = map.into_iter().collect();

    Ok((input, (map, gates)))
}
