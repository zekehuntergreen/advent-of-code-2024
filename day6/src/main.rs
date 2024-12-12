use fnv::FnvHashSet;
use std::{collections::HashSet, time::Instant};

const INPUT: &[u8] = include_bytes!("input.txt");

#[derive(Debug, Eq, Hash, PartialEq, Clone)]
enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn direction(&self) -> (i32, i32) {
        match self {
            Direction::Up => (-1, 0),
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
        }
    }
    fn next(&self) -> Direction {
        match self {
            Direction::Up => Direction::Right,
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
        }
    }
}

fn get_next_coordinates(
    current_coordinates: (usize, usize),
    direction: &Direction,
) -> (usize, usize) {
    // TODO is there a nicer way to reconcile the types?
    let next_row = (current_coordinates.0 as i32 + direction.direction().0) as usize;
    let next_col = (current_coordinates.1 as i32 + direction.direction().1) as usize;
    return (next_row, next_col);
}

fn calculate_path(
    matrix: &Vec<&[u8]>,
    start_row: usize,
    start_col: usize,
    obstruction: Option<(usize, usize)>,
) -> (FnvHashSet<(usize, usize, Direction)>, bool) {
    let (mut current_row, mut current_col) = (start_row, start_col);
    let mut direction = Direction::Up;
    let num_cols = matrix[0].len();
    let num_rows = matrix.len();
    let (mut next_row, mut next_col) = get_next_coordinates((current_row, current_col), &direction);
    let mut steps_with_direction: FnvHashSet<(usize, usize, Direction)> = FnvHashSet::default();
    while next_col < num_cols && next_col >= 0 && next_row < num_rows && next_row >= 0 {
        steps_with_direction.insert((current_row, current_col, direction.clone()));

        if Some((next_row, next_col)) == obstruction {
            direction = direction.next()
        } else {
            match matrix[next_row][next_col] {
                b'#' => direction = direction.next(),
                _ => {
                    (current_row, current_col) = (next_row, next_col);
                }
            }
        }

        // inspect the next spot
        (next_row, next_col) = get_next_coordinates((current_row, current_col), &direction);
        if steps_with_direction.contains(&(next_row, next_col, direction.clone())) {
            return (steps_with_direction, true);
        }
    }
    steps_with_direction.insert((current_row, current_col, direction.clone()));
    return (steps_with_direction, false);
}

fn main() {
    let matrix: Vec<&[u8]> = INPUT.split(|&b| b == b'\n').collect();
    let start_row = matrix.iter().position(|row| row.contains(&b'^')).unwrap();
    let start_col = matrix[start_row]
        .iter()
        .position(|value| value == &b'^')
        .unwrap();

    // part 1
    let start = Instant::now();
    let (steps_with_direction, _) = calculate_path(&matrix, start_row, start_col, None);
    let steps: HashSet<(usize, usize)> = HashSet::from_iter(
        steps_with_direction
            .clone()
            .into_iter()
            .map(|step| (step.0, step.1)),
    );

    let num_steps = steps.len();
    println!(
        "part 1 {:?} completed in {} milliseconds",
        num_steps,
        (Instant::now() - start).as_millis()
    );
    assert!(num_steps == 5531);

    // part 2
    let start = Instant::now();
    let obstructions: Vec<(usize, usize)> = steps
        .into_iter()
        .filter(|possible_obstruction| {
            let (_path, contains_cycle) =
                calculate_path(&matrix, start_row, start_col, Some(*possible_obstruction));
            contains_cycle
        })
        .collect();
    assert!(obstructions.len() == 2165);
    println!(
        "part 2 {} completed in {} milliseconds",
        obstructions.len(),
        (Instant::now() - start).as_millis()
    );
}
