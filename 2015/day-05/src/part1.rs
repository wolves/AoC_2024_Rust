use itertools::Itertools;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let result = input
        .lines()
        .filter(|line| {
            let vowel_count = line
                .chars()
                .filter(|c| matches!(c, 'a' | 'e' | 'i' | 'o' | 'u'))
                .count();
            vowel_count >= 3
        })
        .filter(|line| line.chars().tuple_windows().any(|(a, b)| a == b))
        .filter(|line| {
            line.chars()
                .tuple_windows()
                .all(|(a, b)| !matches!((a, b), ('a', 'b') | ('c', 'd') | ('p', 'q') | ('x', 'y')))
        })
        .count();

    Ok(result.to_string())
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
