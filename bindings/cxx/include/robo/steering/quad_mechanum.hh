#pragma once

#include <complex>
#include <cstdint>
#include <robo/controller.hh>
#include <robo/steering.hh>

/* ***********
 * Functions *
 *************/
extern "C" {
    std::int16_t *robo_steering_quad_mechanum_calc_speed(Robo::Steering::Steering steering, Robo::Steering::PIDData *pid_data, std::complex<float> l, std::complex<float> r);

    std::int16_t *robo_steering_quad_mechanum_calc_speed_from_sticks(Robo::Steering::Steering steering, Robo::Steering::PIDData *pid_data, Robo::Controller::NormalizedSticks sticks);
}

/* ********************
 * Namespace Mappings *
 **********************/
namespace Robo::Steering::QuadMechanum {
    inline std::int16_t *calc_speed(const Steering steering, PIDData *const pid_data, const std::complex<float> l, const std::complex<float> r) {
        return robo_steering_quad_mechanum_calc_speed(steering, pid_data, l, r);
    }

    inline std::int16_t *calc_speed_from_sticks(const Steering steering, PIDData *const pid_data, const Controller::NormalizedSticks sticks) {
        return robo_steering_quad_mechanum_calc_speed_from_sticks(steering, pid_data, sticks);
    }
}
