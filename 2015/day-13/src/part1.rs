use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, relationships) =
        parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    dbg!(relationships);

    let result = "";
    Ok(result.to_string())
}

#[derive(Debug, PartialEq)]
struct Relationship {
    person: String,
    neighbor: String,
    happiness: i32,
}

fn parse_relationship(input: &str) -> IResult<&str, Relationship> {
    let (input, (person, gain_loss_val, happiness_delta, _, neighbor)) = tuple((
        map(alpha1, |s: &str| s.to_string()),
        map(
            tuple((
                tag(" would "),
                alt((value(1, tag("gain")), value(-1, tag("lose")))),
                tag(" "),
            )),
            |(_, multiplier, _)| multiplier,
        ),
        map_res(digit1, |s: &str| s.parse::<i32>()),
        tag(" happiness units by sitting next to "),
        terminated(map(alpha1, |s: &str| s.to_string()), complete::char('.')),
    ))(input)?;

    Ok((
        input,
        Relationship {
            person,
            neighbor,
            happiness: happiness_delta * gain_loss_val,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Relationship>> {
    separated_list1(line_ending, parse_relationship)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        "330"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
