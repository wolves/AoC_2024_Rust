use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, presents) = parse(input).map_err(|e| miette::miette!("Parsing failed {}", e))?;

    let result: u32 = presents
        .iter()
        .map(|present| present.volume() + present.smallest_perim())
        .sum();

    Ok(result.to_string())
}

#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn volume(&self) -> u32 {
        let Present { l, w, h } = self;
        l * w * h
    }

    fn smallest_perim(&self) -> u32 {
        let mut sides = [self.l, self.w, self.h];
        sides.sort();

        2 * (sides[0] + sides[1])
    }
}

fn present(input: &str) -> IResult<&str, Present> {
    let (input, dimensions) = separated_list1(tag("x"), complete::u32)(input)?;

    Ok((
        input,
        Present {
            l: dimensions[0],
            w: dimensions[1],
            h: dimensions[2],
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Present>> {
    separated_list1(line_ending, present)(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("2x3x4", "34")]
    #[case("1x1x10", "14")]
    #[case(
        "2x3x4
1x1x10",
        "48"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(Present { l: 2, w: 3, h: 4 }, 24)]
    #[case(Present { l: 1, w: 1, h: 10 }, 10)]
    fn test_volume(#[case] input: Present, #[case] result: u32) -> miette::Result<()> {
        assert_eq!(result, input.volume());
        Ok(())
    }

    #[rstest]
    #[case(Present { l: 2, w: 3, h: 4 }, 10)]
    #[case(Present { l: 1, w: 1, h: 10 }, 4)]
    #[case(Present { l: 4, w: 3, h: 9 }, 14)]
    fn test_smallest_perim(#[case] input: Present, #[case] result: u32) -> miette::Result<()> {
        assert_eq!(result, input.smallest_perim());
        Ok(())
    }
}
