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

use std::io::{self, Write};
use std::num::NonZeroUsize;
use std::os::unix::io::{FromRawFd, IntoRawFd};

use humantime::format_duration;
use may::coroutine;
use socks::Socks5Stream;

use crate::config::{SocketConfig, TesterConfig};
use crate::helpers;

type StdSocket = std::net::TcpStream;
type MaySocket = may::net::TcpStream;

pub fn run(config: &TesterConfig, portions: &[&[u8]]) {
    let fmt_periodicity = helpers::cyan(format_duration(config.write_periodicity));

    loop {
        let mut socket: MaySocket = connect_socket(&config.socket_config);

        for &portion in portions {
            match send_portion(&mut socket, portion, config.failed_count) {
                SendPortionResult::Success => {
                    info!(
                        "{} bytes have been sent successfully. Waiting {}...",
                        helpers::cyan(portion.len()),
                        fmt_periodicity
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

        info!(
            "All the data portions have been sent successfully. Reconnecting the socket and \
             retrying it again..."
        );
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
}

fn connect_socket(config: &SocketConfig) -> MaySocket {
    loop {
        trace!("Trying to connect a new socket...");

        match try_connect_socket(config) {
            Ok(socket) => {
                trace!("A new socket has been connected successfully.");
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
    let socket = if let Some(proxy) = config.tor_proxy {
        Socks5Stream::connect(proxy, config.receiver)?.into_inner()
    } else {
        StdSocket::connect_timeout(&config.receiver, config.connect_timeout)?
    };

    socket.set_write_timeout(Some(config.write_timeout))?;
    unsafe { Ok(MaySocket::from_raw_fd(socket.into_raw_fd())) }
}
