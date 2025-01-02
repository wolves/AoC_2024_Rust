use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{
        alpha1, line_ending, multispace1,
    },
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let (_input, (towels, designs)) = parse(input)
        .map_err(|e| {
            miette::miette!("parse failed {}", e)
        })?;

    // dbg!(towels, designs);

    let count = designs
        .iter()
        .filter(|design| {
            validate_design(
                design,
                &towels,
                &mut HashMap::new(),
            )
        })
        .count();

    Ok(count.to_string())
}

fn validate_design(
    design: &str,
    towels: &[&str],
    mem: &mut HashMap<String, bool>,
) -> bool {
    if mem.contains_key(design) {
        return mem[design];
    }
    let result = towels.iter().any(|&towel| {
        if design == towel {
            true
        } else if let Some(rem_design) =
            design.strip_prefix(towel)
        {
            let subdesign = rem_design;
            validate_design(subdesign, towels, mem)
        } else {
            false
        }
    });

    mem.insert(design.into(), result);
    result
}

fn parse(
    input: &str,
) -> IResult<&str, (Vec<&str>, Vec<&str>)> {
    separated_pair(
        separated_list1(tag(", "), alpha1),
        multispace1,
        separated_list1(line_ending, alpha1),
    )(input)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
        assert_eq!("6", process(input)?);
        Ok(())
    }
}
