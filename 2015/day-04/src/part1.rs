pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut i = 0;
    let result = loop {
        i += 1;

        let hash = md5::compute(format!("{input}{i}"));
        let hex = format!("{:x}", hash);

        if hex.starts_with("00000") {
            break i;
        }
    };

    Ok(result.to_string())
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
