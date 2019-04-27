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

use crate::config::{SocketConfig, TesterConfig};

use humantime::format_duration;
use may::coroutine;
use socks::Socks5Stream;

use std::io::{self, Write};
use std::net::SocketAddr;
use std::num::NonZeroUsize;
use std::time::Duration;

pub fn run(config: &TesterConfig, portions: &[&[u8]]) {
    let formatted_periodicity = format_duration(config.write_periodicity);

    loop {
        let mut socket = connect_socket(&config.socket_config);

        for &portion in portions {
            match send_portion(&mut socket, portion, config.failed_count) {
                Ok(bytes) => {
                    info!(
                        "{} bytes have been transmitted successfully. Waiting {}...",
                        portion.len(),
                        formatted_periodicity
                    );
                }
                Err(err) => {
                    info!(
                        "Transmitting {} bytes failed {} times >>> {}! Reconnecting the socket...",
                        portion.len(),
                        config.failed_count,
                        err,
                    );
                    break;
                }
            }

            coroutine::sleep(config.write_periodicity);
        }
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq)]
enum SendPortionResult {
    Success,
    Failed,
}

fn send_portion(
    socket: &mut may::net::TcpStream,
    portion: &[u8],
    failed_count: NonZeroUsize,
) -> SendPortionResult {
    for i in 0..failed_count.get() {
        match socket.write_all(portion) {
            Ok(bytes) => SendPortionResult::Success,
            Err(err) => {
                error!(
                    "Transmitting {} bytes failed >>> {}!. Retrying the operation...",
                    portion.len(),
                    err
                );
            }
        }
    }

    SendPortionResult::Failed
}

fn connect_socket(config: &SocketConfig) -> may::net::TcpStream {
    loop {
        match try_connect_socket(config) {
            Ok(socket) => {
                trace!("A new socket has been connected successfully.");
                socket
            }
            Err(err) => {
                error!(
                    "Socket connecting failed >>> {}! Retrying the operation...",
                    err
                );
            }
        }
    }
}

fn try_connect_socket(config: &SocketConfig) -> io::Result<may::net::TcpStream> {
    let socket = match config.tor_proxy {
        Some(addr) => connect_through_tor(addr, config.receiver)?,
        None => connect_timeout(&config.receiver, config.connect_timeout)?,
    };

    socket.set_write_timeout(Some(config.write_timeout))?;
    unsafe { Ok(may::net::TcpStream::from_raw_fd(socket.into_raw_fd())) }
}

// Returns a socket connected to a specified `receiver` with `timeout`.
fn connect_timeout(receiver: &SocketAddr, timeout: Duration) -> io::Result<std::net::TcpStream> {
    std::net::TcpStream::connect_timeout(receiver, timeout)
}

// Returns a socket connected to a specified `receiver` through Tor.
fn connect_through_tor(proxy: SocketAddr, receiver: SocketAddr) -> io::Result<std::net::TcpStream> {
    Ok(Socks5Stream::connect(proxy, receiver)?.into_inner())
}
