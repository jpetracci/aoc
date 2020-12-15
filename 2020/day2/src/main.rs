use std::fs;

fn letter_counter(to_check: String, letter: char) -> i32 {
    let chars: Vec<char> = to_check.chars().collect();
    let mut sum = 0;

    for c in chars {
        if c == letter {
            sum += 1;
        }
    }

    sum
}

fn in_range(range: (i32, i32), value: i32) -> bool {
    value >= range.0 && value <= range.1
}

fn checker(first: i32, second: i32, word: String, letter: char) -> bool {
    let i = (first - 1) as usize;
    let j = first as usize;
    let spot1 = word.get(i..j).unwrap_or(" ").parse::<char>().unwrap_or(' ');
    let i = (second - 1) as usize;
    let j = second as usize;
    let spot2 = word.get(i..j).unwrap_or(" ").parse::<char>().unwrap_or(' ');

    (spot1 == letter) ^ (spot2 == letter)
}

fn part1() {
    let pwd_file = fs::read_to_string("day2_input.txt").expect("error reading file");
    let pwd_lines = pwd_file.split("\n");
    let mut sum = 0;

    for line in pwd_lines {
        if line.len() == 0 {
            break;
        }
        let pwd_data = line.split(" ");
        let pwd_data = pwd_data.collect::<Vec<&str>>();
        let pwd_policy_nums = pwd_data[0].split("-").collect::<Vec<&str>>();
        let low_num = pwd_policy_nums[0].parse::<i32>().unwrap_or(0);
        let high_num = pwd_policy_nums[1].parse::<i32>().unwrap_or(0);
        let pwd_policy_letter = pwd_data[1].split(":").collect::<Vec<&str>>();
        let pwd_policy_letter: Vec<char> = pwd_policy_letter[0].chars().collect();
        let password = pwd_data[2];

        let count = letter_counter(password.to_string(), pwd_policy_letter[0]);
        let valid = in_range((low_num, high_num), count);
        sum += if valid == true { 1 } else { 0 };
    }
    println!("part1: {}", sum);
}

fn part2() {
    let pwd_file = fs::read_to_string("day2_input.txt").expect("error reading file");
    let pwd_lines = pwd_file.split("\n");
    let mut sum = 0;

    for line in pwd_lines {
        if line.len() == 0 {
            break;
        }
        let pwd_data = line.split(" ");
        let pwd_data = pwd_data.collect::<Vec<&str>>();
        let pwd_policy_nums = pwd_data[0].split("-").collect::<Vec<&str>>();
        let first_num = pwd_policy_nums[0].parse::<i32>().unwrap_or(0);
        let second_num = pwd_policy_nums[1].parse::<i32>().unwrap_or(0);
        let pwd_policy_letter = pwd_data[1].split(":").collect::<Vec<&str>>();
        let pwd_policy_letter: Vec<char> = pwd_policy_letter[0].chars().collect();
        let password = pwd_data[2];

        let valid = checker(
            first_num,
            second_num,
            password.to_string(),
            pwd_policy_letter[0],
        );

        sum += if valid { 1 } else { 0 };
    }
    println!("part2: {}", sum);
}

fn main() {
    part1();
    part2();
}
