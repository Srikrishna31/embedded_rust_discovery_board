#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use panic_halt as _;
use microbit as _;

#[entry]
fn main() -> ! {
    let _y: i32;
    let x = 42;
    let _y = x;

    //infinite loop; just so we don't leave this stack frame
    loop {}
}
