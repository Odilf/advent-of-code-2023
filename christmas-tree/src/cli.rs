use std::{fs::File, process::Stdio};

use chrono::{Datelike, Local};
use clap::Parser;

#[derive(Debug, Parser)]
pub struct Options {
    #[clap(short, long)]
    day: Option<u8>,

    #[clap(short, long, default_value_t = true)]
    peg: bool,

    #[clap(short, long, default_value_t = false)]
    glam: bool,

    #[clap(long, value_delimiter = ',', default_value = "")]
    deps: Vec<String>,
}

pub fn run(options: Options) -> std::io::Result<()> {
    let day = options.day.unwrap_or_else(|| {
        println!("No day specified, using today's date");
        let now = Local::now();

        let day = now
            .signed_duration_since(now.with_month(12).unwrap())
            .num_days();
        day as u8
    });

    if !(0..=25).contains(&day) {
        panic!("Current day is not in the advent calendar range");
    }

    let crate_name = format!("day{day}");

    std::process::Command::new("cargo")
        .arg("init")
        .arg(&crate_name)
        .spawn()?
        .wait()?;

    std::process::Command::new("cargo")
        .arg("add")
        .arg("--path")
        .arg("christmas-tree/")
        .arg("--package")
        .arg(&crate_name)
        .spawn()?
        .wait()?;

    let mut deps: Vec<&str> = options
        .deps
        .iter()
        .filter(|d| !d.is_empty())
        .map(|s| &**s)
        .collect();

    if options.peg {
        deps.push("peg")
    }

    if options.glam {
        deps.push("glam")
    }

    std::process::Command::new("cargo")
        .arg("add")
        .args(deps)
        .arg("--package")
        .arg(&crate_name)
        .spawn()?
        .wait()?;

    let file = File::create(format!("{}/src/main.rs", crate_name)).unwrap();
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
