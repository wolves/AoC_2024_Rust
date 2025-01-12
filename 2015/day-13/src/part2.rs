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

    let mut seating = SeatingArrangement::from_relationships(relationships);
    seating.add_self();

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
    fn add_self(&mut self) {
        let me = "Me".to_string();

        for person in &self.people.clone() {
            self.relationships.insert((me.clone(), person.clone()), 0);
            self.relationships.insert((person.clone(), me.clone()), 0);
        }

        self.people.push(me);
    }

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

    #[test]
    fn test_add_self() {
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

        let mut arrangement = SeatingArrangement::from_relationships(relationships);
        arrangement.add_self();

        assert!(arrangement.people.contains(&"Me".to_string()));

        assert_eq!(arrangement.get_happiness("Me", "Alice"), 0);
        assert_eq!(arrangement.get_happiness("Alice", "Me"), 0);
        assert_eq!(arrangement.get_happiness("Me", "Bob"), 0);
        assert_eq!(arrangement.get_happiness("Bob", "Me"), 0);
    }
}
