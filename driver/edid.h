/*
 * This file is part of the coreboot project.
 *
 * Copyright 2013 Google Inc.
 *
 * This program is free software; you can redistribute it and/or modify
 * it under the terms of the GNU General Public License as published by
 * the Free Software Foundation; version 2 of the License.
 *
 * This program is distributed in the hope that it will be useful,
 * but WITHOUT ANY WARRANTY; without even the implied warranty of
 * MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
 * GNU General Public License for more details.
 */

#ifndef EDID_H
#define EDID_H

#include <stdint.h>
#include "coreboot_tables.h"
// #include <stdio.h>

#define u8 uint8_t
#define u16 uint16_t
#define u32 uint32_t
#define console_log_level(x) (1)
#define CONFIG(x) (0)
#define die(...)
// #include "helpers.h"
// #define MIN(x, y) min(x, y)
// #define MAX(x, y) max(x, y)

#ifndef MAX
#define MAX(a, b)                   \
    (                               \
        {                           \
            __typeof__(a) _a = (a); \
            __typeof__(b) _b = (b); \
            _a > _b ? _a : _b;      \
        })
#endif

#ifndef MIN
#define MIN(a, b)                   \
    (                               \
        {                           \
            __typeof__(a) _a = (a); \
            __typeof__(b) _b = (b); \
            _a < _b ? _a : _b;      \
        })
#endif

#include <stdbool.h>
// #include "api/Common.h"

enum edid_modes
{
    EDID_MODE_640x480_60Hz,
    EDID_MODE_720x480_60Hz,
    EDID_MODE_800x600_59Hz,
    EDID_MODE_1024x768_60Hz,
    EDID_MODE_1280x768_60Hz,
    EDID_MODE_1280x720_60Hz,
    EDID_MODE_1920x1080_60Hz,
    NUM_KNOWN_MODES,

    EDID_MODE_AUTO
};

struct edid_mode
{
    const int8_t *name;
    uint32_t pixel_clock;
    int32_t lvds_dual_channel;
    uint32_t refresh;
    uint32_t ha;
    uint32_t hbl;
    uint32_t hso;
    uint32_t hspw;
    uint32_t hborder;
    uint32_t va;
    uint32_t vbl;
    uint32_t vso;
    uint32_t vspw;
    uint32_t vborder;
    uint32_t voffset;
    uint8_t phsync;
    uint8_t pvsync;
    uint32_t x_mm;
    uint32_t y_mm;
};

/* structure for communicating EDID information from a raw EDID block to
 * higher level functions.
 * The size of the data types is not critical, so we leave them as
 * uint32_t. We can move more into into this struct as needed.
 */

#define EDID_ASCII_STRING_LENGTH 13

struct edid
{
    /* These next three things used to all be called bpp.
	 * Merriment ensued. The identifier
	 * 'bpp' is herewith banished from our
	 * Kingdom.
	 */
    /* How many bits in the framebuffer per pixel.
	 * Under all reasonable circumstances, it's 32.
	 */
    uint32_t framebuffer_bits_per_pixel;
    /* On the panel, how many bits per color?
	 * In almost all cases, it's 6 or 8.
	 * The standard allows for much more!
	 */
    uint32_t panel_bits_per_color;
    /* On the panel, how many bits per pixel.
	 * On Planet Earth, there are three colors
	 * per pixel, but this is convenient to have here
	 * instead of having 3*panel_bits_per_color
	 * all over the place.
	 */
    uint32_t panel_bits_per_pixel;
    /* used to compute timing for graphics chips. */
    struct edid_mode mode;
    u8 mode_is_supported[NUM_KNOWN_MODES];
    uint32_t link_clock;
    /* 3 variables needed for coreboot framebuffer.
	 * In most cases, they are the same as the ha
	 * and va variables, but not always, as in the
	 * case of a 1366 wide display.
	 */
    u32 x_resolution;
    u32 y_resolution;
    u32 bytes_per_line;

    int32_t hdmi_monitor_detected;
    int8_t ascii_string[EDID_ASCII_STRING_LENGTH + 1];
    int8_t manufacturer_name[3 + 1];
};

enum edid_status
{
    EDID_CONFORMANT,
    EDID_NOT_CONFORMANT,
    EDID_ABSENT,
};

/* Defined in edid.c */
int32_t decode_edid(uint8_t *edid, int32_t size, struct edid *out);
void edid_set_framebuffer_bits_per_pixel(struct edid *edid, int32_t fb_bpp, int32_t row_byte_alignment);
void set_vbe_mode_info_valid(const struct edid *edid, uintptr_t fb_addr);
void set_vbe_framebuffer_orientation(enum lb_fb_orientation orientation);
int32_t set_display_mode(struct edid *edid, enum edid_modes mode);

#endif /* EDID_H */
