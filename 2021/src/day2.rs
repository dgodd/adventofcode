#[derive(Debug, PartialEq)]
pub enum Dir { F, U, D }

#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<(Dir, u32)> {
    input.lines().map(|l| {
        let arr: Vec<&str> = l.split_whitespace().collect();
        // l.parse().unwrap()
        let num = arr[1].parse().unwrap();
        let dir = match arr[0] {
            "forward" => Dir::F,
            "up" => Dir::U,
            "down" => Dir::D,
            _ => unreachable!(),
        };
        (dir, num)
    }).collect()
}

#[aoc(day2, part1)]
pub fn solve_part1(input: &[(Dir, u32)]) -> u32 {
    let mut x = 0;
    let mut depth = 0;
    for (dir, num) in input {
        match dir {
            Dir::F => x += num,
            Dir::U => depth -= num,
            Dir::D => depth += num,
        }
    }
    x * depth
}

#[aoc(day2, part2)]
pub fn solve_part2(input: &[(Dir, u32)]) -> u32 {
    let mut x = 0;
    let mut depth = 0;
    let mut aim = 0;
    for (dir, num) in input {
        match dir {
            Dir::U => aim -= num,
            Dir::D => aim += num,
            Dir::F => { x += num; depth += aim * num },
        }
    }
    x * depth
}

#[cfg(test)]
mod tests {
    use super::{Dir::{F, U, D}, input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE: &str = "forward 5\ndown 5\nforward 8\nup 3\ndown 8\nforward 2\n";

    #[test]
    fn sample1() {
        assert_eq!(gen(SAMPLE), vec![(F, 5), (D, 5), (F, 8), (U, 3), (D, 8), (F, 2)]);
    }

    #[test]
    fn sample2() {
        let data = gen(SAMPLE);
        assert_eq!(part1(&data), 150);
    }

    #[test]
    fn sample3() {
        let data = gen(SAMPLE);
        assert_eq!(part2(&data), 900);
    }
}
