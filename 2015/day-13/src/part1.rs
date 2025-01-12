use std::collections::{HashMap, HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alpha1, digit1, line_ending},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{terminated, tuple},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, relationships) =
        parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    let seating = SeatingArrangement::from_relationships(relationships);

    let result = seating.find_optimal_happiness();
    Ok(result.to_string())
}

#[derive(Debug, PartialEq)]
struct Relationship {
    person: String,
    neighbor: String,
    happiness: i32,
}

#[derive(Debug)]
struct SeatingArrangement {
    relationships: HashMap<(String, String), i32>,
    people: Vec<String>,
}

impl SeatingArrangement {
    fn from_relationships(relationships: Vec<Relationship>) -> Self {
        let mut happiness_map = HashMap::new();
        let mut unique_people = HashSet::new();

        for rel in relationships {
            happiness_map.insert((rel.person.clone(), rel.neighbor.clone()), rel.happiness);
            unique_people.insert(rel.person);
            unique_people.insert(rel.neighbor);
        }

        SeatingArrangement {
            relationships: happiness_map,
            people: unique_people.into_iter().collect(),
        }
    }

    fn get_happiness(&self, person1: &str, person2: &str) -> i32 {
        self.relationships
            .get(&(person1.to_string(), person2.to_string()))
            .copied()
            .unwrap_or(0)
    }

    fn generate_arrangements(&self) -> Vec<Vec<String>> {
        if self.people.is_empty() {
            return vec![];
        }

        let first_person = &self.people[0];
        let mut others: Vec<String> = self.people[1..].to_vec();
        let mut arrangements = Vec::new();

        let others_size = &others.len();
        generate_permutations(&mut others, others_size, &mut arrangements);

        arrangements.iter_mut().for_each(|arr| {
            arr.insert(0, first_person.clone());
        });

        arrangements
    }

    fn calculate_happiness(&self, arrangement: &[String]) -> i32 {
        if arrangement.len() < 2 {
            return 0;
        }

        let len = arrangement.len();
        let mut total = 0;

        for i in 0..len {
            let current = &arrangement[i];
            let next = &arrangement[(i + 1) % len];

            total += self.get_happiness(current, next);
            total += self.get_happiness(next, current);
        }

        if arrangement.len() == 2 {
            return total / 2;
        }

        total
    }

    fn find_optimal_happiness(&self) -> i32 {
        let arrangements = self.generate_arrangements();

        arrangements
            .iter()
            .map(|arrangement| self.calculate_happiness(arrangement))
            .max()
            .unwrap_or(0)
    }
}
fn generate_permutations(arr: &mut Vec<String>, n: &usize, results: &mut Vec<Vec<String>>) {
    if *n == 1 {
        results.push(arr.clone());
        return;
    }

    for i in 0..*n {
        let next = *n - 1;
        generate_permutations(arr, &next, results);
        if n % 2 == 0 {
            arr.swap(i, next);
        } else {
            arr.swap(0, next);
        }
    }
}

fn parse_relationship(input: &str) -> IResult<&str, Relationship> {
    let (input, (person, gain_loss_val, happiness_delta, _, neighbor)) = tuple((
        map(alpha1, |s: &str| s.to_string()),
        map(
            tuple((
                tag(" would "),
                alt((value(1, tag("gain")), value(-1, tag("lose")))),
                tag(" "),
            )),
            |(_, multiplier, _)| multiplier,
        ),
        map_res(digit1, |s: &str| s.parse::<i32>()),
        tag(" happiness units by sitting next to "),
        terminated(map(alpha1, |s: &str| s.to_string()), complete::char('.')),
    ))(input)?;

    Ok((
        input,
        Relationship {
            person,
            neighbor,
            happiness: happiness_delta * gain_loss_val,
        },
    ))
}

