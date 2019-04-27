#[macro_use]
extern crate log;

use crate::config::ArgsConfig;

mod config;
mod logging;
mod tester;

use colored::Colorize;
use structopt::StructOpt;

fn main() {
    let config = ArgsConfig::from_args();
    title();

    logging::setup_logging(&config.logging_config);
    trace!("{:?}", config);
}

fn title() {
    println!(
        "                 {}",
        r"  __ _           _     _      ".cyan()
    );
    println!(
        "                 {}",
        r" / _(_)_ __  ___| |__ (_)_ __ ".cyan()
    );
    println!(
        "                 {}",
        r"| |_| | '_ \/ __| '_ \| | '__|".cyan()
    );
    println!(
        "                 {}",
        r"|  _| | | | \__ \ | | | | |   ".cyan()
    );
    println!(
        "                 {}",
        r"|_| |_|_| |_|___/_| |_|_|_|   ".cyan()
    );
    println!(
        "                         {}",
        format!("version {}", structopt::clap::crate_version!())
            .red()
            .bold()
    );
    println!(
        "{}\n",
        "Wanna try the fastest Low & Slow traffic generator in existence?"
            .green()
            .underline()
    );
}
