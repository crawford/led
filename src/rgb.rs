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

//! RGB LEDs
//!
//! Various types of RGB LEDs.
//!
//! # Examples
//!
//! ```
//! # use embedded_hal::digital::v2::OutputPin;
//! #
//! # struct Pin {}
//! #
//! # impl OutputPin for Pin {
//! #     fn set_low(&mut self) -> Result<(), ()> {
//! #         Ok(())
//! #     }
//! #     fn set_high(&mut self) -> Result<(), ()> {
//! #         Ok(())
//! #     }
//! # }
//! #
//! # let r = Pin{};
//! # let g = Pin{};
//! # let b = Pin{};
//! #
//! use led::LED;
//! use led::rgb::{Color, CommonAnodeLED};
//!
//! let mut led = CommonAnodeLED::new(r, g, b);
//!
//! led.set(Color::Green);
//! ```
//!
//! ```
//! # use embedded_hal::digital::v2::OutputPin;
//! #
//! # struct Pin {}
//! #
//! # impl OutputPin for Pin {
//! #     fn set_low(&mut self) -> Result<(), ()> {
//! #         Ok(())
//! #     }
//! #     fn set_high(&mut self) -> Result<(), ()> {
//! #         Ok(())
//! #     }
//! # }
//! #
//! # let r1 = Pin{};
//! # let g1 = Pin{};
//! # let b1 = Pin{};
//! # let r2 = Pin{};
//! # let g2 = Pin{};
//! # let b2 = Pin{};
//! #
//! use led::LED;
//! use led::rgb::{Color, CommonAnodeLED, CommonCathodeLED, RGB};
//!
//! let mut leds: [&mut RGB; 2] = [
//!     &mut CommonAnodeLED::new(r1, g1, b1),
//!     &mut CommonCathodeLED::new(r2, g2, b2),
//! ];
//!
//! for led in leds.iter_mut() {
//!     led.set(Color::Red)
//! }
//! ```

use core::marker::PhantomData;
use embedded_hal::digital::v2::OutputPin;

/// The set of primary colors and secondary colors that can be created by an RGB LED along with
/// black and white.
#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
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

/// A common anode LED.
pub type CommonAnodeLED<R, G, B, E> = LED<CommonAnode, R, G, B, E>;

/// A common cathode LED.
pub type CommonCathodeLED<R, G, B, E> = LED<CommonCathode, R, G, B, E>;

/// An RGB LED; either common anode or common cathode.
///
/// # Examples
///
/// ```
/// # use embedded_hal::digital::v2::OutputPin;
/// #
/// # struct Pin {}
/// #
/// # impl OutputPin for Pin {
/// #     fn set_low(&mut self) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// #     fn set_high(&mut self) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # let r1 = Pin{};
/// # let g1 = Pin{};
/// # let b1 = Pin{};
/// # let r2 = Pin{};
/// # let g2 = Pin{};
/// # let b2 = Pin{};
/// #
/// use led::rgb::{Color, CommonAnodeLED, CommonCathodeLED, RGB};
///
/// let mut leds: [&mut RGB; 2] = [
///     &mut CommonAnodeLED::new(r1, g1, b1),
///     &mut CommonCathodeLED::new(r2, g2, b2),
/// ];
///
/// for led in leds.iter_mut() {
///     led.set(Color::Red)
/// }
/// ```
pub trait RGB: crate::LED<Input = Color> {}
impl<L> RGB for L where L: crate::LED<Input = Color> {}

/// An RGB LED
///
/// The RGB LED is represented by three owned instances of `embedded_hal::digital::v2::OutputPin`
/// and a polarity (common anode or common cathode). Because the outputs are binary, only eight
/// colors can be presented: primary colors, secondary colors, white, and black.
pub struct LED<C, R, G, B, E>
where
    C: Common,
    R: OutputPin<Error = E>,
    G: OutputPin<Error = E>,
    B: OutputPin<Error = E>,
{
    common: PhantomData<C>,
    red: R,
    green: G,
    blue: B,
}

