// //! Simple library for a rotary encoder. Based on the Keyes-040 rotary encoder however should be
// //! generic among rotary encoders. Also supports a push button to support pressed/not-pressed
// //! queries.
//
// use embedded_hal as hal;
// use hal::digital::v2::InputPin;
// use hal::{Direction, Qei};
// use either::Either;
//
// // todo! embedded-hal-alpha support
// // todo! struct without button or some way to not require a button
//
// #[derive(Debug)]
// pub enum RotaryEncoderError<A: InputPin, B: InputPin, SW: InputPin> {
//     PinAError(A::Error),
//     PinBError(B::Error),
//     ButtonError(SW::Error),
// }
//
// pub struct RotaryEncoder<A: InputPin, B: InputPin, SW: InputPin> {
//     pin_a: A,
//     pin_b: B,
//     switch: SW,
//     count: i32,
//     direction: Option<Direction>,
//     last_direction: Direction,
//     state: u8,
// }
//
// impl<A, B, SW> RotaryEncoder<A, B, SW>
// where
//     A: InputPin,
//     B: InputPin,
//     SW: InputPin,
// {
//     /// Create a new instance of a rotary encoder from a group of input pins.
//     /// Note: ensure the input pins provided are configured appropriately (with correct push/pull
//     /// etc. configuration for connections).
//     pub fn from_pins(pin_a: A, pin_b: B, switch: SW) -> RotaryEncoder<A, B, SW> {
//         RotaryEncoder {
//             pin_a,
//             pin_b,
//             switch,
//             count: 0,
//             direction: None,
//             last_direction: Direction::Downcounting,
//             state: 0,
//         }
//     }
//
//     /// Destroy the rotary encoder peripheral and return the pins.
//     pub fn release(self) -> (A, B, SW) {
//         (self.pin_a, self.pin_b, self.switch)
//     }
//
//     /// Increment the state machine.
//     pub fn update(&mut self) -> Result<Option<Direction>, Either<A::Error, B::Error>> {
//         let mut state = self.state;
//
//         if self.pin_a.is_high().unwrap() {
//             state |= 1 << 3;
//         }
//         if self.pin_b.is_high()? {
//             state |= 1 << 2;
//         }
//
//         self.state = state >> 2;
//
//         self.direction = match state {
//             0b1000 | 0b1110 | 0b0111 | 0b0001 => {
//                 self.count += 1;
//                 Some(Direction::Upcounting)
//             },
//             0b0100 | 0b1101 | 0b1011 | 0b0010 => {
//                 self.count -= 1;
//                 Some(Direction::Downcounting)
//             },
//             _ => None,
//         };
//
//         if let Some(dir) = self.direction {
//             self.last_direction = dir;
//         }
//
//         Ok(self.direction)
//     }
//
//     /// Returns state of pushbutton switch.
//     pub fn pressed(&mut self) -> Result<bool, SW::Error> {
//         self.switch.is_high()
//     }
//
//     /// Return mutable reference to pin a, useful for clearing interrupt bit for the pin.
//     pub fn pin_a(&mut self) -> &mut A {
//         &mut self.pin_a
//     }
//
//     /// Return mutable reference to pin b, useful for clearing interrupt bit for the pin.
//     pub fn pin_b(&mut self) -> &mut B {
//         &mut self.pin_b
//     }
//
//     /// Return mutable reference to pushbutton, useful for clearing interrupt bit for the pin.
//     pub fn button(&mut self) -> &mut SW {
//         &mut self.switch
//     }
// }
//
// impl<A, DT, SW> Qei for RotaryEncoder<A, DT, SW>
// where
//     A: InputPin,
//     DT: InputPin,
//     SW: InputPin,
// {
//     type Count = i32;
//
//     fn count(&self) -> Self::Count {
//         self.count
//     }
//
//     /// Return the most recent direction of movement.
//     fn direction(&self) -> Direction {
//         self.last_direction
//     }
// }
