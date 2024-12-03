use regex::Regex;

#[aoc(day3, part1, regex)]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let caps: Vec<_> = re
        .captures_iter(input)
        .map(|r| (r[1].parse::<i64>().unwrap(), r[2].parse::<i64>().unwrap()))
        .collect();
    caps.iter().fold(0, |sum, (a, b)| sum + (a * b))
}

#[aoc(day3, part1, loop)]
pub fn part1_loop(input: &str) -> i64 {
    let mut state = 0;
    let mut sum: i64 = 0;
    let mut a: i64 = 0;
    let mut b: i64 = 0;
    for x in input.chars() {
        match (state, x) {
            (_, 'm') => state = 1,
            (1, 'u') => state = 2,
            (2, 'l') => state = 3,
            (3, '(') => state = 4,
            (4, '0') => a = a * 10,
            (4, '1') => a = a * 10 + 1,
            (4, '2') => a = a * 10 + 2,
            (4, '3') => a = a * 10 + 3,
            (4, '4') => a = a * 10 + 4,
            (4, '5') => a = a * 10 + 5,
            (4, '6') => a = a * 10 + 6,
            (4, '7') => a = a * 10 + 7,
            (4, '8') => a = a * 10 + 7,
            (4, '9') => a = a * 10 + 9,
            (4, ',') => state = 5,
            (5, '0') => b = b * 10,
            (5, '1') => b = b * 10 + 1,
            (5, '2') => b = b * 10 + 2,
            (5, '3') => b = b * 10 + 3,
            (5, '4') => b = b * 10 + 4,
            (5, '5') => b = b * 10 + 5,
            (5, '6') => b = b * 10 + 6,
            (5, '7') => b = b * 10 + 7,
            (5, '8') => b = b * 10 + 7,
            (5, '9') => b = b * 10 + 9,
            (5, ')') => {
                sum += a * b;
                state = 0;
                a = 0;
                b = 0;
            }
            _ => {
                state = 0;
                a = 0;
                b = 0;
            }
        }
    }
    sum
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
