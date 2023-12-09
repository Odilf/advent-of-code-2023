use std::collections::HashMap;

christmas_tree::day!(8);

enum Instruction {
    Left,
    Right,
}

impl From<char> for Instruction {
    fn from(value: char) -> Self {
        match value {
            'L' => Self::Left,
            'R' => Self::Right,
            other => panic!("Invalid instruction {other}"),
        }
    }
}

peg::parser! {
    grammar parser() for str {
        pub rule instructions() -> Vec<Instruction>
            = i:$(['L' | 'R']+) { i.chars().map(Instruction::from).collect() }

        rule node() -> &'input str
            = n:$(['0'..='9' | 'A'..='Z']+) { n }

        rule _  = [' ' | '\n']*

        pub rule edge() -> (&'input str, (&'input str, &'input str))
            = a:node() _ "=" _ "(" _ b:node() _ ", " _ c:node() _ ")" { (a, (b, c)) }

        pub rule edges() -> HashMap<&'input str, (&'input str, &'input str)>
            = e:edge() ++ _ { e.into_iter().collect() }

        pub rule whole() -> (Vec<Instruction>, HashMap<&'input str, (&'input str, &'input str)>)
            = i:instructions() _ e:edges() _ { (i, e) }
    }
}

fn part1(input: &str) -> i64 {
    let (instructions, edges) = parser::whole(input).unwrap();

    let mut current_node = "AAA";
    for i in 0.. {
        let instruction = &instructions[i % instructions.len()];

        let (left, right) = edges.get(current_node).unwrap();
        current_node = match instruction {
            Instruction::Left => left,
            Instruction::Right => right,
        };

        if current_node == "ZZZ" {
            return i as i64 + 1;
        }
    }

    unreachable!()
}

fn part2(input: &str) -> i64 {
    let (instructions, edges) = parser::whole(input).unwrap();

    let ends_with = |node: &str, char: u8| node.as_bytes()[2] == char;

    let starts = edges.keys().filter(|node| ends_with(node, b'A'));

    starts
        .map(|start| {
            let mut current_node = start;
            for i in 0.. {
                if ends_with(current_node, b'Z') {
                    return i as i64;
                }

                let instruction = &instructions[i % instructions.len()];

                let (left, right) = edges.get(current_node).unwrap();
                current_node = match instruction {
                    Instruction::Left => left,
                    Instruction::Right => right,
                };
            }

            unreachable!()
        })
        .fold(1, num::integer::lcm)
}

christmas_tree::examples! {
    r"
        LLR

        AAA = (BBB, BBB)
        BBB = (AAA, ZZZ)
        ZZZ = (ZZZ, ZZZ)
    " => 6,

    r"
        LR

        11A = (11B, XXX)
        11B = (XXX, 11Z)
        11Z = (11B, XXX)
        22A = (22B, XXX)
        22B = (22C, 22C)
        22C = (22Z, 22Z)
        22Z = (22B, 22B)
        XXX = (XXX, XXX)
    " => 6,
}
