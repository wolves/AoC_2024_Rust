use itertools::Itertools;

type Report = Vec<i32>;

enum Direction {
    Increasing,
    Decreasing,
}

pub fn process(input: &str) -> miette::Result<String> {
    let reports = parse_reports(input);

    let result = reports
        .iter()
        .filter(|report| check_safety(report).is_ok())
        .count();

    Ok(result.to_string())
}

fn parse_reports(input: &str) -> Vec<Report> {
    input
        .lines()
        .map(|x| {
            x.split_whitespace()
                .map(|x| x.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>()
}

fn check_safety(input: &Report) -> Result<(), String> {
    let mut direction: Option<Direction> = None;

    for (a, b) in input.iter().tuple_windows() {
        let diff = a - b;
        match diff.signum() {
            -1 => match direction {
                Some(Direction::Increasing) => {
                    return Err(format!("{}, {} switched to increasing", a, b));
                }
                Some(Direction::Decreasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{}, {} diff value is {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{}, {} diff value is {}", a, b, diff.abs()));
                    } else {
                        direction = Some(Direction::Decreasing);
                        continue;
                    }
                }
            },
            1 => match direction {
                Some(Direction::Increasing) => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{}, {} diff value is {}", a, b, diff.abs()));
                    } else {
                        continue;
                    }
                }
                Some(Direction::Decreasing) => {
                    return Err(format!("{}, {} switched to decreasing", a, b));
                }
                None => {
                    if !(1..=3).contains(&diff.abs()) {
                        return Err(format!("{}, {} diff value is {}", a, b, diff.abs()));
                    } else {
                        direction = Some(Direction::Increasing);
                        continue;
                    }
                }
            },
            0 => {
                return Err(format!("{}, {} diff was 0", a, b));
            }
            _ => {
                panic!("Should never have a non -1, 1, 0 number");
            }
        }
    }

    Ok(())
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
