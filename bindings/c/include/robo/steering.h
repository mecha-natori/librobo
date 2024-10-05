#pragma once

#include <stdint.h>

/* *******
 * Types *
 *********/
typedef struct {
    float kp;
    float ki;
    float kd;
    float prev_e;
    float prev_ie;
    float now_out;
    float t;
} PIDData;

typedef struct {
    int16_t max_speed;
} Steering;

/* ***********
 * Functions *
 *************/
float robo_steering_process_pid_data(PIDData *pid_data, float target);
