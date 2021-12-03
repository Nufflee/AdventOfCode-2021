pub fn calculate_power_consumption(report: &[u32], bit_count: usize) -> u32 {
    let mut gamma_rate = 0;
    let mut epsilon_rate = 0;

    for i in 0..bit_count {
        let mut one_count = 0;

        for number in report {
            let bit = (number & (1 << i)) >> i;

            one_count += bit;
        }

        if one_count as usize > report.len() / 2 {
            // 1's are more common
            gamma_rate |= 1 << i;
            epsilon_rate |= 0 << i;
        } else {
            // 0's are more common
            gamma_rate |= 0 << i;
            epsilon_rate |= 1 << i;
        }
    }

    println!("\tγ rate is {}, ε rate is {}.", gamma_rate, epsilon_rate);

    gamma_rate * epsilon_rate
}
