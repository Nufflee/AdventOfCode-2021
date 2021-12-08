use crate::meth::Median;

pub fn fuel_consumed(positions: impl Iterator<Item = u32>) -> (u32, u32) {
    let mut positions = positions.collect::<Vec<_>>();
    let optimal_position = positions.median();

    (
        optimal_position,
        positions.iter().fold(0, |acc, &position| {
            acc + (position as i32 - optimal_position as i32).abs() as u32
        }),
    )
}
