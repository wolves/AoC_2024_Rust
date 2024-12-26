use regex::Regex;

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut prog = Program::new(input);

    let result = prog.solve();
    Ok(result.to_string())
}

#[derive(Debug)]
struct Instruction {
    opcode: usize,
    operand: usize,
}

impl Instruction {
    fn new(opcode: usize, operand: usize) -> Self {
        Self { opcode, operand }
    }
}

#[derive(Debug)]
struct Program {
    register_a: usize,
    register_b: usize,
    register_c: usize,
    instructions: Vec<Instruction>,
}

impl Program {
    fn new(input: &str) -> Self {
        let re = Regex::new(r"\d+").unwrap();
        let data: Vec<usize> = re
            .captures_iter(input)
            .map(|capture| capture[0].parse().unwrap())
            .collect();

        let [register_a, register_b, register_c, instructions @ ..] =
            &data[..]
        else {
            panic!("failed new program")
        };

        let instructions = instructions
            .chunks(2)
            .map(|c| Instruction::new(c[0], c[1]))
            .collect();
        Self {
            register_a: *register_a,
            register_b: *register_b,
            register_c: *register_c,
            instructions,
        }
    }

    fn combo_operand(&self, operand: usize) -> usize {
        match operand {
            4 => self.register_a,
            5 => self.register_b,
            6 => self.register_c,
            _ => operand,
        }
    }

    fn solve(&mut self) -> String {
        let mut idx = 0;
        let mut outputs = vec![];

        while let Some(instruction) =
            self.instructions.get(idx)
        {
            idx += 1;
            let combo =
                self.combo_operand(instruction.operand);
            let literal = instruction.operand;
            match instruction.opcode {
                0 => self.register_a >>= combo,
                6 => {
                    self.register_b =
                        self.register_a >> combo
                }
                7 => {
                    self.register_c =
                        self.register_a >> combo
                }
                1 => self.register_b ^= literal,
                2 => self.register_b = combo % 8,
                3 => {
                    if self.register_a != 0 {
                        idx = literal / 2;
                    }
                }
                4 => self.register_b ^= self.register_c,
                5 => {
                    outputs.push(combo % 8);
                }
                _ => panic!("failed to solve"),
            }
        }

        let str: String = outputs
            .iter()
            .map(|n| n.to_string())
            .collect::<Vec<_>>()
            .join(",");

        str
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }
}
