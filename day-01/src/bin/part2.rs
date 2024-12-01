use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let mut lefts = vec![];
    let mut rights = HashMap::new();

    for line in input.lines() {
        // Parse each line
        let (left, right) = line.split_once("   ").unwrap();
        let left: i32 = left.parse().unwrap();
        let right: i32 = right.parse().unwrap();
        // Append to list of left values + add occurences to right HashMap
        lefts.push(left);
        match rights.get(&right) {
            Some(i) => rights.insert(right, i + 1),
            None => rights.insert(right, 1)
        };
    }

    let result: i32 = lefts.iter().map(|l| {
        let occurences = rights.get(l).unwrap_or(&0);
        l * occurences
    }).sum();

    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "3   4
4   3
2   5
1   3
3   9
3   3",
        );
        assert_eq!(result, "31");
    }
}
