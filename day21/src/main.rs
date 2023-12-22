use std::{collections::HashSet, mem};

christmas_tree::day!(21);

type Vec2 = glam::I64Vec2;

fn parse(input: &str) -> (HashSet<Vec2>, Vec2, Vec2) {
    let mut start = Vec2::ZERO;
    let mut size = Vec2::ZERO;

    let mut walls = HashSet::new();

    for (y, line) in input.lines().enumerate() {
        size.y = size.y.max(y as i64 + 1);
        for (x, c) in line.chars().enumerate() {
            size.x = size.x.max(x as i64 + 1);
            let pos = Vec2::new(x as i64, y as i64);

            if c == '#' {
                walls.insert(pos);
            } else if c == 'S' {
                start = pos;
            }
        }
    }

    (walls, start, size)
}

const DIRECTIONS: [Vec2; 4] = [
    Vec2::new(0, -1),
    Vec2::new(1, 0),
    Vec2::new(0, 1),
    Vec2::new(-1, 0),
];

fn part1(input: &str) -> i64 {
    let (walls, start, size) = parse(input);

    count_steps(&walls, start, 64, size)
}

fn count_steps(walls: &HashSet<Vec2>, start: Vec2, steps: i64, size: Vec2) -> i64 {
    let mut count = 0;

    let mut next_queue = vec![start];
    let mut visited = HashSet::new();
    let mut counted = HashSet::new();

    let mut left = [None; 4];
    let mut completed_main = 0;

    for i in 0..=steps {
        let mut queue = Vec::new();
        mem::swap(&mut queue, &mut next_queue);

        if queue.is_empty() {
            completed_main = i;
            break;
        }

        for node in queue {
            let actual = Vec2::new(node.x.rem_euclid(size.x), node.y.rem_euclid(size.y));
            for (j, dir) in DIRECTIONS.iter().enumerate() {
                if actual * *dir != node * *dir {
                    left[j] = Some((i, counted.contains(&actual)));
                    continue;
                }
            }

            if !visited.insert(node) || walls.contains(&actual) {
                continue;
            }

            if i % 2 == 0 {
                count += 1;
                counted.insert(node);
            }

            for dir in DIRECTIONS.iter() {
                if actual * *dir != node * *dir {
                    continue;
                }

                let next = node + *dir;
                next_queue.push(next);
            }
        }
    }

    let main_count = count;

    dbg!(main_count);

    for left in left {
        let (left_index, same_parity) = left.unwrap();

        // count += main_count * (left_index * n + main_completed)
        let mutliplier = (steps - completed_main) as f64 / left_index as f64;

        count += (main_count as f64 * mutliplier).floor() as i64;
    }

    dbg!(count);

    // for y in 0..size.y {
    //     for x in 0..size.x {
    //         let pos = Vec2::new(x, yli;
    //         if walls.contains(&pos) {
    //             print!("#");
    //         } else if counted.contains(&pos) {
    //             print!("O");
    //         } else if visited.contains(&pos) {
    //             print!("o");
    //         } else {
    //             print!(".");
    //         }
    //     }
    //     println!();
    // }
    //
    // 
    // count
    //
    todo!()
}

fn part2(input: &str) -> i64 {
    let (walls, start, size) = parse(input);

    count_steps(&walls, start, 5000, size)
    // count_steps(&walls, start, 26501365)
}

christmas_tree::examples! {
    "
        ...........
        .....###.#.
        .###.##..#.
        ..#.#...#..
        ....#.#....
        .##..S####.
        .##..#...#.
        .......##..
        .##.#.####.
        .##..##.##.
        ...........
    " => 16, 16733044,
}
