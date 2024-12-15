use std::collections::HashMap;

fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    // Parse map
    let mut frequency_map: HashMap<char, Vec<(i32, i32)>> = HashMap::new();
    for (x, line) in input.lines().enumerate() {
        for (y, c) in line.chars().enumerate() {
            if c == '.' {
                continue;
            }
            match frequency_map.get_mut(&c) {
                Some(v) => {
                    v.push((x as i32, y as i32));
                }
                None => {
                    frequency_map.insert(c, vec![(x as i32, y as i32)]);
                }
            }
        }
    }

    let rows = input.lines().count() as i32;
    let cols = input.lines().next().unwrap().len() as i32;

    let mut antinode_map = vec![vec![false; cols as usize]; rows as usize];

    let mut antinodes = 0;

    // Loop over each frequency
    for coords in frequency_map.values() {
        // Loop over each pair of nodes
        for i in 0..(coords.len() - 1) {
            for j in (i + 1)..coords.len() {
                // Calculate coords
                let (x1, y1) = coords[i];
                let (x2, y2) = coords[j];
                let (dx, dy) = (x1 - x2, y1 - y2);
                let (anti_x1, anti_y1) = (x1 + dx, y1 + dy);
                let (anti_x2, anti_y2) = (x2 - dx, y2 - dy);
                // Check if inside bounds
                if is_inside(anti_x1, anti_y1, rows, cols)
                    && !antinode_map[anti_x1 as usize][anti_y1 as usize]
                {
                    antinodes += 1;
                    antinode_map[anti_x1 as usize][anti_y1 as usize] = true;
                }
                if is_inside(anti_x2, anti_y2, rows, cols)
                    && !antinode_map[anti_x2 as usize][anti_y2 as usize]
                {
                    antinodes += 1;
                    antinode_map[anti_x2 as usize][anti_y2 as usize] = true;
                }
            }
        }
    }
    antinodes.to_string()
}

fn is_inside(x: i32, y: i32, rows: i32, cols: i32) -> bool {
    0 <= x && x < rows && 0 <= y && y < cols
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1(
            "............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............",
        );
        assert_eq!(result, "14");
    }
}
