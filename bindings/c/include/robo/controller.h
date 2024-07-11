#pragma once

#include <stdint.h>

void *robo_create_sticks(int8_t lx, int8_t ly, int8_t rx, int8_t ry);

void *robo_sticks_normalize(void *instance);

void *robo_create_controller(uint8_t dead_zone);

void *robo_controller_process_sticks(void *instance, void *sticks);
