use std::collections::HashMap;

use itertools::Itertools;
use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, newline},
    multi::separated_list1,
    sequence::tuple,
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let (_input, route_map) = parse(input).map_err(|e| miette::miette!("Parsing failed {e}"))?;

    route_map
        .find_path_length(false)
        .map(|d| d.to_string())
        .ok_or_else(|| miette::miette!("No valid path found"))
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct City<'a>(&'a str);

#[derive(Debug)]
struct Route {
    distance: u32,
}

#[derive(Debug, Default)]
struct RouteMap<'a> {
    routes: HashMap<City<'a>, HashMap<City<'a>, Route>>,
}

impl<'a> RouteMap<'a> {
    fn new() -> Self {
        Self {
            routes: HashMap::new(),
        }
    }

    fn add_route(&mut self, from: City<'a>, to: City<'a>, distance: u32) {
        self.routes
            .entry(from.clone())
            .or_default()
            .insert(to.clone(), Route { distance });

        self.routes
            .entry(to)
            .or_default()
            .insert(from, Route { distance });
    }

    fn get_distance(&self, from: &City<'a>, to: &City<'a>) -> Option<u32> {
        self.routes
            .get(from)
            .and_then(|routes| routes.get(to))
            .map(|route| route.distance)
    }

    fn cities(&self) -> impl Iterator<Item = &City<'a>> {
        self.routes.keys()
    }

    fn find_path_length(&self, min: bool) -> Option<u32> {
        let cities: Vec<&City> = self.cities().collect();

        cities
            .iter()
            .copied()
            .permutations(cities.len())
            .filter_map(|path| self.calculate_path_length(&path))
            .reduce(if min { u32::min } else { u32::max })
    }

    fn calculate_path_length(&self, path: &[&City<'a>]) -> Option<u32> {
        path.windows(2)
            .map(|cities| self.get_distance(cities[0], cities[1]))
            .sum()
    }
}

fn parse_city(input: &str) -> IResult<&str, City> {
    let (input, city) = alpha1(input)?;
    Ok((input, City(city)))
}

fn parse_route(input: &str) -> IResult<&str, (City, City, Route)> {
    let (input, (from, _, to, _, distance)) =
        tuple((parse_city, tag(" to "), parse_city, tag(" = "), digit1))(input)?;

    let distance = distance.parse().unwrap();
    Ok((input, (from, to, Route { distance })))
}

fn parse(input: &str) -> IResult<&str, RouteMap> {
    let (_input, routes) = separated_list1(newline, parse_route)(input)?;

    let mut route_map = RouteMap::new();

    routes
        .into_iter()
        .for_each(|(from, to, route)| route_map.add_route(from, to, route.distance));

    Ok((input, route_map))
}

#[cfg(test)]
mod tests {
    use super::*;
    use rstest::rstest;

    #[rstest]
    #[case(
        "London to Dublin = 464
London to Belfast = 518
Dublin to Belfast = 141",
        "982"
    )]
    fn test_process(#[case] input: &str, #[case] result: &str) -> miette::Result<()> {
        assert_eq!(result, process(input)?);
        Ok(())
    }
}
