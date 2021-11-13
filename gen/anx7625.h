#pragma once

/* Generated with cbindgen:0.20.0 */

/* Warning, this file is autogenerated by cbindgen. Don't modify this manually. */

#include <stdarg.h>
#include <stdbool.h>
#include <stdint.h>
#include <stdlib.h>

typedef struct Anx {
    void (*delay)(struct Anx *anx, void *delay, uint32_t ms);
    uint8_t (*set_video_on)(struct Anx *anx, bool state);
    uint8_t (*set_video_rst)(struct Anx *anx, bool state);
    uint8_t (*set_otg)(struct Anx *anx, bool state);
    uint8_t (*i2c_writeb)(struct Anx *anx, void *bus, uint8_t addr, const uint8_t *data, uintptr_t len);
    uint8_t (*i2c_readb)(struct Anx *anx, void *bus, uint8_t addr, uint8_t *data, uintptr_t *len);
} Anx;
