christmas_tree::day!(10);

use std::collections::{HashMap, HashSet, VecDeque};

use glam::IVec2;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Tile {
    Pipe { vertical: bool },
    Bend { north: bool, east: bool },
}

impl TryFrom<char> for Tile {
    type Error = ();

    fn try_from(c: char) -> Result<Self, Self::Error> {
        Ok(match c {
            '|' => Tile::Pipe { vertical: true },
            '-' => Tile::Pipe { vertical: false },
            'L' => Tile::Bend {
                north: true,
                east: true,
            },
            'J' => Tile::Bend {
                north: true,
                east: false,
            },
            '7' => Tile::Bend {
                north: false,
                east: false,
            },
            'F' => Tile::Bend {
                north: false,
                east: true,
            },
            '.' | 'S' => return Err(()),
            _ => panic!("Invalid tile: {}", c),
        })
    }
}

impl Tile {
    pub const fn all_neighbor_deltas() -> [IVec2; 4] {
        [
            IVec2::new(0, 1),
            IVec2::new(0, -1),
            IVec2::new(1, 0),
            IVec2::new(-1, 0),
        ]
    }

    // TODO: Make this return [IVec2; 2]
    pub fn neighbor_deltas(&self) -> impl IntoIterator<Item = IVec2> {
        match self {
            Tile::Pipe { vertical } => {
                if *vertical {
                    vec![IVec2::new(0, 1), IVec2::new(0, -1)]
                } else {
                    vec![IVec2::new(1, 0), IVec2::new(-1, 0)]
                }
            }

            Tile::Bend { north, east } => {
                let mut output = Vec::with_capacity(2);
                if *north {
                    output.push(IVec2::new(0, -1));
                } else {
                    output.push(IVec2::new(0, 1));
                }

                if *east {
                    output.push(IVec2::new(1, 0));
                } else {
                    output.push(IVec2::new(-1, 0));
                }
                output
            }
        }
    }

    pub fn north(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => *vertical,
            Tile::Bend { north, .. } => *north,
        }
    }

    pub fn south(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => *vertical,
            Tile::Bend { north, .. } => !*north,
        }
    }

    pub fn east(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => !*vertical,
            Tile::Bend { east, .. } => *east,
        }
    }

    pub fn west(&self) -> bool {
        match self {
            Tile::Pipe { vertical } => !*vertical,
            Tile::Bend { east, .. } => !*east,
        }
    }
}

fn parse(input: &str) -> (HashMap<IVec2, Tile>, IVec2) {
    let mut output = HashMap::<IVec2, Tile>::new();
    let mut start = None;

    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let pos = IVec2::new(x as i32, y as i32);
            if let Ok(tile) = c.try_into() {
                output.insert(pos, tile);
            }

            if c == 'S' {
                start = Some(pos);
            }
        }
    }

    let start = start.unwrap();

    let start_is_north = if let Some(tile) = output.get(&(start + IVec2::new(0, -1))) {
        tile.south()
    } else {
        false
    };

    let start_is_east = if let Some(tile) = output.get(&(start + IVec2::new(1, 0))) {
        tile.west()
    } else {
        false
    };

    output.insert(start, Tile::Bend {
        north: start_is_north,
        east: start_is_east,
    });

    (output, start)
}

fn part1(input: &str) -> i64 {
    let (map, start) = parse(input);

    let mut queue = VecDeque::new();
    let mut visited = HashSet::new();
    let mut max_steps = 0;

    queue.push_back((start, 0, None));

    while let Some((pos, steps, prev_pos)) = queue.pop_front() {
        if !visited.insert(pos) {
            continue;
        }

        let Some(tile) = map.get(&pos) else {
            continue;
        };

        let mut added_count = 0;
        let mut found_previous = 0;
        for delta in tile.neighbor_deltas() {
            let neighbor = pos + delta;

            match map.get(&neighbor) {
                None => continue,
                Some(tile) => {
                    if !tile.neighbor_deltas().into_iter().any(|d| d == -delta) {
                        continue;
                    }
                }
            }

            match visited.contains(&neighbor) {
                false => {
                    queue.push_back((neighbor, steps + 1, Some(pos)));
                    added_count += 1;
                }
                true => {
                    found_previous += 1;
                }
            };
        }

        if prev_pos.is_some() && found_previous == 0 {
            for _ in 0..added_count {
                queue.pop_back();
            }
        }

        if found_previous == 2 {
            assert_eq!(max_steps, 0);
            max_steps = steps;
        }
    }

    max_steps
}

