pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let result = input
        .chars()
        .enumerate()
        .scan(0, |count, (i, c)| {
            *count += match c {
                '(' => 1,
                ')' => -1,
                _ => unreachable!(),
            };
            Some((*count, i + 1))
        })
        .find(|(count, _)| *count == -1)
        .map(|(_, i)| i)
        .unwrap();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case(")", "1")]
    #[case("()())", "5")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
