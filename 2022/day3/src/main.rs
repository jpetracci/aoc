use std::{collections::HashSet, fmt::format, fs};

fn main() {
    let _example = String::from(
        "vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw",
    );
    let _values = fs::read_to_string("input.txt").expect("file error");
    println!("part1: {}", part1(_values.clone()));
    println!("part2: {}", part2(_values));
    //println!("part1: {}", part1(_example.clone()));
    //println!("part2: {}", part2(_example));
}

fn part1(input: String) -> u32 {
    let mut pairs: Vec<u32> = Vec::new();
    for line in input.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        //let first = first.chars();
        //let second = second.chars();
        let mut break_out = false;
        for f in first.chars() {
            for s in second.chars() {
                if f == s {
                    pairs.push((f as u32));
                    break_out = true;
                    break;
                }
            }
            if break_out == true {
                break;
            }
        }
        //println!("{:?}", pairs);
    }
    let values: Vec<u32> = pairs
        .iter()
        .map(|&p| if p >= 97 { p - 96 } else { p - 64 + 26 })
        .collect();
    //println!("{:?}", values);
    values.iter().sum()
}

fn part2(input: String) -> u32 {
    let mut common: Vec<u32> = Vec::new();
    let mut sum = 0;
    let mut lines: Vec<&str> = input.lines().collect();
    for l in (0..lines.len()).into_iter().step_by(3) {
        let first: Vec<char> = lines[l].chars().collect();
        let second: Vec<char> = lines[l + 1].chars().collect();
        let third: Vec<char> = lines[l + 2].chars().collect();

        for c in first {
            if second.contains(&c) && third.contains(&c) {
                common.push(c as u32);
                break;
            }
        }
    }
    //println!("{:?}", common);

    let values: Vec<u32> = common
        .iter()
        .map(|&p| if p >= 97 { p - 96 } else { p - 64 + 26 })
        .collect();
    //println!("{:?}", values);
    values.iter().sum()
}
