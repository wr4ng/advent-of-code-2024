fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let chars = input
        .lines()
        .map(|l| l.chars().collect::<Vec<_>>())
        .collect::<Vec<_>>();

    let mut count = 0;

    let dirs: Vec<(i32, i32)> = vec![
        (-1, -1),
        (0, -1),
        (1, -1),
        (-1, 0),
        (1, 0),
        (-1, 1),
        (0, 1),
        (1, 1),
    ];

    for i in 0..chars.len() {
        for j in 0..chars[i].len() {
            print!("{}", chars[i][j]);
            // If we find an "X", look in all 8 directions to see if it starts a "XMAS"
            if chars[i][j] == 'X' {
                for (dx, dy) in &dirs {
                    if check_char(&chars, 'M', (i as i32) + 1 * dx, (j as i32) + 1 * dy)
                        && check_char(&chars, 'A', (i as i32) + 2 * dx, (j as i32) + 2 * dy)
                        && check_char(&chars, 'S', (i as i32) + 3 * dx, (j as i32) + 3 * dy)
                    {
                        count += 1;
                    }
                }
            }
        }
        println!("");
    }

    count.to_string()
}

fn check_char(chars: &Vec<Vec<char>>, c: char, i: i32, j: i32) -> bool {
    if i < 0 || j < 0 {
        return false;
    }
    let (x, y) = (i as usize, j as usize);
    let is_inside = (x < chars.len()) && (y < chars[x].len());
    if is_inside {
        chars[x][y] == c
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
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
        assert_eq!(result, "18");
    }
}
