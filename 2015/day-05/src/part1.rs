pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    // At least 3 vowels
    // Contains at least one double letter
    // Does not contain ab, cd, pq, or xy
    todo!("day 00 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("ugknbfddgicrmopn", "1")]
    #[case("aaa", "1")]
    #[case("jchzalrnumimnmhp", "0")]
    #[case("haegwjzuvuyypxyu", "0")]
    #[case("dvszwmarrgswjxmb", "0")]
    #[case(
        "ugknbfddgicrmopn
aaa
jchzalrnumimnmhp
haegwjzuvuyypxyu
dvszwmarrgswjxmb
ooo",
        "3"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
