// #[aoc_generator(day2)]
// pub fn input_generator(input: &str) -> (Vec<i64>, Vec<i64>) {
// }

#[aoc(day2, part1)]
pub fn part1(input: &str) -> u64 {
    input.lines().fold(0, |sum, line| {
        let levels: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if is_safe(&levels) {
            sum + 1
        } else {
            sum
        }
    })
}

#[aoc(day2, part2)]
pub fn part2(input: &str) -> u64 {
    input.lines().fold(0, |sum, line| {
        let levels: Vec<u64> = line
            .split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        if safe_with_removal(&levels) {
            sum + 1
        } else {
            sum
        }
    })
}

pub fn is_safe(levels: &Vec<u64>) -> bool {
    let mut sorted = levels.clone();
    sorted.sort();

    let mut revsersed = levels.clone();
    revsersed.sort_by(|a, b| b.cmp(a));

    let all_increasing = &sorted == levels;
    let all_decreasing = &revsersed == levels;

    let diffs = levels
        .iter()
        .map(|a| (*a as i64))
        .collect::<Vec<_>>()
        .windows(2)
        .map(|arr| (arr[0] - arr[1]).abs())
        .all(|a| a >= 1 && a <= 3);
    // println!("DIFFS: {:?} => {:?}", levels, diffs);

    return (all_increasing || all_decreasing) && diffs;
}

pub fn safe_with_removal(levels: &Vec<u64>) -> bool {
    if is_safe(levels) {
        return true;
    }

    for i in 0..levels.len() {
        let mut arr = levels.clone();
        arr.remove(i);
        // println!("DROPPED: {:?} -> {:?}", levels, arr);
        if is_safe(&arr) {
            return true;
        }
    }

    return false;
}

#[cfg(test)]
mod tests {
    use super::*;
    use indoc::indoc;

    #[test]
    fn find_numbers_example() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        println!("{}", input);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn test_safe() {
        assert_eq!(is_safe(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(is_safe(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(is_safe(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 2, 4, 5]), false);
        assert_eq!(is_safe(&vec![8, 6, 4, 4, 1]), false);
        assert_eq!(is_safe(&vec![1, 3, 6, 7, 9]), true);
    }

    #[test]
    fn test_safe_with_removal() {
        assert_eq!(safe_with_removal(&vec![7, 6, 4, 2, 1]), true);
        assert_eq!(safe_with_removal(&vec![1, 2, 7, 8, 9]), false);
        assert_eq!(safe_with_removal(&vec![9, 7, 6, 2, 1]), false);
        assert_eq!(safe_with_removal(&vec![1, 3, 2, 4, 5]), true);
        assert_eq!(safe_with_removal(&vec![8, 6, 4, 4, 1]), true);
        assert_eq!(safe_with_removal(&vec![1, 3, 6, 7, 9]), true);
    }
}
