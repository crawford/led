// Copyright 2019 Alex Crawford
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

#[macro_use]
mod font;

#[cfg(feature = "ascii")]
use ascii::AsciiChar;
#[cfg(feature = "ascii")]
use core::convert::From;
use core::convert::TryFrom;
use core::fmt;
use embedded_hal::digital::OutputPinMatrix;
use ignore_result::Ignore;

/// An Alphanumeric LED module
pub struct LED<'a, P> {
    pins: &'a mut P,
}

impl<'a, P> LED<'a, P>
where
    P: OutputPinMatrix<2, 8>,
{
    pub fn new(pins: &mut P) -> LED<P> {
        LED { pins }
    }
}

impl<'a, P> crate::LED for LED<'a, P>
where
    P: OutputPinMatrix<2, 8>,
{
    type Input = Character;

    fn set(&mut self, c: Character) {
        let glyph =
            font::MAP[c.base as usize & 0x7F] | if c.point { font::MAP['.' as usize] } else { 0 };
        self.pins
            .set_column(
                0,
                &[
                    (glyph >> 0) & 0x01 == 0x01,
                    (glyph >> 1) & 0x01 == 0x01,
                    (glyph >> 2) & 0x01 == 0x01,
                    (glyph >> 3) & 0x01 == 0x01,
                    (glyph >> 4) & 0x01 == 0x01,
                    (glyph >> 5) & 0x01 == 0x01,
                    (glyph >> 6) & 0x01 == 0x01,
                    (glyph >> 7) & 0x01 == 0x01,
                ],
            )
            .ignore();
        self.pins
            .set_column(
                1,
                &[
                    (glyph >> 8) & 0x01 == 0x01,
                    (glyph >> 9) & 0x01 == 0x01,
                    (glyph >> 10) & 0x01 == 0x01,
                    (glyph >> 11) & 0x01 == 0x01,
                    (glyph >> 12) & 0x01 == 0x01,
                    (glyph >> 13) & 0x01 == 0x01,
                    (glyph >> 14) & 0x01 == 0x01,
                    (glyph >> 15) & 0x01 == 0x01,
                ],
            )
            .ignore();
    }
}

pub enum Error {
    NonAscii,
}

impl fmt::Debug for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "not an ascii character")
    }
}

pub struct Character {
    base: u8,
    point: bool,
}

impl Character {
    pub fn set_point(mut self) -> Character {
        self.point = true;
        self
    }
}

impl TryFrom<u8> for Character {
    type Error = Error;

    fn try_from(b: u8) -> Result<Character, Error> {
        if b.is_ascii() {
            Ok(Character {
                base: b,
                point: false,
            })
        } else {
            Err(Error::NonAscii)
        }
    }
}

impl TryFrom<char> for Character {
    type Error = Error;

    fn try_from(c: char) -> Result<Character, Error> {
        if c.is_ascii() {
            Ok(Character {
                base: c as u8,
                point: false,
            })
        } else {
            Err(Error::NonAscii)
        }
    }
}

#[cfg(feature = "ascii")]
impl From<AsciiChar> for Character {
    fn from(a: AsciiChar) -> Character {
        Character {
            base: a as u8,
            point: false,
        }
    }
}
