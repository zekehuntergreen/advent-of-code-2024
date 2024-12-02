use std::{collections::HashMap, time::Instant};

fn get_input_string() -> String {
    std::include_str!("../input.txt").to_string()
}

fn part1() -> isize {
    let contents = get_input_string();
    let mut first_list: Vec<isize> = Vec::new();
    let mut second_list: Vec<isize> = Vec::new();
    for line in contents.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        first_list.push(parts[0].parse().unwrap());
        second_list.push(parts[1].parse().unwrap());
    }
    first_list.sort();
    second_list.sort();
    let sum_of_differences: isize = first_list
        .into_iter()
        .zip(second_list)
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum_of_differences
}

fn part2() -> isize {
    let contents = get_input_string();
    let mut first_list: Vec<isize> = Vec::new();
    let mut second_list_counts: HashMap<isize, isize> = HashMap::new();
    for line in contents.lines() {
        let parts: Vec<isize> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        first_list.push(parts[0]);
        second_list_counts.insert(
            parts[1],
            second_list_counts.get(&parts[1]).unwrap_or(&0) + 1,
        );
    }
    let similarity_score: isize = first_list
        .into_iter()
        .map(|i| i * second_list_counts.get(&i).unwrap_or(&0))
        .sum();
    similarity_score
}

fn main() {
    let start = Instant::now();
    let solution = part1();
    let end = Instant::now();
    println!(
        "part 1: {} complete in {}",
        solution,
        (end - start).as_nanos()
    );

    let start = Instant::now();
    let solution = part2();
    let end = Instant::now();
    println!(
        "part 2: {} complete in {}",
        solution,
        (end - start).as_nanos()
    );
}
