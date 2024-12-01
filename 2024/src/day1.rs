use indoc::indoc;

#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> (Vec<i64>, Vec<i64>) {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        let a: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        list1.push(a[0]);
        list2.push(a[1]);
    }
    (list1, list2)
}

#[aoc(day1, part1)]
pub fn part1(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (ref mut list1, ref mut list2) = input.clone();

    list1.sort();
    list2.sort();
    list1
        .iter()
        .zip(list2)
        .fold(0, |sum, (a, b)| sum + (*a - *b).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &(Vec<i64>, Vec<i64>)) -> i64 {
    let (list1, list2) = input;

    list1.iter().fold(0, |sum, a| {
        sum + ((*a as i64) * list2.iter().filter(|b| &a == b).count() as i64)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part2(&input_generator(input)), 31);
    }
}
