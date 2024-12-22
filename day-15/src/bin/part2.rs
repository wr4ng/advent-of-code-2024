use std::collections::HashSet;

fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
enum Tile {
    Wall,
    Empty,
    BoxLeft,
    BoxRight,
}

impl Tile {
    fn alternate(&self) -> Self {
        match self {
            Tile::BoxLeft => Tile::BoxRight,
            Tile::BoxRight => Tile::BoxLeft,
            _ => panic!("invalid Tile in alternate: {:?}", self),
        }
    }
}

#[derive(Clone, PartialEq, Debug)]
enum Move {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

fn part2(input: &str) -> String {
    let (map_str, moves) = input.split_once("\n\n").unwrap();

    // Parse map
    let height = map_str.lines().count();
    let width = map_str.lines().next().unwrap().len() * 2;
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
                            '#' => {
                                map[x][2 * y] = Tile::Wall;
                                map[x][2 * y + 1] = Tile::Wall;
                            }
                            'O' => {
                                map[x][2 * y] = Tile::BoxLeft;
                                map[x][2 * y + 1] = Tile::BoxRight;
                            }
                            '.' => (),
                            '@' => robot = (x, 2 * y),
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

    // Horizontal: Easy. Same as before. Look in direction until either wall or empty
    // If wall continue
    // If empty, move all box parts. Start with opposite of the first found (Left or Right)

    // Vertical: Less easy.
    // If there is box above we can move it if, for both parts:
    // 1. There is empty space above
    // 2. There is a box above that can be moved (recursive)
    // Tree-like call
    // If all above can be moved, move all one up or down
    // Can collect those to move

    // (try to) perform moves
    for m in moves {
        match m {
            Move::Right | Move::Left => horizontal_move(&mut map, &mut robot, m),
            Move::Up | Move::Down => vertical_move(&mut map, &mut robot, m),
        }
    }

    // Calculate GPS score
    let score: usize = map
        .into_iter()
        .enumerate()
        .flat_map(|(x, row)| row.into_iter().enumerate().map(move |(y, t)| (t, x, y)))
        .map(|(t, x, y)| match t {
            Tile::Wall | Tile::Empty | Tile::BoxRight => 0,
            Tile::BoxLeft => x * 100 + y,
        })
        .sum();

    score.to_string()
}

// Perform horizontal move of robot mutating map
fn horizontal_move(map: &mut [Vec<Tile>], robot: &mut (usize, usize), m: Move) {
    // Starting from robot, loop in move direction:
    // Skip over any boxes
    // If hit a wall, move cannot be performed
    // If hit an empty space, move all found boxes + robot one in the move direction
    let (dx, dy) = match m {
        Move::Right => (0, 1),
        Move::Left => (0, -1),
        Move::Up | Move::Down => panic!("invalid move in horizontal_move: {:?}", m),
    };
    let (mut x, mut y) = (robot.0 as i32 + dx, robot.1 as i32 + dy);
    let mut i = 1;
    let mut b = map[x as usize][y as usize];
    while 0 <= x && x < map.len() as i32 && 0 <= y && y < map[0].len() as i32 {
        let (ux, uy) = (x as usize, y as usize);
        match map[ux][uy] {
            Tile::Wall => break,
            Tile::BoxLeft | Tile::BoxRight => (),
            Tile::Empty => {
                // Move robot + boxes
                for j in 0..(i - 1) {
                    let current = (x - j * dx, y - j * dy);
                    b = b.alternate();
                    map[current.0 as usize][current.1 as usize] = b;
                }
                *robot = ((x - (i - 1) * dx) as usize, (y - (i - 1) * dy) as usize);
                map[robot.0][robot.1] = Tile::Empty;
                break;
            }
        }
        x += dx;
        y += dy;
        i += 1;
    }
}

// Perform horizontal move of robot mutating map
fn vertical_move(map: &mut Vec<Vec<Tile>>, robot: &mut (usize, usize), m: Move) {
    let dx = match m {
        Move::Up => -1,
        Move::Down => 1,
        _ => panic!("invalid move in vertical_move: {:?}", m),
    };

    // Check if move can be performed
    let (x, y) = ((robot.0 as isize + dx) as usize, robot.1);
    let (can_move, removes, updates) = can_move(map, (x, y), dx, HashSet::new(), HashSet::new());
    if can_move {
        *robot = (x, y);
        // Set empty for all
        for (x, y) in removes {
            map[x][y] = Tile::Empty;
        }
        // Move for all
        for ((x, y), t) in updates {
            map[x][y] = t;
        }
    }
}

type CanMoveResult = (
    bool,
    HashSet<(usize, usize)>,
    HashSet<((usize, usize), Tile)>,
);

// Check robot or box can move to (x,y)
// If (x,y) contains a box, recursively calls in same direction (dx) to see if that box can be
// moved. Collects HashSet of locations where the box-parts should be moved to if the result is
// true
fn can_move(
    map: &Vec<Vec<Tile>>,
    (x, y): (usize, usize),
    dx: isize,
    removes: HashSet<(usize, usize)>,
    updates: HashSet<((usize, usize), Tile)>,
) -> CanMoveResult {
    match map[x][y] {
        Tile::Empty => (true, removes, updates),
        Tile::Wall => (false, removes, updates),
        Tile::BoxLeft => {
            // Box can only move if both left and right box part can move to tile above (or below)
            let to_x = (x as isize + dx) as usize;
            let (above_left, r, u) = can_move(map, (to_x, y), dx, removes, updates);
            let (above_right, mut r, mut u) = can_move(map, (to_x, y + 1), dx, r, u);
            let can_move = above_left && above_right;
            if can_move {
                // If both parts of box can move, then both parts should be removed, and updated
                r.insert((x, y));
                r.insert((x, y + 1));
                u.insert(((to_x, y), Tile::BoxLeft));
                u.insert(((to_x, y + 1), Tile::BoxRight));
            }
            (can_move, r, u)
        }
        Tile::BoxRight => {
            // Box can only move if both left and right box part can move to tile above (or below)
            let to_x = (x as isize + dx) as usize;
            let (above_left, r, u) = can_move(map, (to_x, y), dx, removes, updates);
            let (above_right, mut r, mut u) = can_move(map, (to_x, y - 1), dx, r, u);
            let can_move = above_left && above_right;
            if can_move {
                // If both parts of box can move, then both parts should be removed, and updated
                r.insert((x, y));
                r.insert((x, y - 1));
                u.insert(((to_x, y), Tile::BoxRight));
                u.insert(((to_x, y - 1), Tile::BoxLeft));
            }
            (can_move, r, u)
        }
    }
}

#[allow(unused)]
fn print_map(map: &[Vec<Tile>], robot: (usize, usize)) {
    map.iter().enumerate().for_each(|(x, row)| {
        row.iter().enumerate().for_each(|(y, t)| match t {
            Tile::BoxLeft => print!("["),
            Tile::BoxRight => print!("]"),
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
    fn it_works_part_two() {
        let cases = vec![
            // Sample used to test box movement. No result provided
            //            (
            //                "#######
            //#...#.#
            //#.....#
            //#..OO@#
            //#..O..#
            //#.....#
            //#######
            //
            //<vv<<^^<<^^",
            //                "idk",
            //            ),
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
                "9021",
            ),
        ];

        for (input, result) in cases {
            assert_eq!(*result, part2(input));
        }
    }
}
