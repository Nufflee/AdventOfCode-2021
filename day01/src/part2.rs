pub fn count_increases(depths: &[u32]) -> u32 {
    let mut increases = 0u32;

    // TODO: Try doing some intellectual masturbation here using zips 'n stuff.
    // See: Enum.chunk_every and https://doc.rust-lang.org/std/primitive.slice.html#method.windows
    for i in 0..depths.len() - 3 {
        let sum1: u32 = depths[i..i + 3].iter().sum();
        let sum2: u32 = depths[i + 1..i + 4].iter().sum();

        if sum1 < sum2 {
            increases += 1;
        }
    }

    increases
}
