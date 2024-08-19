#pragma once

#include <cstdint>

/* *******
 * Types *
 *********/
namespace Robo::Steering {
    struct PIDData {
        float kp;
        float ki;
        float kd;
        std::int16_t prev_mv;
        std::int16_t prev_e;
        std::int16_t prev_prev_e;
    };

    struct Steering {
        std::int16_t max_speed;
    };
}
