#[aoc_generator(day2)]
pub fn input_generator(input: &str) -> Vec<Vec<i64>> {
    let out: Vec<Vec<i64>> = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect()
        })
        .collect();
    out
}

#[aoc(day2, part1)]
pub fn part1(input: &Vec<Vec<i64>>) -> i64 {
    input
        .iter()
        .fold(0, |sum, levels| if is_safe(levels) { sum + 1 } else { sum })
}

#[aoc(day2, part2)]
pub fn part2(input: &Vec<Vec<i64>>) -> i64 {
    input.iter().fold(0, |sum, levels| {
        if safe_with_removal(levels) {
            sum + 1
        } else {
            sum
        }
    })
}

pub fn is_safe(levels: &Vec<i64>) -> bool {
    let diffs = levels
        .windows(2)
        .map(|arr| (arr[0] - arr[1]))
        .collect::<Vec<_>>();
    let all_incr = diffs.iter().all(|a| *a >= 0);
    let all_decr = diffs.iter().all(|a| *a <= 0);

    if !(all_incr || all_decr) {
        return false;
    }

    diffs.iter().all(|a| {
        let b = a.abs();
        return b >= 1 && b <= 3;
    })
}

pub fn safe_with_removal(levels: &Vec<i64>) -> bool {
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
    fn test_part1_sample() {
        let input = indoc! {"
            7 6 4 2 1
            1 2 7 8 9
            9 7 6 2 1
            1 3 2 4 5
            8 6 4 4 1
            1 3 6 7 9
        "};
        println!("{}", input);
        assert_eq!(part1(&input_generator(input)), 2);
    }

    #[test]
    fn test_is_safe() {
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
