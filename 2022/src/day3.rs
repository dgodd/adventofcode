#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<(String, String)> {
    input.lines()
        .map(|l| {
            let half = l.len() / 2;
            return (l[..half].to_string(), l[half..].to_string());
        })
        .collect()
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[(String, String)]) -> u32 {
    input.iter().map(|(l, r)| {
        let inboth = l.chars().find(|c| r.contains(*c)).unwrap();
        let mut num = inboth as u32;
        match num {
            (65...90) => num - 64 + 26,
            (97...122) => num - 96,
            _ => unreachable!(),
        }
    }).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[(String, String)]) -> u32 {
    0
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    const SAMPLE1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";

    #[test]
    fn sample1() {
        assert_eq!(gen("abcd\nefgh\n"), vec![(String::from("ab"), String::from("cd")), (String::from("ef"), String::from("gh"))]);
        assert_eq!(gen(SAMPLE1), vec![(String::from("vJrwpWtwJgWr"), String::from("hcsFMMfFFhFp"))]);
    }

    #[test]
    fn sample2() {
        let data = gen(SAMPLE1);
        assert_eq!(part1(&data), 16);
    }

    #[test]
    fn sample3() {
        let data = gen("jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL");
        assert_eq!(part1(&data), 38);
    }

    #[test]
    fn sample4() {
        let data = gen(SAMPLE);
        assert_eq!(part1(&data), 157);
    }
}
