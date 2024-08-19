#pragma once

#include <cstdint>

/* *******
 * Types *
 *********/
namespace Robo::Controller {
    struct Sticks {
        std::int16_t l[2];
        std::int16_t r[2];
        std::uint8_t dead_zone;
    };

    struct NormalizedSticks {
        float l[2];
        float r[2];
        std::uint8_t dead_zone;
    };
}

/* ***********
 * Functions *
 *************/
extern "C" {
    Robo::Controller::NormalizedSticks robo_controller_normalize_sticks(Robo::Controller::Sticks sticks);

    bool *robo_controller_is_sticks_in_dead_zone(Robo::Controller::Sticks sticks);

    bool *robo_controller_is_normalized_sticks_in_dead_zone(Robo::Controller::NormalizedSticks sticks);
}

/* ********************
 * Namespace Mappings *
 **********************/
namespace Robo::Controller {
    inline NormalizedSticks normalize_sticks(Sticks sticks) {
        return robo_controller_normalize_sticks(sticks);
    }

    inline bool *is_sticks_in_dead_zone(Sticks sticks) {
        return robo_controller_is_sticks_in_dead_zone(sticks);
    }

    inline bool *is_normalized_sticks_in_dead_zone(NormalizedSticks sticks) {
        return robo_controller_is_normalized_sticks_in_dead_zone(sticks);
    }
}
