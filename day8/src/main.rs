use std::time::Instant;

use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");
// const MATRIX_SIZE: isize = 12;
const MATRIX_SIZE: isize = 50;

fn coordinates_in_matrix(c: (isize, isize)) -> bool {
    c.0 >= 0 && c.0 < MATRIX_SIZE && c.1 >= 0 && c.1 < MATRIX_SIZE
}

fn get_next_antinode(
    a: &(isize, isize),
    diff: &(isize, isize),
    direction: isize,
) -> (isize, isize) {
    (a.0 + (diff.0 * direction), a.1 + (diff.1 * direction))
}

fn find_direction_antinodes(
    antenna: &(isize, isize),
    diff: &(isize, isize),
    direction: isize,
    part2: bool,
) -> FnvHashSet<(isize, isize)> {
    let mut new_antinodes = FnvHashSet::default();

    let mut antinode = if part2 {
        *antenna
    } else {
        get_next_antinode(antenna, diff, direction)
    };
    while coordinates_in_matrix(antinode) && (part2 || new_antinodes.is_empty()) {
        new_antinodes.insert(antinode);
        antinode = get_next_antinode(&antinode, diff, direction)
    }
    new_antinodes
}

fn find_num_antinodes(part2: bool) -> usize {
    INPUT
        .split(|b| b == &b'\n')
        .enumerate()
        .fold(FnvHashMap::default(), |mut acc, (i, row)| {
            for (j, cell) in row.into_iter().enumerate() {
                if cell != &b'.' {
                    acc.entry(*cell)
                        .or_insert(Vec::new())
                        .push((i as isize, j as isize))
                }
            }
            acc
        })
        .iter()
        .fold(FnvHashSet::default(), |mut acc, (_, locations)| {
            for antenna_combination in locations.into_iter().combinations(2) {
                let (a1, a2) = (antenna_combination[0], antenna_combination[1]);
                let diff = (a1.0 - a2.0, a1.1 - a2.1);
                acc.extend(find_direction_antinodes(a1, &diff, 1, part2));
                acc.extend(find_direction_antinodes(a2, &diff, -1, part2));
            }
            acc
        })
        .len()
}

fn main() {
    let start = Instant::now();
    let antinode_locations = find_num_antinodes(false);
    assert!(antinode_locations == 220);
    println!(
        "part1 {} finished in {} nanos",
        antinode_locations,
        (Instant::now() - start).as_nanos()
    );

    let start = Instant::now();
    let antinode_locations = find_num_antinodes(true);
    assert!(antinode_locations == 813);
    println!(
        "part2 {} finished in {} nanos",
        antinode_locations,
        (Instant::now() - start).as_nanos()
    );
}
