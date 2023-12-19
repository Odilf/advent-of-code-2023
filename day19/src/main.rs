use std::{
    collections::HashMap,
    ops::{Index, IndexMut},
};

christmas_tree::day!(19);

#[derive(Debug, Clone)]
struct Part<T: Copy = i64> {
    extremely_cool_looking: T,
    musical: T,
    aerodinamic: T,
    shiny: T,
}
impl Part<i64> {
    fn value(&self) -> i64 {
        self.extremely_cool_looking + self.musical + self.aerodinamic + self.shiny
    }
}

impl<T: Copy> Index<&str> for Part<T> {
    type Output = T;

    fn index(&self, index: &str) -> &Self::Output {
        match index {
            "x" => &self.extremely_cool_looking,
            "m" => &self.musical,
            "a" => &self.aerodinamic,
            "s" => &self.shiny,
            _ => panic!("Invalid index"),
        }
    }
}

impl<T: Copy> IndexMut<&str> for Part<T> {
    fn index_mut(&mut self, index: &str) -> &mut Self::Output {
        match index {
            "x" => &mut self.extremely_cool_looking,
            "m" => &mut self.musical,
            "a" => &mut self.aerodinamic,
            "s" => &mut self.shiny,
            _ => panic!("Invalid index"),
        }
    }
}

impl Default for Range {
    fn default() -> Self {
        Self {
            start: 1,
            end: 4000,
        }
    }
}

impl Default for Part<Range> {
    fn default() -> Self {
        Self {
            extremely_cool_looking: Range::default(),
            musical: Range::default(),
            aerodinamic: Range::default(),
            shiny: Range::default(),
        }
    }
}

struct Rule<'a> {
    subpart: &'a str,
    greater_than: bool,
    y: i64,
}

fn passes(part: &Part, rule: Option<&Rule>) -> bool {
    let Some(rule) = rule else {
        return true;
    };

    let x = part[rule.subpart];
    let y = rule.y;

    if rule.greater_than {
        x > y
    } else {
        x < y
    }
}

enum Action<'a> {
    GoTo(&'a str),
    Reject,
    Accept,
}

struct Workflow<'a> {
    rules: Vec<(Option<Rule<'a>>, Action<'a>)>,
}

peg::parser! {
    grammar parser() for str {
        rule number() -> i64
            = n:$(['0'..='9']+) { n.parse().unwrap() }

        pub rule part() -> Part
            = "{"
                "x" "=" x:number() ","
                "m" "=" m:number() ","
                "a" "=" a:number() ","
                "s" "=" s:number()
            "}" {
                Part {
                    extremely_cool_looking: x,
                    musical: m,
                    aerodinamic: a,
                    shiny: s,
                }
            }

        rule name() -> &'input str
            = n:$(['a'..='z']+) { n }

        rule action() -> Action<'input>
            = "R" { Action::Reject }
            / "A" { Action::Accept }
            / n:name() { Action::GoTo(n) }

        rule single_rule() -> Rule<'input> =
            subpart:name() ">" y:number() { Rule { subpart, y, greater_than: true } }
            / subpart:name() "<" y:number() { Rule { subpart, y, greater_than: false } }

        rule workflow()  -> Workflow<'input>
            = rules:(r:(r:single_rule() ":" { r })? a:action() { (r, a) }) ** "," { Workflow { rules }}

        pub rule entry() -> (&'input str, Workflow<'input>)
            = p:name() "{" w:workflow() "}" { (p, w) }
    }
}

fn part1(input: &str) -> i64 {
    let [workflows, parts] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        panic!("invalid input");
    };

    let workflows = workflows
        .lines()
        .map(|line| parser::entry(line).unwrap())
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|line| parser::part(line).unwrap())
        .collect::<Vec<_>>();

    let mut accepted = Vec::new();

    for part in &parts {
        let mut current = "in";
        'outer: loop {
            let workflow = workflows.get(current).unwrap();

            for (rule, action) in &workflow.rules {
                if passes(part, rule.as_ref()) {
                    match action {
                        Action::GoTo(name) => {
                            current = name;
                            break;
                        }

                        Action::Reject => (),
                        Action::Accept => accepted.push(part),
                    }

                    break 'outer;
                }
            }
        }
    }

    accepted.iter().map(|part| part.value()).sum()
}

impl Part<Range> {
    fn value(&self) -> i64 {
        self.extremely_cool_looking.size()
            * self.musical.size()
            * self.aerodinamic.size()
            * self.shiny.size()
    }
}

fn part2(input: &str) -> i64 {
    let [workflows, _] = input.split("\n\n").collect::<Vec<_>>()[..] else {
        panic!("invalid input");
    };

    let workflows = workflows
        .lines()
        .map(|line| parser::entry(line).unwrap())
        .collect::<HashMap<_, _>>();

    let mut queue = Vec::new();

    queue.push(("in", Part::<Range>::default()));

    let mut accepted = Vec::new();

    while let Some((current, mut ranges)) = queue.pop() {
        let workflow = workflows.get(current).unwrap();

        for (rule, action) in &workflow.rules {
            let (refined, unrefined) = ranges.refine(rule);

            if let Some(unrefined) = unrefined {
                ranges = unrefined;
            };

            let Some(refined) = refined else {
                continue;
            };

            match action {
                Action::GoTo(name) => queue.push((name, refined)),
                Action::Reject => continue,
                Action::Accept => accepted.push(refined),
            }
        }
    }

    accepted.iter().map(|part| part.value()).sum()
}

#[derive(Debug, Clone, Copy)]
struct Range {
    start: i64,
    end: i64,
}

impl Range {
    fn size(&self) -> i64 {
        self.end - self.start + 1
    }
}

impl Part<Range> {
    fn refine(&self, rule: &Option<Rule>) -> (Option<Part<Range>>, Option<Part<Range>>) {
        let mut refined = self.clone();

        let Some(rule) = rule else {
            return (Some(refined), None);
        };

        let mut derefined = self.clone();

        let subpart = rule.subpart;
        let mut y = rule.y;

        let (low, high) = if rule.greater_than {
            y += 1;
            (&mut derefined, &mut refined)
        } else {
            (&mut refined, &mut derefined)
        };

        low[subpart].end = low[subpart].end.min(y - 1);
        high[subpart].start = high[subpart].start.max(y);

        if refined[subpart].start >= refined[subpart].end {
            (None, Some(derefined))
        } else {
            (Some(refined), Some(derefined))
        }
    }
}

christmas_tree::examples! {
    "
        px{a<2006:qkq,m>2090:A,rfg}
        pv{a>1716:R,A}
        lnx{m>1548:A,A}
        rfg{s<537:gd,x>2440:R,A}
        qs{s>3448:A,lnx}
        qkq{x<1416:A,crn}
        crn{x>2662:A,R}
        in{s<1351:px,qqz}
        qqz{s>2770:qs,m<1801:hdj,R}
        gd{a>3333:R,R}
        hdj{m>838:A,pv}

        {x=787,m=2655,a=1222,s=2876}
        {x=1679,m=44,a=2067,s=496}
        {x=2036,m=264,a=79,s=2244}
        {x=2461,m=1339,a=466,s=291}
        {x=2127,m=1623,a=2188,s=1013}
    " => 19114, 167_409_079_868_000,
}
