use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::separated_list1,
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, presents) = parse(input).map_err(|e| miette::miette!("Parsing failed {}", e))?;

    //2*l*w + 2*w*h + 2*h*l
    todo!("day 00 - part 1");
}

#[derive(Debug)]
struct Present {
    l: u32,
    w: u32,
    h: u32,
}

impl Present {
    fn surface_area(&self) -> u32 {
        let Present { l, w, h } = self;
        (2 * l * w) + (2 * w * h) + (2 * h * l)
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
    #[case("2x3x4", "58")]
    #[case("1x1x10", "43")]
    #[case(
        "2x3x4
1x1x10",
        "101"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
