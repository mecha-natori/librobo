#pragma once

#include <stdint.h>

/* *******
 * Types *
 *********/
typedef struct {
    uint16_t ppr;
    int64_t count;
} Encoder;

/* ***********
 * Functions *
 *************/
Encoder robo_encoder_update(Encoder encoder, int64_t delta);

double robo_encoder_get_degree(Encoder encoder);

double robo_encoder_get_radian(Encoder encoder);

int64_t robo_encoder_get_revolution(Encoder encoder);
