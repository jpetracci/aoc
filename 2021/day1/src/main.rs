use std::fs;

fn part1(values: Vec<usize>) -> usize {
    values.windows(2).filter(|a| a[1] > a[0]).count()
}

fn part2(values: Vec<usize>) -> usize {
    // a + b + c < b + c + d --> a < d
    values.windows(4).filter(|a| a[3] > a[0]).count()
}

fn main() {
    let values = fs::read_to_string("day1input.txt").expect("error reading file");
    let values = values.split("\n");
    let values = values.collect::<Vec<&str>>();
    let values: Vec<usize> = values
        .iter()
        .map(|v| {
            v.strip_suffix("\r")
                .unwrap_or("")
                .parse::<usize>()
                .unwrap_or(0)
        })
        .collect();

    println!("{}", part1(values.clone()));
    println!("{}", part2(values));
}
