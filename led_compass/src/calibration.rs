//! Translated from <https://github.com/lancaster-university/codal-microbit-v2/blob/006abf5566774fbcf674c0c7df27e8a9d20013de/source/MicroBitCompassCalibrator.cpp>

/// # Calibration
/// One very important thing to do before using a sensor and trying to develop an application using
/// it is verifying that it's output is actually correct. If this does not happen to be the case we
/// need to calibrate the sensor (alternatively it could also be broken but that's rather unlikely
/// in this case).
use core::fmt::Debug;
use embedded_hal::blocking::delay::DelayUs;
use embedded_hal::blocking::i2c::{Write, WriteRead};
use libm::{fabsf, sqrtf};
use lsm303agr::{
    interface::I2cInterface, mode::MagContinuous, Lsm303agr, Measurement
};
use microbit::display::blocking::Display;
use microbit::display_pins;


const PERIMETER_POINTS: usize = 25;
const PIXEL1_THRESHOLD: i32 = 200;
const PIXEL2_THRESHOLD: i32 = 600;
const CALIBRATION_INCREMENT: i32 = 200;

#[derive(Debug)]
pub struct Calibration {
    center: Measurement,
    scale: Measurement,
    radius: u32
}

impl Default for Calibration {
    fn default() -> Self {
        Calibration {
            center: Measurement{x: 0, y: 0, z:0},
            scale: Measurement{ x: 1024, y: 1024, z: 1024},
            radius: 0
        }
    }
}

pub fn calc_calibration<I, T, E>(
    sensor: &mut Lsm303agr<I2cInterface<I>, MagContinuous>,
    display: &mut Display,
    timer: &mut T,
) -> Calibration
where
    T: DelayUs<u32>,
    I: Write<Error = E> + WriteRead<Error = E>,
    E: Debug,
{
    let data = get_data(sensor, display, timer);
    calibrate(&data)
}

fn get_data<I, T, E>(
    sensor: &mut Lsm303agr<I2cInterface<I>, MagContinuous>,
    display: &mut  Display,
    timer: &mut T,
) -> [Measurement; 25]
where
    T: DelayUs<u32>,
    I: Write<Error = E> + WriteRead<Error = E>,
    E: Debug,
{
    let mut leds = [
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
        [0, 0, 0, 0, 0],
    ];
    let mut cursor = (2,2);
    let mut data = [Measurement {x: 0, y: 0, z: 0}; PERIMETER_POINTS];

    for sample in 0..PERIMETER_POINTS {
        while !sensor.accel_status().unwrap().xyz_new_data {}
        let accel_data = sensor.accel_data().unwrap();
        let (x, y) = (accel_data.x, accel_data.y);
        if x < -PIXEL2_THRESHOLD {
            cursor.1 = 0;
        } else if x < -PIXEL1_THRESHOLD {
            cursor.1 = 1;
        } else if x > PIXEL2_THRESHOLD {
            cursor.1 = 4;
        } else if x > PIXEL1_THRESHOLD {
            cursor.1 = 3;
        } else {
            cursor.1 = 2;
        }

        if y < -PIXEL2_THRESHOLD {
            cursor.0 = 0;
        } else if y < -PIXEL1_THRESHOLD {
            cursor.0 = 1;
        } else if y > PIXEL2_THRESHOLD {
            cursor.0 = 4;
        } else if y > PIXEL1_THRESHOLD {
            cursor.0 = 3;
        } else {
            cursor.0 = 2;
        }

        // Turn the y axis properly
        cursor.0 = 4 - cursor.0;

        if leds[cursor.0][cursor.1] != 1 {
            leds[cursor.0][cursor.1] = 1;
            while !sensor.mag_status().unwrap().xyz_new_data {}
            let mag_data = measurement_to_enu(&sensor.mag_data().unwrap());
            data[sample] = mag_data;
        }
        display.show(timer, leds, 200);
    }

    data
}

