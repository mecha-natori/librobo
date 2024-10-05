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
        float prev_e;
        float prev_ie;
        float now_out;
        float t;
    };

    struct Steering {
        std::int16_t max_speed;
    };
}

/* ***********
 * Functions *
 *************/
extern "C" {
    float robo_steering_process_pid_data(Robo::Steering::PIDData *pid_data, float target);
}

/* ********************
 * Namespace Mappings *
 **********************/
namespace Robo::Steering {
    inline float process_pid_data(PIDData *const pid_data, const float target) {
        return robo_steering_process_pid_data(pid_data, target);
    }
}
