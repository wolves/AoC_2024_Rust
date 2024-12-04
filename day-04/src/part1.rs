use glam::IVec2;

const DIRECTIONS: [[IVec2; 3]; 8] = [
    [IVec2::new(0, -1), IVec2::new(0, -2), IVec2::new(0, -3)], // North
    [IVec2::new(0, 1), IVec2::new(0, 2), IVec2::new(0, 3)],    // South
    [IVec2::new(1, 0), IVec2::new(2, 0), IVec2::new(3, 0)],    // East
    [IVec2::new(-1, 0), IVec2::new(-2, 0), IVec2::new(-3, 0)], // West
    [IVec2::new(1, -1), IVec2::new(2, -2), IVec2::new(3, -3)], // NorthEast
    [IVec2::new(1, 1), IVec2::new(2, 2), IVec2::new(3, 3)],    // SouthEast
    [IVec2::new(-1, -1), IVec2::new(-2, -2), IVec2::new(-3, -3)], // NorthWest
    [IVec2::new(-1, 1), IVec2::new(-2, 2), IVec2::new(-3, 3)], // SouthWest
];

pub fn process(_input: &str) -> miette::Result<String> {
    todo!("day 00 - part 1");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() -> miette::Result<()> {
        let input = "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";
        assert_eq!("18", process(input)?);
        Ok(())
    }
}
