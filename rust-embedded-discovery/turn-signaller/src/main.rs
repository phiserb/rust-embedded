#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::delay::DelayNs;
use embedded_hal::digital::InputPin;
use microbit::{Board, display::blocking::Display, hal::Timer};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

enum Signal {
    Left,
    Neutral,
    Right,
}

impl Signal {
    fn get_display(&self) -> [[u8; 5]; 5] {
        match self {
            Signal::Left => [
                [0, 0, 1, 0, 0],
                [0, 1, 0, 0, 0],
                [1, 1, 1, 1, 1],
                [0, 1, 0, 0, 0],
                [0, 0, 1, 0, 0],
            ],
            Signal::Neutral => [
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 1, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ],
            Signal::Right => [
                [0, 0, 1, 0, 0],
                [0, 0, 0, 1, 0],
                [1, 1, 1, 1, 1],
                [0, 0, 0, 1, 0],
                [0, 0, 1, 0, 0],
            ],
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut button_a = board.buttons.button_a;
    let mut button_b = board.buttons.button_b;
    let mut signal = Signal::Neutral;

    let poll_interval_ms = 10;

    loop {
        let left_pressed = button_a.is_low().unwrap();
        let right_pressed = button_b.is_low().unwrap();
        signal = match (left_pressed, right_pressed) {
            (false, false) => Signal::Neutral,
            (true, false) => Signal::Left,
            (false, true) => Signal::Right,
            (true, true) => signal,
        };
        display.show(&mut timer, signal.get_display(), poll_interval_ms);
        timer.delay_ms(poll_interval_ms);
    }
}
