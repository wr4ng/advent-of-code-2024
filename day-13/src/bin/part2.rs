fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Parse each claw instance
    let claw_machines = input.split("\n\n").map(|machine| {
        let mut lines = machine.lines();
        let (x1, y1) = lines
            .next()
            .unwrap()
            .strip_prefix("Button A: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (x1, y1) = (x1.parse::<i64>().unwrap(), y1.parse::<i64>().unwrap());
        let (x2, y2) = lines
            .next()
            .unwrap()
            .strip_prefix("Button B: X+")
            .unwrap()
            .split_once(", Y+")
            .unwrap();
        let (x2, y2) = (x2.parse::<i64>().unwrap(), y2.parse::<i64>().unwrap());
        let (x_target, y_target) = lines
            .next()
            .unwrap()
            .strip_prefix("Prize: X=")
            .unwrap()
            .split_once(", Y=")
            .unwrap();
        let (x_target, y_target) = (
            x_target.parse::<i64>().unwrap(),
            y_target.parse::<i64>().unwrap(),
        );
        (
            x1,
            y1,
            x2,
            y2,
            x_target + 10000000000000,
            y_target + 10000000000000,
        )
    });

    let solutions = claw_machines.filter_map(|(x1, y1, x2, y2, x, y)| {
        let matrix = [[x1, x2], [y1, y2]];
        let result = [x, y];
        solve_2x2_integer(matrix, result)
    });

    let tokens: i64 = solutions.map(|(a, b)| a * 3 + b).sum();
    tokens.to_string()
}

fn solve_2x2_integer(matrix: [[i64; 2]; 2], result: [i64; 2]) -> Option<(i64, i64)> {
    let (a, b, c, d) = (matrix[0][0], matrix[0][1], matrix[1][0], matrix[1][1]);
    let (e, f) = (result[0], result[1]);

    // Calculate the determinant
    let det = a * d - b * c;
    if det == 0 {
        if a * f - e * c == 0 && b * f - e * d == 0 {
            // Infinite solutions
            return None;
        } else {
            // No solution
            return None;
        }
    }

    // Unique solution
    let x = e * d - b * f;
    if x % det != 0 {
        // No real first value
        return None;
    }
    let y = a * f - e * c;
    if y % det != 0 {
        return None;
    }
    Some((x / det, y / det))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = vec![(
            "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279",
            "875318608908",
        )];

        for (input, result) in cases {
            assert_eq!(result, part1(input));
        }
    }
}
