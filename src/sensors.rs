use std::cell::RefCell;

use bno055::BNO055;
use bmp280::BMP280;
use i2cdev::linux::LinuxI2CDevice;
use i2csensors::{Barometer, Thermometer};
use nalgebra::{Quaternion, Vector3};
use serial::SystemPort;
use time::SteadyTime;

pub struct Sensors {
    pub attitude: Quaternion<f32>,
    pub angular_rate: Vector3<f32>,
    pub pressure: f32,
    pub temp: f32,

    input: SensorInput,
    time: SteadyTime,
}

impl Sensors {
    pub fn new(input: SensorInput) -> Self {
        Sensors {
            attitude: Quaternion::identity(),
            angular_rate: Vector3::zeros(),
            pressure: 0.0,
            temp: 0.0,
            input,
            time: SteadyTime::now(),
        }
    }

    pub fn update(&mut self) {
        self.time = SteadyTime::now();
        self.attitude = self.input.get_absolute_orientation();
        self.angular_rate = self.input.get_angular_rate();
        self.pressure = self.input.get_pressure();
        self.temp = self.input.get_temperature();
    }
}

pub struct SensorInput {
    // TODO: Abstract these to traits later
    imu: BNO055<SystemPort>,
    barometer: BMP280<LinuxI2CDevice>,
}

impl SensorInput {
    pub fn new(imu: BNO055<SystemPort>, barometer: BMP280<LinuxI2CDevice>) -> Self {
        SensorInput { imu, barometer }
    }

    fn get_absolute_orientation(&mut self) -> Quaternion<f32> {
        let q = self.imu.get_quaternion().unwrap();
        Quaternion::new(q.w, q.x, q.y, q.z)
    }

    fn get_angular_rate(&mut self) -> Vector3<f32> {
        let v = self.imu.get_angular_rate().unwrap();
        Vector3::new(v.0, v.1, v.2)
    }

    fn get_pressure(&mut self) -> f32 {
        self.barometer.pressure_kpa().unwrap()
    }

    fn get_temperature(&mut self) -> f32 {
        self.barometer.temperature_celsius().unwrap()
    }
}
