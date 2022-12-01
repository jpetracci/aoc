use std::fs;

fn main() {
    let values = fs::read_to_string("day1_input.txt").expect("error reading file");
    let values = values.split("\n");
    let values = values.collect::<Vec<&str>>();
    part1(values.clone());
    part2(values);
}

fn part1(values: Vec<&str>) {
    let mut largest = 0;
    let mut sum = 0;

    for v in values {
        if v.len() > 0 {
            sum += v.parse::<usize>().unwrap();
        } else {
            if sum > largest {
                largest = sum;
            }
            sum = 0;
        }
    }
    println!("part1: {}", largest);
}

fn part2(values: Vec<&str>) {
    let mut sums = Vec::new();
    let mut sum = 0;

    for v in values {
        if v.len() > 0 {
            sum += v.parse::<usize>().unwrap();
        } else {
            sums.push(sum);
            sum = 0;
        }
    }
    sums.sort();
    let sum3: usize = sums[(sums.len() - 3)..].iter().sum();
    println!("part2: {}", sum3);
}
