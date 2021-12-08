fn main() {
    let input = std::fs::read_to_string("input/input.txt").unwrap();
    let fish = input
        .split(',')
        .map(|x| x.parse::<u32>().unwrap())
        .collect::<Vec<_>>();

    println!("Part 1:");
    println!(
        "\tAfter 80 days there are {} fish.",
        fish_count_naive(&fish, 80)
    );

    println!("Part 2:");
    println!(
        "\tAfter 256 days there are {} fish.",
        fish_count_5Head(&fish, 256)
    );
}

fn fish_count_naive(starting_fish: &[u32], days: u32) -> u64 {
    let mut fish = starting_fish.to_vec();

    (0..days).for_each(|_| {
        for i in 0..fish.clone().len() {
            let timer = &mut fish[i];

            if *timer == 0 {
                // Reset the current fish's timer
                *timer = 6;

                // Create a new fish
                fish.push(8);
            } else {
                *timer -= 1;
            }
        }
    });

    fish.len() as u64
}

#[allow(non_snake_case)]
fn fish_count_5Head(starting_fish: &[u32], days: u32) -> u64 {
    let mut timer_groups: [u64; 9] = [0; 9];

    for timer in starting_fish {
        timer_groups[*timer as usize] += 1;
    }

    (0..days).for_each(|_| {
        let mut next_timer_groups = [0; 9];

        // Not using `clone_from_slice` makes code much cleaner and easier to understand. After all, we are only copying 8 integers.
        #[allow(clippy::manual_memcpy)]
        for i in 1..timer_groups.len() {
            // "Decrement" the timers
            next_timer_groups[i - 1] = timer_groups[i];
        }

        // Create new fish and "wrap around" original fish's timer
        next_timer_groups[8] += timer_groups[0];
        next_timer_groups[6] += timer_groups[0];

        timer_groups = next_timer_groups;
    });

    timer_groups.iter().sum::<u64>()
}
