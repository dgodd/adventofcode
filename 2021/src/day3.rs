#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<u16> {
    input.lines().map(|l| u16::from_str_radix(l, 2).unwrap()).collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[u16]) -> u32 {
    let half = input.len() / 2;
    let mut gamma = 0;
    let mut highest = 0;
    for i in 0..=15 {
        let mut count = 0;
        for x in input {
            if ((x >> i) & 0x1) == 1 {
                count += 1;
            }
        }
        if count > half {
            gamma |= 0x1 << i;
        }
        if count > 0 {
            highest = i;
        }
    }
    let mut epsilon = 0;
    for i in 0..=highest {
        epsilon |= !gamma  & (0x1 << i);
    }
    gamma * epsilon
}

#[aoc(day3, part2)]
pub fn solve_part2(_input: &[u16]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};
    const SAMPLE: &str = "00100\n11110\n10110\n10111\n10101\n01111\n00111\n11100\n10000\n11001\n00010\n01010\n";

    #[test]
    fn sample1() {
        let data = gen(SAMPLE);
        assert_eq!(part1(&data), 198);
    }

    #[test]
    fn sample2() {
        let data = gen(SAMPLE);
        assert_eq!(part2(&data), 230);
    }
}
