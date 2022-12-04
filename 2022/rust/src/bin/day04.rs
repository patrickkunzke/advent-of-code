fn part_one(input: &str) {
    let result = input
        .lines()
        .filter(|line| {
            let (range1, range2) = line.split_once(",").unwrap();
            let (min1, max1) = range1.split_once("-").unwrap();
            let (min2, max2) = range2.split_once("-").unwrap();
            let min1: u32 = min1.parse().unwrap();
            let max1: u32 = max1.parse().unwrap();
            let min2: u32 = min2.parse().unwrap();
            let max2: u32 = max2.parse().unwrap();

            (min1 <= min2 && max1 >= max2) || (min2 <= min1 && max2 >= max1)
        })
        .count();

    println!("{:?}", result);
}

fn part_two(input: &str) {
    let result = input
        .lines()
        .filter(|line| {
            let (range1, range2) = line.split_once(",").unwrap();
            let (min1, max1) = range1.split_once("-").unwrap();
            let (min2, max2) = range2.split_once("-").unwrap();
            let min1: u32 = min1.parse().unwrap();
            let max1: u32 = max1.parse().unwrap();
            let min2: u32 = min2.parse().unwrap();
            let max2: u32 = max2.parse().unwrap();

            min1 <= max2 && max1 >= min2
        })
        .count();

    println!("{:?}", result);
}

fn main() {
    let input: &str = include_str!("../inputs/day04.txt");

    part_one(input);
    part_two(input);
}
