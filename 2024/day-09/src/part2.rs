use std::collections::VecDeque;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let mut files: Vec<_> = input
        .chars()
        .enumerate()
        .map(|(i, c)| {
            let count = c.to_digit(10).unwrap() as usize;

            if i % 2 != 0 {
                (count, None, false)
            } else {
                (count, Some(i / 2), false)
            }
        })
        .collect();

    for i in (0..files.len()).rev() {
        if files[i].1.is_none() || files[i].2 {
            continue;
        }

        for j in 0..i {
            if files[j].1.is_some() || files[j].0 < files[i].0 {
                continue;
            }

            let diff = files[j].0 - files[i].0;

            files[j] = (files[i].0, files[i].1.take(), true);

            if diff > 0 {
                files.insert(j + 1, (diff, None, true));
            }

            break;
        }
    }

    let fixed: Vec<_> = files
        .iter()
        .flat_map(|(count, digit, _)| std::iter::repeat(digit).take(*count))
        .collect();

    let result: usize = fixed
        .iter()
        .enumerate()
        .filter_map(|(i, d)| d.map(|d| i * d))
        .sum();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "2333133121414131402";
        assert_eq!("2858", process(input)?);
        Ok(())
    }
}
