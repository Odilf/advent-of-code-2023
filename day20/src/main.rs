use std::collections::{HashMap, VecDeque};

christmas_tree::day!(20);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord)]
enum Pulse {
    Low,
    High,
}
impl Pulse {
    fn flip(&self, on: bool) -> Pulse {
        match on {
            true => *self,
            false => match self {
                Pulse::Low => Pulse::High,
                Pulse::High => Pulse::Low,
            },
        }
    }
}

impl From<bool> for Pulse {
    fn from(b: bool) -> Self {
        if b {
            Pulse::High
        } else {
            Pulse::Low
        }
    }
}

#[derive(Debug, Clone)]
enum ModuleType<'a> {
    FlipFlop(bool),
    Conjuction(HashMap<&'a str, Pulse>),
    Passthrough,
}

#[derive(Debug, Clone)]
struct Module<'a> {
    from: &'a str,
    to: Vec<&'a str>,
    typ: ModuleType<'a>,
}

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        rule module_type() -> ModuleType<'input>
            = "%" { ModuleType::FlipFlop(false) }
            / "&" { ModuleType::Conjuction(HashMap::new()) }
            / "" { ModuleType::Passthrough }

        rule name() -> &'input str
            = n:$(['a'..='z']+) { n }

        pub rule module() -> (&'input str, Module<'input>)
            = typ:module_type() from:name()
              " -> " to:name() ** ", "
              { (from, Module { from, to, typ }) }
    }
}

fn parse(
    input: &str,
) -> (
    HashMap<&str, Module<'_>>,
    HashMap<&str, Vec<&str>>,
    Vec<&str>,
) {
    let mut map = input
        .lines()
        .map(|line| parser::module(line).unwrap())
        .collect::<HashMap<_, _>>();

    let nodes = map.keys().copied().collect::<Vec<_>>();

    let mut inverse = HashMap::new();

    for node in &nodes {
        let module = map.get(node).unwrap();

        for to in &module.to {
            inverse.entry(*to).or_insert_with(Vec::new).push(*node);
        }
    }

    for node in &nodes {
        let module = map.get_mut(node).unwrap();

        if let ModuleType::Conjuction(memory) = &mut module.typ {
            for parent in &inverse[module.from] {
                memory.insert(parent, Pulse::Low);
            }
        }
    }
    // let mut map = input
    //     .lines()
    //     .map(|line| parser::module(line).unwrap())
    //     .collect::<HashMap<_, _>>();
    //
    // // assert!(map.iter().all(|(name, module)| module.from == *name));
    //
    // let clone = map.clone();
    //
    // for (to, module) in &mut map {
    //     match &mut module.typ {
    //         ModuleType::Conjuction(memory) => {
    //             for (from, module) in &clone {
    //                 for to_inner in &module.to {
    //                     if *to_inner == module.from {
    //                         memory.insert(from, Pulse::Low);
    //                     }
    //                 }
    //             }
    //         }
    //         _ => (),
    //     }
    // }

    (map, inverse, nodes)
}

