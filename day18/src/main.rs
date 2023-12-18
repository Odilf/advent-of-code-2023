use std::{
    collections::{BTreeSet, HashSet},
    ops::Deref,
};

christmas_tree::day!(18);

type Vec2 = glam::I64Vec2;

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule direction() -> Direction
            = "U" { Direction::Up }
            / "D" { Direction::Down }
            / "L" { Direction::Left }
            / "R" { Direction::Right }

        rule hex_digit() -> u8
             = n:$(['0'..='9' | 'a'..='f' | 'A'..='F'] * <2>) { u8::from_str_radix(n, 16).unwrap() }

        rule color() -> [u8; 3]
            = "#" r:hex_digit() g:hex_digit() b:hex_digit() { [r, g, b] }

        pub rule instruction() -> Instruction
            = d:direction() " " n:number() " (" c:color() ")" {
                Instruction { direction: d, distance: n, color: c }
            }
    }
}

enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn delta(&self) -> Vec2 {
        match self {
            Direction::Up => Vec2::new(0, -1),
            Direction::Down => Vec2::new(0, 1),
            Direction::Left => Vec2::new(-1, 0),
            Direction::Right => Vec2::new(1, 0),
        }
    }
}

struct Instruction {
    direction: Direction,
    distance: i64,
    color: [u8; 3],
}

fn part1(input: &str) -> i64 {
    let instructions = input.lines().map(|line| parser::instruction(line).unwrap());

    let mut position = Vec2::ZERO;
    let mut map = HashSet::new();

    map.insert(position);

    for instruction in instructions {
        let delta = instruction.direction.delta();
        for _ in 0..instruction.distance {
            position += delta;
            map.insert(position);
        }
    }

    let min = Vec2::new(
        map.iter().map(|p| p.x).min().unwrap(),
        map.iter().map(|p| p.y).min().unwrap(),
    );

    let max = Vec2::new(
        map.iter().map(|p| p.x).max().unwrap(),
        map.iter().map(|p| p.y).max().unwrap(),
    );

    let mut count = 0;
    for y in min.y..=max.y {
        let mut inside = false;

        for x in min.x..=max.x {
            let position = Vec2::new(x, y);
            if map.contains(&position) {
                match inside {
                    false => inside = map.contains(&(position + Vec2::new(0, -1))),
                    true => inside = !map.contains(&(position + Vec2::new(0, -1))),
                }

                print!("#");
                count += 1;
            } else if inside {
                print!(".");
                count += 1;
            } else {
                print!(".");
            }
        }
        println!();
    }

    count
}

#[derive(Debug, Clone, Copy)]
struct Wall {
    origin: Vec2,
    direction: Vec2,
    length: i64,
}

impl Wall {
    pub fn contains(&self, position: &Vec2) -> bool {
        let delta = (*position - self.origin) * self.direction;
        if self.direction.x == 0 {
            position.x == self.origin.x && 0 <= delta.y && delta.y < self.length 
        } else if self.direction.y == 0 {
            position.y == self.origin.y && 0 <= delta.x && delta.x < self.length
        } else {
            unreachable!()
        }
    }
    pub fn contains_y(&self, y: i64) -> bool {
        if self.direction.x == 0 {
            let delta = (y - self.origin.y) * self.direction.y;
            delta >= 0 && delta < self.length
        } else if self.direction.y == 0 {
            self.origin.y == y
        } else {
            unreachable!()
        }
    }

    pub fn end_x(&self) -> i64 {
        self.origin.x + self.direction.x * (self.length - 1)
    }

    pub fn end_y(&self) -> i64 {
        self.origin.y + self.direction.y * (self.length - 1)
    }
}

struct Map {
    walls: Vec<Wall>,
}

impl Map {
    pub fn contains(&self, position: &Vec2) -> bool {
        self.walls.iter().any(|wall| wall.contains(position))
    }
}

