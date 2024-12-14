fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

const DIRECTIONS: [(i32, i32); 4] = [(-1, 0), (0, 1), (1, 0), (0, -1)];

#[derive(Debug, Clone, PartialEq)]
enum Tile {
    Obstacle,
    Empty,
}

fn part2(input: &str) -> String {
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

    // Let guard walk
    let (mut i, mut j) = guard_start;
    let mut dir_index = 0;
    let mut time_loops = 0;
    let mut used = vec![vec![false; cols as usize]; rows as usize];
    used[i as usize][j as usize] = true;

    loop {
        // Calculate next position
        let (dx, dy) = DIRECTIONS[dir_index];
        let (new_i, new_j) = (i + dx, j + dy);
        // If it's outside the map, stop
        let outside_map = new_i < 0 || rows <= new_i || new_j < 0 || cols <= new_j;
        if outside_map {
            break;
        }

        match map[new_i as usize][new_j as usize] {
            Tile::Obstacle => {
                // Turn right when reaching an obstacle
                dir_index = (dir_index + 1) % DIRECTIONS.len();
            }
            Tile::Empty => {
                if !used[new_i as usize][new_j as usize] {
                    // Check if there is a time-loop if it was an obstacle instead of Empty
                    if has_time_loop(&map, guard_start, 0, (new_i, new_j)) {
                        time_loops += 1;
                    }
                    used[new_i as usize][new_j as usize] = true;
                }
                // Continue, update coords and increment amount of tiles used if it's unused
                (i, j) = (new_i, new_j);
            }
        }
    }
    time_loops.to_string()
}

// Simulate guard with extra obstacle
fn has_time_loop(
    map: &[Vec<Tile>],
    start: (i32, i32),
    start_dir: usize,
    obstacle: (i32, i32),
) -> bool {
    let rows = map.len() as i32;
    let cols = map[0].len() as i32;
    // Create map of booleans to keep track of if we're in a loop
    // I.e. used[x][y][i] indicates whether (x,y) has been visited in direction i.
    // If we reach a tile we have already reached with the same direction index, we're in a loop
    let mut used = vec![vec![vec![false; DIRECTIONS.len()]; cols as usize]; rows as usize];

    let (mut i, mut j) = start;
    let mut dir_index = start_dir;
    used[i as usize][j as usize][dir_index] = true;

    loop {
        // Calculate next position
        let (dx, dy) = DIRECTIONS[dir_index];
        let (new_i, new_j) = (i + dx, j + dy);
        // If it's outside the map, stop and return false (not a loop)
        let outside_map = new_i < 0 || rows <= new_i || new_j < 0 || cols <= new_j;
        if outside_map {
            return false;
        }
        match map[new_i as usize][new_j as usize] {
            Tile::Obstacle => {
                // Turn right when reaching an obstacle
                dir_index = (dir_index + 1) % DIRECTIONS.len();
            }
            Tile::Empty if (new_i, new_j) == obstacle => {
                // Turn right when reaching the extra obstacle
                dir_index = (dir_index + 1) % DIRECTIONS.len();
            }
            Tile::Empty => {
                // On Empty, update next coords
                (i, j) = (new_i, new_j);
                // If we have already been at the next tile from the given direction, rreturn true
                if used[i as usize][j as usize][dir_index] {
                    // Debug
                    //print_map_loop(map, &used, obstacle);
                    return true;
                }
            }
        }
        used[i as usize][j as usize][dir_index] = true;
    }
}

#[allow(dead_code)]
fn print_map_loop(map: &[Vec<Tile>], used: &[Vec<Vec<bool>>], obstacle: (i32, i32)) {
    for x in 0..map.len() {
        for y in 0..map[0].len() {
            let c = match map[x][y] {
                Tile::Obstacle => '#',
                Tile::Empty if (x as i32, y as i32) == obstacle => 'O',
                Tile::Empty if used[x][y].contains(&true) => 'X',
                Tile::Empty => '.',
            };
            print!("{c}");
        }
        println!();
    }
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
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
        assert_eq!(result, "6");
    }
}
