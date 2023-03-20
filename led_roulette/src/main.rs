#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use microbit::{
    board::Board,
    display::blocking::Display,
    hal::{prelude::*, Timer},
};
use panic_rtt_target as _;
use rtt_target::{rtt_init_print};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();
    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);
    let mut leds = [
        [1, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];

    let mut i = 0;
    let mut j = 0;
    //infinite loop; just so we don't leave this stack frame
    loop {
        // Show light_it_all for 1000ms
        display.show(&mut timer, leds, 50);
        leds[i][j] = 0;

        if i == 0 && j < 4 {
            j += 1;
        } else if j == 4 && i < 4 {
            i += 1;
        } else if i == 4 && j > 0 {
            j -= 1;
        } else if j == 0 && i > 0 {
            i -= 1;
        }

        leds[i][j] = 1;
    }
}
