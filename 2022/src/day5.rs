use regex::Regex;

#[derive(Debug)]
pub struct Input {
    crates: Vec<Vec<char>>,
    instructions: Vec<(u32, u8, u8)>,
}

#[aoc_generator(day5)]
pub fn input_generator(input: &str) -> Input {
    let pos = input.find("\n\n").unwrap();
    let (inp1, inp2) = input.split_at(pos);

    let num: usize = ((inp1.lines().next().unwrap().len() as f64) / 4.0).ceil() as usize;
    let crates = (0..num).map(|i| {
        let mut arr = vec![];
        for l in inp1.lines() {
            let line = l.as_bytes();
            let c = line[(i * 4) + 1] as char;
            if c != ' ' {
                arr.push(c);
            }
        }
        arr
    }).collect::<Vec<Vec<char>>>();

    let re = Regex::new(r"move ([0-9]+) from ([0-9]+) to ([0-9]+)").unwrap();
    let instructions = inp2.lines().filter(|l| l.len() > 0).map(|l| {
        let caps = re.captures(l).unwrap();
        (
            caps.get(1).unwrap().as_str().parse().unwrap(),
            caps.get(2).unwrap().as_str().parse::<u8>().unwrap() - 1,
            caps.get(3).unwrap().as_str().parse::<u8>().unwrap() - 1,
        )
    }).collect::<Vec<(u32, u8, u8)>>();

    Input{ crates: crates, instructions: instructions }
}

#[aoc(day5, part1)]
pub fn solve_part1(input: &Input) -> String {
    // let mut crates: Vec<&[char]> = input.crates.iter().map(|c| c.as_slice()).collect();
    let mut crates: Vec<Vec<char>> = input.crates.clone();
    for inst in &input.instructions {
        // println!("INST: {:?}", inst);
        for _ in 0..inst.0 {
            let c = crates[inst.1 as usize].remove(0);
            crates[inst.2 as usize].insert(0, c);
        }
        // println!("CS: {:?}", &cs);
        // println!("CRATES: {:?}", &crates);
    }
    crates.iter().map(|c| c[0]).collect()
}

#[aoc(day5, part2)]
pub fn solve_part2(input: &Input) -> String {
    let mut crates: Vec<Vec<char>> = input.crates.clone();
    for inst in &input.instructions {
        // println!("INST: {:?}", inst);
        let cs: Vec<char> = crates[inst.1 as usize].splice(0..inst.0 as usize, vec![]).collect();
        crates[inst.2 as usize].splice(0..0, cs);
        // println!("CRATES: {:?}", &crates);
    }
    crates.iter().map(|c| c[0]).collect()
}

#[cfg(test)]
mod tests {
    use super::{input_generator as gen, solve_part1 as part1, solve_part2 as part2};

    const SAMPLE: &str = "    [D]    \n[N] [C]    \n[Z] [M] [P]\n1   2   3 \n\nmove 1 from 2 to 1\nmove 3 from 1 to 3\nmove 2 from 2 to 1\nmove 1 from 1 to 2\n";

    #[test]
    fn sample1() {
        let data = gen(SAMPLE);
        assert_eq!(part1(&data), "CMZ");
    }

    #[test]
    fn sample2() {
        let data = gen(SAMPLE);
        assert_eq!(part2(&data), "MCD");
    }
}
