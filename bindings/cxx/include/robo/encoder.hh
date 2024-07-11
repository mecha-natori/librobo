#pragme once

#include <cstdint>

extern "C" {
    void *robo_create_encoder(std::uint16_t ppr);

    void robo_encoder_update(void *instance, std::int64_t delta);

    std::int64_t robo_encoder_get_count(void *instance);

    double robo_encoder_get_degree(void *instance);

    double robo_encoder_get_radian(void *instance);

    std::int64_t robo_encoder_get_revolution(void *instance);
}

namespace Robo::Encoder {
    inline void *create(std::uint16_t ppr) {
        return robo_create_encoder(ppr);
    }

    inline void update(void *instance, std::int64_t delta) {
        return robo_encoder_update(instance, delta);
    }

    inline std::int64_t get_count(void *instance) {
        return robo_encoder_get_count(instance);
    }

    inline double get_degree(void *instance) {
        return robo_encoder_get_degree(instance);
    }

    inline double get_radian(void *instance) {
        return robo_encoder_get_radian(instance);
    }

    inline std::int64_t get_revolution(void *instance) {
        return robo_encoder_get_revolution(instance);
    }
}
