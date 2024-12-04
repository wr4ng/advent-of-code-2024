fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let chars = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    for i in 1..(chars.len() - 1) {
        for j in 1..(chars[i].len() - 1) {
            print!("{}", chars[i][j]);
            // If we find an "A", look diagonally for two "MAS"
            if chars[i][j] == 'A' {
                let diagonal_one = (chars[i - 1][j - 1] == 'M' && chars[i + 1][j + 1] == 'S') ||
                                   (chars[i - 1][j - 1] == 'S' && chars[i + 1][j + 1] == 'M');
                let diagonal_two = (chars[i - 1][j + 1] == 'M' && chars[i + 1][j - 1] == 'S') ||
                                   (chars[i - 1][j + 1] == 'S' && chars[i + 1][j - 1] == 'M');

                if diagonal_one && diagonal_two {
                    count += 1;
                }
            }
        }
        println!("");
    }

    count.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX",
        );
        assert_eq!(result, "9");
    }
}
