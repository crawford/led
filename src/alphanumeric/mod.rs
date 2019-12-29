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

use core::convert::TryFrom;
use core::fmt;
use embedded_hal::digital::OutputPin;

/// An Alphanumeric LED module
pub struct LED<A, B, C, D, E, F, G, P> {
    glyph: u16,
    seg_a: A,
    seg_b: B,
    seg_c: C,
    seg_d: D,
    seg_e: E,
    seg_f: F,
    seg_g: G,
    point: P,
}

impl<A, B, C, D, E, F, G, P> LED<A, B, C, D, E, F, G, P>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
    F: OutputPin,
    G: OutputPin,
    P: OutputPin,
{
    pub fn new(
        seg_a: A,
        seg_b: B,
        seg_c: C,
        seg_d: D,
        seg_e: E,
        seg_f: F,
        seg_g: G,
        point: P,
    ) -> LED<A, B, C, D, E, F, G, P> {
        LED {
            glyph: 0,
            seg_a,
            seg_b,
            seg_c,
            seg_d,
            seg_e,
            seg_f,
            seg_g,
            point,
        }
    }

    #[rustfmt::skip]
    pub fn output_first_half(&mut self) {
        if self.glyph & 0x8000 == 0 { self.seg_a.set_low() } else { self.seg_a.set_high() };
        if self.glyph & 0x4000 == 0 { self.seg_b.set_low() } else { self.seg_b.set_high() };
        if self.glyph & 0x2000 == 0 { self.seg_c.set_low() } else { self.seg_c.set_high() };
        if self.glyph & 0x1000 == 0 { self.seg_d.set_low() } else { self.seg_d.set_high() };
        if self.glyph & 0x0800 == 0 { self.seg_e.set_low() } else { self.seg_e.set_high() };
        if self.glyph & 0x0400 == 0 { self.seg_f.set_low() } else { self.seg_f.set_high() };
        if self.glyph & 0x0200 == 0 { self.seg_g.set_low() } else { self.seg_g.set_high() };
        self.point.set_low();
    }

    #[rustfmt::skip]
    pub fn output_second_half(&mut self) {
        if self.glyph & 0x0080 == 0 { self.seg_a.set_low() } else { self.seg_a.set_high() };
        if self.glyph & 0x0040 == 0 { self.seg_b.set_low() } else { self.seg_b.set_high() };
        if self.glyph & 0x0020 == 0 { self.seg_c.set_low() } else { self.seg_c.set_high() };
        if self.glyph & 0x0010 == 0 { self.seg_d.set_low() } else { self.seg_d.set_high() };
        if self.glyph & 0x0008 == 0 { self.seg_e.set_low() } else { self.seg_e.set_high() };
        if self.glyph & 0x0004 == 0 { self.seg_f.set_low() } else { self.seg_f.set_high() };
        if self.glyph & 0x0002 == 0 { self.seg_g.set_low() } else { self.seg_g.set_high() };
        if self.glyph & 0x0001 == 0 { self.point.set_low() } else { self.point.set_high() };
    }
}

impl<A, B, C, D, E, F, G, P> crate::LED for LED<A, B, C, D, E, F, G, P>
where
    A: OutputPin,
    B: OutputPin,
    C: OutputPin,
    D: OutputPin,
    E: OutputPin,
    F: OutputPin,
    G: OutputPin,
    P: OutputPin,
{
    type Input = Character;

    fn set(&mut self, c: Character) {
        self.glyph = font::MAP[c.base as usize & 0x7F]
            | if c.point {
                glyph!("    .")
            } else {
                glyph!("     ")
            };
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
