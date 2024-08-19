#pragma once

#include <stdint.h>

/* *******
 * Types *
 *********/
typedef struct {
    float kp;
    float ki;
    float kd;
    int16_t prev_mv;
    int16_t prev_e;
    int16_t prev_prev_e;
} PIDData;

typedef struct {
    int16_t max_speed;
} Steering;
