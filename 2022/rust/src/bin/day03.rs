pub const PRIORITY_MAPPING: [&str; 52] = [
    "a", "b", "c", "d", "e", "f", "g", "h", "i", "j", "k", "l", "m", "n", "o", "p", "q", "r", "s",
    "t", "u", "v", "w", "x", "y", "z", "A", "B", "C", "D", "E", "F", "G", "H", "I", "J", "K", "L",
    "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
];

fn part_one(input: &str) {
    let mut priority_score: usize = 0;

    for backpack in input.lines() {
        let mid = backpack.chars().count() / 2;
        let first_half: String = backpack.chars().take(mid).collect();
        let second_half: String = backpack.chars().skip(mid).collect();

        for item in first_half.chars() {
            if second_half.contains(item) {
                if let Some(i) = PRIORITY_MAPPING.iter().position(|&x| x == item.to_string()) {
                    priority_score += i + 1;
                }
                break;
            };
        }
    }

    println!("Total priority score is: {}", priority_score);
}

fn part_two(input: &str) {
    let mut priority_score: usize = 0;

    let backpacks: Vec<&str> = input.split('\n').collect();
    let groups: Vec<&[&str]> = backpacks.chunks(3).collect();

    for group in groups {
        for backpack in group {
            let mut found: bool = false;
            for item in backpack.chars() {
                if group.iter().all(|x| x.contains(item)) {
                    if let Some(i) = PRIORITY_MAPPING.iter().position(|&x| x == item.to_string()) {
                        priority_score += i + 1;
                    }
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
    }

    println!("Total priority score is: {}", priority_score);
}

fn main() {
    let input: &str = include_str!("../inputs/day03.txt");

    part_one(input);
    part_two(input);
}
