use day_07::part1::process;
use miette::Context;

fn main() -> miette::Result<()> {
    let file = include_str!("../../input1.txt");
    let result = process(file, None).context("process part 1")?;
    println!("{}", result);
    Ok(())
}
