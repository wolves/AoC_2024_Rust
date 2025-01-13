use nom::{
    bytes::complete::tag,
    character::complete::{self, alpha1, line_ending},
    combinator::map,
    multi::separated_list1,
    sequence::{delimited, preceded, terminated, tuple},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, ingredients) = parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    dbg!(&ingredients);

    let result = "";

    Ok(result.to_string())
}

#[derive(Debug, PartialEq)]
struct Ingredient {
    name: String,
    capacity: i32,
    durability: i32,
    flavor: i32,
    texture: i32,
    calories: i32,
}

fn parse_ingredient(input: &str) -> IResult<&str, Ingredient> {
    map(
        tuple((
            map(terminated(alpha1, tag(":")), |n: &str| n.to_string()),
            delimited(tag(" capacity "), complete::i32, tag(",")),
            delimited(tag(" durability "), complete::i32, tag(",")),
            delimited(tag(" flavor "), complete::i32, tag(",")),
            delimited(tag(" texture "), complete::i32, tag(",")),
            preceded(tag(" calories "), complete::i32),
        )),
        |(name, capacity, durability, flavor, texture, calories)| Ingredient {
            name,
            capacity,
            durability,
            flavor,
            texture,
            calories,
        },
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Ingredient>> {
    separated_list1(line_ending, parse_ingredient)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Butterscotch: capacity -1, durability -2, flavor 6, texture 3, calories 8
Cinnamon: capacity 2, durability 3, flavor -2, texture -1, calories 3",
        "62842880"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }

    #[rstest]
    #[case(
        "Sprinkles: capacity 2, durability 0, flavor -2, texture 0, calories 3",
        Ingredient { name: "Sprinkles".to_string(), capacity: 2, durability: 0, flavor: -2, texture: 0, calories: 3 },
    )]
    #[case(
        "Butterscotch: capacity 0, durability 5, flavor -3, texture 0, calories 3",
        Ingredient { name: "Butterscotch".to_string(), capacity: 0, durability: 5, flavor: -3, texture: 0, calories: 3 },
    )]
    fn test_parse_ingredient(
        #[case] input: &str,
        #[case] expected: Ingredient,
    ) -> miette::Result<()> {
        let (_, result) = parse_ingredient(input).unwrap();
        assert_eq!(expected, result);
        Ok(())
    }
}