impl<C, R, G, B, E> LED<C, R, G, B, E>
where
    C: Common,
    R: OutputPin<Error = E>,
    G: OutputPin<Error = E>,
    B: OutputPin<Error = E>,
{
    /// Creates a new RGB LED given three GPIOs.
    pub fn new(red: R, green: G, blue: B) -> LED<C, R, G, B, E> {
        LED {
            common: PhantomData,
            red,
            green,
            blue,
        }
    }

    /// Sets the RGB LED to the specified color.
    fn set(&mut self, color: Color) -> Result<(), <R as OutputPin>::Error> {
        match color {
            Color::Red => {
                C::enable(&mut self.red)?;
                C::disable(&mut self.green)?;
                C::disable(&mut self.blue)?;
            }
            Color::Green => {
                C::disable(&mut self.red)?;
                C::enable(&mut self.green)?;
                C::disable(&mut self.blue)?;
            }
            Color::Blue => {
                C::disable(&mut self.red)?;
                C::disable(&mut self.green)?;
                C::enable(&mut self.blue)?;
            }
            Color::Yellow => {
                C::enable(&mut self.red)?;
                C::enable(&mut self.green)?;
                C::disable(&mut self.blue)?;
            }
            Color::Cyan => {
                C::disable(&mut self.red)?;
                C::enable(&mut self.green)?;
                C::enable(&mut self.blue)?;
            }
            Color::Magenta => {
                C::enable(&mut self.red)?;
                C::disable(&mut self.green)?;
                C::enable(&mut self.blue)?;
            }
            Color::White => {
                C::enable(&mut self.red)?;
                C::enable(&mut self.green)?;
                C::enable(&mut self.blue)?;
            }
            Color::Black => {
                C::disable(&mut self.red)?;
                C::disable(&mut self.green)?;
                C::disable(&mut self.blue)?;
            }
        }

        Ok(())
    }
}

impl<C, R, G, B, E> crate::LED for LED<C, R, G, B, E>
where
    C: Common,
    R: OutputPin<Error = E>,
    G: OutputPin<Error = E>,
    B: OutputPin<Error = E>,
{
    type Input = Color;

    /// Sets the RGB LED to the specified color.
    fn set(&mut self, color: Color) {
        let _ = self.set(color);
    }
}

/// The polarity of the LED; either anode or cathode.
pub trait Common {
    /// Enables the pin output.
    fn enable<P: OutputPin>(pin: &mut P) -> Result<(), <P as OutputPin>::Error>;

    /// Disables the pin output.
    fn disable<P: OutputPin>(pin: &mut P) -> Result<(), <P as OutputPin>::Error>;
}

/// A marker type that represents a common anode connection.
///
/// # Examples
///
/// ```
/// # use embedded_hal::digital::v2::OutputPin;
/// #
/// # struct Pin {}
/// #
/// # impl OutputPin for Pin {
/// #     fn set_low(&mut self) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// #     fn set_high(&mut self) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # let r = Pin{};
/// # let g = Pin{};
/// # let b = Pin{};
/// #
/// use led::rgb::{CommonAnode, LED};
///
/// let led: LED<CommonAnode, _, _, _> = LED::new(r, g, b);
/// ```
pub struct CommonAnode {
    private: PhantomData<()>,
}

/// A marker type that represents a common cathode connection.
///
/// # Examples
///
/// ```
/// # use embedded_hal::digital::v2::OutputPin;
/// #
/// # struct Pin {}
/// #
/// # impl OutputPin for Pin {
/// #     fn set_low(&mut self) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// #     fn set_high(&mut self) -> Result<(), ()> {
/// #         Ok(())
/// #     }
/// # }
/// #
/// # let r = Pin{};
/// # let g = Pin{};
/// # let b = Pin{};
/// #
/// use led::rgb::{CommonCathode, LED};
///
/// let led: LED<CommonCathode, _, _, _> = LED::new(r, g, b);
/// ```
pub struct CommonCathode {
    private: PhantomData<()>,
}

impl Common for CommonAnode {
    fn enable<P: OutputPin>(pin: &mut P) -> Result<(), <P as OutputPin>::Error> {
        pin.set_low()
    }

    fn disable<P: OutputPin>(pin: &mut P) -> Result<(), <P as OutputPin>::Error> {
        pin.set_high()
    }
}

impl Common for CommonCathode {
    fn enable<P: OutputPin>(pin: &mut P) -> Result<(), <P as OutputPin>::Error> {
        pin.set_high()
    }

    fn disable<P: OutputPin>(pin: &mut P) -> Result<(), <P as OutputPin>::Error> {
        pin.set_low()
    }
}
