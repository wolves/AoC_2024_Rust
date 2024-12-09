use std::collections::VecDeque;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let nums: Vec<_> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let count = c.to_digit(10).unwrap() as usize;

            if i % 2 != 0 {
                (count, None)
            } else {
                (count, Some(i / 2))
            }
        })
        .collect();

    let mut queue: VecDeque<_> = nums
        .iter()
        .flat_map(|(count, digit)| digit.map(|d| std::iter::repeat(d).take(*count)))
        .flatten()
        .collect();

    let mut fixed = vec![];
    for number in nums.iter() {
        for _ in 0..number.0 {
            let d = if number.1.is_some() {
                queue.pop_front()
            } else {
                queue.pop_back()
            };

            if let Some(d) = d {
                fixed.push(d);
            } else {
                break;
            }
        }
    }

    let result: usize = fixed.iter().enumerate().map(|(i, d)| i * d).sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("1928", process(input)?);
        Ok(())
    }
}
