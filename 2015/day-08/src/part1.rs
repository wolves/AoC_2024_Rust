pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    dbg!(input, input.len());
    todo!("day 00 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(r#""""#, 2, 0)]
    #[case(r#""abc""#, 5, 3)]
    #[case(r#""aaa\"aaa""#, 10, 7)]
    #[case(r#""\x27""#, 6, 1)]
    fn test_string_processing(
        #[case] input: &str,
        #[case] code_len: usize,
        #[case] mem_len: usize,
    ) {
        let result = process_string(input);
        assert_eq!(result.code_chars, code_len);
        assert_eq!(result.memory_chars, mem_len);
    }

    #[test]
    fn test_total_diff() {
        let input = vec![r#""""#, r#""abc""#, r#""aaa\"aaa""#, r#""\x27""#];

        assert_eq!(calc_diff(&input), 12)
    }
    // fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
    //     assert_eq!(result, process(input)?);
    //     Ok(())
    // }
}
