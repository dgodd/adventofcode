// To run: rustc --test intcode.rs && ./intcode

pub fn intcode(arr: &[usize]) -> usize {
    let mut code = Vec::new();
    code.extend_from_slice(arr);
    let mut pos: usize = 0;
    loop {
        let opcode = code[pos];
        if opcode == 99 {
            return code[0];
        }
        let a = code[code[pos + 1]];
        let b = code[code[pos + 2]];
        let store_offset = code[pos + 3];
        let ref mut store = code[store_offset];
        match opcode {
            1 => *store = a + b,
            2 => *store = a * b,
            // 99 => return code[0],
            _ => panic!("unexpected opcode"),
        }
        pos += 4;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let input = vec![1,0,0,0,99];
        assert_eq!(2, intcode(&input));
    }

    #[test]
    fn example_2() {
        let input = vec![2,3,0,3,99];
        assert_eq!(2, intcode(&input));
    }

    #[test]
    fn example_4() {
        let input = vec![1,1,1,4,99,5,6,0,99];
        assert_eq!(30, intcode(&input));
    }

    const INPUT: [usize; 141] = [1,0,0,3,1,1,2,3,1,3,4,3,1,5,0,3,2,13,1,19,1,19,10,23,1,23,6,27,1,6,27,31,1,13,31,35,1,13,35,39,1,39,13,43,2,43,9,47,2,6,47,51,1,51,9,55,1,55,9,59,1,59,6,63,1,9,63,67,2,67,10,71,2,71,13,75,1,10,75,79,2,10,79,83,1,83,6,87,2,87,10,91,1,91,6,95,1,95,13,99,1,99,13,103,2,103,9,107,2,107,10,111,1,5,111,115,2,115,9,119,1,5,119,123,1,123,9,127,1,127,2,131,1,5,131,0,99,2,0,14,0];

    #[test]
    fn answer_1() {
        let mut input = INPUT.clone();
        input[1] = 12;
        input[2] = 2;
        assert_eq!(5866714, intcode(&input));
    }

    #[test]
    fn answer_2() {
        let mut noun = 0;
        let mut verb = 0;
        for x in 0..100 {
            for y in 0..100 {
                let mut input = INPUT.clone();
                input[1] = x;
                input[2] = y;
                if intcode(&input) == 19690720 {
                    noun = x;
                    verb = y;
                    break;
                }
            }
        }
        assert_eq!(52, noun);
        assert_eq!(8, verb);
        assert_eq!(5208, (100 * noun) + verb);
    }
}
