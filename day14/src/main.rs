#![feature(slice_split_once)]

use std::time::Instant;

#[derive(Debug, Clone)]
struct Robot {
    position: (isize, isize),
    velocity: (isize, isize),
}

const INPUT: &str = include_str!("input.txt");
const GRID_WIDTH: isize = 101;
const GRID_HEIGHT: isize = 103;

fn positions_are_unique(robots: &Vec<Robot>) -> bool {
    let mut positions: fnv::FnvHashSet<(isize, isize)> = fnv::FnvHashSet::default();
    for r in robots {
        if positions.contains(&r.position) {
            return false;
        } else {
            positions.insert(r.position);
        }
    }
    true
}

fn _print_grid(robots: &Vec<Robot>) {
    let mut positions: fnv::FnvHashMap<(isize, isize), usize> = fnv::FnvHashMap::default();
    for r in robots {
        if positions.contains_key(&r.position) {
            *positions.get_mut(&r.position).unwrap() += 1;
        } else {
            positions.insert(r.position, 1);
        }
    }
    for i in 0..GRID_HEIGHT {
        for j in 0..GRID_WIDTH {
            if positions.contains_key(&(j, i)) {
                print!("{}", positions.get(&(j, i)).unwrap());
            } else {
                print!(".");
            }
        }
        println!();
    }
}

fn main() {
    let start = Instant::now();
    let mut robots: Vec<Robot> = INPUT
        .split(|c| c == '\n')
        .map(|line| {
            let (position, velocity) = line.split_once(|c| c == ' ').unwrap();
            let position = position
                .split(|c| c == '=')
                .nth(1)
                .unwrap()
                .split(|c| c == ',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            let velocity = velocity
                .split(|c| c == '=')
                .nth(1)
                .unwrap()
                .split(|c| c == ',')
                .map(|s| s.parse::<isize>().unwrap())
                .collect::<Vec<_>>();
            Robot {
                position: (position[0], position[1]),
                velocity: (velocity[0], velocity[1]),
            }
        })
        .collect();

    let mut i = 0;
    let mut part1 = 0;
    while !positions_are_unique(&robots) {
        i += 1;
        for r in robots.iter_mut() {
            let current_position = r.position;
            r.position = (
                (current_position.0 + r.velocity.0).rem_euclid(GRID_WIDTH),
                (current_position.1 + r.velocity.1).rem_euclid(GRID_HEIGHT),
            );
        }
        if i == 100 {
            let quadrants = robots.clone().into_iter().fold([0, 0, 0, 0], |mut acc, r| {
                if r.position.0 < GRID_WIDTH / 2 && r.position.1 < GRID_HEIGHT / 2 {
                    acc[0] += 1;
                } else if r.position.0 > GRID_WIDTH / 2 && r.position.1 < GRID_HEIGHT / 2 {
                    acc[1] += 1;
                } else if r.position.0 < GRID_WIDTH / 2 && r.position.1 > GRID_HEIGHT / 2 {
                    acc[2] += 1;
                } else if r.position.0 > GRID_WIDTH / 2 && r.position.1 > GRID_HEIGHT / 2 {
                    acc[3] += 1;
                }
                acc
            });
            part1 = quadrants.iter().fold(1, |acc, x| acc * x);
        }
    }
    assert!(part1 == 215476074);
    let part2 = i;
    assert!(part2 == 6285);
    println!("finished in: {:?}", start.elapsed());
}
