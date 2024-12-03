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

#[aoc(day3, part1, myloop)]
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
            (4, '0'...'9') => a = (a * 10) + (x as usize - '0' as usize) as i64,
            (4, ',') => state = 5,
            (5, '0'...'9') => b = (b * 10) + (x as usize - '0' as usize) as i64,
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

#[aoc(day3, part2, startswith)]
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
            let a = r[1].parse::<i64>().unwrap();
            let b = r[2].parse::<i64>().unwrap();
            sum += a * b;
            // println!("MUL: {}: {} * {} = {}", sum, a, b, a * b);
        }
    }
    sum
}

#[aoc(day3, part2, myloop)]
pub fn part2_loop(input: &str) -> i64 {
    let mut enabled = true;
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
            (4, '0'...'9') => a = (a * 10) + (x as usize - '0' as usize) as i64,
            (4, ',') => state = 5,
            (5, '0'...'9') => b = (b * 10) + (x as usize - '0' as usize) as i64,
            (5, ')') => {
                if enabled {
                    sum += a * b;
                }
                state = 0;
                a = 0;
                b = 0;
            }
            (_, 'd') => state = 10,
            (10, 'o') => state = 11,
            (11, '(') => state = 12,
            (12, ')') => {
                enabled = true;
                state = 0;
            }
            (11, 'n') => state = 12,
            (12, '\'') => state = 13,
            (13, 't') => state = 14,
            (14, '(') => state = 15,
            (15, ')') => {
                enabled = false;
                state = 0;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn test_part1_loop() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1_loop(&input), 161);
    }

    #[test]
    fn test_part2() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2(&input), 48);
    }

    #[test]
    fn test_part2_loop() {
        let input = "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        assert_eq!(part2_loop(&input), 48);
    }
}
