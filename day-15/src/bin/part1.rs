fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Clone)]
enum Tile {
    Wall,
    Empty,
    Box,
}

#[derive(Clone, Debug)]
enum Move {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

fn part1(input: &str) -> String {
    let (map_str, moves) = input.split_once("\n\n").unwrap();

    // Parse map
    let height = map_str.lines().count();
    let width = map_str.lines().next().unwrap().len();
    let map = vec![vec![Tile::Empty; width]; height];
    let (mut map, mut robot) =
        map_str
            .lines()
            .enumerate()
            .fold((map, (0, 0)), |(map, robot), (x, line)| {
                line.chars()
                    .enumerate()
                    .fold((map, robot), |(mut map, mut robot), (y, c)| {
                        match c {
                            '#' => map[x][y] = Tile::Wall,
                            'O' => map[x][y] = Tile::Box,
                            '.' => map[x][y] = Tile::Empty,
                            '@' => robot = (x, y),
                            _ => {
                                panic!("invalid character: {}", c);
                            }
                        };
                        (map, robot)
                    })
            });

    // Parse moves
    let moves: Vec<Move> = moves
        .trim()
        .chars()
        .filter_map(|c| match c {
            '^' => Some(Move::Up),
            '>' => Some(Move::Right),
            'v' => Some(Move::Down),
            '<' => Some(Move::Left),
            '\n' => None,
            _ => panic!("invalid move: '{}'", c),
        })
        .collect();

    // (try to) perform moves
    for m in &moves {
        // Starting from robot, loop in move direction:
        // Skip over any boxes
        // If hit a wall, move cannot be performed
        // If hit an empty space, move all found boxes + robot one in the move direction
        let (dx, dy) = match m {
            Move::Up => (-1, 0),
            Move::Right => (0, 1),
            Move::Down => (1, 0),
            Move::Left => (0, -1),
        };
        let (mut x, mut y) = (robot.0 as i32 + dx, robot.1 as i32 + dy);
        let mut i = 1;
        while 0 <= x && x < height as i32 && 0 <= y && y < width as i32 {
            let (ux, uy) = (x as usize, y as usize);
            match map[ux][uy] {
                Tile::Wall => break,
                Tile::Box => (),
                Tile::Empty => {
                    // Move robot + boxes
                    for j in 0..(i - 1) {
                        let current = (x - j * dx, y - j * dy);
                        map[current.0 as usize][current.1 as usize] = Tile::Box;
                    }
                    robot = ((x - (i - 1) * dx) as usize, (y - (i - 1) * dy) as usize);
                    map[robot.0][robot.1] = Tile::Empty;
                    break;
                }
            }
            x += dx;
            y += dy;
            i += 1;
        }
    }

    // Calculate GPS score
    let score: usize = map
        .iter()
        .enumerate()
        .flat_map(|(x, row)| row.iter().enumerate().map(move |(y, t)| (t, x, y)))
        .map(|(t, x, y)| match t {
            &Tile::Wall | &Tile::Empty => 0,
            &Tile::Box => 100 * x + y,
        }).sum();

    score.to_string()
}

#[allow(unused)]
fn print_map(map: &[Vec<Tile>], robot: (usize, usize)) {
    map.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, t)| match t {
            Tile::Box => print!("O"),
            Tile::Wall => print!("#"),
            Tile::Empty if (x, y) == robot => print!("@"),
            Tile::Empty => print!("."),
        });
        println!();
    });
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works_part_one() {
        let cases = vec![
            (
                "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<",
                "2028",
            ),
            (
                "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^",
                "10092",
            ),
        ];

        for (input, result) in cases {
            assert_eq!(*result, part1(input));
        }
    }
}
