use esp_idf_svc::hal::gpio::OutputPin;
use esp_idf_svc::hal::ledc::{
    config::TimerConfig, LedcChannel, LedcDriver, LedcTimer, LedcTimerDriver,
};

pub struct Motor<'a> {
    pwm_driver: LedcDriver<'a>,
}

impl<'a> Motor<'a> {
    pub fn new<C: LedcChannel + 'a>(
        channel: C,
        timer: impl LedcTimer<SpeedMode = C::SpeedMode> + 'a,
        pin: impl OutputPin + 'a,
        pwm_fx: u32,
    ) -> anyhow::Result<Self> {
        // Configure the timer
        let timer_config = TimerConfig::new().frequency(pwm_fx.into());
        let timer_driver = LedcTimerDriver::new(timer, &timer_config)?;

        // Initialize PWM driver
        let pwm_driver = LedcDriver::new(channel, timer_driver, pin)?;

        Ok(Self { pwm_driver })
    }

    pub fn set_motor_speed(&mut self, speed_percentage: u32) -> anyhow::Result<()> {
        let speed_percentage = speed_percentage.min(100);
        let max_duty = self.pwm_driver.get_max_duty();

        // Calculate the target duty cycle
        let target_duty = (max_duty * speed_percentage) / 100;

        self.pwm_driver.set_duty(target_duty)?;
        Ok(())
    }
}
