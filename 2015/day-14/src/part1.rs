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

    #[cfg(not(test))]
    let race_dur = 2503;

    #[cfg(test)]
    let race_dur = 1000;

    let result = racers
        .iter()
        .map(|reindeer: &Reindeer| reindeer.distance_over_duration(race_dur))
        .max()
        .unwrap_or(0);

    Ok(result.to_string())
}

#[derive(Debug, PartialEq)]
struct Reindeer {
    name: String,
    speed: u32,
    endurance: u32,
    rest: u32,
}

impl Reindeer {
    fn distance_over_duration(&self, duration: u32) -> u32 {
        let mut cycles = duration / (self.endurance + self.rest);

        if duration % (self.endurance + self.rest) > self.endurance {
            cycles += 1;
        }

        cycles * (self.endurance * self.speed)
    }
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
            endurance: duration,
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
            endurance: 10,
            rest: 127,
        }
    )]
    #[case(
        "Dancer can fly 16 km/s for 11 seconds, but then must rest for 162 seconds.",
        Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            endurance: 11,
            rest: 162,
        }
    )]
    fn test_parse(#[case] input: &str, #[case] expected: Reindeer) -> miette::Result<()> {
        let (_input, result) = parse_reindeer(input).unwrap();

        assert_eq!(expected, result);
        Ok(())
    }

    #[rstest]
    #[case(
        (Reindeer {
            name: "Jeff".to_string(),
            speed: 10,
            endurance: 25,
            rest: 75,
        }, 1000),
        2500
    )]
    #[case(
        (Reindeer {
            name: "Comet".to_string(),
            speed: 14,
            endurance: 10,
            rest: 127,
        }, 1000),
        1120
    )]
    #[case(
        (Reindeer {
            name: "Dancer".to_string(),
            speed: 16,
            endurance: 11,
            rest: 162,
        }, 1000),
        1056
    )]
    fn test_distance_over_duration(
        #[case] (reindeer, duration): (Reindeer, u32),
        #[case] expected: u32,
    ) -> miette::Result<()> {
        assert_eq!(expected, reindeer.distance_over_duration(duration));
        Ok(())
    }
}
