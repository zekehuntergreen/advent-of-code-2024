use std::time::Instant;

use regex::Regex;

fn find_sum(re: &Regex, input: &str, part2: bool) -> u32 {
    let mut enabled = true;
    re.captures_iter(input)
        .map(|m| {
            if let Some(numbers) = m.get(1) {
                return numbers.as_str();
            }
            m.get(0).unwrap().as_str()
        })
        .map(|m| {
            if m == "don't()" {
                enabled = false;
                0
            } else if m == "do()" {
                enabled = true;
                0
            } else {
                if !part2 || enabled {
                    return m
                        .split(",")
                        .map(|n| n.parse::<u32>().unwrap())
                        .product::<u32>();
                }
                0
            }
        })
        .sum()
}

fn main() {
    let re = Regex::new(r"mul\((\d{1,3},\d{1,3})\)|(do\(\))|(don\'t\(\))").unwrap();
    let input = include_str!("input.txt");

    // part 1
    let start = Instant::now();
    let solution = find_sum(&re, input, false);
    assert!(solution == 183669043);
    println!(
        "part 1 {} finished in {} millis",
        solution,
        (Instant::now() - start).as_millis()
    );

    // part 2
    let start = Instant::now();
    let solution = find_sum(&re, input, true);
    assert!(solution == 59097164);
    println!(
        "part 2 {} finished in {} millis",
        solution,
        (Instant::now() - start).as_millis()
    );
}
