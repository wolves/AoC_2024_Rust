use std::collections::HashMap;

use cgmath::Vector2;

type Vec2 = Vector2<isize>;

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

    // let asdf = map
    //     .into_iter()
    //     .filter_map(|(k, v)| if v == 'R' { Some((k, v)) } else { None })
    //     .collect::<HashMap<(i32, i32), char>>();
    // dbg!(asdf);
    todo!("day 00 - part 1");
}

fn split_into_groups(plant_map: HashMap<Vec2, char>) -> Vec<Vec<Vec2>> {
    let mut groups = vec![];

    while let Some((&pos, &c)) = plant_map.iter().next() {
        let group = vec![];

        collect_group(pos, c, &mut plant_map, &mut group);

        groups.push(group);
    }

    groups
}

fn collect_group(
    pos: Vector2<isize>,
    c: char,
    plant_map: &mut HashMap<Vec2, char>,
    group: &mut Vec<Vec2>,
) {
    todo!()
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
