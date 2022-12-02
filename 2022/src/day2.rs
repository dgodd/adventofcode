#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(u8, u8)> {
    input.lines()
        .map(|v| {
            let [a, b]: [&str; 2] = v.split_whitespace().collect::<Vec<&str>>().try_into().unwrap();
            // println!("A: {} -- B: {}", a, b);
            let a1 = match a {
                "A" => 1,
                "B" => 2,
                "C" => 3,
                _ => unreachable!(),
            };
            let b1 = match b {
                "X" => 1,
                "Y" => 2,
                "Z" => 3,
                _ => unreachable!(),
            };
            (a1, b1)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|(a, b)| {
            let win: u32 = match (a, b) {
                (1, 2) => 6, // win
                (1, 3) => 0, // loss
                (2, 3) => 6, // win
                (2, 1) => 0, // loss
                (3, 1) => 6, // win
                (3, 2) => 0, // loss
                _ => 3, // draw
            };
            let mine = *b as u32;
            win + mine
        })
        .sum()
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(u8, u8)]) -> u32 {
    input
        .iter()
        .map(|(a, b)| {
            let win: u32 = match *b {
                1 => 0, // loss
                2 => 3, // draw
                3 => 6, // win
                _ => unreachable!(),
            };
            let mine: u32 = match (a, b) {
                (1, 1) => 3, // loss -> rock : scissors -> 3
                (1, 2) => 1, // draw -> rock : rock -> 1
                (1, 3) => 2, // win -> rock : paper -> 2

                (2, 1) => 1, // loss -> paper : rock -> 1
                (2, 2) => 2, // draw -> paper : paper -> 2
                (2, 3) => 3, // win -> paper : scissors -> 3

                (3, 1) => 2, // loss -> scissors : paper -> 2
                (3, 2) => 3, // draw -> scissors : scissors -> 3
                (3, 3) => 1, // win -> scissors : rock -> 1

                _ => unreachable!(),
            };
            win + mine
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE_TXT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn sample1() {
        assert_eq!(gen(SAMPLE_TXT), vec![(1, 2), (2, 1), (3, 3)]);
    }
    
    #[test]
    fn sample2() {
        let data = gen(SAMPLE_TXT);
        assert_eq!(part1(&data), 15);
    }
    
    #[test]
    fn sample3() {
        let data = gen(SAMPLE_TXT);
        assert_eq!(part2(&data), 12);
    }
}
