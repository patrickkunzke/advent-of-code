// solution from https://nickymeuleman.netlify.app/garden/aoc2022-day06

fn part_one(input: &str) -> usize {
    input
        .as_bytes()
        .windows(4)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap()
        + 4
}

fn part_two(input: &str) -> usize {
    input
        .as_bytes()
        .windows(14)
        .position(|window| {
            window
                .iter()
                .enumerate()
                .all(|(idx, c)| !window[..idx].contains(c))
        })
        .unwrap()
        + 14
}

fn main() {
    let input: &str = include_str!("../inputs/day06.txt");

    println!("{}", part_one(input));
    println!("{}", part_two(input));
}
