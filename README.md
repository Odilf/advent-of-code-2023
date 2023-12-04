# Advent of Code 2023

These are my advent of code solutions for 2023, using Rust (and Neovim).

It is surprisingly challenging to figure out what's the best architecture for AOC in Rust. 
I tried once to just have every day as a module, but that was annoying and full of boilerplate,
that I couldn't easily get rid of. Also, it's sort of wrong because each day is a different thing
with no relation to the previous one.

So, what I'm actually doing is each day as a crate. All these crates are part of a workspace. To this
end, I've also created a small framework called `christmas_tree` that handles most of the boilerplate.
Of course, it fetches data using a session token in `.env` and caches it. 

I think this is as minimal as it gets:

```rust
christmas_tree::day!(5);

fn part1(input: &str) -> i32 {
    todo!()
}

fn part2(input: &str) -> i32 {
    todo!()
}

christmas_tree::examples! {
    r"
        This is an example
    " => 8,

    r"
        This is an example for day 2
        you can indent in because it uses `indoc!` under the hood
    " => 13,
}
```

Also you can run `christmas_tree` as a binary to get a cli interface that can create the day crates. 
If you don't specify a day for the argument then it creates the one for the day you're on (if it's the
advent calendar season). 

Run `christmas_tree --help` to get more info. 
