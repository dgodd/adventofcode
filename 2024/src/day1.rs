#[aoc(day1, part1)]
pub fn part1(input: &str) -> i64 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        // println!("LINE {:?}", line);
        let a: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        // println!("A: {:?}", a);
        list1.push(a[0]);
        list2.push(a[1]);
    }
    println!("LIST 1 {:?}", list1);
    println!("LIST 2 {:?}", list2);

    list1.sort();
    list2.sort();
    println!("LIST 1 {:?}", list1);
    println!("LIST 2 {:?}", list2);

    list1
        .iter()
        .zip(list2)
        .fold(0, |sum, (a, b)| sum + (a - b).abs())
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u64 {
    let mut list1 = Vec::new();
    let mut list2 = Vec::new();
    for line in input.lines() {
        // println!("LINE {:?}", line);
        let a: Vec<i64> = line
            .split_whitespace()
            .map(|i| i.parse().unwrap())
            .collect();
        // println!("A: {:?}", a);
        list1.push(a[0]);
        list2.push(a[1]);
    }

    list1.iter().fold(0, |sum, a| {
        sum + ((*a as u64) * list2.iter().filter(|b| &a == b).count() as u64)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_numbers_example() {
        let input = "3   4\n4   3\n2   5\n1   3\n3   9\n3   3";
        assert_eq!(part2(input), 31);
    }
}
