use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

const DIRECTIONS: [(isize, isize); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn part1(input: &str) -> String {
    // Parse map
    let map: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.chars().map(|c| c.to_digit(10).unwrap()).collect())
        .collect();

    let rows = map.len();
    let cols = map[0].len();

    let mut total = 0;

    map.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, n)| {
            if *n != 0 {
                return;
            }
            // This may be the start of a hiking trail
            // Perform BFS from this position and see how many different 9's can be reached
            let mut visited = vec![vec![false; cols]; rows];
            visited[x][y] = true;
            let mut queue = VecDeque::from([(x, y)]);
            let mut trailheads = 0;

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                // If we reach a 9 we've found a trailhead
                if map[x][y] == 9 {
                    trailheads += 1;
                    continue;
                }
                // Add all surrounding nodes to queue, if:
                // - they haven't already been visited
                // - they value is 1 larger than current node
                for (dx, dy) in DIRECTIONS {
                    let (nx, ny) = (x as isize + dx, y as isize + dy);
                    if nx >= 0 && ny >= 0 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if nx < rows
                            && ny < cols
                            && !visited[nx][ny]
                            && map[nx][ny] == map[x][y] + 1
                        {
                            visited[nx][ny] = true;
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
            total += trailheads;

            // Print out trail
            //println!("({},{}) = {}", x, y, trailheads);
            //print_trail(&map, &visited);
        });
    });

    total.to_string()
}

#[allow(dead_code)]
fn print_trail(map: &[Vec<u32>], visited: &[Vec<bool>]) {
    for (x, row) in map.iter().enumerate() {
        for (y, v) in row.iter().enumerate() {
            print!(
                "{}",
                if visited[x][y] {
                    v.to_string()
                } else {
                    ".".to_string()
                }
            );
        }
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732",
        );
        assert_eq!(result, "36");
    }
}
