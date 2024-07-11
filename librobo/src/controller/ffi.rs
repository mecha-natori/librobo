#![allow(clippy::missing_safety_doc, missing_docs)]

use super::*;

#[cfg(feature = "bind-c")]
mod c {
    use super::*;

    #[no_mangle]
    extern "C" fn robo_create_sticks(lx: i8, ly: i8, rx: i8, ry: i8) -> *const Sticks<i8> {
        let sticks = Sticks::new(lx, ly, rx, ry, Some(false));
        &sticks
    }

    #[no_mangle]
    unsafe extern "C" fn robo_sticks_normalize(instance: *const Sticks<i8>) -> *const Sticks<f32> {
        let instance = &*instance;
        let sticks = instance.normalize();
        &sticks
    }

    #[no_mangle]
    extern "C" fn robo_create_controller(dead_zone: u8) -> *const Controller<u8> {
        let ctrl = Controller::new(dead_zone);
        &ctrl
    }

    #[no_mangle]
    unsafe extern "C" fn robo_controller_process_sticks(instance: *const Controller<u8>, sticks: *const Sticks<i8>) -> *const Sticks<f32> {
        let instance = &*instance;
        let sticks = *sticks;
        let sticks = instance.process_sticks(sticks);
        &sticks
    }
}
