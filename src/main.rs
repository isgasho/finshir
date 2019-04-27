use crate::config::ArgsConfig;

mod config;
mod tester;

use structopt::StructOpt;

fn main() {
    let config = ArgsConfig::from_args();
}
