#[aoc(day4, part1)]
pub fn part1(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn find_numbers_example() {
        let input = indoc! {"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
        "};
        println!("{}", input);
        assert_eq!(part2(&input), 31);
    }
}
