use std::iter::successors;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Parse input
    let parsed = input.lines().map(|line| {
        let (l, r) = line.split_once(": ").unwrap();
        let result = l.parse::<u64>().unwrap();
        let operands = r
            .split(' ')
            .map(|n| n.parse::<u64>().unwrap())
            .collect::<Vec<_>>();
        (result, operands)
    });

    let res = parsed
        .filter_map(|(result, operands)| {
            if matches_result(operands[0], &operands[1..], result) {
                return Some(result);
            }
            None
        })
        .sum::<u64>();

    res.to_string()
}

fn matches_result(value: u64, operands: &[u64], result: u64) -> bool {
    if operands.is_empty() {
        return value == result;
    }
    // Concatenate operator
    let digits = successors(Some(operands[0]), |&n| (n >= 10).then_some(n / 10)).count();
    let con_value = value * 10_u64.pow(digits as u32) + operands[0];

    if matches_result(con_value, &operands[1..], result) {
        return true;
    }
    // Add operator
    if matches_result(value + operands[0], &operands[1..], result) {
        return true;
    }
    // Multiply operator
    match value.checked_mul(operands[0]) {
        Some(v) => matches_result(v, &operands[1..], result),
        None => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20",
        );
        assert_eq!(result, "11387");
    }
}
