#pragme once

#include <stdint.h>

void *robo_create_encoder(uint16_t ppr);

void robo_encoder_update(void *instance, int64_t delta);

int64_t robo_encoder_get_count(void *instance);

double robo_encoder_get_degree(void *instance);

double robo_encoder_get_radian(void *instance);

int64_t robo_encoder_get_revolution(void *instance);
