#include "rust_compat.h"

void printk(uint32_t level, const char *msg)
{
    printk_u8(level, (uint8_t *)msg);
}

void vaprintk(uint32_t level, const char *format, ...)
{
    char buf[PRINTK_BUFFER_SIZE] = {0};
    va_list args;
    va_start(args, format);
    vsnprintf(buf, PRINTK_BUFFER_SIZE, format, args);
    printk_u8(level, (uint8_t *)buf);
    va_end(args);
}