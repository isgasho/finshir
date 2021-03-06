// finshir: A coroutines-driven Low & Slow traffic sender, written in Rust
// Copyright (C) 2019  Temirkhan Myrzamadi <gymmasssorla@gmail.com>
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.
//
// You should have received a copy of the GNU General Public License
// along with this program.  If not, see <https://www.gnu.org/licenses/>.
//
// For more information see <https://github.com/Gymmasssorla/finshir>.

#![feature(result_map_or_else)]

#[macro_use]
extern crate log;

use colored::Colorize;
use structopt::StructOpt;

use crate::config::ArgsConfig;

mod config;
mod logging;
mod testing;

fn main() {
    setup_ctrlc_handler();

    let config = ArgsConfig::from_args();
    title();

    logging::setup_logging(&config.logging_config);
    trace!("{:?}", config);

    std::process::exit(testing::run(&config));
}

fn setup_ctrlc_handler() {
    ctrlc::set_handler(move || {
        info!("Cancellation from the user has been received. Exiting the program...");
        std::process::exit(0);
    })
    .expect("Error while setting the Ctrl-C handler");

    trace!("The Ctrl-C handler has been configured.");
}

fn title() {
    println!(
        "                {}",
        r"  __ _           _     _      ".cyan()
    );
    println!(
        "                {}",
        r" / _(_)_ __  ___| |__ (_)_ __ ".cyan()
    );
    println!(
        "                {}",
        r"| |_| | '_ \/ __| '_ \| | '__|".cyan()
    );
    println!(
        "                {}",
        r"|  _| | | | \__ \ | | | | |   ".cyan()
    );
    println!(
        "                {}",
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
        "A coroutines-driven Low & Slow traffic sender, written in Rust"
            .green()
            .underline()
    );
}
