pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    todo!("day 00 - part 1");
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
