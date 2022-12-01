#[aoc_generator(day1)]
pub fn input_generator(input: &str) -> Vec<Vec<u32>> {
    let mut out = vec![];
    let mut last = vec![];
    for l in input.lines() {
        match l.parse() {
            Ok(n) => last.push(n),
            Err(_) => { out.push(last); last = vec![] },
        }
    }
    out.push(last);
    out
}

#[aoc(day1, part1)]
pub fn solve_part1(input: &[Vec<u32>]) -> u32 {
    input
        .iter()
        .map(|v| v.into_iter().sum())
        .max().unwrap()
}

#[aoc(day1, part2)]
pub fn solve_part2(input: &[Vec<u32>]) -> u32 {
    let mut arr = input
        .iter()
        .map(|v| v.into_iter().sum())
        .collect::<Vec<u32>>();
    arr.sort();
    arr.reverse();
    arr.iter().take(3).sum()
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    #[test]
    fn sample1() {
        assert_eq!(gen("1\n2\n\n3\n\n4\n5\n\n6\n"), vec![vec![1, 2], vec![3], vec![4,5], vec![6]]);
    }

    #[test]
    fn sample2() {
        assert_eq!(part1(&vec![vec![1, 2], vec![3], vec![4,5], vec![6]]), 9);
    }

    #[test]
    fn sample3() {
        let data = gen("1000\n2000\n3000\n\n4000\n\n5000\n6000\n\n7000\n8000\n9000\n\n10000\n");
        assert_eq!(part1(&data), 24000);
        assert_eq!(part2(&data), 45000);
    }
}
