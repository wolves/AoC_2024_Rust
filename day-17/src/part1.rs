use derive_more::derive::TryFrom;
use nom::{
    bytes::complete::tag,
    character::complete::{self, line_ending, multispace1, one_of},
    combinator::{all_consuming, opt},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair},
    IResult,
};

pub fn process(input: &str) -> miette::Result<String> {
    let input = input.trim();
    let (_input, (registers, instructions)) =
        parse(input).map_err(|e| miette::miette!("parsing failed {}", e))?;

    dbg!(registers, instructions);

    todo!("day 00 - part 1");
}

#[derive(Debug)]
struct Registers {
    a: i32,
    b: i32,
    c: i32,
    pointer: usize,
}

#[derive(TryFrom, Debug, Clone, Copy)]
#[try_from(repr)]
#[repr(u32)]
enum Instruction {
    /// The adv instruction (opcode 0) performs division. The numerator is the
    /// value in the A register. The denominator is found by raising 2 to the
    /// power of the instruction's combo operand. (So, an operand of 2 would
    /// divide A by 4 (2^2); an operand of 5 would divide A by 2^B.) The result
    /// of the division operation is truncated to an integer and then written
    /// to the A register.
    Adv = 0,
    /// The bxl instruction (opcode 1) calculates the bitwise XOR of register B
    /// and the instruction's literal operand, then stores the result in
    /// register B.
    Bxl = 1,
    /// The bst instruction (opcode 2) calculates the value of its combo
    /// operand modulo 8 (thereby keeping only its lowest 3 bits), then writes
    /// that value to the B register.
    Bst = 2,
    /// The jnz instruction (opcode 3) does nothing if the A register is 0.
    /// However, if the A register is not zero, it jumps by setting the
    /// instruction pointer to the value of its literal operand; if this
    /// instruction jumps, the instruction pointer is not increased by 2 after
    /// this instruction.
    Jnz = 3,
    /// The bxc instruction (opcode 4) calculates the bitwise XOR of register B
    /// and register C, then stores the result in register B. (For legacy
    /// reasons, this instruction reads an operand but ignores it.)
    Bxc = 4,
    /// The out instruction (opcode 5) calculates the value of its combo
    /// operand modulo 8, then outputs that value. (If a program outputs
    /// multiple values, they are separated by commas.)
    Out = 5,
    /// The bdv instruction (opcode 6) works exactly like the adv instruction
    /// except that the result is stored in the B register. (The numerator is
    /// still read from the A register.)
    Bdv = 6,
    /// The cdv instruction (opcode 7) works exactly like the adv instruction
    /// except that the result is stored in the C register. (The numerator is
    /// still read from the A register.)
    Cdv = 7,
}

fn registers(input: &str) -> IResult<&str, Registers> {
    let (input, a) = delimited(
        tag("Register A: "),
        complete::i32,
        line_ending,
    )(input)?;
    let (input, b) = delimited(
        tag("Register B: "),
        complete::i32,
        line_ending,
    )(input)?;
    let (input, c) = delimited(
        tag("Register C: "),
        complete::i32,
        line_ending,
    )(input)?;

    Ok((
        input,
        Registers {
            a,
            b,
            c,
            pointer: 0,
        },
    ))
}

fn instruction(input: &str) -> IResult<&str, Instruction> {
    let (input, digit) = one_of("01234567")(input)?;
    let ins = Instruction::try_from(digit.to_digit(10).unwrap()).unwrap();

    Ok((input, ins))
}

fn parse(input: &str) -> IResult<&str, (Registers, Vec<Instruction>)> {
    let (input, (registers, instructions)) = separated_pair(
        registers,
        multispace1,
        preceded(
            tag("Program: "),
            separated_list1(tag(","), instruction),
        ),
    )(input)?;

    let (input, _) = all_consuming(opt(line_ending))(input)?;

    Ok((input, (registers, instructions)))
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
