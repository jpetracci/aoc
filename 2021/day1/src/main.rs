use std::fs;

fn part1(values: Vec<&str>) -> i32 {
    let mut prev = values[0].parse::<i32>().unwrap_or(0);
    let mut sum = 0;

    for i in 1..values.len() {
        let cur = values[i].parse::<i32>().unwrap_or(0);
        if cur > prev {
            sum += 1;
        }
        prev = cur;
    }
    return sum;
}

fn part2(values: Vec<&str>) -> i32 {
    let mut last = i32::MAX;
    let mut sum = 0;

    for i in 2..(values.len()) {
        let v1 = values[i-2].parse::<i32>().unwrap_or(0);
        let v2 = values[i-1].parse::<i32>().unwrap_or(0);
        let v3 = values[i].parse::<i32>().unwrap_or(0);

        let cur = v1 + v2 + v3;

        if cur > last {
            sum += 1;
        }
        last = cur;
    }
    return sum;
}

fn main() {
    let expense_report = fs::read_to_string("day1input.txt").expect("error reading file");
    let expense_report = expense_report.split("\n");
    let values = expense_report.collect::<Vec<&str>>();

    println!("{}", part1(values.clone()));
    println!("{}", part2(values));
}
