#pragma once

#include <cstdint>

/* *******
 * Types *
 *********/
namespace Robo::Encoder {
    struct Encoder {
        std::uint16_t ppr;
        std::int64_t count;
    };
}

/* ***********
 * Functions *
 *************/
extern "C" {
    Robo::Encoder::Encoder robo_encoder_update(Robo::Encoder::Encoder encoder, std::int64_t delta);

    double robo_encoder_get_degree(Robo::Encoder::Encoder encoder);

    double robo_encoder_get_radian(Robo::Encoder::Encoder encoder);

    std::int64_t robo_encoder_get_revolution(Robo::Encoder::Encoder encoder);
}

/* ********************
 * Namespace Mappings *
 **********************/
namespace Robo::Encoder {
    inline Encoder update(Encoder encoder, std::int64_t delta) {
        return robo_encoder_update(encoder, delta);
    }

    inline double get_degree(Encoder encoder) {
        return robo_encoder_get_degree(encoder);
    }

    inline double get_radian(Encoder encoder) {
        return robo_encoder_get_radian(encoder);
    }

    inline std::int64_t get_revolution(Encoder encoder) {
        return robo_encoder_get_revolution(encoder);
    }
}
