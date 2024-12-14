fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Obstacle,
    Empty,
}

fn part1(input: &str) -> String {
    let mut guard_start: (i32, i32) = (0, 0);
    let mut map: Vec<Vec<Tile>> = vec![];

    // Parse map
    for (i, line) in input.lines().enumerate() {
        let mut row: Vec<Tile> = vec![];
        for (j, c) in line.chars().enumerate() {
            row.push(match c {
                '#' => Tile::Obstacle,
                '.' => Tile::Empty,
                '^' => Tile::Empty,
                _ => panic!(),
            });
            if c == '^' {
                guard_start = (i as i32, j as i32);
            }
        }
        map.push(row);
    }

    // Create map of which tiles have been used
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;

    let mut used = vec![vec![false; cols as usize]; rows as usize];

    // Let guard walk
    let (mut i, mut j) = guard_start;
    let mut dir_index = 0;
    let directions = [(-1, 0), (0, 1), (1, 0), (0, -1)];
    used[i as usize][j as usize] = true;
    let mut used_tiles = 1;

    loop {
        // Calculate next position
        let (dx, dy) = directions[dir_index];
        let (new_i, new_j) = (i + dx, j + dy);
        // If it's outside the map, stop
        let outside_map = new_i < 0 || rows <= new_i || new_j < 0 || cols <= new_j;
        if outside_map {
            break;
        }

        match map[new_i as usize][new_j as usize] {
            Tile::Obstacle => {
                // Turn right when reaching an obstacle
                dir_index = (dir_index + 1) % directions.len();
            }
            Tile::Empty => {
                // On Empty, update coords and increment amount of tiles used if it's unused
                (i, j) = (new_i, new_j);
                if !used[i as usize][j as usize] {
                    used[i as usize][j as usize] = true;
                    used_tiles += 1;
                }
            }
        }
    }

    used_tiles.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...",
        );
        assert_eq!(result, "41");
    }
}
