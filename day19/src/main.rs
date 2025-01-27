use std::sync::Mutex;

use fnv::{FnvHashMap, FnvHashSet};

const INPUT: &str = include_str!("input.txt");

fn possible_solutions(
    design: &str,
    patterns: &FnvHashSet<&str>,
    cache: &Mutex<FnvHashMap<String, usize>>,
) -> usize {
    if let Some(solutions) = cache.lock().unwrap().get(design) {
        return *solutions;
    }
    let mut solutions = 0;
    if patterns.contains(design) {
        solutions += 1;
    }
    for i in 1..design.len() {
        let (left, right) = design.split_at(i);
        if patterns.contains(left) {
            solutions += possible_solutions(right, patterns, cache);
        }
    }
    cache.lock().unwrap().insert(design.to_string(), solutions);
    return solutions;
}

fn main() {
    let start = std::time::Instant::now();
    let (patterns, designs) = INPUT.split_once("\n\n").unwrap();
    let patterns: FnvHashSet<&str> = patterns.split(", ").collect();
    let designs: Vec<&str> = designs.lines().collect();

    let cache: Mutex<FnvHashMap<String, usize>> = Mutex::new(FnvHashMap::default());

    let possible_designs: Vec<usize> = designs
        .into_iter()
        .map(|design| possible_solutions(design, &patterns, &cache))
        .filter(|d| d > &0)
        .collect();

    let part1 = possible_designs.clone().into_iter().count();
    let part2: usize = possible_designs.into_iter().sum();
    assert!(part1 == 374 && part2 == 1100663950563322);
    println!("finished in {} micros", start.elapsed().as_micros());
}
