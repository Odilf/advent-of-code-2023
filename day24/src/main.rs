use std::collections::HashSet;

use nalgebra::{vector, Matrix6};

christmas_tree::day!(24);

// type Vec2 = glam::I64Vec2;
type Vec3 = glam::I64Vec3;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line {
    position: Vec3,
    direction: Vec3,
}

// impl Line {
//     pub fn distance_squared(&self, other: &Self) -> f32 {
//         let perp = self.direction.cross(other.direction);
//
//         let matrix = glam::mat3(
//             self.direction.as_vec3(),
//             -other.direction.as_vec3(),
//             perp.as_vec3(),
//         );
//
//         let inverse = matrix.inverse();
//
//         let ts = inverse * (other.origin.as_vec3() - self.origin.as_vec3());
//
//         ts.z.abs() * perp.length_squared() as f32
//     }
// }

peg::parser! {
    grammar parser() for str {
        rule _ = [' ' | '\n']*

        rule number() -> i64
            = n:$(['-' | '0'..='9']+) { n.parse().unwrap() }

        pub rule vector() -> Vec3
            = x:number() "," _ y:number() "," _ z:number() { Vec3::new(x, y, z) }

        pub rule line() -> Line
            = position:vector() _ "@" _ direction:vector() { Line { position, direction } }
    }
}

fn part1(input: &str) -> i64 {
    solve1(input, 200_000_000_000_000, 400_000_000_000_000)
}

fn solve1(input: &str, min: i64, max: i64) -> i64 {
    let lines = input
        .lines()
        .map(|line| parser::line(line).unwrap())
        .collect::<Vec<_>>();

    let min = min as f32;
    let max = max as f32;

    let mut count = 0;
    for i in 0..lines.len() {
        for j in i + 1..lines.len() {
            let (a, b) = (&lines[i], &lines[j]);

            let a_p = a.position.as_vec3();
            let b_p = b.position.as_vec3();

            let a_d = a.direction.as_vec3();
            let b_d = b.direction.as_vec3();

            let matrix = glam::Mat3::from_cols(a_d, -b_d, glam::Vec3::Z);

            if matrix.determinant() == 0.0 {
                continue;
            }

            let inverse = matrix.inverse();

            let lambdas = inverse * (b_p - a_p);

            if lambdas.x < 0.0 || lambdas.y < 0.0 {
                continue;
            }

            let intersection = a_p + a_d * lambdas.x;

            if min < intersection.x
                && intersection.x < max
                && min < intersection.y
                && intersection.y < max
            {
                count += 1;
            }
        }
    }

    count
}

// const DIRECTIONS: [Vec3; 6] = [
//     Vec3::X,
//     Vec3::Y,
//     Vec3::Z,
//     Vec3::NEG_X,
//     Vec3::NEG_Y,
//     Vec3::NEG_Z,
// ];

// #[derive(Debug, Clone, Copy, PartialEq)]
// struct Node {
//     guess: Line,
//     loss: f32,
// }
//
// impl Eq for Node {}
//
// impl Ord for Node {
//     fn cmp(&self, other: &Self) -> std::cmp::Ordering {
//         self.loss
//             .partial_cmp(&other.loss)
//             .unwrap_or(std::cmp::Ordering::Equal)
//     }
// }
//
// impl PartialOrd for Node {
//     fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
//         self.loss.partial_cmp(&other.loss)
//     }
// }

fn do_the_thing(lines: [Line; 3]) -> Option<(Vec3, Vec3)> {
    let l = lines;

    let pd1 = (l[0].position - l[1].position).as_dvec3();
    let nd1 = (l[0].direction - l[1].direction).as_dvec3();

    let pd2 = (l[0].position - l[2].position).as_dvec3();
    let nd2 = (l[0].direction - l[2].direction).as_dvec3();

    let matrix = Matrix6::from_rows(&[
        vector![0.0, nd1.z, -nd1.y, 0.0, pd1.z, -pd1.y].transpose(),
        vector![-nd1.z, 0.0, nd1.x, -pd1.z, 0.0, pd1.x].transpose(),
        vector![nd1.y, -nd1.x, 0.0, pd1.y, -pd1.x, 0.0].transpose(),
        vector![0.0, nd2.z, -nd2.y, 0.0, pd2.z, -pd2.y].transpose(),
        vector![-nd2.z, 0.0, nd2.x, -pd2.z, 0.0, pd2.x].transpose(),
        vector![nd2.y, -nd2.x, 0.0, pd2.y, -pd2.x, 0.0].transpose(),
    ]) / 100_000.0;

    let inverse = matrix.try_inverse().unwrap() / 100_000.0;

    let crosses = l.map(|l| l.position.cross(l.direction));

    let v1 = (crosses[0] - crosses[1]).as_dvec3();
    let v2 = (crosses[0] - crosses[2]).as_dvec3();

    let v = vector![v1.x, v1.y, v1.z, v2.x, v2.y, v2.z,];

    let result = inverse * v;

    // assert!(result.iter().all(|x| (x.round() - x).abs() < 0.1), "{result:?}")

    if result.iter().any(|x| (x.round() - x).abs() > 0.0000001) {
        return None;
    }

    let p = Vec3::new(
        result[0].round() as i64,
        result[1].round() as i64,
        result[2].round() as i64,
    );

    let n = -Vec3::new(
        result[3].round() as i64,
        result[4].round() as i64,
        result[5].round() as i64,
    );

    Some((p, n))
}

