#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{display::blocking::Display, hal::Timer, Board};
use panic_rtt_target as _;
use rtt_target::rtt_init_print;

#[derive(Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn delta(&self) -> (isize, isize) {
        match self {
            Direction::Right => (0, 1),
            Direction::Down => (1, 0),
            Direction::Left => (0, -1),
            Direction::Up => (-1, 0),
        }
    }

    fn next(self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

struct Roulette {
    max_row: usize,
    max_col: usize,
    direction: Direction,
    last_led: (usize, usize),
}

impl Roulette {
    fn new(rows: usize, columns: usize) -> Self {
        Roulette {
            max_row: rows - 1,
            max_col: columns - 1,
            direction: Direction::Right,
            last_led: (0, 0),
        }
    }
}

impl Iterator for Roulette {
    type Item = ((usize, usize), (usize, usize));

    fn next(&mut self) -> Option<Self::Item> {
        let (row, col) = self.last_led;
        let (d_row, d_col) = self.direction.delta();

        let mut current_led = (row as isize + d_row, col as isize + d_col);
        let out_of_bounds = current_led.0 < 0
            || current_led.1 < 0
            || current_led.0 > self.max_row as isize
            || current_led.1 > self.max_col as isize;

        if out_of_bounds {
            self.direction = self.direction.next();
            let (d_row, d_col) = self.direction.delta();
            current_led = (row as isize + d_row, col as isize + d_col);
        }

        if current_led == (0, 0) {
            None
        } else {
            let current_led = (current_led.0 as usize, current_led.1 as usize);
            let pixels = (self.last_led, current_led);
            self.last_led = current_led;
            Some(pixels)
        }
    }
}

#[entry]
fn main() -> ! {
    rtt_init_print!();

    let board = Board::take().unwrap();

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let interval_ms = 66;

    loop {
        for n in 2..=5 {
            let mut leds = [
                [1, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0],
            ];

            let roulette = Roulette::new(n, n);

            for (led_off, led_on) in roulette {
                leds[led_off.0][led_off.1] = 0;
                leds[led_on.0][led_on.1] = 1;
                display.show(&mut timer, leds, interval_ms);
            }
        }
    }
}
