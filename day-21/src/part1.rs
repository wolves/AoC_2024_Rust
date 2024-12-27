use std::{collections::HashMap, iter::repeat};

use cgmath::Vector2;
use itertools::Itertools;
use once_cell::sync::Lazy;

type Vec2 = Vector2<isize>;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let codes: Vec<_> = input.lines().collect();

    let result: usize = codes
        .iter()
        .map(|code| {
            let len = get_numpad_code_len(code, 2);
            let numeric_val: usize =
                code[..(code.len() - 1)].parse().unwrap();
            len * numeric_val
        })
        .sum();

    Ok(result.to_string())
}

fn get_numpad_code_len(code: &str, depth: usize) -> usize {
    let mut chars: Vec<char> = code.chars().collect();
    chars.insert(0, 'A');

    chars
        .iter()
        .tuple_windows()
        .map(|(&from, &to)| {
            let paths = &NUMERIC_SHORTEST_PATH[&(from, to)];
            paths
                .iter()
                .map(|path| {
                    get_keypad_code_len(path, depth)
                })
                .min()
                .unwrap()
        })
        .sum()
}

fn get_keypad_code_len(path: &str, depth: usize) -> usize {
    let mut chars: Vec<char> = path.chars().collect();
    chars.insert(0, 'A');
    chars.push('A');

    if depth == 1 {
        chars
            .iter()
            .tuple_windows()
            .map(|(&from, &to)| {
                if from != to {
                    let paths =
                        &KEYPAD_SHORTEST_PATH[&(from, to)];
                    paths[0].len() + 1
                } else {
                    1
                }
            })
            .sum()
    } else {
        chars
            .iter()
            .tuple_windows()
            .map(|(&from, &to)| {
                if from != to {
                    let paths =
                        &KEYPAD_SHORTEST_PATH[&(from, to)];
                    paths
                        .iter()
                        .map(|path| {
                            get_keypad_code_len(
                                path,
                                depth - 1,
                            )
                        })
                        .min()
                        .unwrap()
                } else {
                    1
                }
            })
            .sum()
    }
}

static NUMERIC_SHORTEST_PATH: Lazy<
    HashMap<(char, char), Vec<String>>,
> = Lazy::new(|| {
    let keys: Vec<char> = NUMPAD.keys().copied().collect();

    let mut paths = HashMap::new();

    keys.iter().tuple_combinations().for_each(
        |(&from, &to)| {
            paths.insert(
                (from, to),
                get_numeric_shortest_path(from, to),
            );
            paths.insert(
                (to, from),
                get_numeric_shortest_path(to, from),
            );
        },
    );

    paths
});
static KEYPAD_SHORTEST_PATH: Lazy<
    HashMap<(char, char), Vec<String>>,
> = Lazy::new(|| {
    let keys: Vec<char> = KEYPAD.keys().copied().collect();

    let mut paths = HashMap::new();

    keys.iter().tuple_combinations().for_each(
        |(&from, &to)| {
            paths.insert(
                (from, to),
                get_keypad_shortest_path(from, to),
            );
            paths.insert(
                (to, from),
                get_keypad_shortest_path(to, from),
            );
        },
    );

    paths
});

static NUMPAD: Lazy<HashMap<char, Vec2>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert('7', Vec2::new(0, 0));
        map.insert('8', Vec2::new(1, 0));
        map.insert('9', Vec2::new(2, 0));

        map.insert('4', Vec2::new(0, 1));
        map.insert('5', Vec2::new(1, 1));
        map.insert('6', Vec2::new(2, 1));

        map.insert('1', Vec2::new(0, 2));
        map.insert('2', Vec2::new(1, 2));
        map.insert('3', Vec2::new(2, 2));

        map.insert('0', Vec2::new(1, 3));
        map.insert('A', Vec2::new(2, 3));
        map
    });
static KEYPAD: Lazy<HashMap<char, Vec2>> =
    Lazy::new(|| {
        let mut map = HashMap::new();
        map.insert('^', Vec2::new(1, 0));
        map.insert('A', Vec2::new(2, 0));

        map.insert('<', Vec2::new(0, 1));
        map.insert('v', Vec2::new(1, 1));
        map.insert('>', Vec2::new(2, 1));
        map
    });

fn get_numeric_shortest_path(
    from: char,
    to: char,
) -> Vec<String> {
    let start = &NUMPAD[&from];
    let end = &NUMPAD[&to];

    let delta = end - start;

    let horz = if delta.x > 0 { '>' } else { '<' };
    let vert = if delta.y > 0 { 'v' } else { '^' };

    let x = delta.x.unsigned_abs();
    let y = delta.y.unsigned_abs();

    if (from == 'A' || from == '0')
        && (to == '7' || to == '4' || to == '1')
    {
        vec![repeat(vert)
            .take(y)
            .chain(repeat(horz).take(x))
            .collect()]
    } else if (to == 'A' || to == '0')
        && (from == '7' || from == '4' || from == '1')
    {
        vec![repeat(horz)
            .take(x)
            .chain(repeat(vert).take(y))
            .collect()]
    } else if x > 0 && y > 0 {
        vec![
            repeat(vert)
                .take(y)
                .chain(repeat(horz).take(x))
                .collect(),
            repeat(horz)
                .take(x)
                .chain(repeat(vert).take(y))
                .collect(),
        ]
    } else {
        vec![repeat(horz)
            .take(x)
            .chain(repeat(vert).take(y))
            .collect()]
    }
}
fn get_keypad_shortest_path(
    from: char,
    to: char,
) -> Vec<String> {
    let start = &KEYPAD[&from];
    let end = &KEYPAD[&to];

    let delta = end - start;

    let horz = if delta.x > 0 { '>' } else { '<' };
    let vert = if delta.y > 0 { 'v' } else { '^' };

    let x = delta.x.unsigned_abs();
    let y = delta.y.unsigned_abs();

    if (from == '^' || from == 'A') && (to == '<') {
        vec![repeat(vert)
            .take(y)
            .chain(repeat(horz).take(x))
            .collect()]
    } else if (to == '^' || to == 'A') && (to == '<') {
        vec![repeat(horz)
            .take(x)
            .chain(repeat(vert).take(y))
            .collect()]
    } else if x > 0 && y > 0 {
        vec![
            repeat(vert)
                .take(y)
                .chain(repeat(horz).take(x))
                .collect(),
            repeat(horz)
                .take(x)
                .chain(repeat(vert).take(y))
                .collect(),
        ]
    } else {
        vec![repeat(horz)
            .take(x)
            .chain(repeat(vert).take(y))
            .collect()]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "029A
980A
179A
456A
379A";
        assert_eq!("126384", process(input)?);
        Ok(())
    }
}
