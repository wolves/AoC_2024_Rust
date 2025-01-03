pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    todo!("day 00 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdef", "609043")]
    #[case("pqrstuv", "1048970")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
