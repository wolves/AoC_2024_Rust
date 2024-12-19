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

    let count: usize = designs
        .iter()
        .map(|design| {
            count_valid_designs(
                design,
                &towels,
                &mut HashMap::new(),
            )
        })
        .sum();

    Ok(count.to_string())
}

fn count_valid_designs(
    design: &str,
    towels: &[&str],
    mem: &mut HashMap<String, usize>,
) -> usize {
    if mem.contains_key(design) {
        return mem[design];
    }
    let result = towels
        .iter()
        .map(|&towel| {
            if design == towel {
                1
            } else if let Some(rem_design) =
                design.strip_prefix(towel)
            {
                let subdesign = rem_design;
                count_valid_designs(subdesign, towels, mem)
            } else {
                0
            }
        })
        .sum();

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
        assert_eq!("16", process(input)?);
        Ok(())
    }
}
