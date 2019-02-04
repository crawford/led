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

use embedded_hal::digital::OutputPin;

pub enum Color {
    Red,
    Green,
    Blue,
    Yellow,
    Cyan,
    Magenta,
    White,
    Black,
}

pub struct RGB<R, G, B> {
    red: R,
    green: G,
    blue: B,
}

impl<R, G, B> RGB<R, G, B>
where
    R: OutputPin,
    G: OutputPin,
    B: OutputPin,
{
    pub fn new(red: R, green: G, blue: B) -> RGB<R, G, B> {
        RGB { red, green, blue }
    }

    pub fn set_color(&mut self, color: Color) {
        match color {
            Color::Red => {
                self.red.set_low();
                self.green.set_high();
                self.blue.set_high();
            }
            Color::Green => {
                self.red.set_high();
                self.green.set_low();
                self.blue.set_high();
            }
            Color::Blue => {
                self.red.set_high();
                self.green.set_high();
                self.blue.set_low();
            }
            Color::Yellow => {
                self.red.set_low();
                self.green.set_low();
                self.blue.set_high();
            }
            Color::Cyan => {
                self.red.set_high();
                self.green.set_low();
                self.blue.set_low();
            }
            Color::Magenta => {
                self.red.set_low();
                self.green.set_high();
                self.blue.set_low();
            }
            Color::White => {
                self.red.set_low();
                self.green.set_low();
                self.blue.set_low();
            }
            Color::Black => {
                self.red.set_high();
                self.green.set_high();
                self.blue.set_high();
            }
        }
    }
}