fn part1(input: &str) -> i64 {
    let (mut map, _inv, _nodes) = parse(input);

    let mut count = [0, 0];

    for _ in 0..1000 {
        let mut queue = VecDeque::from([("", "broadcaster", Pulse::Low)]);

        while let Some((from, name, pulse)) = queue.pop_front() {
            count[pulse as usize] += 1;

            let Some(module) = map.get_mut(name) else {
                continue;
            };

            match &mut module.typ {
                ModuleType::FlipFlop(on) => match pulse {
                    Pulse::Low => {
                        *on = !*on;
                        for to in &module.to {
                            queue.push_back((name, to, Pulse::from(*on)));
                        }
                    }
                    Pulse::High => (),
                },
                ModuleType::Conjuction(memory) => {
                    memory.insert(from, pulse);

                    let pulse = if memory.values().all(|pulse| *pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    for to in &module.to {
                        queue.push_back((name, to, pulse));
                    }
                }
                ModuleType::Passthrough => {
                    for to in &module.to {
                        queue.push_back((name, to, pulse));
                    }
                }
            }
        }
    }

    count.iter().product()
}

fn part2(input: &str) -> i64 {
    let (mut map, _inv, _nodes) = parse(input);

    for i in 1.. {
        let mut queue = VecDeque::from([("", "broadcaster", Pulse::Low)]);

        while let Some((from, name, pulse)) = queue.pop_front() {
            let Some(module) = map.get_mut(name) else {
                continue;
            };

            if name == "rx" && pulse == Pulse::Low {
                return i;
            }

            match &mut module.typ {
                ModuleType::FlipFlop(on) => match pulse {
                    Pulse::Low => {
                        *on = !*on;
                        for to in &module.to {
                            queue.push_back((name, to, Pulse::from(*on)));
                        }
                    }
                    Pulse::High => (),
                },
                ModuleType::Conjuction(memory) => {
                    memory.insert(from, pulse);

                    let pulse = if memory.values().all(|pulse| *pulse == Pulse::High) {
                        Pulse::Low
                    } else {
                        Pulse::High
                    };

                    for to in &module.to {
                        queue.push_back((name, to, pulse));
                    }
                }
                ModuleType::Passthrough => {
                    for to in &module.to {
                        queue.push_back((name, to, pulse));
                    }
                }
            }
        }
    }

    panic!("Didn't find rx")
}

fn part2_actual(input: &str) -> i64 {
    let mut map = input
        .lines()
        .map(|line| parser::module(line).unwrap())
        .collect::<HashMap<_, _>>();

    let nodes = map.keys().copied().collect::<Vec<_>>();

    let mut inverse = HashMap::new();

    for node in &nodes {
        let module = map.get(node).unwrap();

        for to in &module.to {
            inverse.entry(*to).or_insert_with(Vec::new).push(*node);
        }
    }

    for node in &nodes {
        let module = map.get_mut(node).unwrap();

        let dbg = *node == "xm";

        if dbg {
            dbg!(&module);
        }

        if let ModuleType::Conjuction(memory) = &mut module.typ {
            for parent in &inverse[module.from] {
                memory.insert(parent, Pulse::Low);
            }
        }
    }

    let mut queue = VecDeque::from([("broadcaster", vec![Some(Pulse::Low)])]);
    // let mut patterns = HashMap::from([("broadcaster", vec![Some(Pulse::Low)])]);

    while let Some((node, pattern)) = queue.pop_front() {
        let module = &map[node];

        match &module.typ {
            ModuleType::FlipFlop(_) => {
                let node_patterns = pattern
                    .iter()
                    .enumerate()
                    .map(|(i, pulse)| match pulse {
                        None | Some(Pulse::High) => None,
                        Some(Pulse::Low) => {
                            if i % 2 == 0 {
                                Some(Pulse::High)
                            } else {
                                Some(Pulse::Low)
                            }
                        }
                    })
                    .collect::<Vec<_>>();

                for to in &module.to {
                    queue.push_back((to, node_patterns.clone()));
                }
            }
            ModuleType::Conjuction(memory) => {
                let mut node_patterns = Vec::new();

                for (i, pulse) in pattern.iter().enumerate() {
                    match pulse {
                        None => (),
                        Some(Pulse::Low) => {
                            todo!()
                        }
                        _ => todo!(),
                    }
                }

                for to in &module.to {
                    queue.push_back((*to, node_patterns.clone()));
                }
            }
            ModuleType::Passthrough => {
                for to in &module.to {
                    queue.push_back((*to, pattern.clone()));
                }
            }
        }
    }

    todo!()

    // let mut distances = HashMap::from([(("broadcaster", Pulse::Low), 0i64)]);
    // let mut queue = VecDeque::new();
    //
    // for to in &map["broadcaster"].to {
    //     queue.push_front((*to, Pulse::Low));
    // }
    //
    // while let Some((name, pulse)) = queue.pop_back() {
    //     let module = &map[name];
    //
    //     let parents = &inverse[name];
    //
    //     dbg!(&module, &parents);
    //
    //     match &module.typ {
    //         ModuleType::FlipFlop(_) => {
    //             if pulse == Pulse::High {
    //                 continue;
    //             }
    //
    //             let first_parent_low = parents
    //                 .iter()
    //                 .map(|parent| distances.get(&(*parent, Pulse::Low)).copied())
    //                 .flatten()
    //                 .min()
    //                 .unwrap();
    //
    //             for to in &module.to {
    //                 distances.insert((to, Pulse::High), first_parent_low);
    //                 distances.insert((to, Pulse::High), first_parent_low + 1);
    //
    //                 queue.push_front((to, Pulse::High));
    //                 queue.push_front((to, Pulse::Low));
    //             }
    //         }
    //         ModuleType::Conjuction(memory) => {
    //             dbg!(&distances);
    //             let first_parent_high = parents
    //                 .iter()
    //                 .map(|parent| distances.get(&(*parent, Pulse::High)).copied())
    //                 .flatten()
    //                 .min()
    //                 .unwrap();
    //
    //             let last_parent_high = parents
    //                 .iter()
    //                 .map(|parent| distances.get(&(*parent, Pulse::High)).copied())
    //                 .flatten()
    //                 .max()
    //                 .unwrap();
    //
    //             for to in &module.to {
    //                 distances.insert((to, Pulse::Low), first_parent_high);
    //                 distances.insert((to, Pulse::High), last_parent_high);
    //
    //                 queue.push_front((to, Pulse::Low));
    //                 queue.push_front((to, Pulse::High));
    //             }
    //         }
    //         ModuleType::Passthrough => {
    //             for to in &map[name].to {
    //                 distances.insert((to, Pulse::Low), distances[&(name, pulse)]);
    //                 distances.insert((to, Pulse::High), distances[&(name, pulse)]);
    //
    //                 queue.push_front((to, pulse));
    //                 queue.push_front((to, pulse));
    //             }
    //         }
    //     }
    // }
    //
    // distances[&("rx", Pulse::Low)]

    // count_cycles("rx", Pulse::Low, &map, &inverse, &mut HashMap::new()).unwrap()
}

// fn count_cycles<'a>(
//     name: &'a str,
//     pulse: Pulse,
//     map: &'a HashMap<&str, Module<'_>>,
//     inverse: &'a HashMap<&str, Vec<&str>>,
//     cache: &mut HashMap<(&'a str, Pulse), Result<i64, bool>>,
// ) -> Result<i64, bool> {
//     let key = (name, pulse);
//     if let Some(cached @ (Ok(_) | Err(true))) = cache.get(&key) {
//         return *cached;
//     }
//
//     if name == "broadcaster" {
//         return match pulse {
//             Pulse::Low => Ok(1),
//             Pulse::High => Err(false),
//         };
//     }
//
//     cache.insert((name, pulse), Err(false));
//
//     dbg!(name, pulse);
//     let parents = &inverse[name];
//
//     let result = parents
//         .iter()
//         .map(|parent| {
//             let module = map.get(parent).unwrap();
//
//             // match &module.typ {
//             //     ModuleType::FlipFlop(on) => {
//             //         count_cycles(parent, Pulse::Low, map, inverse, cache)
//             //     }
//             //     ModuleType::Conjuction(memory) => match pulse {
//             //         Pulse::Low => Some(
//             //             memory
//             //                 .keys()
//             //                 .map(|module| count_cycles(module, Pulse::High, map, inverse, cache))
//             //                 .collect::<Option<Vec<_>>>()?
//             //                 .iter()
//             //                 .fold(1, |a, b| lcm(a, *b)),
//             //         ),
//             //         Pulse::High => memory
//             //             .keys()
//             //             .filter_map(|module| count_cycles(module, Pulse::Low, map, inverse, cache))
//             //             .min(),
//             //     },
//             //     ModuleType::Passthrough => count_cycles(parent, pulse, map, inverse, cache),
//             // }
//         })
//         .min()?;
//
//     if result.is_none() {
//         cache.remove(&key).unwrap();
//     } else {
//         cache.insert(key, result);
//     }
//
//     result
// }

christmas_tree::examples! {
    part1 {
        one: "
            broadcaster -> a, b, c
            %a -> b
            %b -> c
            %c -> inv
            &inv -> a
        " => 32_000_000,

        two: "
            broadcaster -> a
            %a -> inv, con
            &inv -> b
            %b -> con
            &con -> output
        " => 11_687_500,
    }
}
