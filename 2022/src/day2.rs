#[derive(Debug,Eq,PartialEq)]
pub enum Hand {
    Rock,
    Paper,
    Scissors,
}

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Hand, Hand)> {
    input.lines()
        .map(|v| {
            let [a, b]: [&str; 2] = v.split_whitespace().collect::<Vec<&str>>().try_into().unwrap();
            // println!("A: {} -- B: {}", a, b);
            let a1 = match a {
                "A" => Hand::Rock,
                "B" => Hand::Paper,
                "C" => Hand::Scissors,
                _ => unreachable!(),
            };
            let b1 = match b {
                "X" => Hand::Rock,
                "Y" => Hand::Paper,
                "Z" => Hand::Scissors,
                _ => unreachable!(),
            };
            (a1, b1)
        })
        .collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Hand, Hand)]) -> u32 {
    input
        .iter()
        .map(|(a, b)| {
            let win = match (a, b) {
                (Hand::Rock, Hand::Paper) => 6, // win
                (Hand::Rock, Hand::Scissors) => 0, // loss
                (Hand::Paper, Hand::Scissors) => 6, // win
                (Hand::Paper, Hand::Rock) => 0, // loss
                (Hand::Scissors, Hand::Rock) => 6, // win
                (Hand::Scissors, Hand::Paper) => 0, // loss
                _ => 3, // draw
            };
            let mine = match b {
                Hand::Rock => 1,
                Hand::Paper => 2,
                Hand::Scissors => 3,
            };
            win + mine
        })
        .sum()
}

// #[aoc(day2, part2)]
// pub fn solve_part2(input: &[(Hand, Hand)]) -> u32 {
//     input
//         .iter()
//         .map(|(a, b)| {
//             let win = match (a, b) {
//                 (Hand::Rock, Hand::Paper) => 6, // win
//                 (Hand::Rock, Hand::Scissors) => 0, // loss
//                 (Hand::Paper, Hand::Scissors) => 6, // win
//                 (Hand::Paper, Hand::Rock) => 0, // loss
//                 (Hand::Scissors, Hand::Rock) => 6, // win
//                 (Hand::Scissors, Hand::Paper) => 0, // loss
//                 _ => 3, // draw
//             };
//             let mine = match b {
//                 Hand::Rock => 1,
//                 Hand::Paper => 2,
//                 Hand::Scissors => 3,
//             };
//             win + mine
//         })
//         .sum()
// }

#[cfg(test)]
mod tests {
    use super::{Hand, input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE_TXT: &str = "A Y\nB X\nC Z\n";

    #[test]
    fn sample1() {
        assert_eq!(gen(SAMPLE_TXT), vec![(Hand::Rock, Hand::Paper), (Hand::Paper, Hand::Rock), (Hand::Scissors, Hand::Scissors)]);
    }
    
    #[test]
    fn sample2() {
        let data = gen(SAMPLE_TXT);
        assert_eq!(part1(&data), 15);
    }
    
    // #[test]
    // fn sample3() {
    //     let data = gen(SAMPLE_TXT);
    //     assert_eq!(part2(&data), 12);
    // }
}
