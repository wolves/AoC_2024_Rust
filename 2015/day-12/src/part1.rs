pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    todo!("day 00 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("[1,2,3]", "6")]
    #[case(r#"{"a":2,"b":4}"#, "6")]
    #[case("[[[3]]]", "3")]
    #[case(r#"{"a":{"b":4},"c":-1}"#, "3")]
    #[case(r#"{"a":[-1,1]}"#, "0")]
    #[case(r#"[-1,{"a":1}]"#, "0")]
    #[case("[]", "0")]
    #[case("{}", "0")]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
