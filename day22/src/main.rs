use std::collections::{HashMap, HashSet};

christmas_tree::day!(22);

type Vec3 = glam::I64Vec3;

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    p1: Vec3,
    p2: Vec3,
}

impl PartialOrd for Cube {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Cube {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.p1
            .z
            .cmp(&other.p1.z)
            .then_with(|| self.p2.z.cmp(&other.p2.z))
    }
}

impl Cube {
    fn supports(&self, other: &Cube) -> bool {
        assert!(self.p1.z <= self.p2.z);
        self.xy_overlaps(other) && self.p2.z + 1 == other.p1.z
    }

    fn xy_overlaps(&self, other: &Cube) -> bool {
        fn xy_overlaps_impl(a: &Cube, other: &Cube) -> bool {
            a.p1.x <= other.p2.x
                && a.p1.y <= other.p2.y
                && a.p2.x >= other.p1.x
                && a.p2.y >= other.p1.y
        }
        let result = xy_overlaps_impl(self, other);
        assert_eq!(result, xy_overlaps_impl(other, self));
        result
    }

    fn move_by(&mut self, delta: Vec3) {
        let size = self.p2 - self.p1;

        self.p1 += delta;
        self.p2 += delta;

        assert_eq!(size, self.p2 - self.p1);
    }
}

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule vec() -> Vec3
            = x:number() "," y:number() "," z:number() { Vec3::new(x, y, z) }

        pub rule cube() -> Cube
            = p1:vec() "~" p2:vec() {
                assert!(p1.x <= p2.x && p1.y <= p2.y && p1.z <= p2.z);
                Cube { p1, p2 }
            }
    }
}

fn part1(input: &str) -> i64 {
    let mut cubes = input
        .lines()
        .map(|line| parser::cube(line).unwrap())
        .collect::<Vec<_>>();

    cubes.sort();

    for i in 0..cubes.len() {
        let mut bottomest = None;

        let top = cubes[i];

        assert!(top.p1.z <= top.p2.z);

        for j in 0..cubes.len() {
            let bottom = cubes[j];

            if top.xy_overlaps(&bottom) {
                if bottom.p2.z < top.p1.z {
                    bottomest = match bottomest {
                        None => Some(bottom),
                        Some(bottomest) => Some(if bottomest.p2.z >= bottom.p2.z {
                            bottomest
                        } else {
                            bottom
                        }),
                    }
                }
            }
        }

        let delta_z = if let Some(bottomest) = bottomest {
            assert!(bottomest < top);
            let delta = -(top.p1.z - bottomest.p2.z) + 1;
            // dbg!(top, bottomest, delta);
            assert!(delta <= 0);
            delta
        } else {
            -top.p1.z + 1
        };

        cubes[i].move_by([0, 0, delta_z].into())
    }

    // caca start
    for i in 0..cubes.len() {
        let mut bottomest = None;

        let top = cubes[i];

        assert!(top.p1.z <= top.p2.z);

        for j in 0..cubes.len() {
            let bottom = cubes[j];

            if top.xy_overlaps(&bottom) {
                if bottom < top {
                    bottomest = match bottomest {
                        None => Some(bottom),
                        Some(bottomest) => Some(if bottomest.p2.z >= bottom.p2.z {
                            bottomest
                        } else {
                            bottom
                        }),
                    }
                }
            }
        }

        let delta_z = if let Some(bottomest) = bottomest {
            -(top.p1.z - bottomest.p2.z) + 1
        } else {
            -top.p1.z + 1
        };

        assert!(delta_z == 0);

        cubes[i].move_by([0, 0, delta_z].into())
    }
    // caca end

    cubes.sort();

    let mut supported_by = HashMap::new();
    let mut supports = HashMap::new();

    for i in 0..cubes.len() {
        let bottom = cubes[i];
        // for j in i + 1..cubes.len() {
        for j in 0..cubes.len() {
            let top = cubes[j];

            if bottom.supports(&top) {
                {
                    assert!(top > bottom);
                    assert!(bottom.p2.z + 1 == top.p1.z);
                    assert!(bottom.xy_overlaps(&top));
                    let supports = supported_by.entry(top).or_insert_with(Vec::new);
                    assert!(!supports.contains(&bottom));
                    supports.push(bottom);
                }

                {
                    supports.entry(bottom).or_insert_with(Vec::new).push(top);
                }
            }
        }
    }

    assert!(cubes.iter().all(|cube| {
        match supported_by.get(cube) {
            Some(supports) => {
                // dbg!(&cube, &supports);
                supports.len() > 0 && !supports.contains(&cube)
            }
            None => cube.p1.z == 1,
        }
    }));

    // let mut visited = HashSet::new();
    dbg!(supported_by.len());
    dbg!(cubes.len());

    let count_by = cubes
        .iter()
        .filter(|cube| {
            supported_by
                .values()
                .inspect(|supports| assert!(supports.len() > 0))
                .filter(|supports| supports.contains(cube))
                .all(|supports| supports.len() > 1)
        })
        .inspect(|cube| {
            // dbg!(cube);
        })
        .count() as i64;

    let count_from = cubes
        .iter()
        .filter(|cube| {
            supports.get(cube).map_or(true, |supported| {
                supported.iter().all(|other| {
                    supports
                        .iter()
                        .any(|(key, others)| key != *cube && others.contains(other))
                })
            })
        })
        .count() as i64;

    assert_eq!(count_by, count_from);

    count_by
}

fn part2(input: &str) -> i64 {
    todo!()
}

christmas_tree::examples! {
    "
        1,0,1~1,2,1
        0,0,2~2,0,2
        0,2,3~2,2,3
        0,0,4~0,2,4
        2,0,5~2,2,5
        0,1,6~2,1,6
        1,1,8~1,1,9
    " => 5, 69,
}

#[test]
fn overlaps() {
    let a = Cube {
        p1: [0, 0, 0].into(),
        p2: [0, 10, 0].into(),
    };

    let b = Cube {
        p1: [0, 10, 1].into(),
        p2: [10, 10, 1].into(),
    };

    assert!(b.xy_overlaps(&a));
}
