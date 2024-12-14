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
            // Operators expressed with each bit corresponding to 1 if multiply and 0 if add
            let mut operators: u64 = 0;
            let max: u64 = (1 << (operands.len() - 1)) - 1;

            while operators <= max {
                // Try to evaluate the equation with current operators
                let mut value = operands[0];
                for (i, operand) in operands.iter().enumerate().skip(1) {
                    let multiply = ((operators >> (i - 1)) & 1) == 1;
                    if multiply {
                        match value.checked_mul(*operand) {
                            Some(v) => value = v,
                            // Handle multiply overflow
                            None => {
                                operators += 1;
                                continue;
                            }
                        }
                    } else {
                        value += *operand
                    }
                }
                operators += 1;
                if result == value {
                    return Some(result);
                }
                if result < value {
                    continue;
                }
            }
            None
        })
        .sum::<u64>();

    res.to_string()
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
        assert_eq!(result, "3749");
    }
}
