fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

#[derive(Debug, PartialEq)]
enum Entry {
    Empty(u32),
    Id(u32, u32),
}

fn part2(input: &str) -> String {
    // Parse input
    let mut current_id = 0;
    let mut file_system: Vec<Entry> = Vec::with_capacity(input.len());
    for (i, c) in input.lines().next().unwrap().chars().enumerate() {
        let n = c.to_digit(10).unwrap();
        if i % 2 == 0 {
            file_system.push(Entry::Id(n, current_id));
            current_id += 1;
        } else {
            file_system.push(Entry::Empty(n));
        }
    }

    // Loop through and swap if there is space
    let mut back = file_system.len() - 1;
    loop {
        if back == 0 {
            break;
        }
        match file_system[back] {
            Entry::Empty(_) => {
                back -= 1;
                continue;
            }
            Entry::Id(n, _) => {
                for i in 0..=(back - 1) {
                    match file_system[i] {
                        Entry::Empty(l) if l >= n => {
                            // Found space swap and possible add missing space
                            file_system.swap(i, back);
                            if l > n {
                                file_system[back] = Entry::Empty(n);
                                file_system.insert(i + 1, Entry::Empty(l - n));
                            }
                            break
                        }
                        _ => {
                            continue;
                        }
                    }
                }
                back -= 1;
            }
        }
    }

    // Calculate checksumA
    let mut i = 0;
    let mut checksum = 0;
    for entry in file_system {
        match entry {
            Entry::Empty(n) => {i += n;}
            Entry::Id(n, id) => {
                for _ in 0..n {
                    checksum += i as u64 * id as u64;
                    i += 1;
                }
            }
        }
    }
    checksum.to_string()
}

#[allow(dead_code)]
fn print_clean(s: &[Entry]) {
    println!(
        "{}",
        s.iter()
            .map(|e| {
                match e {
                    Entry::Empty(n) => ".".to_string().repeat(*n as usize),
                    Entry::Id(n, id) => id.to_string().repeat(*n as usize),
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
        let result = part2("2333133121414131402");
        assert_eq!(result, "2858");
    }
}
