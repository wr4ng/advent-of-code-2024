fn main() {
    let input = include_str!("./input.txt");
    let output = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    // Split input into ordering rules and updates
    let (rules, updates) = input.split_once("\n\n").unwrap();

    // Parse rules
    let rules = rules
        .lines()
        .map(|line| {
            let (l, r) = line.split_once('|').unwrap();
            (l.parse::<i32>().unwrap(), r.parse::<i32>().unwrap())
        })
        .collect::<Vec<_>>();

    // Parse updates
    let updates = updates
        .lines()
        .map(|line| {
            line.split(',')
                .map(|n| n.parse::<i32>().unwrap())
                .collect::<Vec<_>>()
        })
        .collect::<Vec<_>>();

    // Find each update violating a rule, and the first rule it violates
    let wrong_updates = updates.into_iter().filter_map(|update| {
        // Check each unique pair in an update
        for i in 0..(update.len() - 1) {
            for j in (i + 1)..update.len() {
                // Check if pair violates a rule
                let violates = rules.contains(&(update[j], update[i]));
                if violates {
                    return Some((update, (i, j)));
                }
            }
        }
        None
    });

    let corrected_updates = wrong_updates.map(|(update, (l, r))| {
        let mut corrected = update.clone();

        // Swap initial rule (l, r)
        corrected.swap(l, r);

        // Check if all pairs comply with rules, otherwise swap and continue
        loop {
            let mut did_swap = false;
            for i in 0..(corrected.len() - 1) {
                for j in (i + 1)..corrected.len() {
                    // Check if pair violates a rule
                    let violates = rules.contains(&(corrected[j], corrected[i]));
                    if violates {
                        corrected.swap(i, j);
                        did_swap = true;
                    }
                }
            }
            if !did_swap {
                break;
            }
        }
        corrected
    });

    let result = corrected_updates.map(|u| u[(u.len() - 1) / 2]).sum::<i32>();
    result.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = part2(
            "47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47",
        );
        assert_eq!(result, "123");
    }
}
