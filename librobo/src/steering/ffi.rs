use super::*;
#[cfg(not(any(feature = "alloc", feature = "std")))]
use heapless::Vec;

#[cfg(feature = "bind-c")]
mod c {
    use super::*;

    #[no_mangle]
    unsafe extern "C" fn robo_steering_process_pid_data(
        pid_data: *mut PIDData,
        target: f32
    ) -> f32 {
        if pid_data.is_null() {
            return target;
        }
        let pid_data = pid_data.as_mut().unwrap();
        process_pid_data(pid_data, target)
    }
}
