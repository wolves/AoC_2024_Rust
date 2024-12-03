pub fn process(input: &str) -> miette::Result<String> {
    let reports = parse_reports(input);

    let result = reports.iter().filter(|x| is_safe_report(x)).count();

    Ok(result.to_string())
}

fn parse_reports(input: &str) -> Vec<Vec<i32>> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn is_safe_report(input: &[i32]) -> bool {
    let sig = (input[0] - input[1]).signum();

    input
        .windows(2)
        .map(|w| w[0] - w[1])
        .all(|x| (1..=3).contains(&x.abs()) && x.signum() == sig)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";
        assert_eq!("2", process(input)?);
        Ok(())
    }
}
