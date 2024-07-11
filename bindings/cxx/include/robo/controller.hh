#pragma once

#include <cstdint>

extern "C" {
    void *robo_create_sticks(std::int8_t lx, std::int8_t ly, std::int8_t rx, std::int8_t ry);

    void *robo_sticks_normalize(void *instance);

    void *robo_create_controller(std::uint8_t dead_zone);

    void *robo_controller_process_sticks(void *instance, void *sticks);
}

namespace Robo {
    namespace Sticks {
        inline void *create(std::int8_t lx, std::int8_t ly, std::int8_t rx, std::int8_t ry) {
            return robo_create_sticks(lx, ly, rx, ry);
        }

        inline void *normalize(void *instance) {
            return robo_sticks_normalize(instance);
        }
    }

    namespace Controller {
        inline void *create(std::uint8_t dead_zone) {
            return robo_create_controller(dead_zone);
        }

        inline void *process_sticks(void *instance, void *sticks) {
            return robo_controller_process_sticks(instance, sticks);
        }
    }
}
