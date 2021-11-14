#pragma once

#include "../gen/anx7625.h"
#include "edid.h"

int32_t anx7625_init(AnxFnPtrs *anx, void *bus, void *delay, void *video_on, void *video_rst, void *otg_on);
void anx7625_wait_hpd_event(AnxFnPtrs *anx, void *bus, void *delay);
int32_t anx7625_dp_get_edid(AnxFnPtrs *anx, void *bus, void *delay, struct edid *out);
