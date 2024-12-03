fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let reports = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let safe_reports = reports
        .iter()
        .filter_map(|report| {
            if is_safe(report) {
                Some(1)
            } else {
                // Look through all slices with 1 element removed
                let match_removal = (0..report.len())
                    .map(|i| [&report[..i], &report[i + 1..]].concat())
                    .any(|rep| is_safe(&rep));
                if match_removal {
                    Some(1)
                } else {
                    None
                }
            }
        })
        .sum::<i32>();

    safe_reports.to_string()
}

fn is_safe(report: &Vec<i32>) -> bool {
    // Turn to diffs
    let diffs = report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>();

    // Check constrains
    let positive = diffs.iter().all(|d| d > &0);
    let negative = diffs.iter().all(|d| d < &0);
    let correct_range = diffs.iter().all(|d| 1 <= d.abs() && d.abs() <= 3);
    (positive || negative) && correct_range
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "4");
    }
}
