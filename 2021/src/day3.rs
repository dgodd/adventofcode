#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> (usize, Vec<u16>) {
    let len = input.lines().map(|s| s.len()).max().unwrap();
    let data = input.lines().map(|l| u16::from_str_radix(l, 2).unwrap()).collect();
    (len, data)
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &(usize, Vec<u16>)) -> u32 {
    let half = input.1.len() / 2;
    let mut gamma = 0;
    for i in 0..=15 {
        let mut count = 0;
        for x in &input.1 {
            if ((x >> i) & 0x1) == 1 {
                count += 1;
            }
        }
        if count > half {
            gamma |= 0x1 << i;
        }
    }
    let mut epsilon = 0;
    for i in 0..input.0 {
        epsilon |= !gamma  & (0x1 << i);
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &(usize, Vec<u16>)) -> u32 {
    let len = input.0;

    let mut data_o2: Vec<u16> = input.1.iter().map(|d| *d).collect();
    let mut data_co2: Vec<u16> = input.1.iter().map(|d| *d).collect();
    let mut oxygen: u16 = 0;
    let mut co2: u16 = 0;
    for i in 0..len {
        let count_of_ones: u16 = data_o2.iter().map(|d| (d >> (len - 1 - i)) & 0x1).sum();
        let most_common = if (data_o2.len() as i32) - ((count_of_ones * 2) as i32) > 0 { 0 } else { 1 };
        // println!("I: {i} -- {count_of_ones} -- {most_common}");

        data_o2.retain(|d| (d >> (len - 1 - i)) & 0x1 == most_common);
        // println!("DATA: {:?}", &data_o2);
        if data_o2.len() == 1 {
            oxygen = data_o2[0];
        }

        data_co2.retain(|d| (d >> (len - 1 - i)) & 0x1 != most_common);
        // println!("DATA: {:?}", &data_co2);
        if data_co2.len() == 1 {
            co2 = data_co2[0];
        }
    }
    // println!("OXYGEN: {oxygen:b}");
    // println!("CO2: {co2:b}");

    (oxygen as u32) * (co2 as u32)
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};
    const SAMPLE: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn sample1() {
        let (len, data) = gen(SAMPLE);
        assert_eq!(part1(&(len, data)), 198);
    }

    #[test]
    fn sample2() {
        let (len, data) = gen(SAMPLE);
        assert_eq!(part2(&(len, data)), 230);
    }
}
