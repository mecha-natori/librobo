#pragma once

#include <complex.h>
#include <robo/controller.h>
#include <robo/steering.h>
#include <stdint.h>

/* ***********
 * Functions *
 *************/
int16_t *robo_steering_quad_omni_calc_speed(Steering steering, PIDData *pid_data, complex float l, complex float r);

int16_t *robo_steering_quad_omni_calc_speed_from_sticks(Steering steering, PIDData *pid_data, NormalizedSticks sticks);
