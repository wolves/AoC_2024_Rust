use std::collections::HashMap;

pub fn process(input: &str) -> miette::Result<String> {
    let data = input.trim();

    let nums = data
        .split_whitespace()
        .map(|n| n.parse::<usize>().expect("numbers to be valid in AoC"))
        .collect::<Vec<usize>>();

    let mut mem = HashMap::new();

    let result: usize = nums
        .iter()
        .map(|&num| get_stone_count(num, 75, &mut mem))
        .sum();

    Ok(result.to_string())
}

fn split_num_digits(num: usize) -> (usize, usize) {
    let len = (num as f64).log10().floor() as u32 + 1;
    let divisor = 10u64.pow(len / 2);

    (num / divisor as usize, num % divisor as usize)
}

fn get_stone_count(num: usize, depth: usize, mem: &mut HashMap<(usize, usize), usize>) -> usize {
    let key = (num, depth);

    if mem.contains_key(&key) {
        *mem.get(&key).unwrap()
    } else {
        let result = if depth == 0 {
            1
        } else if num == 0 {
            get_stone_count(1, depth - 1, mem)
        } else if (num.checked_ilog10().unwrap_or(0) + 1) % 2 == 0 {
            let (a, b) = split_num_digits(num);
            get_stone_count(a, depth - 1, mem) + get_stone_count(b, depth - 1, mem)
        } else {
            get_stone_count(num * 2024, depth - 1, mem)
        };

        mem.insert(key, result);
        result
    }
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
