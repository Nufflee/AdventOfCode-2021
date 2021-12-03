use std::collections::HashSet;

#[derive(Debug)]
enum CommonBit {
    Zero,
    One,
    Neither,
}

impl CommonBit {
    fn to_int(&self, neither_value: u32) -> u32 {
        match self {
            CommonBit::Zero => 0,
            CommonBit::One => 1,
            CommonBit::Neither => neither_value,
        }
    }
}

pub fn calculate_life_support_rating(report: &[u32], bit_count: usize) -> u32 {
    let mut oxygen_rating_indexes = HashSet::<_>::from_iter(0..report.len());
    let mut co2_rating_indexes = oxygen_rating_indexes.clone();
    let mut oxygen_generator_rating = None;
    let mut co2_scrubber_rating = None;

    for bit_index in (0..bit_count).rev() {
        let mut one_count = 0;

        for i in &oxygen_rating_indexes {
            let bit = (report[*i] & (1 << bit_index)) >> bit_index;

            one_count += bit;
        }

        let most_common_bit = match (one_count as usize * 2).cmp(&oxygen_rating_indexes.len()) {
            std::cmp::Ordering::Less => CommonBit::Zero,
            std::cmp::Ordering::Equal => CommonBit::Neither,
            std::cmp::Ordering::Greater => CommonBit::One,
        };

        for i in oxygen_rating_indexes.clone() {
            let bit = (report[i] & (1 << bit_index)) >> bit_index;

            if bit != most_common_bit.to_int(1) {
                oxygen_rating_indexes.remove(&i);
            }
        }

        if oxygen_rating_indexes.len() == 1 {
            oxygen_generator_rating = Some(report[*oxygen_rating_indexes.iter().next().unwrap()]);
            break;
        }
    }

    for bit_index in (0..bit_count).rev() {
        let mut one_count = 0;

        for i in &co2_rating_indexes {
            let bit = (report[*i] & (1 << bit_index)) >> bit_index;

            one_count += bit;
        }

        // TODO: Can I do this without FP?
        let least_common_bit = match (one_count as usize * 2).cmp(&co2_rating_indexes.len()) {
            std::cmp::Ordering::Less => CommonBit::One,
            std::cmp::Ordering::Equal => CommonBit::Neither,
            std::cmp::Ordering::Greater => CommonBit::Zero,
        };

        for i in co2_rating_indexes.clone() {
            let bit = (report[i] & (1 << bit_index)) >> bit_index;

            if bit != least_common_bit.to_int(0) {
                co2_rating_indexes.remove(&i);
            }
        }

        if co2_rating_indexes.len() == 1 {
            co2_scrubber_rating = Some(report[*co2_rating_indexes.iter().next().unwrap()]);
            break;
        }
    }

    oxygen_generator_rating.expect("unique oxygen generator rating not found")
        * co2_scrubber_rating.expect("unique CO2 scrubber rating not found")
}
