use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, tuple},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, racers) = parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    dbg!(racers);

    // Temporary result till logic is in place
    let result = "";

    Ok(result.to_string())
}

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    duration: u32,
    rest: u32,
}

// Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
// Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
fn parse_reindeer(input: &str) -> IResult<&str, Reindeer> {
    map(
        tuple((
            map(alpha1, |s: &str| s.to_string()),
            delimited(tag(" can fly "), complete::u32, tag(" km/s for ")),
            complete::u32,
            delimited(
                tag(" seconds, but then must rest for "),
                complete::u32,
                tag(" seconds."),
            ),
        )),
        |(name, speed, duration, rest)| Reindeer {
            name,
            speed,
            duration,
            rest,
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Reindeer>> {
    separated_list1(line_ending, parse_reindeer)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.
Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.
",
        "1120"
    )]
    fn test_process(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(expected, process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(
        "Comet can fly 14 km/s for 10 seconds, but then must rest for 127 seconds.",
        Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            duration: 10,
            rest: 127,
        }
    )]
    #[case(
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            duration: 11,
            rest: 162,
        }
    )]
    fn test_parse(#[case] input: &str, #[case] expected: Reindeer) -> miette::Result<()> {
        let (_input, result) = parse_reindeer(input).unwrap();

        assert_eq!(expected, result);
        Ok(())
    }
}
