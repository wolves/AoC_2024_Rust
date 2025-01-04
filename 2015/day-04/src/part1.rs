pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut buffer = String::with_capacity(input.len() + 20);
    let mut i = 0;

    let result = loop {
        i += 1;

        buffer.clear();
        buffer.push_str(input);
        buffer.push_str(&i.to_string());

        let hash = md5::compute(buffer.as_bytes());

        if format!("{:x}", hash).as_bytes().starts_with(b"00000") {
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