fn parse(input: &str) -> IResult<&str, Vec<Relationship>> {
    separated_list1(line_ending, parse_relationship)(input)
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "Alice would gain 54 happiness units by sitting next to Bob.
Alice would lose 79 happiness units by sitting next to Carol.
Alice would lose 2 happiness units by sitting next to David.
Bob would gain 83 happiness units by sitting next to Alice.
Bob would lose 7 happiness units by sitting next to Carol.
Bob would lose 63 happiness units by sitting next to David.
Carol would lose 62 happiness units by sitting next to Alice.
Carol would gain 60 happiness units by sitting next to Bob.
Carol would gain 55 happiness units by sitting next to David.
David would gain 46 happiness units by sitting next to Alice.
David would lose 7 happiness units by sitting next to Bob.
David would gain 41 happiness units by sitting next to Carol.",
        "330"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }

    #[test]
    fn test_parse_relationship_gain() {
        let input = "Alice would gain 54 happiness units by sitting next to Bob.";
        let (_, relationship) = parse_relationship(input).unwrap();
        assert_eq!(
            relationship,
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Bob".to_string(),
                happiness: 54,
            }
        );
    }

    #[test]
    fn test_parse_relationship_lose() {
        let input = "Alice would lose 79 happiness units by sitting next to Carol.";
        let (_, relationship) = parse_relationship(input).unwrap();
        assert_eq!(
            relationship,
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Carol".to_string(),
                happiness: -79,
            }
        );
    }

    #[test]
    fn test_seating_arrangement_creation() {
        let relationships = vec![
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Bob".to_string(),
                happiness: 54,
            },
            Relationship {
                person: "Bob".to_string(),
                neighbor: "Alice".to_string(),
                happiness: 83,
            },
        ];

        let arrangement = SeatingArrangement::from_relationships(relationships);

        assert_eq!(arrangement.get_happiness("Alice", "Bob"), 54);
        assert_eq!(arrangement.get_happiness("Bob", "Alice"), 83);
        assert_eq!(arrangement.people.len(), 2);
    }

    #[test]
    fn test_arrangement_generation() {
        let relationships = vec![
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Bob".to_string(),
                happiness: 54,
            },
            Relationship {
                person: "Bob".to_string(),
                neighbor: "Carol".to_string(),
                happiness: 83,
            },
            Relationship {
                person: "Carol".to_string(),
                neighbor: "Alice".to_string(),
                happiness: 42,
            },
        ];

        let arrangement = SeatingArrangement::from_relationships(relationships);
        let arrangements = arrangement.generate_arrangements();

        assert_eq!(arrangements.len(), 2);

        for arr in arrangements {
            assert_eq!(arr.len(), 3);
            assert!(arr.contains(&"Alice".to_string()));
            assert!(arr.contains(&"Bob".to_string()));
            assert!(arr.contains(&"Carol".to_string()));
        }
    }

    #[test]
    fn test_happiness_calc() {
        let relationships = vec![
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Bob".to_string(),
                happiness: 54,
            },
            Relationship {
                person: "Bob".to_string(),
                neighbor: "Alice".to_string(),
                happiness: 83,
            },
        ];

        let arrangement = SeatingArrangement::from_relationships(relationships);
        let happiness = arrangement.calculate_happiness(&["Alice".to_string(), "Bob".to_string()]);

        assert_eq!(happiness, 137);
    }

    #[test]
    fn test_happiness_calculation_three_people() {
        let relationships = vec![
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Bob".to_string(),
                happiness: 54,
            },
            Relationship {
                person: "Bob".to_string(),
                neighbor: "Alice".to_string(),
                happiness: 83,
            },
            Relationship {
                person: "Bob".to_string(),
                neighbor: "Carol".to_string(),
                happiness: 20,
            },
            Relationship {
                person: "Carol".to_string(),
                neighbor: "Bob".to_string(),
                happiness: 10,
            },
            Relationship {
                person: "Carol".to_string(),
                neighbor: "Alice".to_string(),
                happiness: 30,
            },
            Relationship {
                person: "Alice".to_string(),
                neighbor: "Carol".to_string(),
                happiness: 40,
            },
        ];

        let arrangement = SeatingArrangement::from_relationships(relationships);
        let happiness = arrangement.calculate_happiness(&[
            "Alice".to_string(),
            "Bob".to_string(),
            "Carol".to_string(),
        ]);

        // For arrangement Alice -> Bob -> Carol -> Alice:
        // Alice next to Bob: 54 + 83
        // Bob next to Carol: 20 + 10
        // Carol next to Alice: 30 + 40
        // Total: 237
        assert_eq!(happiness, 237);
    }
}
