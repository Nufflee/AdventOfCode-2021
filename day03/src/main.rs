mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let lines = input.lines();
    let bit_count = lines.peekable().peek().unwrap().len();
    let report = input
        .lines()
        .map(|line| u32::from_str_radix(line, 2).unwrap())
        .collect::<Vec<_>>();

    {
        println!("Part 1:");

        println!(
            "\tPower consumption is {}.",
            part1::calculate_power_consumption(&report, bit_count)
        );
    }

    {
        println!("Part 2:");

        println!(
            "\tLife support rating is {}.",
            part2::calculate_life_support_rating(&report, bit_count)
        );
    }
}
