use std::collections::HashMap;

use cgmath::Vector2;

type Vec2 = Vector2<isize>;

fn dirs() -> Vec<Vec2> {
    vec![
        Vec2::new(1, 0),
        Vec2::new(-1, 0),
        Vec2::new(0, 1),
        Vec2::new(0, -1),
    ]
}

pub fn process(input: &str) -> miette::Result<String> {
    let data = input.trim();

    let plant_map = data
        .lines()
        .enumerate()
        .flat_map(|(y, row)| {
            row.chars()
                .enumerate()
                .map(move |(x, c)| (Vec2::new(x as isize, y as isize), c))
        })
        .collect::<HashMap<Vec2, char>>();

    let groups = split_into_groups(plant_map);

    let result: usize = groups
        .iter()
        .map(|group| group.len() * get_perimeter(group))
        .sum();

    Ok(result.to_string())
}

fn get_perimeter(group: &[Vec2]) -> usize {
    group
        .iter()
        .map(|p1| {
            dirs()
                .into_iter()
                .filter(|dir| !group.iter().any(|&p2| p2 == p1 + dir))
                .count()
        })
        .sum()
}

fn split_into_groups(mut plant_map: HashMap<Vec2, char>) -> Vec<Vec<Vec2>> {
    let mut groups = vec![];

    while let Some((&pos, &c)) = plant_map.iter().next() {
        let mut group = vec![];

        collect_groups(pos, c, &mut plant_map, &mut group);

        groups.push(group);
    }

    groups
}

fn collect_groups(
    pos: Vector2<isize>,
    c: char,
    plant_map: &mut HashMap<Vec2, char>,
    group: &mut Vec<Vec2>,
) {
    if let Some(&c1) = plant_map.get(&pos) {
        if c1 == c {
            plant_map.remove(&pos);
            group.push(pos);

            for dir in dirs().into_iter() {
                collect_groups(pos + dir, c, plant_map, group);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";
        assert_eq!("1930", process(input)?);
        Ok(())
    }
}
