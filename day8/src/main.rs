use fnv::{FnvHashMap, FnvHashSet};
use itertools::Itertools;

const INPUT: &[u8] = include_bytes!("input.txt");
// const MATRIX_SIZE: isize = 12;
const MATRIX_SIZE: isize = 50;

fn coordinates_in_matrix(x: isize, y: isize) -> bool {
    x >= 0 && x < MATRIX_SIZE && y >= 0 && y < MATRIX_SIZE
}

fn main() {
    let num_antinode_locations = INPUT
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
            for c in locations.into_iter().combinations(2) {
                let (a1, a2) = (c[0], c[1]);
                let diff = (a1.0 - a2.0, a1.1 - a2.1);
                let antinode1 = (a1.0 + diff.0, a1.1 + diff.1);
                let antinode2 = (a2.0 - diff.0, a2.1 - diff.1);
                for a in [antinode1, antinode2] {
                    if coordinates_in_matrix(a.0, a.1) {
                        acc.insert(a);
                    }
                }
            }

            acc
        })
        .len();
    println!("locations {:?} ", num_antinode_locations);
}
