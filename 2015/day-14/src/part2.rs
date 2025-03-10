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

    let mut race = Race::new(racers.as_slice());
    let result = race.run(race_dur);

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
    fn distance_at_second(&self, second: u32) -> u32 {
        let cycle_dur = self.endurance + self.rest;
        let complete_cycles = second / cycle_dur;
        let base_distance = complete_cycles * (self.endurance * self.speed);
        let remaining_secs = second % cycle_dur;

        base_distance + (remaining_secs.min(self.endurance) * self.speed)
    }
}

#[derive(Debug)]
struct Race<'a> {
    reindeer: Vec<(&'a Reindeer, u32)>,
}

impl<'a> Race<'a> {
    fn new(reindeer: &'a [Reindeer]) -> Self {
        Race {
            reindeer: reindeer.iter().map(|r| (r, 0)).collect(),
        }
    }

    fn run(&mut self, duration: u32) -> u32 {
        (1..=duration).for_each(|sec| {
            let positions: Vec<(usize, u32)> = self
                .reindeer
                .iter()
                .enumerate()
                .map(|(i, (r, _))| (i, r.distance_at_second(sec)))
                .collect();

            let max_dist = positions.iter().map(|(_, dist)| *dist).max().unwrap_or(0);

            positions
                .iter()
                .filter(|(_, dist)| *dist == max_dist)
                .for_each(|(i, _)| self.reindeer[*i].1 += 1);
        });

        self.reindeer
            .iter()
            .map(|(_, score)| *score)
            .max()
            .unwrap_or(0)
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
        "689"
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
        Reindeer { name: "Comet".to_string(), speed: 14, endurance: 10, rest: 127 },
        9,
        126
    )]
    #[case(
        Reindeer { name: "Comet".to_string(), speed: 14, endurance: 10, rest: 127 },
        10,
        140
    )]
    #[case(
        Reindeer { name: "Comet".to_string(), speed: 14, endurance: 10, rest: 127 },
        137,
        140
    )]
    #[case(
        Reindeer { name: "Comet".to_string(), speed: 14, endurance: 10, rest: 127 },
        138,
        154
    )]
    #[case(
        Reindeer { name: "Comet".to_string(), speed: 14, endurance: 10, rest: 127 },
        139,
        168
    )]
    fn test_distance_at_second(
        #[case] reindeer: Reindeer,
        #[case] second: u32,
        #[case] expected: u32,
    ) -> miette::Result<()> {
        assert_eq!(expected, reindeer.distance_at_second(second));
        Ok(())
    }
}
