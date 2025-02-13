use super::*;

#[no_mangle]
extern "C" fn robo_controller_normalize_sticks(sticks: Sticks) -> NormalizedSticks {
    normalize_sticks(sticks)
}

#[no_mangle]
extern "C" fn robo_controller_is_sticks_in_dead_zone(sticks: Sticks) -> *const bool {
    let results = is_sticks_in_dead_zone(sticks);
    results.as_ptr()
}

#[no_mangle]
extern "C" fn robo_controller_is_normalized_sticks_in_dead_zone(
    sticks: NormalizedSticks
) -> *const bool {
    let results = is_normalized_sticks_in_dead_zone(sticks);
    results.as_ptr()
}

#[no_mangle]
extern "C" fn robo_controller_process_sticks_dead_zone(sticks: Sticks) -> Sticks {
    process_sticks_dead_zone(sticks)
}

#[no_mangle]
extern "C" fn robo_controller_process_normalized_sticks_dead_zone(
    sticks: NormalizedSticks
) -> NormalizedSticks {
    process_normalized_sticks_dead_zone(sticks)
}
