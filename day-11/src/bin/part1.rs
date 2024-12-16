use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, 25);
    dbg!(output);
}

fn part1(input: &str, blinks: u64) -> String {
    // Parse input
    let stones = input
        .trim()
        .split(' ')
        .map(|s| s.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    let mut memo: HashMap<(u64, u64), u64> = HashMap::new();
    let total: u64 = stones
        .iter()
        .map(|s| count_stones(*s, blinks, &mut memo))
        .sum();
    total.to_string()
}

fn digits(n: u64) -> u32 {
    n.to_string().len() as u32
}

fn count_stones(stone: u64, depth: u64, memo: &mut HashMap<(u64, u64), u64>) -> u64 {
    // When reaching depth == 0, no more blinks, and current stone is the only stone
    if depth == 0 {
        return 1;
    }
    // Try to retrieve value from HashMap
    if let Some(&result) = memo.get(&(stone, depth)) {
        return result;
    }
    // Otherwise have to compute value
    // 1) If value is 0, turn to 1
    if stone == 0 {
        let result = count_stones(1, depth - 1, memo);
        memo.insert((stone, depth), result);
        return result;
    }
    // 2) If value has even digits
    let d = digits(stone);
    if d % 2 == 0 {
        // Split into two stones and compute results
        let left = stone / (10_u64.pow(d / 2));
        let right = stone % 10_u64.pow(d / 2);

        let left_result = count_stones(left, depth - 1, memo);
        let right_result = count_stones(right, depth - 1, memo);
        let result = left_result + right_result;

        memo.insert((stone, depth), result);
        return result;
    }
    // 3) Otherwise, multiply with 2024
    let result = count_stones(stone * 2024, depth - 1, memo);
    memo.insert((stone, depth), result);
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = vec![("0 1 10 99 999", 1, "7"), ("125 17", 6, "22")];

        for (input, blinks, result) in cases {
            assert_eq!(result, part1(input, blinks));
        }
    }
}
