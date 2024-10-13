#[cfg(feature = "bind-c")]
mod c {
    use super::super::*;

    #[no_mangle]
    extern "C" fn robo_encoder_update(encoder: Encoder, delta: i64) -> Encoder {
        update(encoder, delta)
    }

    #[no_mangle]
    extern "C" fn robo_encoder_get_degree(encoder: Encoder) -> f64 {
        get_degree(encoder)
    }

    #[no_mangle]
    extern "C" fn robo_encoder_get_radian(encoder: Encoder) -> f64 {
        get_radian(encoder)
    }

    #[no_mangle]
    extern "C" fn robo_encoder_get_revolution(encoder: Encoder) -> i64 {
        get_revolution(encoder)
    }
}
