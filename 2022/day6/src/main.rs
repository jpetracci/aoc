use std::{fs, iter::Enumerate};

fn main() {
    let _example = String::from("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw");

    // bvwbjplbgvbhsrlpgdmjqwftvncz: first marker after character 5
    // nppdvjthqldpwncqszvftbrmjlhg: first marker after character 6
    // nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg: first marker after character 10
    // zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw: first marker after character 11

    let _values = fs::read_to_string("input.txt").unwrap();
    //println!("part1: {}", part1(&_values));
    println!("part2: {}", part2(&_values));
    //println!("part1: {}", part1(&_example));
    println!("part2: {}", part2(&_example));
}

fn unique(input: &str) -> bool {
    //print!("{} ", input);
    for i in 1..(input.len()) {
        let c = input.chars().nth(i - 1).unwrap();
        let sub = &input[i..];
        //print!(" {}", c);
        if sub.contains(c) {
            //println!(" yes");
            return false;
        }
    }
    //println!(" no");
    true
}

fn part1(input: &str) -> u32 {
    for i in 0..(input.len() - 4) {
        if unique(&input[i..(i + 4)]) {
            return i as u32 + 4;
        }
    }
    0
}

fn part2(input: &str) -> u32 {
    for i in 0..(input.len() - 14) {
        if unique(&input[i..(i + 14)]) {
            return i as u32 + 14;
        }
    }
    0
}
