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
        .map(|group| group.len() * get_fence(group))
        .sum();

    Ok(result.to_string())
}

fn get_fence(group: &[Vec2]) -> usize {
    let mut fence_parts: Vec<(Vec2, Vec2)> = group
        .iter()
        .flat_map(|&p1| {
            dirs()
                .into_iter()
                .filter(move |dir| !group.contains(&(p1 + dir)))
                .map(move |dir| (p1, dir))
        })
        .collect();

    let mut count = 0;

    while let Some(part) = fence_parts.pop() {
        reduce_fence(part, &mut fence_parts);
        count += 1;
    }
    count
}

fn reduce_fence(part: (Vec2, Vec2), fence_parts: &mut Vec<(Vec2, Vec2)>) {
    dirs().into_iter().for_each(|dir| {
        let p = part.0 + dir;
        if let Some(index) = fence_parts
            .iter()
            .position(|part2| part2.1 == part.1 && part2.0 == p)
        {
            let part2 = fence_parts.remove(index);
            reduce_fence(part2, fence_parts);
        }
    });
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
        assert_eq!("1206", process(input)?);
        Ok(())
    }
}
