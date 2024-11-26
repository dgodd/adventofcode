#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Vec<(u64, u64)> {
    let vecs: Vec<_> = input.lines().map(|l| l.as_bytes().to_vec()).collect();
    find_hashes(&vecs)
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<(u64, u64)>) -> u64 {
    let input2 = expand(input, 1);
    paired_distances_summed(&input2)
}

fn paired_distances_summed(input: &Vec<(u64, u64)>) -> u64 {
    let mut count = 0;
    for (i, pos) in input.iter().enumerate() {
        // println!("PAIR: {:?} => {:?}", i, pos);
        for other in input[i + 1..].iter() {
            let dist =
                (pos.0 as i64 - other.0 as i64).abs() + (pos.1 as i64 - other.1 as i64).abs();
            // println!("  OTHER: {:?} -> {:?}", other, dist);
            count += dist;
        }
    }
    count as u64
}

#[aoc(day11, part2)]
pub fn part2(input: &Vec<(u64, u64)>) -> u64 {
    let input2 = expand(input, 1000000 - 1);
    paired_distances_summed(&input2)
}

pub fn expand(input: &Vec<(u64, u64)>, expansion: u64) -> Vec<(u64, u64)> {
    let mut out = input.clone();
    for i in (0..out.len() as u64).rev() {
        if !out.iter().any(|pair| pair.0 == i) {
            // println!("EXPAND 0: {}", i);
            for pair in out.iter_mut() {
                if pair.0 > i {
                    pair.0 += expansion;
                }
            }
        }
        if !out.iter().any(|pair| pair.1 == i) {
            // println!("EXPAND 1: {}", i);
            for pair in out.iter_mut() {
                if pair.1 > i {
                    pair.1 += expansion;
                }
            }
        }
    }
    out
}

pub fn find_hashes(input: &Vec<Vec<u8>>) -> Vec<(u64, u64)> {
    let mut arr = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '#' as u8 {
                arr.push((i as u64, j as u64));
            }
        }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parts_example() {
        let input = parse_input_day11(
            "...#......\n\
            .......#..\n\
            #.........\n\
            ..........\n\
            ......#...\n\
            .#........\n\
            .........#\n\
            ..........\n\
            .......#..\n\
            #...#.....",
        );
        assert_eq!(part1(&input), 374);

        let input10 = expand(&input, 10 - 1);
        assert_eq!(paired_distances_summed(&input10), 1030);

        let input100 = expand(&input, 100 - 1);
        assert_eq!(paired_distances_summed(&input100), 8410);
    }

    #[test]
    fn expand_example() {
        assert_eq!(expand(&vec!((0, 0), (0, 2)), 1), [(0, 0), (0, 3)]);
        assert_eq!(expand(&vec!((0, 0), (2, 0)), 1), [(0, 0), (3, 0)]);
    }

    #[test]
    fn find_hashes_example() {
        assert_eq!(
            find_hashes(&vec!(b"#..".to_vec(), b"...".to_vec(), b".#.".to_vec())),
            vec![(0, 0), (2, 1)]
        );
    }
}
