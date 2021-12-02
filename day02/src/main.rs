mod part1;
mod part2;
mod submarine;

use submarine::*;

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let commands = input.lines().map(|line| line.parse::<Command>().unwrap());

    {
        let Location { position, depth } = part1::compute(commands.clone());

        println!("Part 1:");
        println!(
            "\tFinal position is {} and depth is {}. The product is {}.",
            position,
            depth,
            position * depth
        );
    }

    {
        let Location { position, depth } = part2::compute(commands);

        println!("Part 2:");
        println!(
            "\tFinal corrected position is {} and depth is {}. The product is {}.",
            position,
            depth,
            position * depth
        );
    }
}
