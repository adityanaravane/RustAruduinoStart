use core::pin::Pin;
use arduino_hal::{Peripherals, pins, Pins};
use arduino_hal::hal::port;
use arduino_hal::pac::PORTB;
use arduino_hal::port::mode::{Floating, Input};

struct solenoid {
    pin: Pins:: Pin<>,
    threshold: i8,
    onoutput: i8,
    offoutput: i8
}

impl solenoid {

    fn On(&self) {

    }
    fn TrueFalseOn(&self, given:i8) {
        if given > self.pin {

        }
    }
}
