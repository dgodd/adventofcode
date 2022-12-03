#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<u32> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[u32]) -> u32 {
    input.windows(2).map(|arr| {
        let (a, b) = (arr[0], arr[1]);
        if a < b { 1 } else { 0 }
    }).sum()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[u32]) -> u32 {
    let sums = input.windows(3).map(|arr| arr.iter().sum::<u32>()).collect::<Vec<u32>>();
    sums.windows(2).map(|arr| {
        let (a, b) = (arr[0], arr[1]);
        if a < b { 1 } else { 0 }
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    #[test]
    fn sample1() {
        let data = gen("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(part1(&data), 7);
    }

    #[test]
    fn sample2() {
        let data = gen("199\n200\n208\n210\n200\n207\n240\n269\n260\n263\n");
        assert_eq!(part2(&data), 5);
    }
}
