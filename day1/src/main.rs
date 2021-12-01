mod part1;
mod part2;

fn main() {
    let file = std::fs::read_to_string("input/input.txt").unwrap();
    let depths = file
        .lines()
        .map(|line| line.parse().unwrap())
        .collect::<Vec<u32>>();

    println!("Part 1:");
    println!(
        "\tDepth increased {} times",
        part1::count_increases(&depths)
    );

    println!("Part 2:");
    println!(
        "\tDepth increased {} times after applying the filter",
        part2::count_increases(&depths)
    );
}
