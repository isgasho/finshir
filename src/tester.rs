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

use crate::config::SocketConfig;

use may::{self, coroutine};
use socks::Socks5Stream;

use std::io;
use std::net::TcpStream;
use std::time::Duration;

fn init_socket(config: &SocketConfig) -> TcpStream {
    let socket = match config.tor_proxy {
        Some(addr) => Socks5Stream::connect(addr, config.receiver)?.into_inner(),
        None => TcpStream::connect_timeout(&config.receiver, config.connect_timeout)?,
    };

    socket.set_write_timeout(Some(config.write_timeout));
    socket
}