fn difference_square(a: &Measurement, b: &Measurement) -> f32 {
    let dx = (a.x - b.x) as f32;
    let dy = (a.y - b.y) as f32;
    let dz = (a.z - b.z) as f32;

    (dx * dx) + (dy * dy) + (dz * dz)
}

fn measure_score(center: &Measurement, data: &[Measurement]) -> f32 {
    let mut min_d = difference_square(center, &data[0]);
    let mut max_d = min_d;

    for point in data[1..].iter() {
        let d = difference_square(center, point);
        if d < min_d {
            min_d = d;
        }

        if d > max_d {
            max_d = d;
        }
    }

    max_d - min_d
}

fn calibrate(data: &[Measurement]) -> Calibration {
    // Approximate a center for the data
    let mut center = Measurement{ x: 0, y: 0, z: 0};
    let mut best = center;

    for point in data {
        center.x += point.x;
        center.y += point.y;
        center.z += point.z;
    }

    center.x = center.x / data.len() as i32;
    center.y = center.y / data.len() as i32;
    center.z = center.z / data.len() as i32;

    let mut current = center;
    let mut score = measure_score(&center, &data);

    // Calculate a fixpoint position
    loop {
        for x in [-CALIBRATION_INCREMENT, 0, CALIBRATION_INCREMENT] {
            for y in [-CALIBRATION_INCREMENT, 0, CALIBRATION_INCREMENT] {
                for z in [-CALIBRATION_INCREMENT, 0, CALIBRATION_INCREMENT] {
                    let mut attempt = current;
                    attempt.x += x;
                    attempt.y += y;
                    attempt.z += z;

                    let attempt_score = measure_score(&attempt, &data);
                    if attempt_score < score {
                        score = attempt_score;
                        best = attempt;
                    }
                }
            }
        }

        if best == current {
            break;
        }
        current = best;
    }

    spherify(current, data)
}

fn spherify(center: Measurement, data: &[Measurement]) -> Calibration {
    let mut radius2 = 0;
    for point in data {
        let d2 = difference_square(&center, point) as u32;
        if d2 > radius2 {
            radius2 = d2;
        }
    }

    let mut scale: f32 = 0.0;
    let mut weight_x = 0.0;
    let mut weight_y = 0.0;
    let mut weight_z = 0.0;
    let radius =sqrtf(radius2 as f32);

    for point in data {
        let d = sqrtf(difference_square(&center, point));
        let s = (radius / d) - 1.0;
        scale = scale.max(s);

        let dx = point.x - center.x;
        let dy = point.y - center.y;
        let dz = point.z - center.z;

        weight_x += s * fabsf(dx as f32 / d);
        weight_y += s * fabsf(dy as f32 / d);
        weight_z += s * fabsf(dz as f32 / d);
    }

    let wmag = sqrtf((weight_x * weight_x) + (weight_y * weight_y) + (weight_z * weight_z));
    let scale_x = 1.0 + scale * (weight_x / wmag);
    let scale_y = 1.0 + scale * (weight_y / wmag);
    let scale_z = 1.0 + scale * (weight_z / wmag);

    Calibration{
        center,
        radius: radius as u32,
        scale: Measurement {
            x: (1024 * scale_x) as i32,
            y: (1024 * scale_y) as i32,
            z: (1024 * scale_z) as i32,
        },
    }
}

pub fn calibrated_measurement(measurement: &Measurement, calibration: &Calibration) -> Measurement {
    let mut out = measurement_to_enu(&measurement);
    out = Measurement {
        x: ((out.x - calibration.center.x) * calibration.scale.x) >> 10,
        y: ((out.y - calibration.center.y) * calibration.scale.y) >> 10,
        z: ((out.z - calibration.center.z) * calibration.scale.z) >> 10,
    };

    enu_to_cartesian(&out)
}

fn measurement_to_enu(measurement: &Measurement) -> Measurement {
    Measurement {
        x: -measurement.y,
        y: -measurement.x,
        z: measurement.z
    }
}

fn enu_to_cartesian(measurement: &Measurement) -> Measurement {
    Measurement {
        x: -measurement.y,
        y: -measurement.x,
        z: measurement.z
    }
}
