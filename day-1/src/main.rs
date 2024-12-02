use std::env;
use std::fs;
use std::io;

pub fn read_file() -> io::Result<String> {
    let mut args = env::args();
    // ignore cargo run but get the next value;
    args.next();
    let file_path = args.next().unwrap();

    Ok(fs::read_to_string(file_path)?)
}

pub fn split_line(
    line: &str,
    left_num: &mut Vec<i32>,
    right_num: &mut Vec<i32>,
) -> (Vec<i32>, Vec<i32>) {
    if let Some((left, right)) = line.split_once(" ") {
        match (left.trim().parse::<i32>(), right.trim().parse::<i32>()) {
            (Ok(l), Ok(r)) => {
                left_num.push(l);
                right_num.push(r);
            }
            _ => println!(
                "Error: failed to parse numbers from line: {}, left: {}, right: {}",
                line, left, right
            ),
        }
        (left_num.to_vec(), right_num.to_vec())
    } else {
        println!("Error: line is not split by space: {}", line);
        (left_num.to_vec(), right_num.to_vec())
    }
}

fn part_one(mut left_num: Vec<i32>, mut right_num: Vec<i32>) -> (i32, Vec<i32>, Vec<i32>) {
    let mut total_distance = 0;
    match read_file() {
        Ok(content) => {
            for line in content.lines() {
                split_line(line, &mut left_num, &mut right_num);
            }

            left_num.sort();
            right_num.sort();
        }
        Err(err) => eprintln!("Error reading file: {}", err),
    }

    for i in 0..left_num.len() {
        let distance = left_num[i] - right_num[i];

        total_distance += distance.abs();
    }

    (total_distance, left_num, right_num)
}

fn part_two(left_num: Vec<i32>, right_num: Vec<i32>) -> i64 {
    let mut similarity_score: i64 = 0;

    let mut digit_score: i64;
    let mut score = 0;
    for i in 0..left_num.len() {
        for j in 0..right_num.len() {
            if left_num[i] == right_num[j] {
                score += 1;
            }
        }
        digit_score = (left_num[i] as i64) * (score as i64);

        similarity_score += digit_score;
        score = 0;
    }

    similarity_score
}

fn main() {
    println!("Current directory main: {:?}", env::current_dir().unwrap());

    let left_num: Vec<i32> = Vec::new();
    let right_num: Vec<i32> = Vec::new();

    let (total_distance, left_result, right_result) = part_one(left_num, right_num);
    println!("total_distance: {}", total_distance);

    let similarity_score = part_two(left_result, right_result);
    println!("similarity_score: {}", similarity_score);
}
