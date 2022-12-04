#[aoc_generator(day3)]
pub fn input_generator(input: &str) -> Vec<String> {
    input.lines().map(|l| String::from(l)).collect()
}

fn priority(c: char) -> u32 {
        let num = c as u32;
        match num {
            (65..=90) => num - 64 + 26,
            (97..=122) => num - 96,
            _ => unreachable!(),
        }
}

#[aoc(day3, part1)]
pub fn solve_part1(input: &[String]) -> u32 {
    input.iter().map(|line| {
        let half = line.len() / 2;
        let l = line[..half].to_string();
        let r = line[half..].to_string();
        let inboth = l.chars().find(|c| r.contains(*c)).unwrap();
        priority(inboth)
    }).sum()
}

#[aoc(day3, part2)]
pub fn solve_part2(input: &[String]) -> u32 {
    input.chunks(3).map(|group| {
        // println!("GROUP: {:?}", group);
        let x = &group[0];
        let y = &group[1];
        let z = &group[2];
        let shared = x.chars().find(|c| y.contains(*c) && z.contains(*c)).unwrap();
        priority(shared)
    }).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE: &str = "vJrwpWtwJgWrhcsFMMfFFhFp\njqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL\nPmmdzqPrVvPwwTWBwg\nwMqvLMZHhHMvwLHjbvcjnnSBnvTQFn\nttgJtRGJQctTZtZT\nCrZsJsPPZsGzwwsLwLmpwMDw";
    const SAMPLE1: &str = "vJrwpWtwJgWrhcsFMMfFFhFp";

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

    #[test]
    fn sample5() {
        let data = gen(SAMPLE);
        assert_eq!(part2(&data), 70);
    }
}
