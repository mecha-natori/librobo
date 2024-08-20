#pragma once

#include <stdint.h>

/* *******
 * Types *
 *********/
typedef struct {
    uint16_t min_ms;
    uint16_t max_ms;
    int16_t min_deg;
    int16_t max_deg;
} ServoDefinition;

/* ***********
 * Functions *
 *************/
uint16_t robo_servo_calc_servo_duty(int16_t deg, uint16_t max_duty, ServoDefinition servo);
