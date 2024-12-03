use itertools::Itertools;
use std::time::Instant;

const INPUT: &[u8] = include_bytes!("input.txt");

fn parse(array: &[u8]) -> i32 {
    if array.len() == 1 {
        return (array[0] - b'0') as i32;
    }
    return (array[0] - b'0') as i32 * 10 + (array[1] - b'0') as i32;
}

fn report_is_safe(report: &Vec<i32>) -> bool {
    let asc = report[1] > report[0];
    report
        .windows(2)
        .find(|w| {
            let diff = if asc { w[1] - w[0] } else { w[0] - w[1] };
            diff < 1 || diff > 3
        })
        .is_none()
}

fn part1() -> usize {
    INPUT
        .split(|&b| b == b'\n')
        .map(|report| report.split(|&b| b == b' ').map(parse).collect())
        .filter(report_is_safe)
        .count()
}

fn part2() -> i32 {
    let mut num_safe = 0;
    for report in INPUT.split(|&b| b == b'\n') {
        let report: Vec<i32> = report.split(|&b| b == b' ').map(parse).collect();
        let report_length = report.len();
        if report
            .into_iter()
            .combinations((report_length - 1).try_into().unwrap())
            .find(report_is_safe)
            .is_some()
        {
            num_safe += 1
        }
    }
    num_safe
}

fn main() {
    let start = Instant::now();
    assert!(part1() == 598);
    let end = Instant::now();
    println!("part 1 complete in {}", (end - start).as_micros());

    let start = Instant::now();
    assert!(part2() == 634);
    let end = Instant::now();
    println!("part 2 complete in {}", (end - start).as_micros());
}
