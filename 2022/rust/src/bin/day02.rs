fn part_one(input: &str) {
    /*
    Notes:
    A X Rock 1
    B Y Paper 2
    C Z Scissors 3
    Lose 0
    Draw 3
    Win 6
    */

    let score = input
        .lines()
        .map(|game| match game {
            "A X" => 1 + 3,
            "A Y" => 2 + 6,
            "A Z" => 3 + 0,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 1 + 6,
            "C Y" => 2 + 0,
            "C Z" => 3 + 3,

            _ => 0,
        })
        .sum::<i32>();

    println!("{:?}", score)
}

fn part_two(input: &str) {
    /*
    Notes:
    Opponent:
    A Rock 1
    B Paper 2
    C Scissors 3
    Me:
    Y Draw
    X Lose
    Z Win
    Lose 0
    Draw 3
    Win 6
    */

    let score = input
        .lines()
        .map(|game| match game {
            "A X" => 3 + 0,
            "A Y" => 1 + 3,
            "A Z" => 2 + 6,

            "B X" => 1 + 0,
            "B Y" => 2 + 3,
            "B Z" => 3 + 6,

            "C X" => 2 + 0,
            "C Y" => 3 + 3,
            "C Z" => 1 + 6,

            _ => 0,
        })
        .sum::<i32>();

    println!("{:?}", score)
}

fn main() {
    let input: &str = include_str!("../inputs/day02.txt");

    part_one(input);
    part_two(input);
}