fn parse2(input: &str) -> (HashMap<IVec2, Tile>, IVec2, IVec2) {
    let mut output = HashMap::<IVec2, Tile>::new();
    let mut start = None;
    let mut size = IVec2::ZERO;

    for (y, line) in input.lines().enumerate() {
        size.y = size.y.max(y as i32);
        size.x = size.y.max(line.len() as i32);

        for (x, c) in line.chars().enumerate() {
            if let Ok(tile) = c.try_into() {
                output.insert(IVec2::new(x as i32 * 2, y as i32 * 2), tile);
            }

            if c == 'S' {
                start = Some(IVec2::new(x as i32 * 2, y as i32 * 2));
            }
        }
    }

    let size = size * 2;

    let start = start.unwrap();

    let start_is_north = if let Some(tile) = output.get(&(start + IVec2::new(0, -1))) {
        tile.south()
    } else {
        false
    };

    let start_is_east = if let Some(tile) = output.get(&(start + IVec2::new(1, 0))) {
        tile.west()
    } else {
        false
    };

    output.insert(start, Tile::Bend {
        north: start_is_north,
        east: start_is_east,
    });

    for y in 0..=size.y {
        for x in 0..=size.x {
            if y % 2 == 0 && x % 2 == 0 {
                continue;
            }

            if let [Some(left), Some(right)] = [
                output.get(&[x - 1, y].into()),
                output.get(&[x + 1, y].into()),
            ] {
                if left.east() || right.west() {
                    output.insert(IVec2::new(x, y), Tile::Pipe { vertical: false });
                }
            }

            if let [Some(top), Some(bottom)] = [
                output.get(&[x, y - 1].into()),
                output.get(&[x, y + 1].into()),
            ] {
                if top.south() || bottom.north() {
                    output.insert(IVec2::new(x, y), Tile::Pipe { vertical: true });
                }
            }
        }
    }


    (output, start, size)
}

fn get_loop(map: &HashMap<IVec2, Tile>, start: IVec2) -> HashSet<IVec2> {
    let mut output = HashSet::new();

    let mut pos = start;

    loop {
        let tile = map.get(&pos).unwrap();

        output.insert(pos);

        let mut found = false;

        for delta in tile.neighbor_deltas() {
            if output.contains(&(pos + delta)) {
                continue;
            }

            let neighbor = pos + delta;

            if map.contains_key(&neighbor) {
                pos = neighbor;
                found = true;
                break;
            }
        }

        if !found {
            break;
        }
    }

    output
}

fn part2(input: &str) -> i64 {
    let (map, start, size) = parse2(input);

    for y in 0..=size.y {
        for x in 0..size.x {
            print!(
                "{}",
                match map.get(&IVec2::new(x, y)) {
                    None => '.',
                    Some(Tile::Pipe { vertical: true }) => '|',
                    Some(Tile::Pipe { vertical: false }) => '-',
                    Some(Tile::Bend {
                        north: true,
                        east: true,
                    }) => 'L',
                    Some(Tile::Bend {
                        north: true,
                        east: false,
                    }) => 'J',
                    Some(Tile::Bend {
                        north: false,
                        east: false,
                    }) => '7',
                    Some(Tile::Bend {
                        north: false,
                        east: true,
                    }) => 'F',
                }
            );
        }

        println!();
    }

    let main_loop = get_loop(&map, start);

    dbg!(&main_loop);

    let is_outside = |pos: IVec2| pos.x < 0 || pos.y < 0 || pos.x > size.x || pos.y > size.y;

    let mut count = 0;

    for (x, y) in (0..=size.x).flat_map(|x| (0..=size.y).map(move |y| (x, y))) {
        let starting_pos = IVec2::new(x, y);
        let mut queue = Vec::new();
        let mut is_always_inside = true;
        let mut visited = HashSet::new();
        queue.push(starting_pos);

        if
        main_loop.contains(&starting_pos) ||
        !(x % 2 == 0 && y % 2 == 0) {
            continue;
        }

        while let Some(pos) = queue.pop() {
            if is_outside(pos) {
                is_always_inside = false;
                break;
            }

            if main_loop.contains(&pos) {
                continue;
            }

            if !visited.insert(pos) {
                continue;
            }

            for delta in Tile::all_neighbor_deltas() {
                queue.push(pos + delta);
            }
        }

        if is_always_inside {
            // dbg!(starting_pos);
            count += 1;
        }
    }

    count
}

christmas_tree::examples! {
    r"
        7-F7-
        .FJ|7
        SJLL7
        |F--J
        LJ.LJ
    " => 8,

    // r"
    // ..........
    // .S------7.
    // .|F----7|.
    // .||....||.
    // .||....||.
    // .|L-7F-J|.
    // .|..||..|.
    // .L--JL--J.
    // ..........
    // " => 4,
    //
    // r"
    //     .F----7F7F7F7F-7....
    //     .|F--7||||||||FJ....
    //     .||.FJ||||||||L7....
    //     FJL7L7LJLJ||LJ.L-7..
    //     L--J.L7...LJS7F-7L7.
    //     ....F-J..F7FJ|L7L7L7
    //     ....L7.F7||L7|.L7L7|
    //     .....|FJLJ|FJ|F7|.LJ
    //     ....FJL-7.||.||||...
    //     ....L---J.LJ.LJLJ...
    // " => 8,

    r"
        FF7FSF7F7F7F7F7F---7
        L|LJ||||||||||||F--J
        FL-7LJLJ||||||LJL-77
        F--JF--7||LJLJ7F7FJ-
        L---JF-JLJ.||-FJLJJ7
        |F|F-JF---7F7-L7L|7|
        |FFJF7L7F-JF7|JL---7
        7-L-JL7||F7|L7F-7F7|
        L.L7LFJ|||||FJL7||LJ
        L7JLJL-JLJLJL--JLJ.L
    " => 10,
}
