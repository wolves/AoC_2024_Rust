use cgmath::Vector2;
use regex::Regex;

type Vec2 = Vector2<isize>;

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

pub fn process(input: &str) -> miette::Result<String> {
    let data = input.trim();

    let data: Vec<_> = data
        .lines()
        .map(|line| {
            let re = Regex::new(r"(-?\d+).*?(-?\d+).*?(-?\d+).*?(-?\d+)").unwrap();
            let capture = re.captures_iter(line).next().unwrap();
            (
                Vec2::new(capture[1].parse().unwrap(), capture[2].parse().unwrap()),
                Vec2::new(capture[3].parse().unwrap(), capture[4].parse().unwrap()),
            )
        })
        .collect();

    let data: Vec<_> = data
        .iter()
        .map(|r| {
            let p = r.0 + r.1 * 100;
            Vec2::new(p.x.rem_euclid(WIDTH), p.y.rem_euclid(HEIGHT))
        })
        .collect();

    let data: Vec<_> = data
        .iter()
        .flat_map(|p| {
            if p.x == WIDTH / 2 || p.y == HEIGHT / 2 {
                None
            } else if p.x < WIDTH / 2 {
                if p.y < HEIGHT / 2 {
                    Some(0)
                } else {
                    Some(1)
                }
            } else if p.y < HEIGHT / 2 {
                Some(2)
            } else {
                Some(3)
            }
        })
        .collect();

    let result = data.iter().filter(|&&i| i == 0).count()
        * data.iter().filter(|&&i| i == 1).count()
        * data.iter().filter(|&&i| i == 2).count()
        * data.iter().filter(|&&i| i == 3).count();

    Ok(result.to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";
        assert_eq!("21", process(input)?);
        Ok(())
    }
}
