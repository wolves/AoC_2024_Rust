use glam::IVec2;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{preceded, separated_pair},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, instructions) = parse(input).map_err(|e| miette::miette!("Parsing error {e}"))?;

    let mut grid = Grid::new(1000, 1000);

    for instruction in instructions {
        grid.apply_instruction(&instruction);
    }

    let result = grid.count_lit();

    Ok(result.to_string())
}

struct Grid {
    lights: Vec<bool>,
    width: usize,
    height: usize,
}

impl Grid {
    fn new(width: usize, height: usize) -> Self {
        Self {
            lights: vec![false; width * height],
            width,
            height,
        }
    }

    fn get_index(&self, x: i32, y: i32) -> usize {
        (y as usize * self.width) + x as usize
    }

    fn apply_instruction(&mut self, instruction: &Instruction) {
        match instruction {
            Instruction::TurnOn(start, end) => self.apply_to_region(start, end, |_| true),
            Instruction::TurnOff(start, end) => self.apply_to_region(start, end, |_| false),
            Instruction::Toggle(start, end) => self.apply_to_region(start, end, |x| !x),
        }
    }

    fn apply_to_region<F>(&mut self, start: &IVec2, end: &IVec2, operation: F)
    where
        F: Fn(bool) -> bool,
    {
        for y in start.y..=end.y {
            for x in start.x..=end.x {
                let idx = self.get_index(x, y);
                self.lights[idx] = operation(self.lights[idx]);
            }
        }
    }

    fn count_lit(&self) -> usize {
        self.lights.iter().filter(|&&light| light).count()
    }
}

#[derive(Debug)]
enum Instruction {
    TurnOn(IVec2, IVec2),
    TurnOff(IVec2, IVec2),
    Toggle(IVec2, IVec2),
}

fn parse_num(input: &str) -> IResult<&str, i32> {
    map_res(digit1, str::parse)(input)
}

fn parse_point(input: &str) -> IResult<&str, IVec2> {
    map(separated_pair(parse_num, char(','), parse_num), |(x, y)| {
        IVec2::new(x, y)
    })(input)
}

fn parse_turn_on(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("turn on "),
            separated_pair(parse_point, tag(" through "), parse_point),
        ),
        |(start, end)| Instruction::TurnOn(start, end),
    )(input)
}
fn parse_turn_off(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("turn off "),
            separated_pair(parse_point, tag(" through "), parse_point),
        ),
        |(start, end)| Instruction::TurnOff(start, end),
    )(input)
}
fn parse_toggle(input: &str) -> IResult<&str, Instruction> {
    map(
        preceded(
            tag("toggle "),
            separated_pair(parse_point, tag(" through "), parse_point),
        ),
        |(start, end)| Instruction::Toggle(start, end),
    )(input)
}

fn parse(input: &str) -> IResult<&str, Vec<Instruction>> {
    separated_list1(
        line_ending,
        alt((parse_turn_on, parse_turn_off, parse_toggle)),
    )(input)
}