fn part2(input: &str) -> i64 {
    // let mut lines_iter = input.lines().map(|line| parser::line(line).unwrap());
    let lines = input
        .lines()
        .map(|line| parser::line(line).unwrap())
        .clone()
        .collect::<Vec<_>>();

    let mut solutions = HashSet::new();
    let mut visited = HashSet::new();

    for i in 0..lines.len() {
        for j in 0..lines.len() {
            if i == j {
                continue;
            }
            for k in 0..lines.len() {
                if i == k || j == k {
                    continue;
                }

                let l = [lines[i], lines[j], lines[k]];

                let Some((p, n)) = do_the_thing(l) else {
                    continue;
                };

                if !visited.insert((p, n)) {
                    // dbg!("seen");
                    continue;
                }

                // dbg!(p, n);

                let output = p.x + p.y + p.z;

                // dbg!(output);
                // if output >= 786617556008960 || output <= 786616817811456 {
                //     continue;
                // }

                let mut valid = false;
                'outer: for line in &lines {
                    let mut lambdas = Vec::new();
                    for i in 0..3 {
                        let denom = line.direction[i] - n[i];
                        if denom == 0 {
                            if p[i] != line.position[i] {
                                break 'outer;
                            }
                        } else {
                            let lambda = (p[i] - line.position[i]) as f64 / denom as f64;

                            if lambda < 0.0 && (lambda.round() - lambda).abs() > 0.0001 {
                                break 'outer;
                            }

                            lambdas.push(lambda);
                        }
                    }

                    if lambdas.len() > 0 && !(lambdas.iter().all(|lambda| lambdas[0] == *lambda)) {
                        break 'outer;
                    }

                    valid = true;
                    dbg!(lambdas[0]);
                }

                if valid && solutions.insert((p, n)) {
                    dbg!(&solutions.len());
                };
                // return p.x + p.y + p.z
            }
        }
    }

    dbg!(visited.len());
    dbg!(visited.iter().min_by_key(|(p, _)| p.x + p.y + p.z));
    dbg!(visited.iter().max_by_key(|(p, _)| p.x + p.y + p.z));

    let solutions = solutions
        .iter()
        .map(|(p, _)| p.x + p.y + p.z)
        .collect::<Vec<_>>();

    // dbg!(&solutions);
    let mut sorted = solutions.iter().collect::<Vec<_>>();
    sorted.sort_unstable();
    dbg!(sorted);

    return solutions[0];
}

// 786617556008960 too high
// 786616817811456 too low

