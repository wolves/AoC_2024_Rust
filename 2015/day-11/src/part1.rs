pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let result = next_password(input);

    Ok(result.unwrap())
}

fn next_password(input: &str) -> miette::Result<String> {
    let mut password = Password::from_str(input).unwrap();

    loop {
        for pos in 0..8 {
            password.skip_forbidden(pos);
        }

        if !password.is_valid() {
            password.increment();
        } else {
            break;
        }
    }

    Ok(password.to_string())
}

#[derive(Debug, PartialEq, Eq)]
struct Password([u8; 8]);

impl Password {
    fn from_str(s: &str) -> miette::Result<Self, &'static str> {
        if s.len() != 8 || !s.chars().all(|c| c.is_ascii_lowercase()) {
            return Err("Password must be 8 lowercase ASCII characters");
        }

        let bytes: Vec<u8> = s.bytes().collect();

        Ok(Password(bytes.try_into().unwrap()))
    }

    fn to_string(&self) -> String {
        String::from_utf8(self.0.to_vec()).unwrap()
    }

    // RULES
    // Rule 1: Must include one increasing straight of at least three letters
    fn has_increasing_straight(&self) -> bool {
        // self.0.windows(3).any(|window| {
        //     window[0].wrapping_add(1) == window[1] && window[1].wrapping_add(1) == window[2]
        // })
        for i in 0..6 {
            if self.0[i] + 1 == self.0[i + 1] && self.0[i + 1] + 1 == self.0[i + 2] {
                return true;
            }
        }
        false
    }
    // Rule 2: Cannot contain i, o, or l
    fn has_forbidden_letters(&self) -> bool {
        self.0.iter().any(|&b| b == b'i' || b == b'o' || b == b'l')
    }
    // Rule 3: Must contain at least two different, non-overlapping pairs
    fn has_two_pairs(&self) -> bool {
        let mut pairs = 0;
        let mut i = 0;
        while i < 7 {
            if self.0[i] == self.0[i + 1] {
                pairs += 1;
                i += 2;
            } else {
                i += 1;
            }
        }
        pairs >= 2
    }

    fn is_valid(&self) -> bool {
        self.has_increasing_straight() && !self.has_forbidden_letters() && self.has_two_pairs()
    }

    fn increment(&mut self) {
        for i in (0..8).rev() {
            let current = self.0[i];

            self.0[i] = if current == b'z' { b'a' } else { current + 1 };

            if current != b'z' {
                break;
            }
        }
    }

    fn skip_forbidden(&mut self, position: usize) {
        if self.0[position] == b'i' {
            self.0[position] = b'j';
            for i in (position + 1)..8 {
                self.0[i] = b'a';
            }
        } else if self.0[position] == b'l' {
            self.0[position] = b'm';
            for i in (position + 1)..8 {
                self.0[i] = b'a';
            }
        } else if self.0[position] == b'o' {
            self.0[position] = b'p';
            for i in (position + 1)..8 {
                self.0[i] = b'a';
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case("abcdefgh", "abcdffaa")]
    #[case("ghijklmn", "ghjaabcc")]
    fn test_next_password(#[case] input: &str, #[case] expected: &str) -> miette::Result<()> {
        assert_eq!(next_password(input)?, expected);
        Ok(())
    }

    #[rstest]
    #[case("hijklmmn", false)]
    #[case("abbceffg", false)]
    #[case("abbcegjk", false)]
    #[case("abcdffaa", true)]
    fn test_is_valid_password(#[case] input: &str, #[case] expected: bool) -> miette::Result<()> {
        let password = Password::from_str(input).unwrap();
        assert_eq!(password.is_valid(), expected);
        Ok(())
    }

    #[rstest]
    #[case("abc", true)]
    #[case("xyz", true)]
    #[case("abd", false)]
    fn test_has_increasing_straight(#[case] input: &str, #[case] expected: bool) {
        let bytes = input.bytes().collect::<Vec<_>>();
        let mut arr = [0u8; 8];
        arr[..bytes.len()].copy_from_slice(&bytes);
        let password = Password(arr);

        assert_eq!(password.has_increasing_straight(), expected);
    }

    #[rstest]
    #[case("aabb", true)]
    #[case("aaaa", true)]
    #[case("aaca", false)]
    fn test_has_two_pairs(#[case] input: &str, #[case] expected: bool) {
        let bytes = input.bytes().collect::<Vec<_>>();
        let mut arr: [u8; 8] = (0..=7).collect::<Vec<u8>>().try_into().unwrap();
        arr[..bytes.len()].copy_from_slice(&bytes);
        let password = Password(arr);

        assert_eq!(password.has_two_pairs(), expected);
    }

    #[test]
    fn test_increment() {
        let mut password = Password::from_str("aaaaaaaa").unwrap();
        password.increment();
        assert_eq!(password.to_string(), "aaaaaaab");

        let mut password = Password::from_str("aaaaaaaz").unwrap();
        password.increment();
        assert_eq!(password.to_string(), "aaaaaaba");
    }
}
