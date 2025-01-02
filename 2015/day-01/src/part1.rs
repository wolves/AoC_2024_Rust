pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    todo!("day 00 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("(())", "0")]
    #[case("()()", "0")]
    #[case("(((", "3")]
    #[case("(()(()(", "3")]
    #[case("))(((((", "3")]
    #[case("())", "-1")]
    #[case("))(", "-1")]
    #[case(")))", "-3")]
    #[case(")())())", "-3")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
