use std::collections::HashSet;

const INPUT: &[u8] = include_bytes!("test_input.txt");

// fn find_starting_position
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

fn main() {
    let matrix: Vec<&[u8]> = INPUT.split(|&b| b == b'\n').collect();
    let mut current_row = matrix.iter().position(|row| row.contains(&b'^')).unwrap();
    let mut current_col = matrix[current_row]
        .iter()
        .position(|value| value == &b'^')
        .unwrap();
    let mut direction = Direction::Up;
    let num_cols = matrix[0].len();
    let num_rows = matrix.len();
    println!("num rows {} num cols {}", num_rows, num_cols);
    let (mut next_row, mut next_col) = get_next_coordinates((current_row, current_col), &direction);

    let mut steps: HashSet<(usize, usize)> = HashSet::new();
    let mut steps_with_direction: HashSet<(usize, usize, Direction)> = HashSet::new();
    while next_col < num_cols && next_col >= 0 && next_row < num_rows && next_row >= 0 {
        steps.insert((current_row, current_col));
        steps_with_direction.insert((current_row, current_col, direction.clone()));

        match matrix[next_row][next_col] {
            b'#' => direction = direction.next(),
            _ => {
                (current_row, current_col) = (next_row, next_col);
            }
        }

        // inspect the next spot
        (next_row, next_col) = get_next_coordinates((current_row, current_col), &direction);
    }
    steps.insert((next_row, next_col));
    steps_with_direction.insert((next_row, next_col, direction.clone()));

    println!("{:?}", steps);
    let num_steps = steps.len();
    // assert!(num_steps == 5531);
    println!("num steps {:?}", num_steps);
}
