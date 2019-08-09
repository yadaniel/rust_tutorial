#ifndef MAIN_INCLUDED
#define MAIN_INCLUDED

#include <stdint.h>

struct X {
    uint8_t x0;
    uint16_t x1;
    uint32_t x2;
    uint64_t x3;
};

extern uint8_t called_by_c(uint8_t, uint8_t);
extern void print_x(struct X *);

#endif

