use std::{collections::HashMap, time::Instant};

const INPUT: &str = std::include_str!("../input.txt");

fn part1() -> i32 {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list: Vec<i32> = Vec::new();
    for line in INPUT.lines() {
        let parts: Vec<&str> = line.split_whitespace().collect();
        first_list.push(parts[0].parse().unwrap());
        second_list.push(parts[1].parse().unwrap());
    }
    first_list.sort();
    second_list.sort();
    let sum_of_differences: i32 = first_list
        .into_iter()
        .zip(second_list)
        .map(|(a, b)| (a - b).abs())
        .sum();
    sum_of_differences
}

fn part2() -> i32 {
    let mut first_list: Vec<i32> = Vec::new();
    let mut second_list_counts: HashMap<i32, i32> = HashMap::new();
    for line in INPUT.lines() {
        let parts: Vec<i32> = line
            .split_whitespace()
            .map(|n| n.parse().unwrap())
            .collect();
        first_list.push(parts[0]);
        second_list_counts.insert(
            parts[1],
            second_list_counts.get(&parts[1]).unwrap_or(&0) + 1,
        );
    }
    let similarity_score: i32 = first_list
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
        (end - start).as_micros()
    );

    let start = Instant::now();
    let solution = part2();
    let end = Instant::now();
    println!(
        "part 2: {} complete in {}",
        solution,
        (end - start).as_micros()
    );
}
