use regex::Regex;

fn part1(input: &str) -> u32 {
    let re: Regex = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();
    re.captures_iter(input)
        .map(|m| vec![m[1].to_string(), m[2].to_string()])
        .map(|m| m.iter().map(|n| n.parse::<u32>().unwrap()).product::<u32>())
        .sum()
}

fn main() {
    let example_input = "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    let input = include_str!("input.txt");

    println!("part 1 example {}", part1(example_input));
    println!("part 1 {}", part1(input));
}
