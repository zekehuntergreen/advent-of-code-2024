use std::time::Instant;

const INPUT: &[u8] = include_bytes!("input.txt");
const DIRECTIONS: [(i32, i32); 8] = [
    (-1, 0),
    (-1, 1),
    (0, 1),
    (1, 1),
    (1, 0),
    (1, -1),
    (0, -1),
    (-1, -1),
];
const MATRIX_SIZE: usize = 140;

fn part1(matrix: &Vec<Vec<u8>>) -> i32 {
    let mut total = 0;
    for (i, row) in matrix.iter().enumerate() {
        for (j, cell) in row.into_iter().enumerate() {
            if cell != &b'X' {
                continue;
            }
            'direction: for (x, y) in DIRECTIONS {
                let (x, y) = (x as isize, y as isize);
                for (k, letter) in b"XMAS".into_iter().enumerate().skip(1) {
                    let k = k as isize;
                    let a = i as isize + (x * k);
                    let b = j as isize + (y * k);
                    if a < 0
                        || a >= MATRIX_SIZE as isize
                        || b < 0
                        || b >= MATRIX_SIZE as isize
                        || &matrix[a as usize][b as usize] != letter
                    {
                        continue 'direction;
                    }
                }
                total += 1
            }
        }
    }
    total
}

fn part2(matrix: &Vec<Vec<u8>>) -> i32 {
    let mut total = 0;
    for (i, row) in matrix.iter().enumerate().take(MATRIX_SIZE - 1).skip(1) {
        for (j, cell) in row.iter().enumerate().take(MATRIX_SIZE - 1).skip(1) {
            if cell != &b'A' {
                continue;
            }
            let nw = matrix[i - 1][j - 1];
            let ne = matrix[i - 1][j + 1];
            let se = matrix[i + 1][j + 1];
            let sw = matrix[i + 1][j - 1];
            if ((nw == b'M' && se == b'S') || (nw == b'S' && se == b'M'))
                && ((ne == b'M' && sw == b'S') || (ne == b'S' && sw == b'M'))
            {
                total += 1;
            }
        }
    }
    total
}

fn main() {
    let matrix: Vec<Vec<u8>> = INPUT
        .split(|b| b == &b'\n')
        .map(|line| line.to_vec())
        .collect();

    let start = Instant::now();
    let part1_total = part1(&matrix);
    assert!(part1_total == 2521);
    println!(
        "part 1 {} finished in {}",
        part1_total,
        (Instant::now() - start).as_nanos()
    );

    let start = Instant::now();
    let part2_total = part2(&matrix);
    assert!(part2_total == 1912);
    println!(
        "part 2 {} finished in {}",
        part2_total,
        (Instant::now() - start).as_nanos()
    );
}
