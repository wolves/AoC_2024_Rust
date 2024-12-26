use std::{collections::HashMap, iter::successors};

use itertools::Itertools;
use miette::IntoDiagnostic;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let hashmap = input.lines().fold(
        HashMap::<[i32; 4], usize>::new(),
        |mut map, line| {
            let inner_map = cost_and_change(line)
                .unwrap()
                .take(2000)
                .tuple_windows()
                .fold(
                    HashMap::<[i32; 4], usize>::with_capacity(1996),
                    |mut map, (a,b, c,d)| {
                        let key = [a.1, b.1, c.1, d.1];
                        map.entry(key).or_insert(d.0);
                        map
                    },
                    );

            for (key, inner_value) in inner_map.into_iter() {
                map.entry(key)
                    .and_modify(|value| {
                        *value += inner_value;
                    })
                .or_insert(inner_value);
            }
            map
        },
    );

    let result: &usize = hashmap.values().max().unwrap();

    Ok(result.to_string())
}

fn cost_and_change(
    secret: &str,
) -> miette::Result<impl Iterator<Item = (usize, i32)>> {
    Ok(process_secret(secret)
        .unwrap()
        .map(|num| num % 10)
        .tuple_windows()
        .map(|(a, b)| (b, b as i32 - a as i32)))
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

    #[test]
    fn test_cost_and_change() {
        let input = "123";
        let output: Vec<(usize, i32)> = vec![
            // (123, 3),
            (0, -3),
            (6, 6),
            (5, -1),
            (4, -1),
            (4, 0),
            (6, 2),
            (4, -2),
            (4, 0),
            (2, -2),
        ];
        assert_eq!(
            output,
            cost_and_change(input)
                .unwrap()
                .take(9) // maybe 10th according to puzzle?
                .collect::<Vec<(usize, i32)>>(),
        )
    }
}
