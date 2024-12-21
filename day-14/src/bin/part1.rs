fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input, (101, 103));
    dbg!(output);
}

fn part1(input: &str, (x, y): (i32, i32)) -> String {
    // Parse input
    let robots = input.lines().map(|line| {
        let (p, v) = line.split_once(' ').unwrap();

        let p = p.strip_prefix("p=").unwrap();
        let (px, py) = p.split_once(',').unwrap();
        let (px, py) = (px.parse::<i32>().unwrap(), py.parse::<i32>().unwrap());

        let v = v.strip_prefix("v=").unwrap();
        let (vx, vy) = v.split_once(',').unwrap();
        let (vx, vy) = (vx.parse::<i32>().unwrap(), vy.parse::<i32>().unwrap());

        ((px, py), (vx, vy))
    });

    // Calculate final positions
    let positions = robots.map(|((px, py), (vx, vy))| {
        let final_x = (px + vx * 100).rem_euclid(x);
        let final_y = (py + vy * 100).rem_euclid(y);
        (final_x, final_y)
    });

    // Calculate robots in each quadrant 
    let xmid = x / 2;
    let ymid = y / 2;
    let (q1, q2, q3, q4) = positions.fold((0, 0, 0, 0), |(q1, q2, q3, q4), (x, y)| {
        if x < xmid && y < ymid {
            return (q1 + 1, q2, q3, q4);
        }
        else if x > xmid && y < ymid {
            return (q1, q2 + 1, q3, q4);
        }
        else if x < xmid && y > ymid {
            return (q1, q2, q3 + 1, q4);
        }
        else if x > xmid && y > ymid {
            return (q1, q2, q3, q4 + 1);
        }
        (q1, q2, q3, q4)
    });

    // Return safety score
    let safety_score = q1 * q2 * q3 * q4;
    safety_score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let cases = vec![(
            "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3",
            (11, 7),
            "12",
        )];

        for (input, size, result) in cases {
            assert_eq!(result, part1(input, size));
        }
    }
}
