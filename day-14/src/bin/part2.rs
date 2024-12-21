fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input, (101, 103));
    dbg!(output);
}

fn part2(input: &str, (x, y): (usize, usize)) -> String {
    // Parse input
    let robots = input
        .lines()
        .map(|line| {
            let (p, v) = line.split_once(' ').unwrap();

            let p = p.strip_prefix("p=").unwrap();
            let (px, py) = p.split_once(',').unwrap();
            let (px, py) = (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap());

            let v = v.strip_prefix("v=").unwrap();
            let (vx, vy) = v.split_once(',').unwrap();
            let (vx, vy) = (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap());

            ((px, py), (vx, vy))
        })
        .collect::<Vec<_>>();

    let result;
    let mut i = 0;

    'outer: loop {
        // Generate map
        let mut map = vec![vec![0; y]; x];
        for ((px, py), (vx, vy)) in robots.iter() {
            let (nx, ny) = (
                (px + vx * i).rem_euclid(x as i32),
                (py + vy * i).rem_euclid(y as i32),
            );
            map[nx as usize][ny as usize] += 1;
        }
        // Print map
        for y in 0..y {
            let mut line = String::new();
            for x in 0..x {
                let c = map[x][y].to_string();
                line.push_str(if map[x][y] > 0 { &c } else { "." });
            }
            if line.contains("1111111111111111111111111111111") {
                result = i;
                break 'outer;
            }
        }
        i += 1;
    }

    result.to_string()
}
