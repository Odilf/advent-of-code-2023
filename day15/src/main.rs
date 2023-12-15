christmas_tree::day!(15);

fn hash(input: &str) -> i64 {
    let mut current = 0;
    for byte in input.bytes() {
        current += byte as i64;
        current *= 17;
        current %= 256;
    }

    current
}

fn part1(input: &str) -> i64 {
    input.trim().split(',').map(|s| hash(s)).sum()
}

fn part2<'a>(input: &'a str) -> i64 {
    let mut lens = vec![Vec::new(); 256];
    let mut instructions = input.trim().split(',');
    for s in instructions {
        let equals = |lens: &mut Vec<Vec<(&'a str, i64)>>| {
            let mut iter = s.split('=');
            let chars = iter.next().unwrap();
            let hash = hash(chars);

            let vx = &mut lens[hash as usize];

            let value = iter.next()?.parse::<i64>().unwrap();

            if let Some((_, v)) = vx.iter_mut().find(|(s, _)| s.contains(chars)) {
                *v = value;
            } else {
                lens[hash as usize].push((chars, value));
            }

            Some(())
        };

        let dash = |lens: &mut Vec<Vec<(&'a str, i64)>>| {
            let mut iter = s.split('-');
            let chars = iter.next().unwrap();
            let hash = hash(chars);

            let bx = &mut lens[hash as usize];
            bx.retain(|(s, _)| !s.contains(chars));

            Some(())
        };

        equals(&mut lens).or_else(|| dash(&mut lens)).unwrap();
    };

    let mut count = 0;
    for (i, lens) in lens.iter().enumerate() {
        let i = i as i64 + 1;
        for (j, (s, val)) in lens.iter().enumerate() {
            count += i * *val * (j as i64 + 1);
        }
    }

    count
}

christmas_tree::examples! {
    "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7" => 1320, 145,
}
