pub fn process(input: &str) -> miette::Result<String> {
    let data = input.trim();

    let nums = data
        .split_ascii_whitespace()
        .map(|n| n.parse::<u64>().expect("numbers to be valid in AoC"))
        .collect::<Vec<u64>>();

    let mut all_iterations = std::iter::successors(Some(nums), |nums| {
        let iter_nums: Vec<u64> = nums
            .iter()
            .flat_map(|num| match num {
                0 => vec![1],
                n if (n.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 => {
                    let (a, b) = split_num_digits(*n);
                    vec![a, b]
                }
                _ => vec![num * 2024],
            })
            .collect();
        Some(iter_nums)
    });

    let result = all_iterations.nth(25).unwrap().len();

    Ok(result.to_string())
}

fn split_num_digits(num: u64) -> (u64, u64) {
    let len = (num as f64).log10().floor() as u32 + 1;
    let divisor = 10u64.pow(len / 2);

    (num / divisor, num % divisor)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "125 17";
        assert_eq!("55312", process(input)?);
        Ok(())
    }
}
