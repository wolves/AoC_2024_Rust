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

    let result = find_best_score_with_calories(&ingredients);

    Ok(result.to_string())
}

fn calculate_total_calories(ingredients: &[Ingredient], quantities: &[i32]) -> i32 {
    ingredients
        .iter()
        .zip(quantities.iter())
        .map(|(ingredient, &quantity)| ingredient.calculate_calories(quantity))
        .sum()
}

fn find_best_score_with_calories(ingredients: &[Ingredient]) -> i32 {
    let mut max_score = 0;

    match ingredients.len() {
        2 => {
            for i in 0..=100 {
                let quantities = vec![i, 100 - i];
                if calculate_total_calories(ingredients, &quantities) == 500 {
                    let score = calculate_score(ingredients, &quantities);
                    max_score = max_score.max(score);
                }
            }
        }
        3 => {
            for i in 0..=100 {
                for j in 0..=(100 - i) {
                    let k = 100 - i - j;
                    let quantities = vec![i, j, k];
                    if calculate_total_calories(ingredients, &quantities) == 500 {
                        let score = calculate_score(ingredients, &quantities);
                        max_score = max_score.max(score);
                    }
                }
            }
        }
        4 => {
            for i in 0..=100 {
                for j in 0..=(100 - i) {
                    for k in 0..=(100 - i - j) {
                        let l = 100 - i - j - k;
                        let quantities = vec![i, j, k, l];
                        if calculate_total_calories(ingredients, &quantities) == 500 {
                            let score = calculate_score(ingredients, &quantities);
                            max_score = max_score.max(score);
                        }
                    }
                }
            }
        }
        _ => panic!("Unexpected number of ingredients"),
    }

    max_score
}

fn calculate_score(ingredients: &[Ingredient], quantities: &[i32]) -> i32 {
    let prop_totals = ingredients
        .iter()
        .zip(quantities.iter())
        .map(|(ingredient, &quantity)| ingredient.calculate_property_total(quantity))
        .fold((0, 0, 0, 0), |acc, x| {
            (acc.0 + x.0, acc.1 + x.1, acc.2 + x.2, acc.3 + x.3)
        });

    let (capacity, durability, flavor, texture) = prop_totals;
    capacity.max(0) * durability.max(0) * flavor.max(0) * texture.max(0)
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

impl Ingredient {
    fn calculate_property_total(&self, quantity: i32) -> (i32, i32, i32, i32) {
        (
            self.capacity * quantity,
            self.durability * quantity,
            self.flavor * quantity,
            self.texture * quantity,
        )
    }

    fn calculate_calories(&self, quantity: i32) -> i32 {
        self.calories * quantity
    }
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
        "57600000"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
