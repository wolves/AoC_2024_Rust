use std::iter::successors;

use miette::IntoDiagnostic;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let result: usize = input
        .lines()
        .map(|line| {
            process_secret(line).unwrap().nth(2000).unwrap()
        })
        .sum();

    Ok(result.to_string())
}
fn process_secret(
    secret: &str,
) -> miette::Result<impl Iterator<Item = usize>> {
    let secret =
        secret.parse::<usize>().into_diagnostic()?;

    Ok(successors(Some(secret), |secret| {
        let value = secret * 64;
        let secret = prune(mix(*secret, value));

        let value = secret / 32;
        let secret = prune(mix(secret, value));

        let value = secret * 2048;
        let secret = prune(mix(secret, value));

        Some(secret)
    }))
}

fn mix(secret: usize, value: usize) -> usize {
    secret ^ value
}

fn prune(secret: usize) -> usize {
    secret.rem_euclid(16777216)
}

#[cfg(test)]
mod tests {
    use super::*;

    use rstest::rstest;

    #[rstest]
    #[case("1", "8685429")]
    #[case("10", "4700978")]
    #[case("100", "15273692")]
    #[case("2024", "8667524")]
    fn test_process(
        #[case] input: &str,
        #[case] output: &str,
    ) -> miette::Result<()> {
        assert_eq!(
            output,
            process_secret(input)?
                .nth(2000)
                .unwrap()
                .to_string()
        );
        Ok(())
    }

    #[test]
    fn test_mix() {
        assert_eq!(37, mix(42, 15));
    }
    #[test]
    fn test_prune() {
        assert_eq!(16113920, prune(100000000));
    }
}
