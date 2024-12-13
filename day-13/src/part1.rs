use cgmath::Vector2;
use regex::Regex;

type Vec2 = Vector2<isize>;

struct Equation {
    a: Vec2,
    b: Vec2,
    prize: Vec2,
}
impl Equation {
    fn count_tokens(&self) -> Option<usize> {
        let px = self.prize.x;
        let py = self.prize.y;

        let ax = self.a.x;
        let ay = self.a.y;

        let bx = self.b.x;
        let by = self.b.y;

        let a_btn_presses = (by * px - bx * py) / (ax * by - ay * bx);
        let b_btn_presses = (ay * px - ax * py) / (ay * bx - ax * by);

        if px == a_btn_presses * ax + b_btn_presses * bx
            && py == a_btn_presses * ay + b_btn_presses * by
        {
            Some((3 * a_btn_presses + b_btn_presses) as usize)
        } else {
            None
        }
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let data = input.trim();

    let data: Vec<_> = data.lines().collect();
    let data: Vec<_> = data.split(|line| line.is_empty()).collect();

    let data: Vec<_> = data
        .iter()
        .map(|lines| {
            let mut iter = lines.iter();
            let a = iter.next().map(|s| str_to_vec2(s)).unwrap();
            let b = iter.next().map(|s| str_to_vec2(s)).unwrap();
            let prize = iter.next().map(|s| str_to_vec2(s)).unwrap();
            Equation { a, b, prize }
        })
        .collect();

    let sum: usize = data.iter().filter_map(|e| e.count_tokens()).sum();
    Ok(sum.to_string())
}

fn str_to_vec2(s: &str) -> Vec2 {
    let re = Regex::new(r"(\d+).*?(\d+)").unwrap();
    let capture = re.captures_iter(s).next().unwrap();
    Vec2::new(capture[1].parse().unwrap(), capture[2].parse().unwrap())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";
        assert_eq!("480", process(input)?);
        Ok(())
    }
}
