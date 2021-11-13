#include <stdint.h>
#include <stdlib.h>
#include <stdio.h>

#include "../gen/anx7625.h"

uint8_t anx_init(Anx *anx, void *bus, void *delay)
{
    uint8_t data[2];
    data[0] = 0x12;
    data[1] = 0x34;

    anx->delay(anx, delay, 1000);

    return anx->i2c_writeb(anx, bus, 0x08, (uint8_t *)data, 2);
}
