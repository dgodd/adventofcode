use regex::Regex;

#[aoc(day3, part1)]
pub fn part1(input: &str) -> i64 {
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let caps: Vec<_> = re.captures_iter(input).map(|r| (r[1].parse::<i64>().unwrap(), r[2].parse::<i64>().unwrap())).collect();
    caps.iter().fold(0, |sum, (a,b)| sum + (a * b))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_numbers_example() {
        let input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        assert_eq!(part1(&input), 161);
    }
}
