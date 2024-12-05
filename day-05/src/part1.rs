use miette::miette;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending},
    multi::{fold_many1, separated_list1},
    sequence::{separated_pair, terminated},
    IResult,
};
use std::collections::HashMap;

pub fn process(input: &str) -> miette::Result<String> {
    let (input, (rules, updates)) = parse(input).map_err(|e| miette!("parse failed {}", e))?;
    todo!("day 00 - part 1");
}

fn rules(input: &str) -> IResult<&str, HashMap<u32, Vec<u32>>> {
    fold_many1(
        terminated(
            separated_pair(complete::u32, tag("|"), complete::u32),
            line_ending,
        ),
        HashMap::default,
        |mut acc: HashMap<u32, Vec<u32>>, (page, after)| {
            acc.entry(page)
                .and_modify(|afters| {
                    afters.push(after);
                })
                .or_insert(vec![after]);
            acc
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, (HashMap<u32, Vec<u32>>, Vec<Vec<u32>>)> {
    let (input, parsed_rules) = rules(input)?;
    dbg!(input, parsed_rules);
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";
        assert_eq!("143", process(input)?);
        Ok(())
    }
}
