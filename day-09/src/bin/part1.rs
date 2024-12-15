fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum Entry {
    Empty,
    Id(u32),
}

fn part1(input: &str) -> String {
    // Parse input
    let mut current_id = 0;
    let mut file_system: Vec<Entry> = Vec::with_capacity(input.len());
    for (i, c) in input.lines().next().unwrap().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            for _ in 0..n {
                file_system.push(Entry::Id(current_id));
            }
            current_id += 1;
        } else {
            for _ in 0..n {
                file_system.push(Entry::Empty);
            }
        }
    }

    // Loop through and swap
    let mut front = 0;
    let mut back = file_system.len() - 1;
    loop {
        if front == back {
            break;
        }
        if file_system[front] != Entry::Empty {
            front += 1;
            continue;
        }
        if file_system[back] == Entry::Empty {
            back -= 1;
            continue;
        }
        // Perform swap
        file_system.swap(front, back);
    }

    // Calculate checksum
    let checksum = file_system.into_iter().enumerate().map(|(i, e)| {
        match e {
            Entry::Id(id) => i as u64 * id as u64,
            Entry::Empty => 0
        }
    }).sum::<u64>();

    checksum.to_string()
}

#[allow(dead_code)]
fn print_clean(s: &[Entry]) {
    println!(
        "{}",
        s.iter()
            .map(|e| {
                match e {
                    Entry::Empty => ".".to_string(),
                    Entry::Id(n) => n.to_string(),
                }
            })
            .collect::<String>()
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part1("2333133121414131402");
        assert_eq!(result, "1928");
    }
}
