const INPUT: &str = include_str!("input.txt");

#[derive(Debug, Clone)]
struct Item {
    size: u32,
    id: Option<usize>,
}

fn _print_list(list: &Vec<Item>) {
    // println!("{:?}", list);
    println!("{:?}", _format_list(list));
}

fn _format_list(list: &Vec<Item>) -> String {
    let mut s = String::new();
    for e in list {
        for _ in 0..e.size {
            if let Some(id) = e.id {
                s.push(char::from_digit(id.try_into().unwrap(), 10).unwrap());
            } else {
                s.push('.');
            }
        }
    }
    s
}

fn input_to_file_list(input: &str) -> Vec<Item> {
    let mut list = Vec::<Item>::new();

    for (i, char) in input.chars().enumerate() {
        let number = char.to_digit(10).unwrap();
        if i % 2 == 0 {
            list.push(Item {
                size: number,
                id: Some(i / 2),
            });
        } else {
            list.push(Item {
                size: number,
                id: None,
            });
        }
    }
    list
}

fn part1(mut list: Vec<Item>) -> Vec<Item> {
    let mut b = list.len() - 1;
    let mut f = 1;
    while b >= f {
        if list[b].size == list[f].size {
            list.swap(b, f);
            b -= 2;
            f += 2;
        } else if list[b].size > list[f].size {
            list[f].id = list[b].id;
            list[b].size -= list[f].size;
            f += 2;
        } else {
            let new_element_size = list[f].size - list[b].size;
            list[f] = list[b].clone();
            list[b].id = None;
            f += 1;
            list.insert(
                f,
                Item {
                    size: new_element_size,
                    id: None,
                },
            );
            b -= 1;
        }
    }
    list
}

fn part2(mut list: Vec<Item>) -> Vec<Item> {
    let mut b = list.len() - 1;

    while b > 0 {
        if list[b].id.is_none() {
            b -= 1;
            continue;
        }

        let mut f = 0;
        while f <= b {
            if list[f].id.is_some() {
                f += 1;
                continue;
            }
            if list[b].size == list[f].size {
                list.swap(b, f);
                break;
            } else if list[b].size < list[f].size {
                let new_element_size = list[f].size - list[b].size;
                list[f] = list[b].clone();
                list[b].id = None;
                list.insert(
                    f + 1,
                    Item {
                        size: new_element_size,
                        id: None,
                    },
                );
                break;
            } else {
                f += 1;
            }
        }
        b -= 1;
    }
    list
}

fn find_solution(list: Vec<Item>) -> usize {
    let mut i = 0;
    let mut solution = 0;
    for e in list {
        if let Some(id) = e.id {
            for _ in 0..e.size {
                solution += i * id;
                i += 1;
            }
        } else {
            i += e.size as usize;
        }
    }
    solution
}

fn main() {
    let _example = "2333133121414131402";
    let input = INPUT;
    let list = input_to_file_list(input);

    let part1_list = part1(list.clone());
    let part1_solution = find_solution(part1_list);
    assert!(part1_solution == 6307275788409);
    println!("{}", part1_solution);

    let part2_list = part2(list.clone());
    let part2_solution = find_solution(part2_list);
    assert!(part2_solution == 6327174563252);
    println!("{}", part2_solution);
}
