#include <stdio.h>
#include "foo.h"

/* void cdata_print(struct CData const * ptr) { */
/*     printf("CData.u8=%u", ptr->u8); */
/*     printf("CData.u16=%u", ptr->u16); */
/*     printf("CData.u32=%u", ptr->u32); */
/* } */

void cdata_print(struct CData self) {
    printf("CData.x0/u8=%u",  self.x0);
    printf("CData.x1/u16=%u", self.x1);
    printf("CData.x2/u32=%u", self.x2);
    printf("CData.x3/u64=%llu", self.x3);
}
