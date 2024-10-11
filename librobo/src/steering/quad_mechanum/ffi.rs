use super::*;
#[cfg(feature = "heapless")]
use core::slice;
#[cfg(feature = "heapless")]
use heapless::Vec;

#[cfg(feature = "bind-c")]
mod c {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn robo_steering_quad_mechanum_calc_speed(
        steering: Steering,
        pid_data: *mut PIDData,
        l: Complex<f32>,
        r: Complex<f32>
    ) -> *const i16 {
        #[cfg(any(feature = "alloc", feature = "std"))]
        let mut buf = Vec::from_raw_parts(pid_data, 4, 4);
        #[cfg(feature = "heapless")]
        let mut buf = Vec::from_slice(slice::from_raw_parts(pid_data, 4)).unwrap();
        let pid_data = if pid_data.is_null() {
            None
        } else {
            Some(&mut buf)
        };
        #[cfg(any(feature = "alloc", feature = "std"))]
        return <QuadMechanum as ISteering>::calc_speed(steering, pid_data, l, r).as_ptr();
        #[cfg(feature = "heapless")]
        <QuadMechanum as ISteering<4>>::calc_speed(steering, pid_data, l, r).as_ptr()
    }

    #[cfg(feature = "controller")]
    #[no_mangle]
    unsafe extern "C" fn robo_steering_quad_mechanum_calc_speed_from_sticks(
        steering: Steering,
        pid_data: *mut PIDData,
        sticks: NormalizedSticks
    ) -> *const i16 {
        #[cfg(any(feature = "alloc", feature = "std"))]
        let mut buf = Vec::from_raw_parts(pid_data, 4, 4);
        #[cfg(feature = "heapless")]
        let mut buf = Vec::from_slice(slice::from_raw_parts(pid_data, 4)).unwrap();
        let pid_data = if pid_data.is_null() {
            None
        } else {
            Some(&mut buf)
        };
        #[cfg(any(feature = "alloc", feature = "std"))]
        return <QuadMechanum as ISteeringFromSticks>::calc_speed(steering, pid_data, sticks)
            .as_ptr();
        #[cfg(feature = "heapless")]
        <QuadMechanum as ISteeringFromSticks<4>>::calc_speed(steering, pid_data, sticks).as_ptr()
    }
}
