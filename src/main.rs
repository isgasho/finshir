use crate::config::ArgsConfig;

mod config;

use structopt::StructOpt;

fn main() {
    let config = ArgsConfig::from_args();
}
