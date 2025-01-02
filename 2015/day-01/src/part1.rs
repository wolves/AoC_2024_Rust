pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut count = 0;
    for c in input.chars() {
        match c {
            '(' => count += 1,
            ')' => count -= 1,
            _ => unreachable!(),
        }
    }

    Ok(count.to_string())
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
