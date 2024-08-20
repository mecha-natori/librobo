#pragma once

#include <cstdint>

/* *******
 * Types *
 *********/
namespace Robo::Servo {
    struct ServoDefinition {
        std::uint16_t min_ms;
        std::uint16_t max_ms;
        std::int16_t min_deg;
        std::int16_t max_deg;
    };
}

/* ***********
 * Functions *
 *************/
extern "C" {
    std::uint16_t robo_servo_calc_servo_duty(std::int16_t deg, std::uint16_t max_duty, Robo::Servo::ServoDefinition servo);
}


/* ********************
 * Namespace Mappings *
 **********************/
namespace Robo::Servo {
    inline std::uint16_t calc_servo_duty(std::int16_t deg, std::uint16_t max_duty, ServoDefinition servo) {
        return robo_servo_calc_servo_duty(deg, max_duty, servo);
    }
}
