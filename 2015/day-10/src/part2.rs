use std::iter::from_fn;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let result = look_and_say(input, 50).unwrap().len();

    Ok(result.to_string())
}

fn look_and_say(input: &str, iterations: usize) -> miette::Result<String> {
    let mut current = input.to_string();

    for _ in 0..iterations {
        current = generate_string(&current)?;
    }

    Ok(current)
}

fn generate_string(input: &str) -> miette::Result<String> {
    let mut result = String::with_capacity(input.len() * 2);
    let mut buffer = itoa::Buffer::new();

    for (digit, count) in consecutive_groups(input) {
        result.push_str(buffer.format(count));
        result.push(digit);
    }

    Ok(result)
}

fn consecutive_groups(input: &str) -> impl Iterator<Item = (char, usize)> + '_ {
    let mut chars = input.chars().peekable();

    from_fn(move || {
        chars.next().map(|current| {
            let mut count = 1;

            while let Some(next) = chars.peek() {
                if *next == current {
                    count += 1;
                    chars.next();
                } else {
                    break;
                }
            }

            (current, count)
        })
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("1", "11")]
    #[case("11", "21")]
    #[case("21", "1211")]
    #[case("1211", "111221")]
    #[case("111221", "312211")]
    fn test_generate_string(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, generate_string(input)?);
        Ok(())
    }

    #[test]
    fn test_consecutive_groups() {
        let input = "111221";
        let result: Vec<(char, usize)> = consecutive_groups(input).collect();

        assert_eq!(result, vec![('1', 3), ('2', 2), ('1', 1),]);
    }
    #[test]
    fn test_look_and_say_multiple() {
        assert_eq!(look_and_say("1", 4).unwrap(), "111221");
        // "1" -> "11" -> "21" -> "1211" -> "111221"
    }
}