// fn part2_ml(input: &str) -> i64 {
//     let lines = input
//         .lines()
//         .map(|line| parser::line(line).unwrap())
//         .collect::<Vec<_>>();
//
//     let mut guess = Line {
//         origin: Vec3::ZERO,
//         direction: Vec3::X,
//     };
//
//     let loss = |guess: &Line| {
//         lines
//             .iter()
//             .map(|line| guess.distance_squared(line))
//             .sum::<f32>()
//     };
//
//     let mut queue = BinaryHeap::from([Node {
//         guess,
//         loss: loss(&guess),
//     }]);
//
//     // let mut visited = HashSet::new();
//
//     let mut min_loss = f32::INFINITY;
//
//     while let Some(node) = queue.pop() {
//         if node.loss < min_loss + 0.001 {
//             dbg!(min_loss, node.loss, guess);
//             min_loss = node.loss;
//
//             if node.loss == 0.0 {
//                 return guess.origin.x + guess.origin.y + guess.origin.z;
//             }
//         } else {
//             continue;
//         }
//
//         for dir in DIRECTIONS {
//             let delta = (node.loss * 0.1) as i64 * dir;
//
//             for neighbor in [
//                 Line {
//                     origin: node.guess.origin + delta,
//                     direction: node.guess.direction,
//                 },
//                 Line {
//                     origin: node.guess.origin,
//                     direction: node.guess.direction + delta,
//                 },
//             ] {
//                 queue.push(Node {
//                     guess: neighbor,
//                     loss: loss(&neighbor),
//                 });
//             }
//         }
//     }
//
//     panic!("no solution found");
// }
//
// #[derive(Debug, Clone, Copy)]
// struct Plane {
//     origin: Vec3,
//     normal: Vec3,
// }
//
// impl Plane {
//     fn from_lines(a: &Line, b: &Line) -> Option<Self> {
//         // Check if lines actually intersect
//         let a_d = a.direction.as_vec3();
//         let b_d = b.direction.as_vec3();
//
//         let matrix = glam::Mat3::from_cols(a_d, -b_d, glam::Vec3::Z);
//
//         // TODO: Am I missing something?
//         if matrix.determinant() == 0.0 {
//             return None;
//         }
//
//         let lambdas = matrix.inverse() * (b.origin.as_vec3() - a.origin.as_vec3());
//
//         if lambdas.z != 0.0 {
//             return None;
//         }
//
//         // Calculate plane
//         let normal = a.direction.cross(b.direction);
//         let origin = a.origin;
//
//         Some(Self { origin, normal })
//     }
// }
//
// fn _part2_smart_in_theory(input: &str) -> i64 {
//     let lines = input
//         .lines()
//         .map(|line| parser::line(line).unwrap())
//         .collect::<Vec<_>>();
//
//     let pairs_iter = lines
//         .iter()
//         .enumerate()
//         .flat_map(|(i, a)| lines[i + 1..].iter().map(move |b| (a, b)));
//
//     let plane = pairs_iter
//         .clone()
//         .find_map(|(a, b)| Plane::from_lines(&a, &b))
//         .unwrap();
//
//     dbg!(plane);
//
//     let mut caca = lines[2..].iter().filter_map(|line| {
//         let denom = line.direction.dot(plane.normal);
//
//         // Ignore lines parallel to plane
//         if denom == 0 {
//             return None;
//         }
//
//         let num = (plane.origin - line.origin).dot(plane.normal);
//
//         let result = num as f64 / denom as f64;
//
//         dbg!(result);
//         assert!(result.round() == result);
//
//         Some(result)
//     });
//
//     let t2 = caca.next().unwrap();
//     let t3 = caca.next().unwrap();
//
//     todo!()
// }
//
// fn part2_brute(input: &str) -> i64 {
//     let lines = input
//         .lines()
//         .map(|line| parser::line(line).unwrap())
//         .collect::<Vec<_>>();
//
//     for total in 0.. {
//         if total % 100 == 0 {
//             dbg!(total);
//         }
//
//         'outer: for t0 in 0..total {
//             let t0 = t0 as f64;
//             let t1 = total as f64 - t0;
//
//             let matrix = Matrix6::from_rows(&[
//                 vector![1., 0., 0., t0, 0., 0.].transpose(),
//                 vector![0., 1., 0., 0., t0, 0.].transpose(),
//                 vector![0., 0., 1., 0., 0., t0].transpose(),
//                 vector![1., 0., 0., t1, 0., 0.].transpose(),
//                 vector![0., 1., 0., 0., t1, 0.].transpose(),
//                 vector![0., 0., 1., 0., 0., t1].transpose(),
//             ]);
//
//             let Some(inverse) = matrix.try_inverse() else {
//                 continue;
//             };
//
//             let mut pairs_iter = lines
//                 .iter()
//                 .enumerate()
//                 .flat_map(|(i, a)| lines[i + 1..].iter().map(move |b| (a, b)));
//
//             if let Some(result) = pairs_iter.find_map(|(a, b)| {
//                 let v = vector![
//                     a.origin.x as f64 + a.direction.x as f64 * t0,
//                     a.origin.y as f64 + a.direction.y as f64 * t0,
//                     a.origin.z as f64 + a.direction.z as f64 * t0,
//                     b.origin.x as f64 + b.direction.x as f64 * t1,
//                     b.origin.y as f64 + b.direction.y as f64 * t1,
//                     b.origin.z as f64 + b.direction.z as f64 * t1,
//                 ];
//
//                 let result = inverse * v;
//
//                 let p = Vec3::new(result[0] as i64, result[1] as i64, result[2] as i64);
//                 let n = Vec3::new(result[3] as i64, result[4] as i64, result[5] as i64);
//
//                 // Check that trajectories collide with current path
//                 for line in &lines {
//                     let denom = line.direction - n;
//                     if denom.x == 0 || denom.y == 0 || denom.z == 0 {
//                         continue;
//                         // return None;
//                     }
//
//                     let lambda = (p - line.origin) / denom;
//
//                     if lambda.x != lambda.y || lambda.x != lambda.z {
//                         return None;
//                     }
//                 }
//
//                 dbg!(p, n);
//                 Some(p.x + p.y + p.z)
//             }) {
//                 return result;
//             }
//         }
//     }
//
//     panic!("no solution found");
// }

#[cfg(test)]
mod tests {
    use super::*;
    const TEST_INPUT: &str = christmas_tree::indoc! {
        "
            19, 13, 30 @ -2,  1, -2
            18, 19, 22 @ -1, -1, -2
            20, 25, 34 @ -2, -2, -4
            12, 31, 28 @ -1, -2, -1
            20, 19, 15 @  1, -5, -3
        "
    };

    #[test]
    fn part1() {
        assert_eq!(solve1(TEST_INPUT, 7, 27), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(TEST_INPUT), 47);
    }
}
