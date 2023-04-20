// Copyright 2021 Alex Crawford
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

//! Monocolor LEDs
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
//! # let p = Pin{};
//! #
//! use led::LED;
//! use led::monocolor::CommonAnodeLED;
//!
//! let mut led = CommonAnodeLED::new(p);
//!
//! led.set(State::On);
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
//! # let p1 = Pin{};
//! # let p2 = Pin{};
//! #
//! use led::LED;
//! use led::monocolor::{CommonAnodeLED, CommonCathodeLED, Monocolor};
//!
//! let mut leds: [&mut Monocolor; 2] = [
//!     &mut CommonAnodeLED::new(p1),
//!     &mut CommonCathodeLED::new(p2),
//! ];
//!
//! for led in leds.iter_mut() {
//!     led.set(State::On)
//! }
//! ```

use core::marker::PhantomData;
use embedded_hal::digital::v2::OutputPin;

/// A common anode LED.
pub type CommonAnodeLED<P> = LED<CommonAnode, P>;

/// A common cathode LED.
pub type CommonCathodeLED<P> = LED<CommonCathode, P>;

/// A monocolor LED; either common anode or common cathode.
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
pub trait Monocolor: crate::LED<Input = State> {}
impl<L> Monocolor for L where L: crate::LED<Input = State> {}

#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum State {
    On,
    Off,
}

/// An RGB LED
///
/// The RGB LED is represented by three owned instances of `embedded_hal::digital::v2::OutputPin`
/// and a polarity (common anode or common cathode). Because the outputs are binary, only eight
/// colors can be presented: primary colors, secondary colors, white, and black.
pub struct LED<C, P> {
    common: PhantomData<C>,
    pin: P,
}

impl<C, P> LED<C, P>
where
    C: Common,
    P: OutputPin,
{
    /// Creates a new RGB LED given three GPIOs.
    pub fn new(pin: P) -> LED<C, P> {
        LED {
            common: PhantomData,
            pin,
        }
    }

    /// Sets the RGB LED to the specified color.
    fn set(&mut self, state: State) {
        let _ = match state {
            State::Off => C::disable(&mut self.pin),
            State::On => C::enable(&mut self.pin),
        };
    }
}

impl<C, P> crate::LED for LED<C, P>
where
    C: Common,
    P: OutputPin,
{
    type Input = State;

    /// Sets the RGB LED to the specified color.
    fn set(&mut self, state: State) {
        let _ = self.set(state);
    }
}

/// The polarity of the LED; either anode or cathode.
// XXX: split this out
pub trait Common {
    /// Enables the pin output.
    fn enable<P: OutputPin>(pin: &mut P) -> Result<(), P::Error>;

    /// Disables the pin output.
    fn disable<P: OutputPin>(pin: &mut P) -> Result<(), P::Error>;
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
    fn enable<P: OutputPin>(pin: &mut P) -> Result<(), P::Error> {
        pin.set_low()
    }

    fn disable<P: OutputPin>(pin: &mut P) -> Result<(), P::Error> {
        pin.set_high()
    }
}

impl Common for CommonCathode {
    fn enable<P: OutputPin>(pin: &mut P) -> Result<(), P::Error> {
        pin.set_high()
    }

    fn disable<P: OutputPin>(pin: &mut P) -> Result<(), P::Error> {
        pin.set_low()
    }
}
