use super::*;

#[no_mangle]
unsafe extern "C" fn robo_steering_process_pid_data(pid_data: *mut PIDData, target: f32) -> f32 {
    if pid_data.is_null() {
        return target;
    }
    let Some(pid_data) = pid_data.as_mut() else {
        unreachable!()
    };
    process_pid_data(pid_data, target)
}
