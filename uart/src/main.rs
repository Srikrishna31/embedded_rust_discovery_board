#![no_main]
#![no_std]

use cortex_m_rt::entry;
use rtt_target::{rprintln, rtt_init_print};
use heapless::Vec;
use panic_rtt_target as _;
use core::fmt::Write;

#[cfg(feature = "v1")]
use microbit::{
    hal::prelude::*,
    hal::uart,
    hal::uart::{Baudrate, Parity},
};

#[cfg(feature="v2")]
use microbit::{
    hal::prelude::*,
    hal::uarte,
    hal::uarte::{Baudrate, Parity}
};

#[cfg(feature="v2")]
mod serial_setup;
#[cfg(feature="v2")]
use serial_setup::UartePort;

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = microbit::Board::take().unwrap();

    #[cfg(feature = "v1")]
    let mut serial = {
        uart::Uart::new(
            board.UART0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        )
    };

    #[cfg(feature = "v2")]
    let mut serial = {
        let serial = uarte::Uarte::new(
            board.UARTE0,
            board.uart.into(),
            Parity::EXCLUDED,
            Baudrate::BAUD115200,
        );
        UartePort::new(serial)
    };

    // nb is a "Minimal and reusable non-blocking I/O layer. It allows us to write code that can
    // conduct hardware operations in the background while we go and do other work (non-blocking).
    // write!(serial, "The quick brown fox jumps over the lazy dog.\r\n").unwrap();
    // nb::block!(serial.flush()).unwrap();

    // A buffer with 32 bytes of capacity
    let mut buffer: Vec<u8,32> = Vec::new();
    loop {
        buffer.clear();

        // TODO: Receive a user request. Each user request ends with ENTER
        // NOTE `buffer.push` returns a `Result`. handle the error by responding with an error message
        loop {
            let b = nb::block!(serial.read()).unwrap();

            nb::block!(serial.write(b)).unwrap();

            if b == 0x0D {
                break;
            }

            match buffer.push(b)  {
                Err(e) => rprintln!("Error pushing byte value into error: {}", e),
                _ => {}
            };
        }
        buffer.reverse();

        write!(serial, "\r\n");
        for b in buffer.iter() {
            nb::block!(serial.write(*b)).unwrap();
        }

        // write!(serial, "{}", buffer as &str).unwrap();

        write!(serial, "\r\n");

        nb::block!(serial.flush()).unwrap();
    }
}
