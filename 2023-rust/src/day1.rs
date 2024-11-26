#[aoc(day1, part1)]
pub fn part1_chars(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let nums: Vec<_> = line.chars().filter(|x| *x >= '0' && *x <= '9').map(|d| d as u32 - ('0' as u32)).collect();
        // print!("{:?}\n", nums);
        let first = nums[0];
        let last = nums[nums.len() -1];
        sum + (first * 10) + last
    })
}

#[aoc(day1, part2)]
pub fn part2(input: &str) -> u32 {
    input.lines().fold(0, |sum, line| {
        let nums: Vec<_> = find_numbers(line);
        // println!("{:?} -> {:?}", line, nums);
        let first = nums[0];
        let last = nums[nums.len() -1];
        sum + (first * 10) + last
    })
}

pub fn find_numbers(input: &str) -> Vec<u32> {
    let mut arr = [].to_vec();
    for i in 0..input.len() {
        let val = &input[i..];
        if val.starts_with("one") || val.starts_with("1") { arr.push(1) }
        else if val.starts_with("two") || val.starts_with("2") { arr.push(2) }
        else if val.starts_with("three") || val.starts_with("3") { arr.push(3) }
        else if val.starts_with("four") || val.starts_with("4") { arr.push(4) }
        else if val.starts_with("five") || val.starts_with("5") { arr.push(5) }
        else if val.starts_with("six") || val.starts_with("6") { arr.push(6) }
        else if val.starts_with("seven") || val.starts_with("7") { arr.push(7) }
        else if val.starts_with("eight") || val.starts_with("8") { arr.push(8) }
        else if val.starts_with("nine") ||val.starts_with("9") { arr.push(9) }
    }
    arr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn find_numbers_example() {
        assert_eq!(find_numbers("two1nine"), &[2, 1, 9]);
    }
}
