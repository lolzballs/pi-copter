use std::io;
use std::time::Duration;
use std::thread;

use gpio::{self, pwm};

pub struct MotorManager {
    pub motors: [u32; 4],
}

impl MotorManager {
    pub fn new(motors: [u32; 4]) -> Result<Self, String> {
        let mut mm = MotorManager { motors };
        mm.initialize()?;
        Ok(mm)
    }

    fn initialize(&mut self) -> Result<(), String> {
        for &m in self.motors.iter() {
            gpio::set_mode(m, gpio::OUTPUT)?;
            pwm::set_pwm_range(m, 10000)?;
            pwm::set_pwm_frequency(m, 100)?;
        }
        Ok(())
    }

    pub fn arm(&self) -> Result<(), String> {
        for &m in self.motors.iter() {
            pwm::pwm(m, 1000)?;
        }
        thread::sleep(Duration::from_secs(2));
        Ok(())
    }

    pub fn calibrate(&self) -> Result<(), String> {
        for &m in self.motors.iter() {
            pwm::pwm(m, 2000)?;
        }

        println!("Plug in");
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Error");

        for &m in self.motors.iter() {
            pwm::pwm(m, 1000)?;
        }
        Ok(())
    }

    pub fn set_power(&self, i: usize, p: u32) -> Result<(), String> {
        pwm::pwm(self.motors[i], p)
    }
}
