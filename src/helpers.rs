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
