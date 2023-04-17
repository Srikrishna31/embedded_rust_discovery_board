#![deny(unsafe_code)]
#![no_main]
#![no_std]

use cortex_m_rt::entry;
use embedded_hal::Direction;
use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

mod calibration;
mod led;

use crate::calibration::{calc_calibration, calibrated_measurement};
use microbit::{display::blocking::Display, hal::Timer, Board};
use led::Direction as LedDirection;

use core::f32::consts::PI;
use libm::{atan2f, sqrtf};

#[cfg(feature="v1")]
use microbit::{hal::twi, pac::twi0::frequency::FREQUENCY_A};

#[cfg(feature="v2")]
use microbit::{hal::twim, pac::twim0::frequency::FREQUENCY_A};

use lsm303agr::{AccelOutputDataRate, Lsm303agr, MagOutputDataRate};

#[entry]
fn main() -> ! {
    rtt_init_print!();
    let board = Board::take().unwrap();

    #[cfg(feature="v1")]
    let i2c = { twi::Twi::new(board.TWI0, board.i2c.into(), FREQUENCY_A::K100)};

    #[cfg(feature="v2")]
    let i2c = {twim::Twim::new(board.TWIM0, board.i2c_internal.into(), FREQUENCY_A::K100)};

    let mut timer = Timer::new(board.TIMER0);
    let mut display = Display::new(board.display_pins);

    let mut sensor = Lsm303agr::new_with_i2c(i2c);
    sensor.init().unwrap();
    sensor.set_mag_odr(MagOutputDataRate::Hz10).unwrap();
    sensor.set_accel_odr(AccelOutputDataRate::Hz10).unwrap();

    let mut sensor = sensor.into_mag_continuous().ok().unwrap();

    let calibraion = calc_calibration(&mut sensor, &mut display, &mut timer);
    rprintln!("Calibration: {:?}", calibraion);
    rprintln!("Calibration done, entering busy loop");

    loop {
        while !sensor.mag_status().unwrap().xyz_new_data {}
        let mut data = sensor.mag_data().unwrap();
        data = calibrated_measurement(&data, &calibraion);

        let theta = atan2f(data.y as f32, data.x as f32);

        // Figure out the direction based on theta
        let dir = if theta < -7. * PI / 8. {
            LedDirection::West
        } else if theta < -5. * PI / 8. {
            LedDirection::SouthWest
        } else if theta < -3. * PI / 8. {
            LedDirection::South
        } else if theta < -PI / 8. {
            LedDirection::SouthEast
        } else if theta < PI / 8. {
            LedDirection::East
        } else if theta < 3. * PI / 8. {
            LedDirection::NorthEast
        } else if theta < 5. * PI / 8. {
            LedDirection::North
        } else if theta < 7. * PI / 8. {
            LedDirection::NorthWest
        } else {
            LedDirection::West
        };

        // use the led module to turn the direction into an LED arrow and the led display functions
        // from chapter 5 to display the arrow
        display.show(&mut timer, led::direction_to_led(dir), 100);

        let (x, y, z)  = (data.x as f32, data.y as f32, data.z as f32);
        let magnitude = sqrtf(x* x + y * y + z * z);
        rprintln!("{} nT, {} mG", magnitude, magnitude / 100.0);
    }
}
