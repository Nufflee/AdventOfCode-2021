use crate::meth::Mean;

fn fuel_consumed_to_align<'a>(
    positions: impl Iterator<Item = &'a u32>,
    target_position: u32,
) -> u32 {
    positions.fold(0, |acc, &position| {
        let diff = (position as i32 - target_position as i32).abs() as u32;

        // Using partial sum formula for (0..diff).sum() (https://www.wolframalpha.com/input/?i=sum+of+n)
        acc + (diff * (diff + 1) / 2)
    })
}

pub fn fuel_consumed_bruteforce(positions: impl Iterator<Item = u32>) -> u32 {
    let positions = positions.collect::<Vec<_>>();
    let max_position = *positions.iter().max().unwrap();

    (0..=max_position)
        .map(|target_position| fuel_consumed_to_align(positions.iter(), target_position))
        .min()
        .unwrap()
}

// Based on this paper: https://www.reddit.com/r/adventofcode/comments/rawxad/2021_day_7_part_2_i_wrote_a_paper_on_todays/
#[allow(non_snake_case)]
pub fn fuel_consumed_5Head(positions: impl Iterator<Item = u32>) -> u32 {
    let positions = positions.collect::<Vec<_>>();
    let mean_position = positions.mean() as u32;

    ((mean_position - 1)..=(mean_position + 1))
        .map(|position| fuel_consumed_to_align(positions.iter(), position))
        .min()
        .unwrap()
}
