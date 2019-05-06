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

use std::io::{self, Write};
use std::num::NonZeroUsize;
use std::os::unix::io::{FromRawFd, IntoRawFd};

use humantime::format_duration;
use may::{self, coroutine, go};
use tor_stream::TorStream;

use crate::config::{ArgsConfig, SocketConfig, TesterConfig};
use crate::helpers;
use std::time::Instant;

type StdSocket = std::net::TcpStream;
type MaySocket = may::net::TcpStream;

pub fn run(config: &ArgsConfig) -> i32 {
    let portions = match helpers::read_portions(&config.portions_file) {
        Err(err) => {
            error!("Failed to parse the JSON >>> {}!", err);
            return 1;
        }
        Ok(res) => res,
    };
    let portions: Vec<&[u8]> = portions.iter().map(Vec::as_slice).collect();

    warn!(
        "Waiting {} and then spawning {} coroutines connected through the {}.",
        helpers::cyan(format_duration(config.wait)),
        helpers::cyan(config.connections),
        if config.tester_config.socket_config.use_tor {
            "Tor network"
        } else {
            "regular Web"
        }
    );
    std::thread::sleep(config.wait);

    coroutine::scope(|scope| {
        let portions = &portions;
        let config = &config;
        let iters = config.connections.get();

        for _ in 0..iters {
            go!(scope, move || run_tester(&config.tester_config, portions));
        }

        info!("All the coroutines have been spawned.");
    });

    return 0;
}

fn run_tester(config: &TesterConfig, portions: &[&[u8]]) {
    let fmt_per = helpers::cyan(format_duration(config.write_periodicity));
    let start = Instant::now();

    loop {
        let mut socket: MaySocket = connect_socket(&config.socket_config);

        for &portion in portions {
            if start.elapsed() >= config.test_duration {
                info!("The allotted time has passed. The coroutine has exited.");
                return;
            }

            match send_portion(&mut socket, portion, config.failed_count) {
                SendPortionResult::Success => {
                    info!(
                        "{} bytes have been sent. Waiting {}...",
                        helpers::cyan(portion.len()),
                        fmt_per
                    );
                }
                SendPortionResult::Failed(err) => {
                    error!(
                        "Sending {} bytes failed {} times >>> {}! Reconnecting the socket...",
                        helpers::cyan(portion.len()),
                        helpers::cyan(config.failed_count),
                        err,
                    );
                    break;
                }
            }

            coroutine::sleep(config.write_periodicity);
        }

        info!("All the data portions have been sent. Reconnecting the socket...");
    }
}

#[derive(Debug)]
enum SendPortionResult {
    Success,
    Failed(io::Error),
}

fn send_portion(
    socket: &mut MaySocket,
    portion: &[u8],
    failed_count: NonZeroUsize,
) -> SendPortionResult {
    let res = {
        for _ in 0..(failed_count.get() - 1) {
            match socket.write_all(portion) {
                Ok(_) => return SendPortionResult::Success,
                Err(err) => {
                    error!(
                        "Failed to send {} bytes >>> {}! Retrying the operation...",
                        helpers::cyan(portion.len()),
                        err
                    );
                    continue;
                }
            }
        }

        match socket.write_all(portion) {
            Ok(_) => SendPortionResult::Success,
            Err(err) => SendPortionResult::Failed(err),
        }
    };

    socket
        .flush()
        .map_or_else(SendPortionResult::Failed, |_| res)
}

fn connect_socket(config: &SocketConfig) -> MaySocket {
    loop {
        match try_connect_socket(config) {
            Ok(socket) => {
                info!("A new socket has been connected.");
                return socket;
            }
            Err(err) => {
                error!(
                    "Failed to connect a socket >>> {}! Retrying the operation...",
                    err
                );
                continue;
            }
        }
    }
}

fn try_connect_socket(config: &SocketConfig) -> io::Result<MaySocket> {
    let socket = if config.use_tor {
        TorStream::connect(config.receiver)?.unwrap()
    } else {
        StdSocket::connect_timeout(&config.receiver, config.connect_timeout)?
    };

    // We send packets quite rarely (the default is 30secs), so the Nagle algorithm
    // doesn't help us
    socket
        .set_nodelay(true)
        .expect("Cannot disable TCP_NODELAY");

    socket.set_write_timeout(Some(config.write_timeout))?;

    if let Some(val) = config.ip_ttl {
        socket.set_ttl(val)?;
    }

    unsafe { Ok(MaySocket::from_raw_fd(socket.into_raw_fd())) }
}
