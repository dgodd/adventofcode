use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let caps: Vec<_> = re
        .captures_iter(input)
        .map(|r| (r[1].parse::<i64>().unwrap(), r[2].parse::<i64>().unwrap()))
        .collect();
    caps.iter().fold(0, |sum, (a, b)| sum + (a * b))
}

#[aoc(day3, part2)]
pub fn part2(input: &str) -> i64 {
    let re = Regex::new(r"^mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let mut enabled = true;
    let mut sum = 0;
    for i in 0..input.len() {
        let curr = &input[i..];
        if curr.starts_with("do()") {
            enabled = true;
        } else if curr.starts_with("don't()") {
            enabled = false;
        } else if enabled && re.is_match(curr) {
            let r = re.captures(curr).unwrap();
            sum += r[1].parse::<i64>().unwrap() * r[2].parse::<i64>().unwrap();
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&input), 48);
    }
}
