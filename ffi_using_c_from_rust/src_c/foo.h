#ifndef FOO_INCLUDED
#define FOO_INCLUDED

#include <stdint.h>

struct CData {
    uint8_t  x0;
    uint16_t x1;
    uint32_t x2;
    uint64_t x3;
};

// void cdata_print(struct CData const * ptr);
void cdata_print(struct CData self);

#endif

