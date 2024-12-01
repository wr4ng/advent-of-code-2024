fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (mut left, mut right) = input
        .lines()
        .fold((vec![], vec![]), |(mut lefts, mut rights), line| {
            // Parse each line and add to separate vec's
            let (left, right) = line.split_once("   ").unwrap();
            let left: i32 = left.parse().unwrap();
            let right: i32 = right.parse().unwrap();
            lefts.push(left);
            rights.push(right);
            (lefts, rights)
        });

    left.sort();
    right.sort();

    let diffs: i32 = left
        .iter()
        .zip(right.iter())
        .map(|(l, r)| (l - r).abs())
        .sum();

    diffs.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, "11");
    }
}
