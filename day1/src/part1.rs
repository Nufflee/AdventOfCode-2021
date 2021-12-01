pub fn count_increases(depths: &[u32]) -> u32 {
    let mut increases = 0u32;

    for i in 1..depths.len() {
        if depths[i - 1] < depths[i] {
            increases += 1;
        }
    }

    increases
}