fn part2(input: &str) -> i64 {
    let instructions = input.lines().map(|line| parser::instruction(line).unwrap());

    let mut position = Vec2::ZERO;
    let mut map = Map { walls: Vec::new() };

    for instruction in instructions {
        dbg!(instruction.color);
        let [r, g, b] = instruction.color;
        let encoded = (r as u32) << 16 | (g as u32) << 8 | (b as u32);
        let direction = encoded % 16;
        let distance = encoded as i64 / 16;

        let delta = match direction {
            0 => Direction::Right,
            1 => Direction::Down,
            2 => Direction::Left,
            3 => Direction::Up,
            _ => unreachable!(),
        }
        .delta();

        // let delta = instruction.direction.delta();
        // let distance = instruction.distance;
        
        map.walls.push(Wall {
            origin: position,
            direction: delta,
            length: distance,
        });

        dbg!(&map.walls.last().unwrap());

        position += delta * distance as i64;
    }


    let min = Vec2::new(
        map.walls
            .iter()
            .map(|p| p.origin.x.min(p.end_x()))
            .min()
            .unwrap(),
        map.walls
            .iter()
            .map(|p| p.origin.y.min(p.end_y()))
            .min()
            .unwrap(),
    );

    let max = Vec2::new(
        map.walls
            .iter()
            .map(|p| p.origin.x.max(p.end_x()))
            .max()
            .unwrap(),
        map.walls
            .iter()
            .map(|p| p.origin.y.max(p.end_y()))
            .max()
            .unwrap(),
    );

    map.walls.sort_unstable_by_key(|wall| wall.origin.x.min(wall.end_x()));

    dbg!(min, max);

    let mut count = 0;
    for y in min.y..=max.y {

        if y % 100_00
        // dbg!(y);
        let mut walls = map
            .walls
            .iter()
            .filter(|wall| wall.contains_y(y))
            .peekable();

        let walls_vec = walls.clone().collect::<Vec<_>>();

        let mut next_wall = || {
            let wall = walls.next()?;
            let start = wall.origin.x.min(wall.end_x());
            let end_first = wall.origin.x.max(wall.end_x());

            if let Some(next) = walls.peek() {
                if wall.direction.y == 0 || next.direction.y == 0 {
                    let first_goes_up = map.contains(&Vec2::new(start, y - 1));
                    let first_goes_down = map.contains(&Vec2::new(start, y + 1));
                    let start_second = next.origin.x.min(next.end_x());
                    let end_second = next.origin.x.max(next.end_x());

                    if end_first + 1 != start_second {
                        return Some((
                            start,
                            end_first,
                        ))
                    }

                    let second_goes_up = map.contains(&Vec2::new(end_second, y - 1));
                    let second_goes_down = map.contains(&Vec2::new(end_second, y + 1));

                    // dbg!(first_goes_up, first_goes_down, second_goes_up, second_goes_down, start, end_second);

                    assert_ne!(first_goes_up, first_goes_down);
                    assert_ne!(second_goes_up, second_goes_down);

                    if first_goes_up != second_goes_up {
                        let _next = walls.next().unwrap();

                        return Some((
                            start,
                            end_second,
                        ))
                    }
                }
            };

            Some((
                start,
                end_first,
            ))
        };

        // dbg!(y);

        while let Some((start, _)) = next_wall() {
            let Some((_, end)) = next_wall() else {
                dbg!(&walls_vec);
                dbg!(&y);
                // continue;
                panic!("odd walls");
            };
            count += end - start + 1;
        }
    }

    count

    // let mut count = 0;
    // let mut y = min.y;
    // let mut start;
    // let mut end = 0;
    //
    // for pos in &map {
    //     dbg!(pos);
    //     if pos.y == y {
    //         end = pos.x;
    //     } else {
    //         y = pos.y;
    //         start = pos.x;
    //
    //         count += end - start + 1;
    //     }
    // }
    // for y in min.y..=max.y {
    //     dbg!(y);
    //     let mut inside = false;
    //
    //     for x in min.x..=max.x {
    //         let position = Vec2::new(x, y);
    //         if map.contains(&OVec2(position)) {
    //             match inside {
    //                 false => inside = map.contains(&OVec2(position + Vec2::new(0, -1))),
    //                 true => inside = !map.contains(&OVec2(position + Vec2::new(0, -1))),
    //             }
    //
    //             count += 1;
    //         } else if inside {
    //             count += 1;
    //         }
    //     }
    // }
    //
    // count
}

christmas_tree::examples! {
    "
        R 6 (#70c710)
        D 5 (#0dc571)
        L 2 (#5713f0)
        D 2 (#d2c081)
        R 2 (#59c680)
        D 2 (#411b91)
        L 5 (#8ceee2)
        U 2 (#caa173)
        L 1 (#1b58a2)
        U 2 (#caa171)
        R 2 (#7807d2)
        U 3 (#a77fa3)
        L 2 (#015232)
        U 2 (#7a21e3)
    " => 62, 952_408_144_115,
}
