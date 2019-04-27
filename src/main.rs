// MIT License
//
// Copyright (c) 2019 Temirkhan Myrzamadi
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

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
