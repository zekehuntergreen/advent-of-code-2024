use std::time::Instant;

const INPUT: &str = include_str!("input.txt");

fn valid_expression(
    numbers: &[usize],
    target: usize,
    current_total: usize,
    index: usize,
    part2: bool,
) -> bool {
    if current_total > target {
        return false;
    }
    if index == numbers.len() {
        return current_total == target;
    }
    valid_expression(
        numbers,
        target,
        current_total + numbers[index],
        index + 1,
        part2,
    ) || valid_expression(
        numbers,
        target,
        if current_total == 0 {
            numbers[index]
        } else {
            current_total * numbers[index]
        },
        index + 1,
        part2,
    ) || {
        if !part2 {
            return false;
        }
        valid_expression(
            numbers,
            target,
            (current_total.to_string() + &numbers[index].to_string())
                .parse::<usize>()
                .unwrap(),
            index + 1,
            part2,
        )
    }
}

fn main() {
    let solution: (usize, usize) = INPUT
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
            let part1 = if valid_expression(&numbers, target, 0, 0, false) {
                target
            } else {
                0
            };
            let part2 = if valid_expression(&numbers, target, 0, 0, true) {
                target
            } else {
                0
            };
            (part1, part2)
        })
        .fold(
            (0usize, 0usize),
            |(part1_total, part2_total), (part1, part2)| (part1_total + part1, part2_total + part2),
        );
    let start = Instant::now();
    assert!(solution.0 == 21572148763543);
    assert!(solution.1 == 581941094529163);
    println!(
        "part1 {} part2 {} \nfound in {} nanos",
        solution.0,
        solution.1,
        (Instant::now() - start).as_nanos()
    )
}
