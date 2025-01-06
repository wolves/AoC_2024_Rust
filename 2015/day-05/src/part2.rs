pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let result = input
        .lines()
        .filter(|line| {
            line.as_bytes()
                .windows(2)
                .enumerate()
                .any(|(i, current_pair)| {
                    line.as_bytes()
                        .windows(2)
                        .skip(i + 2)
                        .any(|next_pair| current_pair == next_pair)
                })
        })
        .filter(|line| line.as_bytes().windows(3).any(|win| win[0] == win[2]))
        .count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("qjhvhtzxzqqjkmpb", "1")]
    #[case("xxyxx", "1")]
    #[case("uurcxstgmygtbstg", "0")]
    #[case("ieodomkazucvgmuy", "0")]
    #[case(
        "qjhvhtzxzqqjkmpb
xxyxx
uurcxstgmygtbstg
ieodomkazucvgmuy",
        "2"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
