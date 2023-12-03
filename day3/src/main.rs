use std::collections::{HashMap, HashSet};

christmas_tree::day!(3);

fn parse(
    input: &str,
    symbol_predicate: impl Fn(char) -> bool,
) -> (HashMap<(i32, i32), (i32, i32, i32)>, HashSet<(i32, i32)>) {
    let mut numbers = HashMap::<(i32, i32), (i32, i32, i32)>::new();
    let mut symbols = HashSet::<(i32, i32)>::new();

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;
        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            if let Some(digit) = c.to_digit(10) {
                let digit = digit as i32;
                if let Some(&(prev, length, _index)) = numbers.get(&(x - 1, y)) {
                    let number = prev * 10 + digit;
                    for i in 0..=length {
                        numbers.insert((x - length + i, y), (number, length + 1, i));
                    }
                } else {
                    numbers.insert((x, y), (digit, 1, 0));
                }
            } else if symbol_predicate(c) {
                symbols.insert((x, y));
            }
        }
    }

    (numbers, symbols)
}

fn neighbor_offsets() -> impl Iterator<Item = (i32, i32)> {
    (-1i32..=1)
        .flat_map(|dy| (-1i32..=1).map(move |dx| (dx, dy)))
        .filter(|(dx, dy)| *dx != 0 || *dy != 0)
}

fn part1(input: &str) -> i32 {
    let (mut numbers, symbols) = parse(input, |c| c != '.');

    let mut count = 0i32;
    for (x, y) in symbols {
        for (dx, dy) in neighbor_offsets() {
            let neighbor = (x + dx, y + dy);
            if let Some(&(number, length, index)) = numbers.get(&neighbor) {
                count += number;

                for i in 0..length {
                    let removed = numbers.remove(&(neighbor.0 - index + i, neighbor.1));
                    assert!(removed.is_some());
                }
            }
        }
    }

    count
}

fn part2(input: &str) -> i32 {
    let (mut numbers, symbols) = parse(input, |c| c == '*');

    let mut output = 0;
    'outer: for (x, y) in symbols {
        let mut total_neighbors = 0;
        let mut part_numbers = [0; 2];

        for (dx, dy) in neighbor_offsets() {
            let neighbor = (x + dx, y + dy);
            if let Some(&(number, length, index)) = numbers.get(&neighbor) {
                total_neighbors += 1;

                if total_neighbors > 2 {
                    continue 'outer;
                }

                part_numbers[total_neighbors as usize - 1] = number;

                for i in 0..length {
                    let removed = numbers.remove(&(neighbor.0 - index + i, neighbor.1));
                    assert!(removed.is_some());
                }
            }
        }

        let gear_ratio = part_numbers[0] * part_numbers[1];
        output += gear_ratio;
    }

    output
}

christmas_tree::examples! {
    r"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    " => 4361,

    r"
        467..114..
        ...*......
        ..35..633.
        ......#...
        617*......
        .....+.58.
        ..592.....
        ......755.
        ...$.*....
        .664.598..
    " => 467835,
}
