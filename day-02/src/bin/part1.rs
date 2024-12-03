fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let reports = input
        .lines()
        .map(|line| {
            line.split(" ")
                .map(|v| v.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    let diffs = reports
        .iter()
        .map(|report| report.windows(2).map(|w| w[0] - w[1]).collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let safe_reports = diffs.iter().filter_map(|report| {
        let positive = report.iter().all(|d| d > &0);
        let negative = report.iter().all(|d|d < &0);
        let correct_range = report.iter().all(|d| 1 <= d.abs() && d.abs() <= 3);
        if (positive || negative) && correct_range {
            Some(1)
        } else {
            None
        }
    }).sum::<i32>();

    safe_reports.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9",
        );
        assert_eq!(result, "2");
    }
}
