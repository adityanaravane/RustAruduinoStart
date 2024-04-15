#![no_std]
#![no_main]


//mod solenoid;
//mod solenoid_states;

use panic_halt as _;

#[arduino_hal::entry]
fn main() -> ! {
    let dp = arduino_hal::Peripherals::take().unwrap();
    let pins = arduino_hal::pins!(dp);
    // let Solenoid1 = solenoid{
    //     pins,
    //     1,
    //     1,
    //     0
    //
    // };


    loop {
        pins.d13.into_output().set_low()
    }
}
