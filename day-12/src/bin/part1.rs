use std::collections::VecDeque;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Parse input
    let map: Vec<Vec<_>> = input.lines().map(|line| line.chars().collect()).collect();

    let rows = map.len();
    let cols = map[0].len();

    // Find connected regions (BFS)
    let mut regions = Vec::new();
    let mut visited = vec![vec![false; cols]; rows];

    for x in 0..rows {
        for y in 0..cols {
            if visited[x][y] {
                continue;
            }
            // If we find a node we haven't visited, it's the start of a new region.
            // Perform BFS from this node until whole region is explored
            let plant = map[x][y];
            let mut region = Vec::new();
            let mut queue = VecDeque::from([(x, y)]);

            while !queue.is_empty() {
                let (x, y) = queue.pop_front().unwrap();
                if visited[x][y] {
                    continue;
                }
                visited[x][y] = true;
                region.push((x, y));
                // Look around to see if plants match and add to queue
                for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                    let (nx, ny) = (x as isize + dx, y as isize + dy);
                    if nx >= 0 && ny >= 0 {
                        let (nx, ny) = (nx as usize, ny as usize);
                        if nx < rows && ny < cols && !visited[nx][ny] && map[nx][ny] == plant {
                            queue.push_back((nx, ny));
                        }
                    }
                }
            }
            // Push final region to list of regions
            regions.push((plant, region));
        }
    }

    // Calculate fence for each region
    let regions = regions.iter().map(|(plant, region)| {
        let mut fences = region.len() * 4;
        // Loop over each node and for each surrounding node with same plant, remove 1 fence
        for (x, y) in region {
            for (dx, dy) in &[(1, 0), (-1, 0), (0, 1), (0, -1)] {
                let (nx, ny) = (*x as isize + dx, *y as isize + dy);
                if nx >= 0 && ny >= 0 {
                    let (nx, ny) = (nx as usize, ny as usize);
                    if nx < rows && ny < cols && map[nx][ny] == *plant {
                        fences -= 1;
                    }
                }
            }
        }
        let area = region.len();
        (plant, region, area, fences)
    }).collect::<Vec<_>>();

    let result: usize = regions.iter().map(|(_, _, area, region)| area * region).sum();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = vec![
            (
                "AAAA
BBCD
BBCC
EEEC",
                "140",
            ),
            (
                "OOOOO
OXOXO
OOOOO
OXOXO
OOOOO",
                "772",
            ),
            (
                "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE",
                "1930",
            ),
        ];

        for (input, result) in cases {
            assert_eq!(result, part1(input));
        }
    }
}
