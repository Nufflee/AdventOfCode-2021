mod meth;
mod part1;
mod part2;

fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let positions = input.split(',').map(|x| x.parse::<u32>().unwrap());

    let (optimal_position, fuel_used) = part1::fuel_consumed(positions.clone());

    println!("Part 1:");
    println!(
        "\tOptimal position is {} and fuel used to align is {}.",
        optimal_position, fuel_used
    );

    println!("Part 2:");
    println!(
        "\tBruteforce approach: Fuel used to align is {}.",
        part2::fuel_consumed_bruteforce(positions.clone())
    );

    println!(
        "\t5Head approach: Fuel used to align is {}.",
        part2::fuel_consumed_5Head(positions)
    );
}
