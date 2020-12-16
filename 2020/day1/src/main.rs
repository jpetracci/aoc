use std::fs;

fn part1() {
    let expense_report = fs::read_to_string("day1_input.txt").expect("error reading file");
    let expense_report = expense_report.split("\n");
    let values = expense_report.collect::<Vec<&str>>();

    for i in 0..values.len() {
        let val1 = values[i].parse::<i32>().unwrap_or(0);
        for j in i..values.len() {
            let val2 = values[j].parse::<i32>().unwrap_or(0);
            let sum = val1 + val2;
            if sum == 2020 {
                println!("part 1: {} {} {}", val1, val2, val1 * val2);
            }
        }
    }
}

fn part2() {
    let expense_report = fs::read_to_string("day1_input.txt").expect("error reading file");
    let expense_report = expense_report.split("\n");
    let values = expense_report.collect::<Vec<&str>>();

    for i in 0..values.len() {
        let val1 = values[i].parse::<i32>().unwrap_or(0);
        for j in i..values.len() {
            let val2 = values[j].parse::<i32>().unwrap_or(0);
            for k in j..values.len() {
                let val3 = values[k].parse::<i32>().unwrap_or(0);
                let sum = val1 + val2 + val3;
                if sum == 2020 {
                    println!("part 2: {}", val1 * val2 * val3);
                }
            }
        }
    }
}

fn main() {
    part1();
    part2();
}
