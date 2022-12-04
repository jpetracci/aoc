use std::fs;

fn main() {
    let _example = String::from(
        "2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8",
    );
    let _values = fs::read_to_string("input.txt").unwrap();
    println!("part1: {}", part1(&_values));
    println!("part2: {}", part2(&_values));
    //println!("part1: {}", part1(&_example));
    //println!("part2: {}", part2(&_example));
}

fn part1(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let elfs: Vec<&str> = line.split(",").collect();

        let first: Vec<&str> = elfs[0].split("-").collect();
        let first: (u32, u32) = (
            first[0].parse::<u32>().unwrap(),
            first[1].parse::<u32>().unwrap(),
        );

        let second: Vec<&str> = elfs[1].split("-").collect();
        let second: (u32, u32) = (
            second[0].parse::<u32>().unwrap(),
            second[1].parse::<u32>().unwrap(),
        );

        if first.0 <= second.0 && first.1 >= second.1 || second.0 <= first.0 && second.1 >= first.1
        {
            sum += 1;
        }

        //println!("{:?} {:?}", first, second);
    }
    sum
}

fn part2(input: &str) -> u32 {
    let mut sum: u32 = 0;
    for line in input.lines() {
        let elfs: Vec<&str> = line.split(",").collect();

        let first: Vec<&str> = elfs[0].split("-").collect();
        let first: (u32, u32) = (
            first[0].parse::<u32>().unwrap(),
            first[1].parse::<u32>().unwrap(),
        );

        let second: Vec<&str> = elfs[1].split("-").collect();
        let second: (u32, u32) = (
            second[0].parse::<u32>().unwrap(),
            second[1].parse::<u32>().unwrap(),
        );

        if ((first.0 <= second.0 && first.1 >= second.0)
            || (first.0 >= second.0 && first.1 <= second.0))
            || ((second.0 <= first.0 && second.1 >= first.0)
                || (second.0 >= first.0 && second.1 <= first.0))
        {
            sum += 1;
        }

        //println!("{:?} {:?}", first, second);
    }
    sum
}
