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

use std::error::Error;
use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io;
use std::path::Path;

use colored::{ColoredString, Colorize};
use serde_json;

pub type ReadPortionsResult = Result<Vec<Vec<u8>>, ReadPortionsError>;

// Extracts data portions from a specified file
pub fn read_portions<P: AsRef<Path>>(path: P) -> ReadPortionsResult {
    let file = File::open(path).map_err(ReadPortionsError::ReadFailed)?;

    Ok(serde_json::from_reader::<_, Vec<String>>(file)
        .map_err(ReadPortionsError::JsonParseFailed)?
        .into_iter()
        .map(String::into_bytes)
        .collect())
}

#[derive(Debug)]
pub enum ReadPortionsError {
    // Used when the function cannot read file content.
    ReadFailed(io::Error),

    // Used when the function cannot parse JSON structure.
    JsonParseFailed(serde_json::Error),
}

impl Display for ReadPortionsError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ReadPortionsError::ReadFailed(err) => write!(fmt, "{}", err),
            ReadPortionsError::JsonParseFailed(err) => write!(fmt, "{}", err),
        }
    }
}

impl Error for ReadPortionsError {}

pub fn cyan<S: ToString>(value: S) -> ColoredString {
    value.to_string().cyan()
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test that `read_portions()` reads all the portions correctly
    #[test]
    fn reads_all_portions() {
        let res = read_portions("files/test.json").expect("Failed to parse JSON");

        assert_eq!(res[0].as_slice(), b"abc def g");
        assert_eq!(res[1].as_slice(), b"ghi kkl j");
        assert_eq!(res[2].as_slice(), b"mno pqr e");
        assert_eq!(res[3].as_slice(), b"stu vwx f");
    }
}
