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

use std::time::Duration;

use humantime::parse_duration;
use std::net::SocketAddr;
use std::path::PathBuf;
use structopt::StructOpt;

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
#[structopt(
    author = "Temirkhan Myrzamadi <gymmasssorla@gmail.com>",
    about = "Wanna try the fastest Low & Slow traffic generator in existence?",
    after_help = "For more information see <https://github.com/Gymmasssorla/finshir>.",
    set_term_width = 80
)]
pub struct ArgsConfig {
    /// A waiting time span before a test execution used to prevent a
    /// launch of an erroneous (unwanted) test
    #[structopt(
        short = "w",
        long = "wait",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "5secs",
        parse(try_from_str = "parse_duration")
    )]
    pub wait: Duration,

    /// A location to a file consisting of a single JSON array of data portions,
    /// specified as strings.
    ///
    /// If an amount of data portions is reached on a certain connection, a
    /// connection will be reopened.
    #[structopt(
        short = "f",
        long = "portions-file",
        takes_value = true,
        value_name = "LOCATION",
        default_value = "finshir.json"
    )]
    pub portions_file: PathBuf,

    #[structopt(flatten)]
    pub socket_config: SocketConfig,

    #[structopt(flatten)]
    pub logging_config: LoggingConfig,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct SocketConfig {
    /// A receiver of generator traffic, specified as an IP address or a domain
    /// name and a port, separated by a colon
    #[structopt(
        short = "r",
        long = "receiver",
        takes_value = true,
        value_name = "SOCKET-ADDRESS"
    )]
    pub receiver: String,

    /// If a timeout is reached and a socket wasn't connected, the program will
    /// retry the operation later
    #[structopt(
        long = "connect-timeout",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "30secs",
        parse(try_from_str = "parse_duration")
    )]
    pub connect_timeout: Duration,

    /// If a timeout is reached and a data portion wasn't sent, the program will
    /// retry the operation later
    #[structopt(
        long = "write-timeout",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "30secs",
        parse(try_from_str = "parse_duration")
    )]
    pub write_timeout: Duration,

    /// A time interval between writing data portions. This option can be used
    /// to decrease test intensity
    #[structopt(
        long = "write-periodicity",
        takes_value = true,
        value_name = "TIME-SPAN",
        default_value = "10secs",
        parse(try_from_str = "parse_duration")
    )]
    pub write_periodicity: Duration,

    /// Connect all future sockets to a local tor proxy, specified as an IP
    /// address or a domain name and a port, separated by a colon.
    ///
    /// Typically, a tor proxy runs on 127.0.0.1:9050. You can edit its
    /// configuration located in `/etc/tor/torrc`.
    #[structopt(long = "tor-proxy", takes_value = true, value_name = "SOCKET-ADDRESS")]
    pub tor_proxy: Option<SocketAddr>,
}

#[derive(StructOpt, Debug, Clone, Eq, PartialEq)]
pub struct LoggingConfig {
    /// Enable one of the possible verbosity levels. The zero level doesn't
    /// print anything, and the last level prints everything
    #[structopt(
        short = "v",
        long = "verbosity",
        takes_value = true,
        value_name = "LEVEL",
        default_value = "3",
        possible_value = "0",
        possible_value = "1",
        possible_value = "2",
        possible_value = "3",
        possible_value = "4",
        possible_value = "5"
    )]
    pub verbosity: i32,

    /// A format for displaying local date and time in log messages. Type `man
    /// strftime` to see the format specification.
    ///
    /// Specifying a different format with days of weeks might be helpful when
    /// you want to test a server more than one day.
    #[structopt(
        long = "date-time-format",
        takes_value = true,
        value_name = "STRING",
        default_value = "%X",
        parse(try_from_str = "parse_time_format")
    )]
    pub date_time_format: String,
}

pub fn parse_time_format(format: &str) -> Result<String, time::ParseError> {
    // If the `strftime` call succeeds, then the format is correct
    time::strftime(format, &time::now())?;
    Ok(String::from(format))
}

#[cfg(test)]
mod tests {
    use super::*;

    // Check that ordinary formats are passed correctly
    #[test]
    fn parses_valid_time_format() {
        assert_eq!(&parse_time_format("%x %X %e").unwrap(), "%x %X %e");
        assert_eq!(&parse_time_format("%H %a %G").unwrap(), "%H %a %G");

        assert_eq!(&parse_time_format("something").unwrap(), "something");
        assert_eq!(&parse_time_format("flower %d").unwrap(), "flower %d");
    }

    // Invalid formats must produce the invalid format error
    #[test]
    fn parses_invalid_time_format() {
        assert!(parse_time_format("%_=-%vbg=").is_err());
        assert!(parse_time_format("yufb%44htv").is_err());
        assert!(parse_time_format("sf%jhei9%990").is_err());
    }
}
