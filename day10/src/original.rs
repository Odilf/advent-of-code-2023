use super::*;

fn is_original(position: impl Into<IVec2>) -> bool {
    let position = position.into();
    position.x % 2 == 0 && position.y % 2 == 0
}

fn parse_part2(input: &str) -> (HashMap<IVec2, Tile>, IVec2, IVec2) {
    let mut output = HashMap::<IVec2, Tile>::new();
    let mut start = None;
    let mut size = IVec2::ZERO;

    for (y, line) in input.lines().enumerate() {
        let y = y as i32;

        size.y = size.y.max(y);
        size.x = size.y.max(line.len() as i32);

        for (x, c) in line.chars().enumerate() {
            let x = x as i32;
            let pos = IVec2::new(x, y) * 2;
            if let Ok(tile) = c.try_into() {
                output.insert(pos, tile);
            }

            if c == 'S' {
                assert!(start.is_none());
                start = Some(pos);
            }
        }
    }

    let size = size * 2;

    let start = start.unwrap();
    set_start(&mut output, start);

    for y in 0..=size.y {
        for x in 0..=size.x {
            let pos = IVec2::new(x, y);
            if is_original(pos) {
                continue;
            }

            if let [Some(west), Some(east)] = [output.get(&(pos + WEST)), output.get(&(pos + EAST))]
            {
                if west.east() || east.west() {
                    output.insert(IVec2::new(x, y), Tile::Pipe { vertical: false });
                }
            }

            if let [Some(north), Some(south)] =
                [output.get(&(pos + NORTH)), output.get(&(pos + SOUTH))]
            {
                if north.south() || south.north() {
                    output.insert(IVec2::new(x, y), Tile::Pipe { vertical: true });
                }
            }
        }
    }

    (output, start, size)
}

pub fn part2(input: &str) -> i64 {
    let (map, start, size) = parse_part2(input);

    let main_loop = get_loop(&map, start);
    
    let is_outside = |pos: IVec2| pos.x < 0 || pos.y < 0 || pos.x > size.x || pos.y > size.y;

    let mut count = 0;

    for (x, y) in (0..=size.x).flat_map(|x| (0..=size.y).map(move |y| (x, y))) {
        let starting_pos = IVec2::new(x, y);
        let mut queue = Vec::new();
        let mut is_always_inside = true;
        let mut visited = HashSet::new();

        queue.push(starting_pos);

        if main_loop.contains_key(&starting_pos) || !(x % 2 == 0 && y % 2 == 0) {
            continue;
        }

        while let Some(pos) = queue.pop() {
            if is_outside(pos) {
                is_always_inside = false;
                break;
            }

            if main_loop.contains_key(&pos) || !visited.insert(pos) {
                continue;
            }

            for delta in Tile::all_neighbor_deltas() {
                queue.push(pos + delta);
            }
        }

        if is_always_inside {
            count += 1;
        }
    }

    count
}
