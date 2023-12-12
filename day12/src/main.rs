use std::{collections::HashMap, cell::RefCell};

use memoize::memoize;
use rayon::{prelude::ParallelIterator, str::ParallelString};

christmas_tree::day!(12);

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule tile() -> Option<Tile>
            = "#" { Some(Tile::Beacon) }
            / "?" { Some(Tile::Broken) }
            / "." { None }

        rule _ = [' ' | '\t' | '\n']*

        pub rule line() -> (Vec<Option<Tile>>, Vec<i64>)
            = tiles:tile()+ _ numbers:number() ** "," { (tiles, numbers) }

        pub rule lines() -> Vec<(Vec<Option<Tile>>, Vec<i64>)>
            = l:line() ** "\n" { l }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
enum Tile {
    Beacon,
    Broken,
}

fn count_arrangments(
    beacons: &mut [Option<Tile>],
    bundles: &mut [i64],
    prev_was_beacon: bool,
    cache: &mut HashMap<(Vec<Option<Tile>>, Vec<i64>), i64>
) -> i64 {
    let key = (beacons.to_vec(), bundles.to_vec());
    if let Some(output) = cache.get(&key) {
        return *output;
    }

    let Some(bundle) = bundles.get(0) else {
        return if beacons.iter().any(|b| matches!(b, Some(Tile::Beacon))) {
            0
        } else {
            1
        };
    };

    let output = match beacons.get(0) {
        Some(Some(Tile::Beacon)) => {
            if *bundle == 0 {
                0
            } else {
                debug_assert!(*bundle > 0);
                bundles[0] -= 1;
                let output = count_arrangments(&mut beacons[1..], bundles, true, cache);
                bundles[0] += 1;
                output
            }
        }
        Some(None) => {
            if prev_was_beacon {
                if *bundle == 0 {
                    count_arrangments(&mut beacons[1..], &mut bundles[1..], false, cache)
                } else {
                    debug_assert!(*bundle > 0);
                    0
                }
            } else {
                debug_assert!(*bundle > 0);
                count_arrangments(&mut beacons[1..], bundles, false, cache)
            }
        }
        Some(Some(Tile::Broken)) => {
            let mut output = 0;

            beacons[0] = Some(Tile::Beacon);
            output += count_arrangments(beacons, bundles, prev_was_beacon, cache);

            beacons[0] = None;
            output += count_arrangments(beacons, bundles, prev_was_beacon, cache);

            beacons[0] = Some(Tile::Broken);

            output
        }
        None => {
            if *bundle == 0 && bundles.len() == 1 {
                1
            } else {
                0
            }
        }
    };

    cache.insert(key, output);
    output
}

fn part1(input: &str) -> i64 {
    input
        .lines()
        .map(|line| {
            let (mut beacons, mut bundles) = parser::line(line).unwrap();
            count_arrangments(&mut beacons, &mut bundles, false, &mut HashMap::new())
        })
        .sum()
}

fn part2(input: &str) -> i64 {
    input
        .par_lines()
        .map(|line| {
            let (beacons, bundles) = parser::line(line).unwrap();
            let mut beacons = vec![beacons; 5].join(&Some(Tile::Broken));
            let mut bundles = std::iter::repeat(bundles)
                .take(5)
                .flatten()
                .collect::<Vec<_>>();

            count_arrangments(
                &mut beacons,
                &mut bundles,
                false,
                &mut HashMap::new(),
            )
        })
        .sum()
}

christmas_tree::examples! {
    r"
        ???.### 1,1,3
        .??..??...?##. 1,1,3
        ?#?#?#?#?#?#?#? 1,3,1,6
        ????.#...#... 4,1,1
        ????.######..#####. 1,6,5
        ?###???????? 3,2,1
    " => 21, 525152,
/*
*/
}

#[test]
fn is_valid() {
    let beacons = [
        Some(Tile::Beacon),
        Some(Tile::Beacon),
        Some(Tile::Beacon),
        None,
        Some(Tile::Beacon),
        Some(Tile::Beacon),
        None,
        Some(Tile::Beacon),
    ];

    assert!(is_valid_beacon_arrangement(&beacons, &[3, 2, 1]));
    assert!(!is_valid_beacon_arrangement(&beacons, &[2, 1, 2, 2]));
}
