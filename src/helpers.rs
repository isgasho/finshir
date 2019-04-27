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

use std::fmt::{self, Display, Formatter};
use std::fs::File;
use std::io;
use std::path::PathBuf;

use serde_json::{self, Error};

pub type ReadPortionsResult = Result<Vec<Vec<u8>>, ReadPortionsError>;

#[derive(Debug)]
pub enum ReadPortionsError {
    ReadFailed(io::Error),
    JsonParseFailed(Error),
}

impl Display for ReadPortionsError {
    fn fmt(&self, fmt: &mut Formatter) -> fmt::Result {
        match self {
            ReadPortionsError::ReadFailed(err) => write!(fmt, "{}", err),
            ReadPortionsError::JsonParseFailed(err) => write!(fmt, "{}", err),
        }
    }
}

pub fn read_portions(filename: &PathBuf) -> ReadPortionsResult {
    let file = File::open(filename).map_err(|err| ReadPortionsError::ReadFailed(err))?;
    let array: Vec<String> =
        serde_json::from_reader(file).map_err(|err| ReadPortionsError::JsonParseFailed(err))?;

    Ok(array.into_iter().map(|s| s.into_bytes()).collect())
}
