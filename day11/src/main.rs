use glam::I64Vec2;
use std::collections::HashSet;

christmas_tree::day!(11);

fn parse(input: &str, expansion_multiplier: i64) -> Vec<I64Vec2> {
    let expansion_size = expansion_multiplier - 1;

    let size = I64Vec2::new(
        input.lines().next().unwrap().len() as i64,
        input.lines().count() as i64,
    );

    let input = input.as_bytes();

    let index = |x, y| input[(y * (size.x + 1) + x) as usize];

    let empty_columnns = (0..size.x).filter(|&x| (0..size.y).all(|y| index(x, y) == b'.')).collect::<HashSet<_>>();

    let mut output = Vec::new();
    let mut offset = I64Vec2::ZERO;
    for y in 0..size.y {
        offset.x = 0;
        let mut is_column_empty = true;

        for x in 0..size.x {
            if empty_columnns.contains(&x) {
                offset.x += expansion_size;
            } else if index(x, y) == b'#' {
                output.push(I64Vec2::new(x, y) + offset);
                is_column_empty = false;
            }
        }

        if is_column_empty {
            offset.y += expansion_size;
        }
    }

    output
}

fn solve(input: &str, expansion_multiplier: i64) -> i64 {
    let galaxies = parse(input, expansion_multiplier);

    let mut iter = galaxies.into_iter();
    let mut count = 0;

    while let Some(galaxy) = iter.next() {
        for other in iter.clone() {
            let I64Vec2 { x, y } = (galaxy - other).abs();
            count += x + y;
        }
    }

    count
}

fn part1(input: &str) -> i64 {
    solve(input, 2)
}

fn part2(input: &str) -> i64 {
    solve(input, 1_000_000)
}

christmas_tree::examples! {
    r"
        ...#......
        .......#..
        #.........
        ..........
        ......#...
        .#........
        .........#
        ..........
        .......#..
        #...#.....
    " => 374, 82000210, // Got the second example myself, because given example has different parameters
}
