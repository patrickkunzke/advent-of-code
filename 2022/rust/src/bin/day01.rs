fn part_one(input: &str) {
    let parsed = input.split("\n");

    let mut total = 0;
    let mut totals: Vec<i32> = vec![];

    for x in parsed {
        let x = x.parse::<i32>();
        if x.is_ok() {
            total += x.unwrap();
        } else {
            totals.push(total);
            total = 0;
        }
    }

    println!("{:?}", totals.iter().max().unwrap());
}

fn part_two(input: &str) {
    let parsed = input.split("\n");

    let mut total = 0;
    let mut totals: Vec<i32> = vec![];

    for x in parsed {
        let x = x.parse::<i32>();
        if x.is_ok() {
            total += x.unwrap();
        } else {
            totals.push(total);
            total = 0;
        }
    }

    totals.sort_by(|a, z| z.cmp(a));
    println!("{:?}", totals[0..3].iter().sum::<i32>());
}

fn main() {
    let input = include_str!("../inputs/day01.txt");

    part_one(input);
    part_two(input);
}
