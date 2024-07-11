#![allow(clippy::missing_safety_doc, missing_docs)]

use super::*;

#[cfg(feature = "bind-c")]
mod c {
    use super::*;

    #[no_mangle]
    extern "C" fn robo_create_encoder(ppr: u16) -> *const Encoder {
        let encoder = Encoder::new(ppr);
        &encoder
    }

    #[no_mangle]
    unsafe extern "C" fn robo_encoder_update(instance: *mut Encoder, delta: i64) {
        let instance = &mut *instance;
        instance.update(delta);
    }

    #[no_mangle]
    unsafe extern "C" fn robo_encoder_get_count(instance: *const Encoder) -> i64 {
        let instance = &*instance;
        instance.get_count()
    }
    
    #[no_mangle]
    unsafe extern "C" fn robo_encoder_get_degree(instance: *const Encoder) -> f64 {
        let instance = &*instance;
        instance.get_degree()
    }

    #[no_mangle]
    unsafe extern "C" fn robo_encoder_get_radian(instance: *const Encoder) -> f64 {
        let instance = &*instance;
        instance.get_radian()
    }

    #[no_mangle]
    unsafe extern "C" fn robo_encoder_get_revolution(instance: *const Encoder) -> i64 {
        let instance = &*instance;
        instance.get_revolution()
    }
}
