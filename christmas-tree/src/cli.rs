use std::{fs::File, process::Stdio};

use chrono::{Datelike, Local};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    /// Day of the month. If it's blank, it will automatically use the day from the system clock.
    #[clap(short, long, value_parser = clap::value_parser!(u8).range(1..=25))]
    day: Option<u8>,

    /// Whether to use `peg` for parsing. Defaults to `true`
    #[clap(short, long, default_value_t = true)]
    peg: bool,

    /// Whether to use `glam` for vectors. Defaults to `false`
    #[clap(short, long, default_value_t = false)]
    glam: bool,

    /// Comma separated list of additional dependencies.
    #[clap(long, value_delimiter = ',', default_value = "")]
    deps: Vec<String>,
}

fn get_todays_day() -> u8 {
    let now = Local::now();

    let dec_1st = now.with_month(12).unwrap().with_day(1).unwrap();
    let day = now.signed_duration_since(dec_1st);
    day.num_days() as u8 + 1
}

pub fn run(options: &Options) -> std::io::Result<()> {
    let day = options.day.unwrap_or_else(|| {
        println!("No day specified, using today's date");
        get_todays_day()
    });

    assert!((0..=25).contains(&day), "Day must be between 1 and 25");

    let crate_name = format!("day{day:0>2}");

    std::process::Command::new("cargo")
        .args(["init", &crate_name])
        .spawn()?
        .wait()?;

    std::process::Command::new("cargo")
        .args(["add", "--path", "christmas-tree/"])
        .args(["--package", &crate_name])
        .spawn()?
        .wait()?;

    let mut deps: Vec<&str> = options
        .deps
        .iter()
        .filter(|d| !d.is_empty())
        .map(|s| &**s)
        .collect();

    if options.peg {
        deps.push("peg");
    }

    if options.glam {
        deps.push("glam");
    }

    std::process::Command::new("cargo")
        .arg("add")
        .args(deps)
        .args(["--package", &crate_name])
        .spawn()?
        .wait()?;

    let file = File::create(format!("{crate_name}/src/main.rs")).unwrap();
    let stdio = Stdio::from(file);

    std::process::Command::new("echo")
        .arg(get_boilerplate(day))
        .stdout(stdio)
        .spawn()?
        .wait()?;

    Ok(())
}

fn get_boilerplate(day: u8) -> String {
    format!(include_str!("example_program.rs"), day = day)
}
