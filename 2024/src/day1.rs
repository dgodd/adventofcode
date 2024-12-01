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
        println!("A: {:?}", a);
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

// #[aoc(day1, part2)]
// pub fn part2(input: &str) -> u32 {
//     input.lines().fold(0, |sum, line| {
//         let nums: Vec<_> = find_numbers(line);
//         // println!("{:?} -> {:?}", line, nums);
//         let first = nums[0];
//         let last = nums[nums.len() -1];
//         sum + (first * 10) + last
//     })
// }

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_numbers_example() {
        assert_eq!(find_numbers("two1nine"), &[2, 1, 9]);
    }
}
