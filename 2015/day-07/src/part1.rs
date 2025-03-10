use std::collections::HashMap;

use nom::{
    branch::alt,
    bytes::complete::{tag, take_while1},
    character::complete::{digit1, line_ending, multispace0, multispace1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn process(input: &str, wire: Option<&str>) -> miette::Result<String> {
    let input = input.trim();

    let (_input, instructions) = parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    let mut circuit = Circuit::new(instructions);

    if let Some(targ_wire) = wire {
        let result = circuit.evaluate(targ_wire);
        Ok(result.to_string())
    } else {
        let result = circuit.evaluate("a");
        Ok(result.to_string())
    }
}

#[derive(Debug)]
struct Circuit {
    instructions: HashMap<String, Operation>,
    cache: HashMap<String, u16>,
}

impl Circuit {
    fn new(instructions: HashMap<String, Operation>) -> Self {
        Circuit {
            instructions,
            cache: HashMap::new(),
        }
    }

    fn evaluate(&mut self, wire: &str) -> u16 {
        if let Some(&value) = self.cache.get(wire) {
            return value;
        }

        let operation = self.instructions.get(wire).expect("Wire not found").clone();

        let value = match operation {
            Operation::Direct(source) => self.evaluate_source(&source),
            Operation::Not(source) => !self.evaluate_source(&source),
            Operation::And(source1, source2) => {
                self.evaluate_source(&source1) & self.evaluate_source(&source2)
            }
            Operation::Or(source1, source2) => {
                self.evaluate_source(&source1) | self.evaluate_source(&source2)
            }
            Operation::LSHIFT(source, bits) => self.evaluate_source(&source) << bits,
            Operation::RSHIFT(source, bits) => self.evaluate_source(&source) >> bits,
        };

        self.cache.insert(wire.to_string(), value);
        value
    }

    fn evaluate_source(&mut self, source: &Source) -> u16 {
        match source {
            Source::Wire(wire) => self.evaluate(wire),
            Source::Value(val) => *val,
        }
    }
}

#[derive(Debug, Clone)]
enum Source {
    Wire(String),
    Value(u16),
}

#[derive(Debug, Clone)]
enum Operation {
    Direct(Source),
    And(Source, Source),
    Or(Source, Source),
    Not(Source),
    LSHIFT(Source, u16),
    RSHIFT(Source, u16),
}

// PARSING
fn parse_num(input: &str) -> IResult<&str, u16> {
    map_res(digit1, str::parse)(input)
}

fn parse_wire_name(input: &str) -> IResult<&str, String> {
    map(take_while1(|c: char| c.is_lowercase()), |s: &str| {
        s.to_string()
    })(input)
}

fn parse_source(input: &str) -> IResult<&str, Source> {
    alt((
        map(parse_wire_name, Source::Wire),
        map(parse_num, Source::Value),
    ))(input)
}
// Direct assignment: "123 -> x" or "y -> x"
fn parse_direct(input: &str) -> IResult<&str, (String, Operation)> {
    map(
        tuple((
            parse_source,
            multispace0,
            tag("->"),
            multispace0,
            parse_wire_name,
        )),
        |(src, _, _, _, dest)| (dest, Operation::Direct(src)),
    )(input)
}
// Binary operations: "x AND y -> z", "x LSHIFT 2 -> z"
fn parse_binary(input: &str) -> IResult<&str, (String, Operation)> {
    map(
        tuple((
            parse_source,
            multispace0,
            alt((tag("AND"), tag("OR"), tag("LSHIFT"), tag("RSHIFT"))),
            multispace0,
            parse_source,
            multispace0,
            tag("->"),
            multispace0,
            parse_wire_name,
        )),
        |(src1, _, op, _, src2, _, _, _, dest)| {
            let operation = match op {
                "AND" => Operation::And(src1, src2),
                "OR" => Operation::Or(src1, src2),
                "LSHIFT" => {
                    if let Source::Value(n) = src2 {
                        Operation::LSHIFT(src1, n)
                    } else {
                        panic!("LSHIFT requires a numeric value")
                    }
                }
                "RSHIFT" => {
                    if let Source::Value(n) = src2 {
                        Operation::RSHIFT(src1, n)
                    } else {
                        panic!("RSHIFT requires a numeric value")
                    }
                }
                _ => unreachable!(),
            };
            (dest, operation)
        },
    )(input)
}
// NOT operation: "NOT x -> y"
fn parse_unary(input: &str) -> IResult<&str, (String, Operation)> {
    map(
        tuple((
            tag("NOT"),
            multispace1,
            parse_source,
            multispace0,
            tag("->"),
            multispace0,
            parse_wire_name,
        )),
        |(_, _, src, _, _, _, dest)| (dest, Operation::Not(src)),
    )(input)
}
fn parse(input: &str) -> IResult<&str, HashMap<String, Operation>> {
    map(
        separated_list1(line_ending, alt((parse_direct, parse_binary, parse_unary))),
        |instructions| instructions.into_iter().collect(),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;
    /*
    d: 72
    e: 507
    f: 492
    g: 114
    h: 65412
    i: 65079
    x: 123
    y: 456
    */
    #[rstest]
    // Case value for i
    #[case(
        "123 -> x
456 -> y
x AND y -> d
x OR y -> e
x LSHIFT 2 -> f
y RSHIFT 2 -> g
NOT x -> h
NOT y -> i",
        "65079"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input, Some("i"))?);
        Ok(())
    }
}
