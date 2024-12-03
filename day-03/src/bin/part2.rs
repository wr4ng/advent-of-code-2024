use regex::Regex;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // Trim input to remove "don't()...do()" (and "don't()...")
    let mut trimmed = input.to_string();
    loop {
        // Check if there are parts than can be removed
        if trimmed.contains("don't()") {
            let (before, after) = trimmed.split_once("don't()").unwrap();
            // If after doesn't contain a "do()", it's disabled
            if !after.contains("do()") {
                trimmed = before.to_string();
                break;
            }
            // Only remove until first "do()" in after
            let (_, end) = after.split_once("do()").unwrap();
            trimmed = before.to_string() + end;
        } else {
            break;
        }
    }
    // Find remaining "mul(X,Y)"
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    let result: i32 = re
        .captures_iter(&trimmed)
        .map(|cap| cap[1].parse::<i32>().unwrap() * cap[2].parse::<i32>().unwrap())
        .sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result =
            part2("xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))");
        assert_eq!(result, "48");
    }
}
