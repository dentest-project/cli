mod client;
mod config;
mod entities;
mod error;
mod pull;
mod utils;

use clap::Clap;
use dotenv::dotenv;
use pull::pull;
use termion::{color, style};

#[derive(Clap)]
#[clap(version = "1.0.0", author = "Pierre Rolland <pierre@rolland.dev>")]
struct Opts {
    #[clap(short, long, default_value = ".dentest.yml")]
    config: String,
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Clap)]
enum SubCommand {
    #[clap(version = "1.0.0", author = "Pierre Rolland <pierre@rolland.dev>")]
    Pull(Pull),
}

#[derive(Clap)]
struct Pull {}

fn main() {
    let opts: Opts = Opts::parse();

    dotenv().ok();

    let result = match opts.subcmd {
        SubCommand::Pull(_p) => pull(opts.config),
    };

    match result {
        Ok(_) => println!(
            "{}{}Finished{}",
            style::Bold,
            color::Fg(color::LightGreen),
            style::Reset
        ),
        Err(e) => e.print_message(),
    };
}
