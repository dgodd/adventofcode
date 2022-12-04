use std::ops::RangeInclusive;

#[aoc_generator(day4)]
pub fn input_generator(input: &str) -> Vec<(RangeInclusive<u32>, RangeInclusive<u32>)> {
    input.lines().map(|l| {
        let arr: Vec<&str> = l.split(",").collect();
        let l: Vec<u32> = arr[0].split("-").map(|s| s.parse::<u32>().unwrap()).collect();
        let r: Vec<u32> = arr[1].split("-").map(|s| s.parse::<u32>().unwrap()).collect();
        return (l[0]..=l[1], r[0]..=r[1]);
    }).collect()
}

#[aoc(day4, part1)]
pub fn solve_part1(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> u32 {
    input.iter().filter(|(l, r)| (l.contains(r.start()) && l.contains(r.end())) || (r.contains(l.start()) && r.contains(l.end()))).count() as u32
}

#[aoc(day4, part2)]
pub fn solve_part2(input: &[(RangeInclusive<u32>, RangeInclusive<u32>)]) -> u32 {
    input.iter().filter(|(l, r)| l.contains(r.start()) || l.contains(r.end()) || r.contains(l.start()) || r.contains(l.end())).count() as u32
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE: &str = "2-4,6-8\n2-3,4-5\n5-7,7-9\n2-8,3-7\n6-6,4-6\n2-6,4-8\n";

    #[test]
    fn sample1() {
        let data = gen(SAMPLE);
        assert_eq!(part1(&data), 2);
    }

    #[test]
    fn sample2() {
        let data = gen(SAMPLE);
        assert_eq!(part2(&data), 4);
    }
}
