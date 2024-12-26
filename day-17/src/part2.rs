use regex::Regex;

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
    orig_b: usize,
    orig_c: usize,
    expected: Vec<usize>,
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

        let expected = instructions.to_vec();

        let instructions = instructions
            .chunks(2)
            .map(|c| Instruction::new(c[0], c[1]))
            .collect();
        Self {
            register_a: *register_a,
            register_b: *register_b,
            register_c: *register_c,
            instructions,
            orig_b: *register_b,
            orig_c: *register_c,
            expected,
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

    fn solve(&mut self) -> usize {
        let mut idx = 0;
        // let mut outputs = vec![];
        let mut oidx = 0;

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
                    // outputs.push(combo % 8);
                    let o = combo % 8;

                    if self
                        .expected
                        .get(oidx)
                        .is_some_and(|&e| e == o)
                    {
                        oidx += 1;
                    } else {
                        if oidx > self.expected.len() {
                            oidx += 1;
                        }
                        return oidx;
                    }
                }
                _ => panic!("failed to solve"),
            }
        }

        oidx
    }

    fn reset(&mut self) {
        self.register_b = self.orig_b;
        self.register_c = self.orig_c;
    }
}

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();

    let mut prog = Program::new(input);

    let goal = prog.expected.len();

    let mut guaranteed_bits = vec![];

    for min in 1..goal {
        let mut potential_as = vec![];
        for a in 0..1_000_000 {
            let mut a = a;
            for (index, bit) in guaranteed_bits.iter() {
                a = match bit {
                    Bit::One => {
                        insert_one_at_index(a, *index)
                    }
                    Bit::Zero => {
                        insert_zero_at_index(a, *index)
                    }
                }
            }
            prog.register_a = a;
            prog.reset();
            let result = prog.solve();
            if result > min {
                potential_as.push(a);
                // println!("{:032b}", a);
                if result == goal {
                    println!("a: {}", a);
                    return Ok(result.to_string());
                }
            }
        }

        guaranteed_bits = common_bit_indices(&potential_as);
    }

    Ok("No Result".to_string())
}

fn insert_one_at_index(n: usize, index: usize) -> usize {
    let mask = !0 << index;
    let higher_bits = n & mask;
    let lower_bits = n & !mask;

    (higher_bits << 1) | (1 << index) | lower_bits
}
fn insert_zero_at_index(n: usize, index: usize) -> usize {
    let mask = !0 << index;
    let higher_bits = n & mask;
    let lower_bits = n & !mask;

    (higher_bits << 1) | lower_bits
}

#[derive(Debug, PartialEq, Eq)]
enum Bit {
    One,
    Zero,
}

fn common_bit_indices(
    nums: &Vec<usize>,
) -> Vec<(usize, Bit)> {
    let mut result: Vec<(usize, Bit)> = Vec::new();
    let mut zeroes = Vec::new();

    for bit_index in 0..usize::BITS as usize {
        if nums
            .iter()
            .all(|&num| num & (1 << bit_index) != 0)
        {
            result.push((bit_index, Bit::One));
        }
        if nums
            .iter()
            .all(|&num| num & (1 << bit_index) == 0)
        {
            //result.push((bit_index, Bit::One));
            zeroes.push(bit_index);
        }
    }

    zeroes.reverse();

    for bit_index in (0..usize::BITS as usize).rev() {
        if zeroes.get(0).is_some_and(|z| *z == bit_index) {
            zeroes.remove(0);
        } else {
            break;
        }
    }

    for zero in zeroes.iter() {
        result.push((*zero, Bit::Zero));
    }

    result.sort_by_key(|e| e.0);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[ignore]
    fn test_process() -> miette::Result<()> {
        let input = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";
        assert_eq!("4,6,3,5,6,3,5,2,1,0", process(input)?);
        Ok(())
    }

    #[test]
    fn test_common_bit_indices() {
        let numbers = vec![0b101, 0b10010101, 0b1111101];
        let result = common_bit_indices(&numbers);
        let expected = vec![
            (0, Bit::One),
            (1, Bit::Zero),
            (2, Bit::One),
        ];

        assert_eq!(expected, result);
    }

    #[test]
    fn test_insert_one_at_index() {
        let n = 0b1001011101;
        let i = 2;
        assert_eq!(
            0b10010111101,
            insert_one_at_index(n, i)
        );
    }
    #[test]
    fn test_insert_zero_at_index() {
        let n = 0b1001011101;
        let i = 2;
        assert_eq!(
            0b10010111001,
            insert_zero_at_index(n, i)
        );
    }
}
