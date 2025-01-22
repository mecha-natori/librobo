#[cfg(feature = "bind-c")]
mod c {
    use super::super::*;
    use crate::steering::NormalizedSticks;
    use crate::steering::PIDData;
    use core::slice;

    #[no_mangle]
    unsafe extern "C" fn robo_steering_quad_mechanum_calc_speed(
        steering: Steering,
        pid_data: *mut PIDData,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> *const i16 {
        let mut buf = (*slice::from_raw_parts(pid_data, 4)).try_into().unwrap();
        let pid_data = if pid_data.is_null() {
            None
        } else {
            Some(&mut buf)
        };
        let speeds = <QuadMechanum as ISteering<4>>::calc_speed(steering, pid_data, l, r);
        speeds.as_ptr()
    }

    #[cfg(feature = "controller")]
    #[no_mangle]
    unsafe extern "C" fn robo_steering_quad_mechanum_calc_speed_from_sticks(
        steering: Steering,
        pid_data: *mut PIDData,
        sticks: NormalizedSticks
    ) -> *const i16 {
        let mut buf = (*slice::from_raw_parts(pid_data, 4)).try_into().unwrap();
        let pid_data = if pid_data.is_null() {
            None
        } else {
            Some(&mut buf)
        };
        let speeds =
            <QuadMechanum as ISteeringFromSticks<4>>::calc_speed(steering, pid_data, sticks);
        speeds.as_ptr()
    }
}
