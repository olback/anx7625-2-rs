#pragma once

#include <stdint.h>
#include <stdarg.h>
#include <stdio.h>
#include <string.h>
#include "../gen/anx7625.h"

#define ANXERROR(format, ...) \
    vaprintk(BIOS_ERR, "ERROR: %s: " format, __func__, ##__VA_ARGS__)
#define ANXINFO(format, ...) \
    vaprintk(BIOS_INFO, "%s: " format, __func__, ##__VA_ARGS__)
#define ANXDEBUG(format, ...) \
    vaprintk(BIOS_DEBUG, "%s: " format, __func__, ##__VA_ARGS__)

void printk(uint32_t level, const char *msg);
void vaprintk(uint32_t level, const char *format, ...);
