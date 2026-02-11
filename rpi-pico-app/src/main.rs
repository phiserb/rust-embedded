#![no_std]
#![no_main]

use cortex_m::asm;
use panic_halt as _;
use rp_pico::entry;

use embedded_hal::digital::OutputPin;

use rp_pico::hal::pac;

use rp_pico::hal;

#[entry]
fn main() -> ! {
    let mut pac = pac::Peripherals::take().unwrap();

    let sio = hal::Sio::new(pac.SIO);

    let pins = rp_pico::Pins::new(
        pac.IO_BANK0,
        pac.PADS_BANK0,
        sio.gpio_bank0,
        &mut pac.RESETS,
    );
    // Set the LED to be an output
    let mut led_pin = pins.led.into_push_pull_output();

    led_pin.set_high();

    loop {
        asm::nop();
    }
}
