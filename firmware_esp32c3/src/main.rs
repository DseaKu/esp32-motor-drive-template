mod motor;

use esp_idf_svc::hal::delay::FreeRtos;
use esp_idf_svc::hal::peripherals::Peripherals;
use motor::Motor;

const DELAY: u32 = 100;
const PWM_FX: u32 = 5000;

fn main() -> anyhow::Result<()> {
    esp_idf_svc::sys::link_patches();

    let peripherals = Peripherals::take()?;

    // Initialize the motor
    let mut motor = Motor::new(
        peripherals.ledc.channel0,
        peripherals.ledc.timer0,
        peripherals.pins.gpio5,
        PWM_FX,
    )?;

    loop {
        for i in 1..100 {
            motor.set_motor_speed(i)?;
            FreeRtos::delay_ms(DELAY);
        }
    }
}
