#[aoc_generator(day11)]
fn parse_input_day11(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|l| l.as_bytes().to_vec()).collect()
}

#[aoc(day11, part1)]
pub fn part1(input: &Vec<Vec<u8>>) -> u32 {
    let input2 = &expand_columns(input);
    let input3 = &expand_rows(input2);
    let hashes = find_hashes(input3);
    let mut count = 0;
    for (i, pos) in hashes.iter().enumerate() {
        // println!("PAIR: {:?} => {:?}", i, pos);
        for other in hashes[i + 1..].iter() {
            let dist =
                (pos.0 as i32 - other.0 as i32).abs() + (pos.1 as i32 - other.1 as i32).abs();
            // println!("  OTHER: {:?} -> {:?}", other, dist);
            count += dist;
        }
    }
    // println!("COUNT: {}", count);
    count as u32
}

pub fn expand_rows(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    input
        .into_iter()
        .flat_map(|x| {
            if x.len() > 0 && x.iter().all(|y| *y == '.' as u8) {
                [x.clone(), x.clone()].to_vec()
            } else {
                [x.clone()].to_vec()
            }
        })
        .collect()
}

pub fn expand_columns(input: &Vec<Vec<u8>>) -> Vec<Vec<u8>> {
    let mut input2 = input.clone();
    for i in (0..input[0].len()).rev() {
        if input.iter().all(|row| row[i] == '.' as u8) {
            for row in input2.iter_mut() {
                row.insert(i, '.' as u8);
            }
        }
    }
    input2.clone()
}

pub fn find_hashes(input: &Vec<Vec<u8>>) -> Vec<(usize, usize)> {
    let mut arr = Vec::new();
    for i in 0..input.len() {
        for j in 0..input[i].len() {
            if input[i][j] == '#' as u8 {
                arr.push((i, j));
            }
        }
    }
    arr
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
    fn expand_rows_example() {
        assert_eq!(expand_rows(&vec!(b"".to_vec())), [[]]);
        assert_eq!(expand_rows(&vec!(b".#.".to_vec())), [b".#.".to_vec()]);
        assert_eq!(
            expand_rows(&vec!(b"...".to_vec())),
            [b"...".to_vec(), b"...".to_vec()]
        );
        assert_eq!(
            expand_rows(&vec!(b"#..".to_vec(), b"...".to_vec(), b".#.".to_vec())),
            [
                b"#..".to_vec(),
                b"...".to_vec(),
                b"...".to_vec(),
                b".#.".to_vec()
            ]
        );
    }
    #[test]
    fn expand_columns_example() {
        assert_eq!(expand_columns(&vec!(b"".to_vec())), [[]]);
        assert_eq!(
            expand_columns(&vec!(b"#.#.".to_vec())),
            [b"#..#..".to_vec()]
        );
    }

    #[test]
    fn find_hashes_example() {
        assert_eq!(
            find_hashes(&vec!(b"#..".to_vec(), b"...".to_vec(), b".#.".to_vec())),
            vec![(0, 0), (2, 1)]
        );
    }
}
