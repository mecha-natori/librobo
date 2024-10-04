use super::*;

#[cfg(feature = "bind-c")]
mod c {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn robo_steering_quad_mechanum_calc_speed(
        steering: Steering,
        pid_data: *const PIDData,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> *const i16 {
        let pid_data = if pid_data.is_null() {
            None
        } else {
            Some(*pid_data)
        };
        QuadMechanum::calc_speed(steering, pid_data, l, r).as_ptr()
    }

    #[cfg(feature = "controller")]
    #[no_mangle]
    unsafe extern "C" fn robo_steering_quad_mechanum_calc_speed_from_sticks(
        steering: Steering,
        pid_data: *const PIDData,
        sticks: NormalizedSticks
    ) -> *const i16 {
        let pid_data = if pid_data.is_null() {
            None
        } else {
            Some(*pid_data)
        };
        QuadMechanum::calc_speed(steering, pid_data, sticks).as_ptr()
    }
}
