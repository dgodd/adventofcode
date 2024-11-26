#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> u32 {
    input.len() as u32
}

pub fn expand_rows(input: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input
        .into_iter()
        .flat_map(|x| {
            if x.len() > 0 && x.iter().all(|y| *y == '.' as u8) {
                [x.clone(), x].to_vec()
            } else {
                [x].to_vec()
            }
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = vec![
            b"...#......".to_vec(),
            b".......#..".to_vec(),
            b"#.........".to_vec(),
            b"..........".to_vec(),
            b"......#...".to_vec(),
            b".#........".to_vec(),
            b".........#".to_vec(),
            b"..........".to_vec(),
            b".......#..".to_vec(),
            b"#...#.....".to_vec(),
        ];
        assert_eq!(part1(&input), 374);
    }

    #[test]
    fn expand_example() {
        assert_eq!(expand_rows(vec!(b"".to_vec())), [[]]);
        assert_eq!(expand_rows(vec!(b".#.".to_vec())), [b".#.".to_vec()]);
        assert_eq!(
            expand_rows(vec!(b"...".to_vec())),
            [b"...".to_vec(), b"...".to_vec()]
        );
        assert_eq!(
            expand_rows(vec!(b"#..".to_vec(), b"...".to_vec(), b".#.".to_vec())),
            [
                b"#..".to_vec(),
                b"...".to_vec(),
                b"...".to_vec(),
                b".#.".to_vec()
            ]
        );
    }
}
