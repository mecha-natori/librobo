#[cfg(feature = "bind-c")]
mod c {
    use super::super::*;

    #[no_mangle]
    extern "C" fn robo_servo_calc_servo_duty(
        deg: i16,
        max_duty: u16,
        servo: ServoDefinition
    ) -> u16 {
        calc_servo_duty(deg, max_duty, servo)
    }
}
