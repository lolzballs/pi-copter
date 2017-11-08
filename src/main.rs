extern crate i2csensors;
extern crate i2cdev;
extern crate i2cdev_bmp280 as bmp280;
extern crate nalgebra;
extern crate rust_pigpio as gpio;
extern crate serial;
extern crate time;
extern crate uart_bno055 as bno055;

mod motors;
mod sensors;

use sensors::{Sensors, SensorInput};
use motors::MotorManager;

use bno055::BNO055;
use bmp280::*;
use i2cdev::linux::LinuxI2CDevice;

fn main() {
    gpio::initialize().expect("Unable to initialize pigpio");
    let mm = MotorManager::new([4, 17, 18, 27]).expect("Unable to initialize MotorManager");
    mm.arm().unwrap();

    let sensors = Sensors::new(SensorInput::new(
        BNO055::new(serial::open("/dev/ttyAMA0").unwrap()).unwrap(),
        BMP280::new(
            LinuxI2CDevice::new("/dev/i2c-1", BMP280_I2C_ADDR).unwrap(),
            BMP280Settings {
                compensation: BMP280CompensationAlgorithm::B64,
                t_sb: BMP280Timing::ms0_5,
                iir_filter_coeff: BMP280FilterCoefficient::Medium,
                osrs_t: BMP280TemperatureOversampling::x1,
                osrs_p: BMP280PressureOversampling::StandardResolution,
                power_mode: BMP280PowerMode::NormalMode,
            },
        ).unwrap(),
    ));

    /*
    for i in 1000..2000 {
        println!("{}", i);
        mm.set_power(0, i).unwrap();
        mm.set_power(1, i).unwrap();
        mm.set_power(2, i).unwrap();
        mm.set_power(3, i).unwrap();
        thread::sleep_ms(25);
    }

    mm.set_power(0, 1000).unwrap();
    mm.set_power(1, 1000).unwrap();
    mm.set_power(2, 1000).unwrap();
    mm.set_power(3, 1000).unwrap();
    */
}
