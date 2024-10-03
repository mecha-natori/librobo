#pragma once

#include <stdbool.h>
#include <stdint.h>

/* *******
 * Types *
 *********/
typedef struct {
    int16_t l[2];
    int16_t r[2];
    uint8_t dead_zone;
} Sticks;

typedef struct {
    float l[2];
    float r[2];
    uint8_t dead_zone;
} NormalizedSticks;

/* ***********
 * Functions *
 *************/
NormalizedSticks robo_controller_normalize_sticks(Sticks sticks);

bool *robo_controller_is_sticks_in_dead_zone(Sticks sticks);

bool *robo_controller_is_normalized_sticks_in_dead_zone(NormalizedSticks sticks);

Sticks robo_controller_process_sticks_dead_zone(Sticks sticks);

NormalizedSticks robo_controller_process_normalized_sticks_dead_zone(NormalizedSticks sticks);
