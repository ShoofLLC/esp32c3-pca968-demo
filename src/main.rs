use esp_idf_svc::hal::{
    delay::Delay,
    gpio::*, i2c::{I2cDriver, I2C0}
};

use esp_idf_svc::hal::i2c::config as i2c_config;

pub mod robots {
    use esp32c3_pca968::{Driver, Motor};

    pub struct TiltRobot<'a> {
        pub theta:u8,
        pub gamma:u8,
        theta_motor: Motor,
        alpha_motor: Motor,
        driver: Driver<'a>
    }

    impl<'a> TiltRobot<'a> {
        pub fn new(theta_motor: Motor,alpha_motor: Motor,driver: Driver<'a>) -> TiltRobot<'a> {
            return TiltRobot{theta: 0, gamma: 0, theta_motor, alpha_motor, driver}
        }

        pub fn tilt(&mut self, mut tilt: f32){

            if tilt< -1.0 {
                tilt= -1.0;
            } else if tilt>1.0 {
                tilt=1.0;
            }

            let max_tilt_angle = esp32c3_pca968::Motor::MAX_ANGLE;
            // f(v) = (max_angle/(max-min))*(v+1)
            let tilt_angle = max_tilt_angle/2.0*(tilt+1.0);

            self.alpha_motor.set_angle(tilt_angle,&mut self.driver);
        }

        pub fn side_tilt(&mut self, mut tilt: f32){

            if tilt< -1.0 {
                tilt= -1.0;
            } else if tilt>1.0 {
                tilt=1.0;
            }

            let max_tilt_angle = esp32c3_pca968::Motor::MAX_ANGLE;
            // f(v) = (max_angle/(max-min))*(v+1)
            let tilt_angle = max_tilt_angle/2.0*(tilt+1.0);

            self.theta_motor.set_angle(tilt_angle,&mut self.driver);
        }
    }

}


fn main() {
    /*
      It is necessary to call this function once. Otherwise some patches to the runtime
      implemented by esp-idf-sys might not link properly. See https://github.com/esp-rs/esp-idf-template/issues/71
    */
    esp_idf_svc::sys::link_patches();

    // Bind the log crate to the ESP Logging facilities
    esp_idf_svc::log::EspLogger::initialize_default();

    let delay = Delay::new_default();
    let m_sda_gpio_0:AnyIOPin;
    let m_scl_gpio_1:AnyIOPin;
    let m_i2c_0:I2C0;
    let slave_address: u8 = 0b01110000;
    //let slave_address: u8 = 0b10010110;

    unsafe {
        m_scl_gpio_1 = AnyIOPin::new(1);
        m_sda_gpio_0 = AnyIOPin::new(0);
        m_i2c_0 = I2C0::new(); 
    }
    
    let m_i2c_config = i2c_config::Config::new();
    let m_i2c_driver = I2cDriver::new(m_i2c_0, m_sda_gpio_0, m_scl_gpio_1, &m_i2c_config);
    match m_i2c_driver {
        Ok(m_i2c_driver) => {
            let mut m_driver = esp32c3_pca968::Driver::new(m_i2c_driver, slave_address, 1000);
            m_driver.write_prescale_value(125);
            let motor_1 = esp32c3_pca968::Motor::new(0, 0.0); 
            let motor_2 = esp32c3_pca968::Motor::new(1, 0.0);
            let mut tilt_robot = robots::TiltRobot::new(motor_1, motor_2, m_driver);
            //let mut motor_3 = esp32c3_pca968::Motor::new(2, 0.0); 
            tilt_robot.side_tilt(0.0);
            tilt_robot.tilt(0.0);
            delay.delay_ms(5000);
            loop {
                // too lazy
                tilt_robot.side_tilt(1.0);
                delay.delay_ms(500);
                tilt_robot.tilt(-1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.tilt(1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.side_tilt(0.0);
                delay.delay_ms(500);
                tilt_robot.tilt(-1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.tilt(1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.side_tilt(-1.0);
                delay.delay_ms(500);
                tilt_robot.tilt(-1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.tilt(1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.side_tilt(0.0);
                delay.delay_ms(500);
                tilt_robot.tilt(-1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
                tilt_robot.tilt(1.0);
                delay.delay_ms(200);
                tilt_robot.tilt(0.0);
                delay.delay_ms(200);
            }
        },
        _ => log::info!("failed to init i2c!!"),
    }
}
