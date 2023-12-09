use clap::Parser;

mod cli;

pub fn main() {
    let options = cli::Options::parse();

    cli::run(&options).unwrap();
}
