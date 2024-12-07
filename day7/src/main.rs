use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn valid_expression(numbers: &[usize], target: usize, current_total: usize, index: usize) -> bool {
    // println!(
    //     "numbers {:?} target {} current_total {} index {}",
    //     numbers, target, current_total, index
    // );
    if current_total > target {
        return false;
    }
    let next_index = index + 1;
    if next_index == numbers.len() {
        current_total == target
    } else {
        valid_expression(
            numbers,
            target,
            current_total + numbers[next_index],
            next_index,
        ) || valid_expression(
            numbers,
            target,
            current_total * numbers[next_index],
            next_index,
        )
    }
}

fn main() {
    let solution: usize = INPUT
        .lines()
        .map(|line| {
            let mut s = line.split(':');
            let target = s.next().unwrap().trim().parse::<usize>().unwrap();
            let numbers: Vec<usize> = s
                .next()
                .unwrap()
                .trim()
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect();
            (target, numbers)
        })
        .map(|(target, numbers)| {
            if valid_expression(&numbers, target, numbers[0], 0) {
                target
            } else {
                0
            }
        })
        .sum();
    let start = Instant::now();
    assert!(21572148763543 == solution);
    println!(
        "solution found in {} nanos",
        (Instant::now() - start).as_nanos()
    )
}
